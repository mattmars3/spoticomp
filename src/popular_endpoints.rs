use rspotify::{AuthCodeSpotify, prelude::*};
use rspotify_model::{CurrentlyPlayingContext, FullTrack, PlayableItem};

pub async fn current_playing_context(spotify_obj: &AuthCodeSpotify) -> Option<CurrentlyPlayingContext> {
    let track = spotify_obj.current_user_playing_item().await.expect("one");
    track
}

pub async fn current_playing_fulltrack(spotify_obj: &AuthCodeSpotify) -> Option<FullTrack> {
    let playing_context = match current_playing_context(spotify_obj).await {
        Some(track) => {
            match track.item.unwrap() {
                PlayableItem::Track(track) => {Some(track)},
                PlayableItem::Episode(_) => {None},
            }
        },
        None => None
    };
    playing_context
}
