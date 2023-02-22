use std::{ops::Index, str::FromStr};

use clap::Parser;
use reqwest::{Client, Url};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Mangadex url for a specific manga
    #[arg(long, short)]
    url: String,
    // url: Option<String>,

    // /// UUID found in the link of a manga's page on mangadex
    // #[arg(long)]
    // uuid: Option<String>,
}

fn validate_url(url: &String) -> Result<String, ()> {
    if !url.starts_with("https://mangadex.org/title/") {
        eprintln!("Invalid url");
        return Err(());
    }

    let uuid = url.split("/").collect::<Vec<&str>>().index(4).to_string();

    Ok(uuid)
    // (true, "".to_string())
    // todo!()
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = Args::parse();

    // println!("Hello, world!");
    let client = Client::new();

    // let url = match &args.uuid {
    //     Some(uuid) => format!("https://api.mangadex.org/manga/{}", uuid),
    //     None => args.url.unwrap()
    // };
    // https://mangadex.org/title/4535092c-36d9-49ce-982e-ad827d35a238/diana-is-a-strange-mermaid
    // let url = args.url;
    // let res = client.get("https://api.mangadex.org/manga/{}").send().await.unwrap();

    let uuid = match validate_url(&args.url) {
        Ok(uuid) => uuid,
        Err(_) => std::process::exit(1),
    };
    let mut url = Url::from_str(format!("https://api.mangadex.org/manga/{}", uuid).as_str()).unwrap();
    url.set_query(Some("order[chapter]=asc&order[volume]=asc&limit=500&translatedLanguage[]=en"));
    // url.set_query(Some("order[volume]=asc"));
    // url.set_query(Some("limit=500"));
    // url.set_query(Some("translatedLanguage[]=en"));
    dbg!(uuid, &url);

    let res = client.get(url).send().await.unwrap().text().await.unwrap();
    println!("{:#?}", res);

    Ok(())
}
