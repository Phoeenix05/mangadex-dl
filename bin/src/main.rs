use mangadex_dl_lib::*;

#[tokio::main]
async fn main() {
    let info = Downloader::get_manga_info(Some("e78a489b-6632-4d61-b00b-5206f5b8b22b"))
        .await
        .unwrap();
    let _ = Downloader::download_chapters(info).await.unwrap();
}
