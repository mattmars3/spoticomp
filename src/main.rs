use album_art::download_album_art_from_track_vec;
use rspotify::{AuthCodeSpotify, prelude::*};

use rspotify_model::{FullTrack, PlayHistory};
// for main function
use tokio::{main, join};

use std::{thread, time::Duration};

// get rid of this

mod authorize;
mod recently_played;
mod popular_endpoints;
mod album_art;
mod lyrics;
mod configuration;
mod command_interface;

#[tokio::main]
async fn main() {
    // let spotify = authorize::get_authcode().await;
    let resp = lyrics::get_lyrics().await;
    
}

fn write_to_file(content: String) {
    std::fs::write("Assets/lyrics_test.json", content).unwrap();
}

async fn download_rec_songs(spotify: &AuthCodeSpotify) {
    let rec_songs = recently_played::get_all_recent_songs(&spotify).await.into_iter().map(|song| song.track).collect::<Vec<FullTrack>>();
    album_art::download_album_art_from_track_vec(&rec_songs, 0).await;
}

async fn download_current_song_art(spotify: &AuthCodeSpotify) {
    let song = popular_endpoints::current_playing_fulltrack(&spotify).await.expect("No song playing");
    // album_art::download_album_art(song, 0).await;
}
