use mangadex_dl_lib::*;

#[tokio::main]
async fn main() {
    let info = Downloader::get_info(Some("c288b108-5162-4065-aa3a-5857ec38c8d9"))
        .await
        .unwrap();
    dbg!(&info);
    let _ = Downloader::download_images(info).await.unwrap();
}
