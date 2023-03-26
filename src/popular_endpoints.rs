use rspotify::{AuthCodeSpotify, prelude::*};
use rspotify_model::{CurrentlyPlayingContext, FullTrack, PlayableItem};

pub async fn current_playing_context(spotify_obj: &AuthCodeSpotify) -> CurrentlyPlayingContext {
    let track = spotify_obj.current_user_playing_item().await.unwrap().unwrap();
    track
}

pub async fn current_playing_fulltrack(spotify_obj: &AuthCodeSpotify) -> FullTrack {
    let playing_context = current_playing_context(spotify_obj).await;
    let track: Option<FullTrack> = match playing_context.item.unwrap() {
        PlayableItem::Track(track) => {Some(track)},
        PlayableItem::Episode(_) => {None},
    };
    track.expect("Failed to unwrap track")
}
