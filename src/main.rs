use rspotify::{AuthCodeSpotify, prelude::*};

use rspotify_model::{FullTrack, PlayHistory};
// for main function
use tokio::main;

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
    let song = popular_endpoints::current_playing_fulltrack(&spotify).await.unwrap();
    album_art::download_album_art(&song, 0).await;

   
}

