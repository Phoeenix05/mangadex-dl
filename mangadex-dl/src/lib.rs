#![allow(dead_code)]

#[derive(Debug)]
pub struct Info<T> {
    data: T,
}

#[derive(Debug)]
pub struct Downloader;

impl Downloader {
    pub async fn get_info(query: Option<&str>) -> Result<Info<String>, reqwest::Error> {
        // Header
        use reqwest::header::HeaderValue;

        let mut headers = reqwest::header::HeaderMap::new();
        headers.append("accept", HeaderValue::from_static("application/json"));

        // Build the request url
        use reqwest::Url;

        let query = query.unwrap_or("c288b108-5162-4065-aa3a-5857ec38c8d9");
        let mut url = Url::try_from(format!("https://api.mangadex.org/manga/{query}/feed").as_str()).unwrap();
        url.set_query(Some(
            "order[chapter]=asc&order[volume]=asc&limit=500&translatedLanguage[]=en",
        ));

        // Build the request client and send a request
        use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache};
        use reqwest::Client;
        use reqwest_middleware::ClientBuilder;

        let client = ClientBuilder::new(Client::new())
            .with(Cache(HttpCache {
                mode: CacheMode::Default,
                manager: CACacheManager::default(),
                options: None,
            }))
            .build();
        let req = client.get(url).headers(headers);
        dbg!(&req);
        let res = req.send().await.unwrap().text().await.unwrap();

        // Convert String to Mangadex Feed.

        Ok(Info { data: res.to_string() })
    }

    pub async fn download_images(_info: Info<String>) -> Result<(), reqwest::Error> {
        Ok(())
    }
}
