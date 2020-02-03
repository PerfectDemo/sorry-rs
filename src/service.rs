use std::path::{ Path, PathBuf };
use std::io;
use std::fs::{ read_dir, File, canonicalize };
use std::io::Read;
use std::process::Command;

use serde_json::{ Result as JsonResult, Value, to_string as json_to_string };
use md5::compute;
use handlebars::Handlebars;

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



pub fn make(item: &MakeItem) -> Result<PathBuf, Box<dyn std::error::Error>> {
    println!("name: {:?}", item.name);
    println!("sentence: {:?}", item.sentences);
    let hashes = json_to_string(&item).unwrap();
    let md5_str = format!("{:x}", compute(hashes));

    let mut filename = String::new();
    filename.push_str(&item.name);
    filename.push_str("-");
    filename.push_str(&md5_str);
    filename.push_str(".gif");

    println!("filename: {:?}", filename);
    println!("name: {:?}", item.name);
    println!("sentence: {:?}", item.sentences);
    let ass_path = render_ass(&md5_str, item).unwrap();
    println!("ass path: {:?}", ass_path);
    let git_path = render_gif(&ass_path, &filename, &item.name)?;
    Ok(git_path)
}

fn render_gif(ass_path: &PathBuf, filename: &str, name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut gif_path = PathBuf::from(TMP_DIR);
    gif_path.push(filename);

    let mut video_path = PathBuf::from(RESOURCE_DIR);
    video_path.push(name);
    video_path.push("template.mp4");

    println!("ass path: {:?}; video_path: {:?}, git_path: {:?}", ass_path, video_path, gif_path);
    
    // var cmd = "ffmpeg -i " + videoPath + " -r 8 -vf ass=." + assPath + ",scale=300:-1 -y " + gifPath
    let mut ass_params = String::from("ass=");
    ass_params.push_str(&ass_path.to_str().unwrap());
    ass_params.push_str(",scale=300:-1");
    let ouput = Command::new("ffmpeg")
                    .arg("-i")
                    .arg(&video_path)
                    .arg("-r")
                    .arg("8")
                    .arg("-vf")
                    .arg(ass_params)
                    .arg(&gif_path)
                    .output()
                    .expect("failed to execute process");
    Ok(gif_path)
}

fn render_ass(md5_str: &str,  item: &MakeItem) ->  Result<PathBuf, Box<dyn std::error::Error>> {
    let mut out_put_file = PathBuf::from(TMP_DIR);
    let mut ass_file_name = String::from(md5_str);
    ass_file_name.push_str(&".ass");
    out_put_file.push(ass_file_name);
    let mut ass_file = File::create(&out_put_file)?;

    let mut template_file = PathBuf::from(RESOURCE_DIR);
    template_file.push(&item.name);
    template_file.push("template.hbs");
    let mut source_template = File::open(template_file)?;
    let handlebars = Handlebars::new();
    handlebars.render_template_source_to_write(&mut source_template, &item, &mut ass_file)?;
    Ok(out_put_file)
}

fn make_mpeg() {

}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_render_ass() {
        let item = MakeItem {
            name: String::from("jinkela"),
            sentences: vec![String::from("1"), String::from("2")]
        };
        let result = render_ass(&"afiejfajfjalefjailefjal", &item).unwrap();
        println!("ass result: {:?}", "tmp/afiejfajfjalefjailefjal.ass");
    }
}