use std::path::{ Path, PathBuf };
use std::io;
use std::fs::{ read_dir, File };
use std::io::Read;

use serde_json::{ Result as JsonResult, Value, to_string as json_to_string };
use md5::compute;

const RESOURCE_DIR: &str = "resource";
const SENTENCE_FILE: &str = "sentence.json";
const TMP_DIR: &str = "tmp";

use crate::controller::MakeItem;

pub fn get_configs() -> Vec<PathBuf> {
    let path = Path::new(RESOURCE_DIR);

    let entires = read_dir(path).unwrap()
        .map(| res | 
            res.map( | e | e.path()
                            .strip_prefix(RESOURCE_DIR).unwrap()
                            .to_path_buf()
                    )
            )
        .collect::<Result<Vec<_>, io::Error>>().unwrap();
    
    entires
}

pub fn get_config(config: &str) -> std::io::Result<String> {
    let mut file_path = PathBuf::new();
    file_path.push("resource");
    file_path.push(config);
    file_path.push(SENTENCE_FILE);

    let file = File::open(file_path)?;
    let mut buf_reader = io::BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}



pub fn make(item: &MakeItem) {
    let mut hashes = json_to_string(&item).unwrap();
    let md5_str = compute(hashes);
    let mut filename = String::new();
    filename.push_str(&item.name);
    filename.push_str("-");
    filename.push_str(&format!("{:x}", md5_str));
    filename.push_str(".gif");

    println!("filename: {:?}", filename);
    println!("name: {:?}", item.name);
    println!("sentence: {:?}", item.sentence);
}

fn render_gif() {

}

fn render_ass() {

}

fn make_mpeg() {

}