use download_video_yt::download_video;

#[tokio::main(flavor = "current_thread")]
async fn main() {
  let path_url = download_video(
                "https://www.youtube.com/watch?v=EmH9ltj5KJE",
                "/home/felipemateus/Documentos/download-video-youtube/output/",
            )
            .await;

    print!("Path: {}\n", path_url);
}
