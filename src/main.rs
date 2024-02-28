// This starter uses the `axum` crate to create an asyncrohnous web server
// The async runtime being used, is `tokio`
// This starter also has logging, powered by `tracing` and `tracing-subscriber`
mod crawler;
mod error;
mod nlpcut;
mod search;
use axum::routing::post;
use axum::{
    extract::Query, extract::State, http::StatusCode, response::IntoResponse, routing::get, Json,
    Router,
};
use deadpool_diesel::{Manager, Pool};
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use rust_starter::Result;
use serde_derive::{Deserialize, Serialize};
use std::net::SocketAddr;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{doc, Index, IndexWriter};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

// normally part of your generated schema.rs file
table! {
    docs (id) {
        id -> Integer,
        title -> VarChar,
        url -> VarChar,
        content -> Text,
        doc_type -> Text,
       published -> Nullable<Bool>,
    }
}

#[derive(serde::Serialize, Selectable, Queryable)]
struct Doc {
    id: i32,
    title: String,
    url: String,
    content: String,
    doc_type: String,
    published: Option<bool>,
}

#[derive(serde::Deserialize, Insertable)]
#[diesel(table_name = docs)]
struct NewDoc {
    title: String,
    url: String,
    content: String,
    doc_type: String,
    published: Option<bool>,
}

#[derive(Deserialize, Serialize)]
struct SearchQuery {
    keyword: String,
    offset: usize,
}

#[derive(Deserialize, Serialize)]
struct InsertDoc {
    title: String,
    doc: String,
    url: String,
    id: usize,
}

#[derive(Clone)]
struct AppState {
    index: tantivy::Index,
    feild: (Field, Field, Field, Field),
    pgpool: Pool<Manager<PgConnection>>,
}

// This derive macro allows our main function to run asyncrohnous code. Without it, the main function would run syncrohnously
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let mut schema_builder = Schema::builder();

    let text_field_indexing = TextFieldIndexing::default()
        .set_tokenizer("jieba")
        .set_index_option(IndexRecordOption::WithFreqsAndPositions);
    let text_options = TextOptions::default()
        .set_indexing_options(text_field_indexing)
        .set_stored();

    let title = schema_builder.add_text_field("title", text_options.clone());

    let body = schema_builder.add_text_field("body", text_options.clone());
    let id = schema_builder.add_u64_field("idstr", INDEXED);
    let url = schema_builder.add_text_field("url", text_options);

    let schema = schema_builder.build();

    let index_path = tempfile::TempDir::new().unwrap();
    let index = Index::create_in_dir(&index_path, schema.clone()).unwrap();

    let tokenizer = tantivy_jieba::JiebaTokenizer {};
    index.tokenizers().register("jieba", tokenizer);

    let db_url = std::env::var("DATABASE_URL").unwrap();

    // set up connection pool
    let manager = deadpool_diesel::postgres::Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);
    let pgpool = deadpool_diesel::postgres::Pool::builder(manager)
        .build()
        .unwrap();

    // run the migrations on server startup
    {
        let conn = pgpool.get().await.unwrap();
        conn.interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
            .await
            .unwrap()
            .unwrap();
    }
    let state = AppState {
        index,
        feild: (title, body, id, url),
        pgpool,
    };
    let app = Router::new()
        .route("/", get(root))
        .route("/search", get(search))
        .route("/insert", post(insert))
        .route("/delete", get(delete))
        .route("/feed", get(feed))
        .route("/insert_doc", post(insert_doc))
        .with_state(state);

    let port: u16 = std::env::var("PORT")
        .unwrap_or("3000".into())
        .parse()
        .expect("failed to convert to number");
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("listening on {}", addr);

    // Then, we run the server, using the `bind` method on `Server`
    // `axum::Server` is a re-export of `hyper::Server`
    axum::Server::bind(&addr)
        // We then convert our Router into a `Service`, provided by `tower`
        .serve(app.into_make_service())
        // This function is async, so we need to await it
        .await
        .unwrap();
}

// This is our route handler, for the route root
// Make sure the function is `async`
// We specify our return type, `&'static str`, however a route handler can return anything that implements `IntoResponse`

async fn root() -> &'static str {
    "Hello, World!"
}

// This is our route handler, for the route complex
// Make sure the function is async
// We specify our return type, this time using `impl IntoResponse`

async fn delete() -> impl IntoResponse {
    // For this route, we are going to return a Json response
    // We create a tuple, with the first parameter being a `StatusCode`
    // Our second parameter, is the response body, which in this example is a `Json` instance
    // We construct data for the `Json` struct using the `serde_json::json!` macro
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "message": "delete, delete!"
        })),
    )
}

async fn insert(
    State(state): State<AppState>,
    Json(doc): Json<InsertDoc>,
) -> Result<impl IntoResponse> {
    // For this route, we are going to return a Json response
    // We create a tuple, with the first parameter being a `StatusCode`
    // Our second parameter, is the response body, which in this example is a `Json` instance
    // We construct data for the `Json` struct using the `serde_json::json!` macro
    let mut index_writer: IndexWriter = state.index.writer(50_000_000)?;
    let (title, body, id, url) = state.feild;
    index_writer.add_document(doc!(
        title => doc.title.as_str(),
        id => doc.id as u64,
        body => doc.doc.as_str(),
        url => doc.url.as_str(),
    ))?;
    index_writer.commit()?;
    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "id": query.id,
            "title": query.title,
            "doc": query.doc,
            "message": "insert, insert!"
        })),
    ))
}

async fn search(
    query: Query<SearchQuery>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    let reader = state.index.reader()?;
    let (title, body, _, url) = state.feild;
    let searcher = reader.searcher();
    let query_parser = QueryParser::for_index(&state.index, vec![title, body, url]);

    let tquery = query_parser.parse_query(query.keyword.as_str())?;

    let top_docs = searcher
        .search(&tquery, &TopDocs::with_limit(10))
        .unwrap_or_default();
    let mut res: Vec<Vec<FieldValue>> = Vec::new();
    for (_, doc_address) in top_docs {
        let retrieved_doc = searcher.doc(doc_address).unwrap_or_default();
        res.push(retrieved_doc.field_values().to_owned());
    }
    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "query": query.keyword,
            "offset": query.offset,
            "res": res,
            "message": "ok"
        })),
    ))
}

async fn feed(State(state): State<AppState>) -> Result<impl IntoResponse> {
    let conn = state.pgpool.get().await?;
    let res = conn
        .interact(|conn| docs::table.select(Doc::as_select()).load(conn))
        .await??;
    Ok(Json(res))
}

async fn insert_doc(State(state): State<AppState>, Json(doc): Json<NewDoc>) -> Result<Json<Doc>> {
    let conn = state.pgpool.get().await?;
    let res = conn
        .interact(|conn| {
            diesel::insert_into(docs::table)
                .values(doc)
                .returning(Doc::as_returning())
                .get_result(conn)
        })
        .await??;
    Ok(Json(res))
}

#[cfg(test)]
mod tests {
    use crate::search::engine::exc_search;

    #[test]
    pub fn test_search() -> Result<(), ()> {
        exc_search();
        Ok(())
    }
}
