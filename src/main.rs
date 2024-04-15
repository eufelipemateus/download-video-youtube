use mylib::download_video;


#[tokio::main(flavor = "current_thread")]
async fn main(){
   let path_url = download_video("https://www.youtube.com/watch?v=0hI0_fIcF3Y", "/home/felipemateus/Documentos/download-video-youtube/output").await;
   

}
