use std::fs;
use std::io::{self, Read};
use std::path::Path;
use walkdir::WalkDir;

fn read_text_file(file_path: &Path) -> io::Result<String> {
    let mut file = fs::File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn read_file_data<'a>() -> Vec<(String, String)> {
    let folder_path = Path::new("./data");
    let mut res = Vec::new();

    for entry in WalkDir::new(folder_path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let file_path = entry.path();
            if let Some(extension) = file_path.extension() {
                if extension == "txt" {
                    match read_text_file(&file_path) {
                        Ok(content) => {
                            println!("Content of {}:", file_path.display());

                            let contents: Vec<&str> = content.split("\n").collect();

                            // 输出每个部分
                            for item in contents {
                                let rawabs: Vec<&str> = item.split("_!_").collect();
                                if rawabs.len() > 2 {
                                    res.push((
                                        rawabs[rawabs.len() - 2].to_owned(),
                                        rawabs[rawabs.len() - 1].to_owned(),
                                    ));
                                }
                            }
                        }
                        Err(err) => {
                            eprintln!("Error reading {}: {}", file_path.display(), err);
                        }
                    }
                }
            }
        }
    }
    res
}
