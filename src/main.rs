use rspotify::{AuthCodeSpotify, prelude::*};


use rspotify_model::{FullTrack, PlayHistory};
// for main function
use tokio::main;

// get rid of this

mod authorize;
mod recently_played;
mod popular_endpoints;
mod album_cover;
mod lyrics;
mod ui;

#[tokio::main]
async fn main() {
    let spotify = authorize::get_authcode().await;
    // let song = popular_endpoints::current_playing_fulltrack(&spotify).await.expect("No song playing");
    // album_cover::download_album_art(song).await;
    // album_cover::clear_image_assets();

    let last_songs = recently_played::get_all_recently_played(&spotify).await;
    recently_played::write_json(recently_played::playhistory_to_json(last_songs).await);
    
    let songs: Vec<PlayHistory> = recently_played::read_songs_from_recent_file();
    for song in songs {
        println!("{}", song.track.name);
    }

     

}


