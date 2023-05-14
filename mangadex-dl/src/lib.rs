#![allow(dead_code)]

const BASE_URL: &str = "https://api.mangadex.org/";

use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache};
use reqwest::{header::HeaderValue, Client, Url};
use reqwest_middleware::{ClientBuilder, RequestBuilder};

mod response_schema;
use response_schema::{AtHomeServer, Feed};

fn get_client(path: String, cache_mode: Option<CacheMode>) -> RequestBuilder {
    // Header
    let mut headers = reqwest::header::HeaderMap::new();
    headers.append("accept", HeaderValue::from_static("application/json"));

    // Build the request url
    let mut url = Url::try_from(format!("{BASE_URL}/{path}").as_str()).unwrap();
    url.set_query(Some(
        "order[chapter]=asc&order[volume]=asc&limit=500&translatedLanguage[]=en",
    ));

    let client = ClientBuilder::new(Client::new())
        .with(Cache(HttpCache {
            mode: cache_mode.unwrap_or(CacheMode::Default),
            manager: CACacheManager::default(),
            options: None,
        }))
        .build();

    client.get(url).headers(headers)
}

#[derive(Debug)]
pub struct Info<T> {
    data: T,
}

#[derive(Debug)]
pub struct Downloader;

impl Downloader {
    pub async fn get_manga_info(uuid: Option<&str>) -> Result<Info<Feed>, reqwest::Error> {
        // Build the request client and send a request
        let path = match uuid {
            Some(uuid) => format!("manga/{uuid}/feed"),
            None => "manga/e78a489b-6632-4d61-b00b-5206f5b8b22b/feed".to_owned(),
        };

        let client = get_client(path, None);
        let res: Feed = client.send().await.unwrap().json().await.unwrap();

        Ok(Info { data: res })
    }

    async fn get_server(uuid: String) -> Result<String, reqwest::Error> {
        let client = get_client(format!("at-home/server/{uuid}"), Some(CacheMode::NoCache));
        let res = client.send().await.unwrap().text().await.unwrap();
        Ok(res)
    }

    pub async fn download_chapters(info: Info<Feed>) -> Result<(), reqwest::Error> {
        for (_, chapter) in info.data.data.into_iter().enumerate() {
            let pages = chapter.attributes.as_ref().unwrap().pages.unwrap();
            let uuid = chapter.id.unwrap();

            if pages == 0 {
                println!("Skipping {uuid}, due to the chapter having no pages");
                continue;
            }

            // match Downloader::get_server(uuid.clone()).await {
            //     Ok(text) => {
            //         let json: Option<AtHomeServer> = match serde_json::from_str(text.as_str()) {
            //             Ok(json) => json,
            //             Err(_) => {
            //                 println!("{}", text);
            //                 None
            //             }
            //         };
            //         let _ = std::fs::write(format!("./chapters/{uuid}"), serde_json::to_string(&json).unwrap());
            //     }
            //     Err(err) => println!("Error occurred while downloading chapter [{uuid}]: \n {err:#?}"),
            // };
            // println!("{} {} {}", i, uuid, pages)
        }

        Ok(())
    }
}
