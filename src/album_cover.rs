use rspotify::{AuthCodeSpotify};
use rspotify_model::{PlayableItem, FullTrack};
use crate::popular_endpoints;

// clean up this import
use image::{*};

// TEMPORARY FEATURE!!!!
use show_image::{ImageView, ImageInfo, create_window};

// downloads album art for song
pub async fn download_album_art(song_playing: FullTrack) {
    // get the url for the image
    let image = song_playing.album.images;
    let im1 = image.get(0).unwrap().url.clone();

    let img_bytes = reqwest::get(&im1).await.unwrap().bytes().await.unwrap();
    let image = load_from_memory(&img_bytes).unwrap();

    let file_name = format!("ImageAssets/{}.png", song_playing.id.unwrap().to_string());
    image.save_with_format(file_name, image::ImageFormat::Png);
}

