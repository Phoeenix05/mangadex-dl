#![allow(dead_code)]

#[derive(Debug)]
pub enum DownloadStatus {
    Ok,
    Timeout,
    Failed,
}

#[derive(Debug)]
pub struct Info<T> {
    data: T,
}

#[derive(Debug)]
pub struct ImageDownload {
    status: DownloadStatus,
    url: String,
    path: String,
}

#[derive(Debug)]
pub struct Downloader {
    query: String,
}

impl Downloader {
    pub async fn get_info(&self) -> Result<Info<String>, reqwest::Error> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.append(
            "accept",
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        let url_str = format!(
            "https://api.mangadex.org/manga/{}/feed",
            self.query.as_str()
        );
        let mut url = reqwest::Url::parse(&url_str).unwrap();
        url.set_query(Some(
            "order[chapter]=asc&order[volume]=asc&limit=500&translatedLanguage[]=en",
        ));
        // dbg!(&url);

        let client = reqwest::Client::new();

        let req = client.get(url).headers(headers);
        dbg!(&req);

        let res = req.send().await.unwrap().text().await.unwrap();
        // Convert String to Mangadex Feed.

        Ok(Info {
            data: res.to_string(),
        })
    }

    pub async fn download_images(
        &self,
        _info: Info<String>,
    ) -> Result<Vec<ImageDownload>, reqwest::Error> {
        Ok(Vec::new())
    }
}

#[derive(Debug, Default)]
pub struct DownloaderBuilder {
    query: Option<String>,
}

impl DownloaderBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_query(mut self, query: String) -> Self {
        self.query = Some(query);
        self
    }

    pub fn build(self) -> Downloader {
        Downloader {
            query: self
                .query
                .unwrap_or("c288b108-5162-4065-aa3a-5857ec38c8d9".to_string()),
        }
    }
}
