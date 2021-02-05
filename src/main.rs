extern crate serde_json;
extern crate tinyget;

use std::fs;

use serde::Deserialize;
use tinyget::Error;

#[derive(Deserialize, Debug)]
struct Info {
    id: u32,
    name: String,
    body: String,
    published_at: String,
}

fn main() {
    let filename = "repos.json";
    let urls: Vec<String> = get_urls(filename);
    for url in urls.iter() {
        println!("{}", url);
    }
    let mut url_iter = urls.iter();
    let url = url_iter.next().unwrap();
    get_data(url);
}


fn get_data(url: &String) -> Vec<Info> {
    let response = tinyget::get(url).with_header("User-Agent", "github_release").send().expect("some error");
    let body_str = response.as_str().unwrap();
    let json: Vec<Info> = serde_json::from_str(body_str).unwrap();
    return json;
}

fn get_urls(filename: &str) -> Vec<String> {
    let file = fs::File::open(filename).unwrap();
    let v: serde_json::Value = serde_json::from_reader(file).unwrap();
    let mut urls: Vec<String> = Vec::new();
    for a in v.as_array() {
        for b in a {
            let own = "https://api.github.com/repos/".to_owned();
            let owner = b["owner"].as_str().unwrap();
            let repos = b["repos"].as_str().unwrap();
            let url = own + owner + "/" + repos + "/releases";
            urls.push(url);
        }
    }
    urls
}
