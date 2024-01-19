use std::process::Command;

use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::blocking::Client;

pub fn parse_main_page(path: &str) -> Vec<String> {
    let str = get_page_from_path(path);
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new("<a href=\"(.*?)/\">").unwrap());
    RE.captures_iter(&str).map(|c| c.extract()).map(|(_, [path])| {
        path.to_string()
    }).collect()
}

pub fn parse_rsync_size(path: &str) -> u64 {

    static RE: Lazy<Regex> = Lazy::new(|| Regex::new("Total file size: (.*?) bytes").unwrap());
    //println!("{path}");
    let output = Command::new("rsync").args(["--info=stats2", "-r", "--exclude=*-debug", path]).output().unwrap();
    //let err = String::from_utf8(output.stderr).unwrap();
    let output = String::from_utf8(output.stdout).unwrap();
    //println!("Output: {output}\nStderr: {err}");
    let size = RE.captures(&output).unwrap().get(1).unwrap().as_str();
    //println!("{size}");
    size.replace(',', "").parse().unwrap()

}

pub fn get_page_from_path(path: &str) -> String {

    static MAIN_PATH: &str = "https://mirrors.edge.kernel.org/";
    static CLIENT: Lazy<Client> = Lazy::new(|| {
        reqwest::blocking::ClientBuilder::new()
            .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/117.0")
            .build().unwrap()
    });
    CLIENT.get(format!("{MAIN_PATH}{path}")).send().unwrap().text().unwrap()

}
