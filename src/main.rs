mod mangadex_feed;
use mangadex_feed::Feed;

use std::{ops::Index, str::FromStr};

use clap::Parser;
use reqwest::{Client, Url};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Mangadex url for a specific manga
    #[arg(long, short)]
    url: String,
}

fn validate_url(url: &String) -> Result<String, ()> {
    if !url.starts_with("https://mangadex.org/title/") {
        eprintln!("Invalid url");
        return Err(());
    }
    let uuid = url.split("/").collect::<Vec<&str>>().index(4).to_string();

    Ok(uuid)
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = Args::parse();
    let client = Client::new();

    // Get uuid from arguments. Exit program if url wasnt valid
    let uuid = match validate_url(&args.url) {
        Ok(uuid) => uuid,
        Err(_) => std::process::exit(1),
    };
    let url_str = format!("https://api.mangadex.org/manga/{}/feed", uuid);
    let mut url = Url::from_str(url_str.as_str()).unwrap();
    url.set_query(Some(
        "order[chapter]=asc&order[volume]=asc&limit=500&translatedLanguage[]=en",
    ));

    // #[cfg(debug_assertions)]
    // dbg!(uuid, url.clone());

    let res = client.get(url).send().await.unwrap().text().await.unwrap();
    // println!("{:#?}", res);
    let json: Feed = serde_json::from_str(res.as_str()).unwrap();
    // println!("{:#?}", json);

    Ok(())
}
