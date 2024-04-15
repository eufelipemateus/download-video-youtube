use rustube::{Id, Video};

pub async fn download_video(url: &str, dest: &str) -> String {
    let id = Id::from_raw(url).unwrap();

    let video = Video::from_id(id.into_owned()).await.unwrap();
    print!("Initializing... ");

    let path_video = video
        .best_quality()
        .unwrap()
        .download_to_dir(dest)
        .await
        .unwrap();

    print!("Finishing Download \n");
    let path = format!("{:?}/{:?}", dest, path_video.as_path().file_name().unwrap());
    return path;
}



#[no_mangle]
pub async extern "C" fn  download_video_dart(url: &str, dest: &str) -> String{   
     let path_url = download_video(url, dest).await;

     return  path_url;
}