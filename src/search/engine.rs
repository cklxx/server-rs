use crate::crawler;
use crate::nlpcut::stopwords::STOPWORDS_CMN;
use jieba_rs::Jieba;
use std::collections::HashMap;
pub fn exc_search() {
    let data = crawler::read_file_data();

    let mut store: HashMap<i32, String> = HashMap::new();
    let mut terms: HashMap<String, Vec<i32>> = HashMap::new();

    let mut abs_index = 0;
    let jieba = Jieba::new();
    'absloop: for (ads, _abs1) in data {
        store.insert(abs_index, ads.clone());
        let words = jieba.cut(ads.as_str(), false);
        for w in words.clone() {
            let tf: f32 = words.iter().filter(|&word| word == &w).count() as f32
                / words.iter().count() as f32;

            println!("{} word tf {}", abs_index, tf);
            if STOPWORDS_CMN.contains(&w) {
                continue;
            }
            let v: Vec<i32> = Vec::new();
            let mut abs_indexs = terms.get(w).unwrap_or_else(|| &v).clone();
            if abs_indexs.contains(&abs_index) {
                continue;
            }
            abs_indexs.push(abs_index);
            terms.insert(w.to_string(), abs_indexs.to_owned());
        }

        abs_index += 1;
        if abs_index > 10000 {
            break 'absloop;
        }
    }
    let mut res = Vec::new();
    let mut res_indexs = Vec::new();

    for search_term in jieba.cut("谢娜", false) {
        let search_res = terms.get(search_term);
        println!("search xiena: {:?}", search_res);
        match search_res {
            Some(searchs) => {
                for indexs in searchs {
                    if !res_indexs.contains(&indexs) {
                        res_indexs.push(indexs);
                        match store.get(indexs) {
                            Some(v) => {
                                if !res.contains(&v) {
                                    res.push(v)
                                }
                            }
                            None => {}
                        }
                    }
                }
            }
            None => {}
        }
    }
    println!("search res: {:?}", res);
}
