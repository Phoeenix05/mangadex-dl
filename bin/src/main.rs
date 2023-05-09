use mangadex_dl_lib::*;

#[tokio::main]
async fn main() {
    let downloader = DownloaderBuilder::new()
        // .with_query("Druid of Seoul Station".to_string())
        .build();

    let info = downloader.get_info().await.unwrap();
    dbg!(&info);
    let _ = downloader.download_images(info).await.unwrap();
}
