-- Your SQL goes here
CREATE TABLE docs (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  content TEXT NOT NULL,
  doc_type VARCHAR NOT NULL,
  published BOOLEAN NOT NULL DEFAULT FALSE
)
