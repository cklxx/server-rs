// # Defining a tokenizer pipeline
//
// In this example, we'll see how to define a tokenizer
// by creating a custom `NgramTokenizer`.
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{doc, Index, IndexWriter};

fn main() -> tantivy::Result<()> {
    // # Defining the schema
    //
    // The Tantivy index requires a very strict schema.
    // The schema declares which fields are in the index,
    // and for each field, its type and "the way it should
    // be indexed".

    // first we need to define a schema ...
    let mut schema_builder = Schema::builder();

    // Our first field is title.
    // In this example we want to use NGram searching
    // we will set that to 3 characters, so any three
    // char in the title should be findable.
    let text_field_indexing = TextFieldIndexing::default()
        .set_tokenizer("jieba")
        .set_index_option(IndexRecordOption::WithFreqsAndPositions);
    let text_options = TextOptions::default()
        .set_indexing_options(text_field_indexing)
        .set_stored();

    let title = schema_builder.add_text_field("title", text_options.clone());

    // Our second field is body.
    // We want full-text search for it, but we do not
    // need to be able to be able to retrieve it
    // for our application.
    //
    // We can make our index lighter by omitting the `STORED` flag.
    let body = schema_builder.add_text_field("body", text_options);
    let idstr = schema_builder.add_text_field("idstr", TEXT);

    let schema = schema_builder.build();

    // # Indexing documents
    //
    // Let's create a brand new index.
    // To simplify we will work entirely in RAM.
    // This is not what you want in reality, but it is very useful
    // for your unit tests... Or this example.
    let index = Index::create_in_ram(schema.clone());

    let tokenizer = tantivy_jieba::JiebaTokenizer {};
    // here we are registering our custom tokenizer
    // this will store tokens of 3 characters each
    index.tokenizers().register("jieba", tokenizer);

    // To insert document we need an index writer.
    // There must be only one writer at a time.
    // This single `IndexWriter` is already
    // multithreaded.
    //
    // Here we use a buffer of 50MB per thread. Using a bigger
    // memory arena for the indexer can increase its throughput.
    let mut index_writer: IndexWriter = index.writer(50_000_000)?;
    index_writer.add_document(doc!(
        title => "长相思引言",
        idstr => "1",
        body => "宇宙混沌，鸿蒙初开，盘古大帝劈开了天地。
        那时候，神族、人族、妖族混居于天地之间。天与地的距离并非遥不可及，人居于陆地，神居于神山，人可以通过天梯见神。
        盘古大帝有三位情如兄妹的下属，神力最高的是一位女子，因年代过于久远，名字已不可考，只知道她后来建立了华胥国 ，后世尊称她为华胥氏。另外两位是男子：神农氏，驻守中原，守四方安宁；高辛氏，驻守东方，守护日出之地汤谷 和万水之眼归墟 。
        盘古大帝仙逝后，天下战火频起，华胥氏厌倦了无休无止的战争，避世远走，创建了美丽祥和的华胥国。可她之所以被后世铭记，并不是因为华胥国，而是因为她的儿子伏羲、女儿女娲。
        伏羲、女娲恩威并重，令天下英雄敬服，制止了兵戈之争。伤痕累累的大荒迎来太平，渐渐恢复了生机。
        伏羲、女娲被尊为伏羲大帝、女娲大帝。
        伏羲大帝仙逝后，女娲大帝悲痛不已，避居华胥国，从此再没有人见过她，生死成谜，伏羲女娲一族日渐没落。
        大荒的西北，一个小神族——轩辕族，在他们年轻首领的带领下正在慢慢崛起。几千年之后，轩辕族已经可以和古老的神农、高辛两族抗衡。
        中原的神农、东南的高辛、西北的轩辕，三大神族，三分天下。
        神农炎帝遍尝百草，以身试药，为世人解除疾苦，受万民爱戴，被天下人尊为医祖。因为神农炎帝，大荒形成了三足鼎立的局面。
        神农炎帝的逝世打破了三足鼎立的局面，轩辕黄帝雄才伟略，经过和神农族的激烈斗争，统一了中原。
        统一并不是斗争的结束，而是另一种斗争的开始。
        神农、轩辕两个部族经过痛苦的斗争，逐渐能和平相处，可一切的矛盾犹如休眠的火山，随时会爆发。"
    ))?;
    index_writer.add_document(doc!(
        title => "洛阳伽蓝记",
        idstr => "2",
        body => r#"
        永宁寺，熙平元年，灵太后胡氏所立也，在宫前阊阖门南一里御道西。其寺东有太尉府，西对永康里，南界昭玄曹，北邻御史台。

        阊阖门前御道东有左卫府，府南有司徒府。司徒府南有国子学，堂内有孔丘像，颜渊问仁、子路问政在侧。国子南有宗正寺，寺南有太庙，庙南有护军府，府南有衣冠里。御道西有右卫府，府南有太尉府，府南有将作曹，曹南有九级府，府南有太社，社南有凌阴里，即四朝时藏冰处也。

        中有九层浮图一所，架木为之，举高九十丈。上有金刹，复高十丈，合去地一千尺。去京师百里，已遥见之。初掘基至黄泉下，得金像三十躯，太后以为信法之征，是以营建过度也。刹上有金宝瓶，容二十五斛。宝瓶下有承露金盘一十一重，周匝皆垂金铎。复有铁锁四道，引刹向浮图四角，锁上亦有金铎。铎大小如一石瓮子。浮图有九级，角角皆悬金铎，合上下有一百三十铎。浮图有四面，面有三户六窗，并皆朱漆。扉上有五行金铃，合有五千四百枚。复有金环铺首，殚土木之功，穷造形之巧，佛事精妙，不可思议。绣柱金铺，骇人心目。至於高风永夜，宝铎和鸣，铿锵之声，闻及十馀里。

        浮图北有佛殿一所，形如太极殿。中有丈八金像一躯、中长金像十躯，绣珠像三躯，金织成像五躯，玉像二躯。作工奇巧，冠於当世。僧房楼观，一千馀间，雕梁粉壁，青琐绮疏，难得而言。栝柏椿松，扶疏檐霤，丛竹香草，布护阶墀。是以常景碑云：“须弥宝殿，兜率净宫，莫尚於斯也。”1外国所献经像皆在此寺。"#
    ))?;
    index_writer.add_document(doc!(
        title => "最后一片叶",
        idstr => "3",
        body => r#"“哦，我可从来没听过这种无稽之谈，”休极其不满地奚落道，“那老藤叶和你健康的恢复有什么关系呢？你过去一直很喜欢那株藤树，你这淘气的姑娘，别犯傻了。对了，今早大夫告诉我，你很快就会康复的——让我想想他到底是怎么说的——他说希望有百分之九十!啊，那就是说康复的可能性几乎与我们在纽约搭街车或是走过一幢新建筑物一样。来喝点儿肉汤吧，让苏迪回去作她的画吧，这样才能卖给那些编辑，来给她生病的孩子买葡萄酒，也给贪吃的自己买点儿猪排。”
            　　“你没必要再买什么酒了，”琼珊说，眼睛定定地看着窗外，“又落了一片，不，我也不要什么肉汤，叶子只剩四片了。我想天黑前看到最后一片叶子落下来，那时，我也该走了。”
            　　“琼珊，亲爱的，”休俯下身说，“拜托你在我画完前闭上眼睛，不要看窗外，好不好？那些画我明天必须得交。要不是需要光线，我早就把窗帘拉上了。”
            　　“你不能到另一间屋子里去画吗?”琼珊冷冷地问。
            　　“我宁愿呆在你这儿，”休说，“再说，我也不想你老盯着那些无聊的藤叶。”
            　　“你一画完就告诉我。”琼珊闭上眼睛躺了下来。她面色苍白，一动不动，像一尊倒下的雕塑，“因为我想看到最后一片藤叶落下。我累了，不想再等了，也不愿再想了，我想摆脱一切，像那可怜的、疲惫的藤叶一样慢悠悠地飘下去，飘下去。”
            　　“赶紧睡吧，”休说，“我得把贝尔曼叫上来，让他给我当那个隐居老矿工的模特，一分钟之内我是回不来的，我回来之前别乱动。”"#
    ))?;
    index_writer.commit()?;

    let reader = index.reader()?;
    let searcher = reader.searcher();

    // The query parser can interpret human queries.
    // Here, if the user does not specify which
    // field they want to search, tantivy will search
    // in both title and body.
    let query_parser = QueryParser::for_index(&index, vec![title, body]);
    // here we want to get a hit on the 'ken' in Frankenstein
    let query = query_parser.parse_query("一")?;

    let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;

    for (_, doc_address) in top_docs {
        let retrieved_doc = searcher.doc(doc_address)?;

        println!("{:?} ", retrieved_doc.field_values());
    }

    Ok(())
}
