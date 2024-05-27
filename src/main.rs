use std::{
    collections::HashMap,
    error::Error,
    sync::{Arc, Mutex},
};

use reqwest::get;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let links = ["www.google.com", "www.jacobzim.com", "www.recurse.com"];
    let content_cache = Arc::new(Mutex::new(HashMap::new()));

    for link in links {
        add_uri_to_cache(&(String::from("https://") + link), content_cache.clone())
            .await
            .unwrap();
    }

    println!("{:?}", content_cache);
    Ok(())
}

// return true iff in cache
async fn add_uri_to_cache(
    uri: &str,
    cache: Arc<Mutex<HashMap<String, String>>>,
) -> Result<bool, reqwest::Error> {
    let resp = get(uri).await?;
    let text = resp.text().await?;
    {
        let mut internal_cache_locked = cache.lock().unwrap();
        internal_cache_locked.insert(uri.to_string(), text);
    }
    Ok(true)
}
