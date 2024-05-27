use pear_search::lib::print_hello_world;
use serde_json;
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

fn main() {
    // let webpage = "https://totbwf.github.io/posts/tactic-haskell.html";
    let bookmarks = "bookmarks-2024-05-27.json";
    // let bookmarks_text = fs::read_to_string(bookmarks).unwrap();
    let bm_json = get_bookmark_json(bookmarks).unwrap();
    println!("{}", bm_json);
    print_hello_world();
}

fn get_bookmark_json<P: AsRef<Path>>(path: P) -> Result<Value, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let u = serde_json::from_reader(reader)?;

    Ok(u)
}
