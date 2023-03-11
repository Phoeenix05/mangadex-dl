use std::io::Write;

use crate::api::AtHomeServer;

use reqwest::Client;

use super::get_path;

pub async fn dl_resource(name: String, id: String) -> Result<(), reqwest::Error> {
    let client = Client::new();
    let url = format!("https://api.mangadex.org/at-home/server/{}", id);
    // let response = reqwest::blocking::get(url).unwrap().text().unwrap();
    let response = client.get(url).send().await?.text().await?;
    let json: AtHomeServer = serde_json::from_str(response.as_str()).unwrap();

    std::fs::create_dir(get_path(format!("{name}/{id}").as_str())).unwrap();

    for image in json.chapter.data.iter() {
        // println!("{}/{}", json.base_url, image)
        let url = format!("{}/{}", json.base_url, image);
        let bytes = client.get(url).send().await?.text().await?;
        let file_path = get_path(format!("{name}/{id}/{image}").as_str());
        let mut file = std::fs::File::create(file_path).unwrap();
        file.write(&bytes.as_bytes()).unwrap();
    }

    Ok(())
}
