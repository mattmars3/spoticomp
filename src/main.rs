use album_art::download_album_art_from_vec;
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

#[tokio::main]
async fn main() {
    let spotify = authorize::get_authcode().await;
    let track = popular_endpoints::current_playing_fulltrack(&spotify).await.unwrap();
    album_art::download_album_art(album_art::get_links(&track), 2).await;
}

async fn download_recent_songs(spotify: &AuthCodeSpotify) {
    /*
    let rec_songs = recently_played::get_all_recent_songs(&spotify).await;
    let join = download_album_art_from_vec(rec_songs, 2);
    rec_songs.iter().for_each(|item| println!("{:?}", item.track.album.images));

    join!(join); 
    */
}

async fn download_current_song_art(spotify: &AuthCodeSpotify) {
    let song = popular_endpoints::current_playing_fulltrack(&spotify).await.expect("No song playing");
    // album_art::download_album_art(song, 0).await;
}
