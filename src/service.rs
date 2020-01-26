use std::path::{ Path, PathBuf };
use std::io;
use std::fs;

pub fn get_configs() -> Vec<std::path::PathBuf> {
    let path = Path::new("resource/");

    let entires = fs::read_dir(path).unwrap()
        .map(| res | 
            res.map( | e | e.path()
                            .strip_prefix("resource").unwrap()
                            .to_path_buf()
                    )
            )
        .collect::<Result<Vec<_>, io::Error>>().unwrap();
    entires
}