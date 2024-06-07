use rustube::{Id, Video};
use std::ffi::{CStr, CString};
use std::fs;
use std::os::raw::c_char;

mod thumbnail;

pub async fn download_video(url: &str, dest: &str) -> String {

    let metadata_dir_dest = fs::metadata(dest).unwrap();
    if !metadata_dir_dest.is_dir() {
        return   String::from("Directory not exists!");
    } 

    let id = Id::from_raw(url).unwrap();
    let video = Video::from_id(id.into_owned()).await.unwrap();

    log::debug!("Initializing. Download  .. ");
    let path_video = video
        .best_quality()
        .unwrap()
        .download_to_dir(dest)
        .await
        .unwrap();
    log::debug!("Finishing Download \n");

    log::debug!("Generating Thumbnail... \n");
    let filename =  path_video.as_path().file_name().unwrap().to_str().unwrap();
    let path = format!("{}{}", dest, filename);
    print!("Path: {}\n", path);
    thumbnail::generate_thumbnail(&path, &path.replace(".mp4", ".jpg"));
    log::debug!("Thumbnail Generated! \n");

    return path;
}

pub async extern "C" fn download_video_dart(url: *const c_char, dest: *const c_char) -> CString {
    let url_str = unsafe { CStr::from_ptr(url).to_str().unwrap() };
    let dest_str = unsafe { CStr::from_ptr(dest).to_str().unwrap() };

    let path_url = download_video(url_str, dest_str).await;
    return CString::new(path_url).unwrap();
}
