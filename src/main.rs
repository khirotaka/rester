mod io_tool;
mod github;

use clap::{App, Arg};


fn main() {
    let matches = App::new("rester")
        .version("0.1.0")
        .arg(Arg::new("dir")
            .short('d')
            .long("dir")
            .value_name("TARGET_PATH")
            .required(true))
        .arg(Arg::new("tag")
            .short('t')
            .long("tag")
            .value_name("TAG")
            .required(true))
        .arg(Arg::new("config")
            .long("config")
            .value_name("FILE")
            .required(true))
        .get_matches();

    let config = io_tool::load_yaml(matches.value_of("config").unwrap());
    let message: String = String::from("This is Test.");
    github::post_issue(
        &config[0]["repository"]["owner"].as_str().unwrap(),
        &config[0]["repository"]["name"].as_str().unwrap(),
        1,
        &message
    );
    /*
    let files = io_tool::fetch_filenames(
        &matches.value_of("dir").unwrap().to_string(),
        io_tool::fetch_target_filetype(&config[0]["filetype"]).as_str()
    );
    io_tool::create_archive(&files);

    github::post_issue();
     */
}
