// threading used in some functions
use std::{fs, thread, future};

// Types for Parameters
use rspotify_model::{FullTrack, PlayHistory};

// imports for image manipulation and saving
// clean up this import
use image::{Rgba, ImageFormat, load_from_memory, codecs::gif, Frame, Delay, ImageBuffer, Rgb};

// import configuration settings
use crate::configuration::{get_image_dir, get_config_value, get_assets_dir};

// gets the links from a track
pub fn get_image_links_from_track(track: &FullTrack) -> Vec<String> {
    let mut url_vec: Vec<String> = vec![];
    // get the url for the proper image
    let image_urls = &track.album.images;
    for im in image_urls {
        url_vec.push(im.url.clone()); 
    }
    url_vec
}

// downloads album art for song
pub async fn download_album_art(image_urls: Vec<String>, image_quality: i32) {
    // ensure that quality is within range
    if image_quality < 0 || image_quality > 2 {
        panic!("Quality rating for download_album_art must be 0, 1, or 2");
    }
    
    // get the url for the proper image
    let image_url = image_urls.get(image_quality as usize).expect("Failed access URL in vector");

    // send a request to download the image
    let image_request_bytes = reqwest::get(image_url).await.expect("Failed to download").bytes().await.unwrap();
    let image = load_from_memory(&image_request_bytes).expect("Failed to load image from memory");

    // create base_path
    let base_path = format!("{}{}", get_config_value("assets_path"), "Images/");
    // 25+
    let file_name = &image_url[25..]; 
    // create the file name
    let full_file_name = format!("{base_path}{file_name}.png");
    println!("Saving image as {}", &file_name);

    // handle file errors and save the image
    match image.save_with_format(full_file_name, image::ImageFormat::Png) {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    };
}

// concurrently downloads album art
pub async fn download_album_art_from_track_vec(spotify_tracks: &Vec<FullTrack>, quality: i32) {
    let mut vec_of_joins = vec![];
    for track in spotify_tracks {
        // spawn a green thread that downloads the data
        let handle = tokio::spawn(download_album_art(get_image_links_from_track(&track), quality));

        // push the joinhandle to the vector of joinhandles to be later awaited
        vec_of_joins.push(handle);
    }
    
    // await all of the futures and return when finished 
    futures::future::join_all(vec_of_joins).await;
}

// deletes all the art work in image assets folder
pub fn clear_image_assets() {
    let im_dir = get_image_dir();
    for entry in fs::read_dir(im_dir).unwrap() {
        match fs::remove_file(entry.unwrap().path()) {
            Ok(_) => (),
            Err(e) => eprintln!("Unable to clear images: {}", e),
        };
    }
}

pub fn get_stored_images() -> Vec<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let image_dir = get_image_dir();
    let mut vec_of_images = vec![];
    for item in std::fs::read_dir(image_dir).expect("Unable to read dir.") {
        let image = image::open(item.unwrap().path()).unwrap().to_rgba8();
        vec_of_images.push(image);
    }
    vec_of_images
}

fn color_distance() {
    /* compute the color distance
    *
    * */
}

pub fn create_gif_of_all(file_name: &str) {
    const DELAY: u32 = 250;

    // create file_path with current time
    let gif_path: String = format!("{}Gifs/{}.gif", get_assets_dir(), file_name);

    // create the gif file
    let image = std::fs::File::create(gif_path).expect("Unable to create gif file");

    // create gif encoder and set to infinite repeat
    let mut gif_enc = gif::GifEncoder::new_with_speed(image, 3);
    gif_enc.set_repeat(gif::Repeat::Infinite);

    let stored_images = get_stored_images();
    println!("Retrieved stored images!");
    for image in stored_images {
        let frame = Frame::from_parts(image, 0u32, 0u32, Delay::from_numer_denom_ms(DELAY, 1));

        gif_enc.encode_frame(frame);
    }
}

pub fn background_from_library() {
    let items = std::fs::read_dir("./Assets/Images/").expect("Unable to read dir.");
    let width = (64 * 13) as u32;
    let height = (64 * 8) as u32;
    // let mut im: ImageBuffer<_, Vec<_>> = ImageBuffer::new(width, height);
}
