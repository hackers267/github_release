use std::fs;

extern crate serde_json;
extern crate tinyget;

fn main() {
    let filename = "repos.json";
    let urls:Vec<String> = get_urls(filename);
    for url in urls.iter() {
        println!("{}",url);
    }
    let mut url_iter = urls.iter();
    let url = url_iter.next().unwrap();
    let response =match tinyget::get(url).send(){
        Ok(response) => response,
        Err(err) => {
            println!("Network error: {}",&err);
            std::process::exit(1)
        }
    };
    let str = response.as_bytes();
    let result:serde_json::Value = serde_json::from_slice(str).unwrap();
    println!("{:?}",result);

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
