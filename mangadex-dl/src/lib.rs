#![allow(dead_code)]

use std::thread::Thread;

use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache};
use reqwest::{header::HeaderValue, Client, Url};
use reqwest_middleware::ClientBuilder;

mod response_schema;
use response_schema::Feed;

#[derive(Debug)]
pub struct Info<T> {
    data: T,
}

#[derive(Debug)]
pub struct Downloader;

impl Downloader {
    pub async fn get_info(query: Option<&str>) -> Result<Info<Feed>, reqwest::Error> {
        // Header
        let mut headers = reqwest::header::HeaderMap::new();
        headers.append("accept", HeaderValue::from_static("application/json"));

        // Build the request url
        let query = query.unwrap_or("c288b108-5162-4065-aa3a-5857ec38c8d9");
        let mut url = Url::try_from(format!("https://api.mangadex.org/manga/{query}/feed").as_str()).unwrap();
        url.set_query(Some(
            "order[chapter]=asc&order[volume]=asc&limit=500&translatedLanguage[]=en",
        ));

        // Build the request client and send a request
        let client = ClientBuilder::new(Client::new())
            .with(Cache(HttpCache {
                mode: CacheMode::Default,
                manager: CACacheManager::default(),
                options: None,
            }))
            .build();
        let req = client.get(url).headers(headers);
        dbg!(&req);
        let res: Feed = req.send().await.unwrap().json().await.unwrap();

        // Convert String to Mangadex Feed.

        Ok(Info { data: res })
    }

    pub async fn download_images(_info: Info<Feed>) -> Result<(), reqwest::Error> {
        let mut download_processes: Vec<Thread> = Vec::new();

        Ok(())
    }
}
