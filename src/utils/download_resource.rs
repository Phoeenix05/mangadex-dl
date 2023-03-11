use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};

use crate::api::AtHomeServer;
use reqwest::{header::CONTENT_TYPE, Error};
use std::thread;

pub fn dl_resource(chapter_id: String) -> Result<(), Error> {
    // Initialize request client and url
    let url = format!("https://api.mangadex.org/at-home/server/{}", &chapter_id);

    let response = reqwest::blocking::get(url)?.text()?;
    let json: AtHomeServer = serde_json::from_str(response.as_str()).unwrap();

    // Get the image server base url
    let server_url = Arc::new(Mutex::new(json.base_url.to_string()));

    for image in json.chapter.data {
        let server_url_clone = Arc::clone(&server_url);

        thread::spawn(move || {
            let guard = server_url_clone.lock().unwrap();
            let url = format!("{}/{}", guard, image);
            println!("{}", url)
            // let client = Client::new();
            // let response = reqwest::blocking::get(url).unwrap().bytes().unwrap();
            
        });
        // t.join().unwrap();
    }

    // let content_type = response.headers().get(CONTENT_TYPE).and_then(|value| value.to_str().ok());
    // if let Some(content_type) = content_type {
    //     if content_type.starts_with("image/") {
    //     }
    // }

    Ok(())
}
