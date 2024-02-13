use std::process::Command;

use once_cell::sync::Lazy;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use regex::Regex;
use reqwest::blocking::Client;

use crate::types::{LibraryItem, Document, DownloadType::Rsync};

pub fn parse_mirror_source(http_root: &str, rsync_root: &str, main_page: &str, exclude_str: &str) -> Vec<LibraryItem> {

    // TODO: Make exclude str optional
    let results: Vec<(String, String, u64)> = parse_main_page(http_root, main_page)
        .into_par_iter()
        .filter(|path| {
            !path.trim().is_empty() && !path.trim().eq("..") && !path.trim().ends_with(exclude_str)
        })
        .map(|path| {
            let url = format!("{rsync_root}{main_page}/{path}/");
            let size = parse_rsync_size(&url, exclude_str);
            (path, url, size)
        })
        .collect();

    results
        .into_iter()
        .map(|(name, url, size)| LibraryItem::Document(Document::new(name, url, size, Rsync)))
        .collect()

}

pub fn parse_main_page(root: &str, path: &str) -> Vec<String> {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new("<a href=\"(.*?)/\">").unwrap());
    let str = get_page_from_path(root, path);
    RE.captures_iter(&str)
        .map(|c| c.extract())
        .map(|(_, [path])| path.to_string())
        .collect()
}

pub fn parse_rsync_size(path: &str, exclude_str: &str) -> u64 {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new("Total file size: (.*?) bytes").unwrap());
    //println!("{path}");
    let output = Command::new("rsync")
        .args(["--info=stats2", "-r", &format!("--exclude=*{exclude_str}"), path])
        .output()
        .unwrap();
    //let err = String::from_utf8(output.stderr).unwrap();
    let output = String::from_utf8(output.stdout).unwrap();
    //println!("Output: {output}\nStderr: {err}");
    let size = RE.captures(&output).unwrap().get(1).unwrap().as_str();
    //println!("{size}");
    size.replace(',', "").parse().unwrap()
}

pub fn get_page_from_path(root: &str, path: &str) -> String {
    static CLIENT: Lazy<Client> = Lazy::new(|| {
        reqwest::blocking::ClientBuilder::new()
            .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/117.0")
            .build()
            .unwrap()
    });
    CLIENT
        .get(format!("{root}{path}"))
        .send()
        .unwrap()
        .text()
        .unwrap()
}
