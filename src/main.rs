
use rspotify::{AuthCodeSpotify, prelude::*};


// for main function
use tokio::main;

mod authorize;
mod recently_played;
mod popular_endpoints;
mod album_cover;
mod lyrics;
mod ui;

#[tokio::main]
async fn main() {
    let spotify = authorize::get_authcode().await;
    let song = popular_endpoints::current_playing_fulltrack(&spotify).await;
    album_cover::download_album_art(song).await;

     

}


