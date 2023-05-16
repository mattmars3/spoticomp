// authorization object
use rspotify::{AuthCodeSpotify, Credentials, OAuth, scopes, Config, Token};
use rspotify::prelude::*;

// default cache token location
use rspotify::{DEFAULT_CACHE_PATH, DEFAULT_API_PREFIX, DEFAULT_PAGINATION_CHUNKS};
// for creating config
use std::path::PathBuf;

// hashset for scopes
use std::collections::HashSet;

// generate random state strings
use random_string::generate;

use crate::configuration::get_assets_dir;

use env_logger;

pub async fn get_authcode() -> AuthCodeSpotify {
    // declare all scopes
    let api_scopes = scopes!(
            "user-read-email",
            "user-read-private",
            "user-top-read",
            "user-read-recently-played",
            "user-follow-read",
            "user-library-read",
            "user-read-currently-playing",
            "user-read-playback-state",
            "user-read-playback-position",
            "playlist-read-collaborative",
            "playlist-read-private",
            "user-follow-modify",
            "user-library-modify",
            "user-modify-playback-state",
            "playlist-modify-public",
            "playlist-modify-private",
            "ugc-image-upload"
    );
    
    // You can use any logger for debugging.
    env_logger::init();
    let creds = Credentials::from_env().unwrap();
    let oauth = OAuth::from_env(api_scopes).unwrap();

    let cached_token_path: PathBuf = (get_assets_dir() + DEFAULT_CACHE_PATH).into();

    let config = Config {
        token_cached: true,
        token_refreshing: true,
        cache_path: cached_token_path,
        ..Config::default()
    };

    let spotify = AuthCodeSpotify::with_config(creds, oauth, config);


    let cached_token = Token::from_cache(get_assets_dir() + DEFAULT_CACHE_PATH);
    match cached_token {
        // if the token is already cached
        Ok(token) => {
            // set token for spotify object
            *spotify.token.lock().await.unwrap() = Some(token)
        },
        // if the token is not already cached
        Err(_) => {
            // Obtaining the access token
            let url = spotify.get_authorize_url(false).unwrap();
            // This function requires the `cli` feature enabled.
            spotify.prompt_for_token(&url).await.unwrap();
        }
    };

    // cache the token for later use
    spotify.write_token_cache().await;

    // return the authenticated spotify object
    spotify
}
