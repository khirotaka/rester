extern crate yaml_rust;
extern crate tar;
extern crate flate2;

use yaml_rust::{YamlLoader};
use regex::Regex;
use self::yaml_rust::Yaml;
use flate2::Compression;
use flate2::write::GzEncoder;


pub fn load_yaml(path: &str) -> Vec<yaml_rust::Yaml> {
    let content: String = match std::fs::read_to_string(path) {
        Ok(file) => file.to_string(),
        Err(error) => panic!("Error Opening File Failed: {}", error)
    };

    let config = match YamlLoader::load_from_str(&content) {
        Ok(config) => config,
        Err(error) => panic!("Failed to read the yaml file: {}", error)
    };

    config
}


pub fn fetch_target_filetype(yaml: &Yaml) -> String {
    let mut parsed: Vec<String> = Vec::new();

    for filetype in yaml.as_vec().unwrap() {
        parsed.push(String::from(filetype.as_str().unwrap()))
    }

    let patterns: Vec<String> = parsed.iter().map(|x| format!(r"{}$", x)).collect();
    let patterns: String = patterns.join("|");
    patterns
}


fn dfs(path: &String) -> Vec<String> {
    let paths = match std::fs::read_dir(path) {
        Ok(paths) => paths,
        Err(error) => panic!("Failed: {}", error)
    };
    let mut target_paths: Vec<String> = Vec::new();

    for path in paths {
        let path = match path {
            Ok(path) => path,
            Err(e) => panic!("Error: {}", e)
        };

        if path.path().as_path().is_dir() {
            target_paths.extend(dfs(&path.path().to_str().unwrap().to_string()));
        }
        else {
            let tmp = path
                .path()
                .to_str()
                .unwrap()
                .to_string();
            target_paths.push(tmp);
        }
    }

    target_paths
}


pub fn fetch_filenames(dir_path: &String, patten: &str) -> Vec<String> {
    let paths = dfs(dir_path);
    let re = Regex::new(patten).unwrap();

    let tmp = paths.into_iter().filter(|x| re.is_match(x)).collect();
    tmp
}


pub fn create_archive(files: &Vec<String>) {
    let tar_gz = match std::fs::File::create("archive.tar.gz") {
        Ok(file) => file,
        Err(e) => panic!("Error: {}", e)
    };

    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut archive = tar::Builder::new(enc);
    for file in files {
        match archive.append_path(file) {
            Ok(_) => "",
            Err(msg) => panic!("Error: {}", msg)
        };
    }
}
