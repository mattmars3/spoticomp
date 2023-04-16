use std::{fs, thread::JoinHandle};

use rspotify::{AuthCodeSpotify};
use rspotify_model::{PlayableItem, FullTrack, PlayHistory};
use crate::popular_endpoints;

use std::thread;

// clean up this import
use image::{ImageFormat, load_from_memory, codecs::gif, Frame, Delay, ImageBuffer};

const IMAGE_DIR: &str = "./Assets/Images/";

// downloads album art for song
pub async fn download_album_art(spotify_track: FullTrack, quality: i32) {
    if quality < 0 || quality > 2 {
        panic!("Quality rating for download_album_art must be 0, 1, or 2");
    }
    
    // get the url for the proper image
    let images = spotify_track.album.images;
    let im1 = images.get(quality as usize).unwrap().url.clone();

    // send a request to download the image
    let img_bytes = reqwest::get(&im1).await.unwrap().bytes().await.unwrap();
    let image = load_from_memory(&img_bytes).unwrap();

    // create the file name
    let file_name = format!("{}{}.png", IMAGE_DIR, spotify_track.id.unwrap().to_string());

    // handle file errors
    match image.save_with_format(file_name, image::ImageFormat::Png) {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    };
}

pub fn background_from_library() {
    let items = std::fs::read_dir("./Assets/Images/").expect("Unable to read dir.");
    let width = (64 * 13) as u32;
    let height = (64 * 8) as u32;
    // let mut im: ImageBuffer<_, Vec<_>> = ImageBuffer::new(width, height);
}

pub async fn download_album_art_from_vec(spotify_tracks: Vec<PlayHistory>, quality: i32) {
    let mut thread_vec: Vec<JoinHandle<Result<(), ()>>> = vec![];
    for play_history in spotify_tracks {
        let handle = thread::spawn(|| {
            download_album_art(play_history.track, 0)
        });
    }
}

// deletes all the art work in image assets folder
pub fn clear_image_assets() {
    for entry in fs::read_dir(IMAGE_DIR).unwrap() {
        fs::remove_file(entry.unwrap().path());
    }
}


pub fn create_gif_of_all() {
    const DELAY: u32 = 250;

    let gif_path = "./Assets/Gifs/one.gif";
    let image = std::fs::File::create(gif_path).unwrap();
    let mut gif_enc = gif::GifEncoder::new_with_speed(image, 3);
    gif_enc.set_repeat(gif::Repeat::Infinite);

    for item in std::fs::read_dir("./Assets/Images/").expect("Unable to read dir.") {
        let image = image::open(item.unwrap().path()).unwrap().to_rgba8();
        let frame = Frame::from_parts(image, 0u32, 0u32, Delay::from_numer_denom_ms(DELAY, 1));

        gif_enc.encode_frame(frame);

    }
}
