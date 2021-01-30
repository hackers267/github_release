use std::fs;

extern crate serde_json;

fn main() {
    let filename = "repos.json";
    let urls:Vec<String> = get_urls(filename);
    for url in urls.iter() {
        println!("{}",url);
    }
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
