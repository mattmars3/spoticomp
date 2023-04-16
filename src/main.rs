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
    
    let rec_played = recently_played::get_all_recent_songs(&spotify).await;
    album_cover::download_album_art_from_vec(rec_played, 0).await;
}

async fn image_test(spotify_object: &AuthCodeSpotify) {
    let song = popular_endpoints::current_playing_fulltrack(&spotify_object).await.expect("No song playing");
    // 0 lowest, 2 best
    album_cover::download_album_art(song, 1).await;
    // album_cover::clear_image_assets();
}

async fn download_hella_pictures(spotify_object: &AuthCodeSpotify) {
    for song in recently_played::get_all_recent_songs(spotify_object).await {
        album_cover::download_album_art(song.track, 0).await;
    }
}

async fn get_rec_played_count(spotify_object: &AuthCodeSpotify, print_songs: bool) {
    let songs = recently_played::get_all_recent_songs(spotify_object).await;
    if print_songs {
        for song in &songs {
            println!("{}", song.track.name);
        }
    }
    println!("{} total songs.", songs.len());
}
