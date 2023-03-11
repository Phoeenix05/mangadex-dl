mod api;
mod utils;

use api::Feed;
use utils::{dl_resource, get_path};

use std::{ops::Index, str::FromStr};

use clap::Parser;
use reqwest::{Client, Url};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Mangadex url for a specific manga
    #[arg(long, short)]
    url: String,
    // all: Option<bool>
}

fn validate_url(url: &String) -> Result<(String, String), ()> {
    if !url.starts_with("https://mangadex.org/title/") {
        eprintln!("Invalid url");
        return Err(());
    }

    let parts = url.split("/").collect::<Vec<&str>>();
    let uuid = parts.index(4).to_string();
    let name = parts.index(5).to_string();

    Ok((name, uuid))
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = Args::parse();
    let client = Client::new();

    // println!("{:#?}", args);

    // Get uuid from arguments. Exit program if url wasnt valid
    let (name, uuid) = match validate_url(&args.url) {
        Ok(uuid) => uuid,
        Err(_) => std::process::exit(1),
    };
    let url_str = format!("https://api.mangadex.org/manga/{}/feed", uuid);
    let mut url = Url::from_str(url_str.as_str()).unwrap();
    url.set_query(Some(
        "order[chapter]=asc&order[volume]=asc&limit=500&translatedLanguage[]=en",
    ));

    let res = client.get(url).send().await.unwrap().text().await.unwrap();
    let json: Feed = serde_json::from_str(res.as_str()).unwrap();

    std::fs::create_dir_all(get_path(&name)).unwrap();
    // match std::fs::create_dir_all(format!("/Users/antonfredriksson/Desktop/mangadex-dl/{}", name)) {
    //     Ok(_) => println!("created dir"),
    //     Err(_) => eprintln!("error"),
    // };

    for chapter in json.data[0..10].iter() {
        println!("Downloading chapter - {:?}", chapter.attributes.title);
        dl_resource(name.clone(), chapter.id.clone()).await.unwrap();
    }

    Ok(())
}
