use crate::authorize;
use rspotify::{prelude::*, AuthCodeSpotify};
use rspotify_model::{TimeLimits, PlayHistory};
use chrono::{DateTime, Utc, TimeZone, Duration};

// for turning songs into json
use serde_json::{to_string, to_string_pretty};

// for writing json data to file
use std::fs::{File, write};

pub async fn get_all_recently_played(spotify_object: &AuthCodeSpotify) -> Vec<PlayHistory> {
    // internal function
    fn get_after_newest_song_datetime(songs: &Vec<PlayHistory>) -> DateTime<Utc> {
        // get the newest song datetime
        let mut newest_song = songs.get(0).unwrap();
        for song in songs {
            if song.played_at > newest_song.played_at {newest_song = song;}
        }
        // println!("{:?}: {}", newest_song.track.name, newest_song.played_at);
        let after_newest = newest_song.played_at + Duration::nanoseconds(1);
        after_newest
    }

    // gets songs after specified DateTime
    async fn get_recently_played(spotify_object: &AuthCodeSpotify, limit: u32, after_time: DateTime<Utc>) -> Vec<PlayHistory> {
        // declare an internal spotify timelimits object after the specified datetime
        let after_date_timelimits = Some(TimeLimits::After(after_time));
        let recent_played = spotify_object.current_user_recently_played(Some(limit), after_date_timelimits).await.unwrap();
        recent_played.items
    }

    // declare super old datetime
    let limit = 1u32;
    // this should check the json file and see what the newest time is and set that as the oldest
    let mut datetime_counter: DateTime<Utc> = Utc.with_ymd_and_hms(2020, 1, 1, 1, 1, 1).unwrap();
    let mut playhistory_vector: Vec<PlayHistory> = vec![];

    // loop and get all recently played songs
    loop {
        // get songs after old date
        let songs = get_recently_played(spotify_object, limit, datetime_counter).await;
        if songs.is_empty() {
            break;
        }
        
        // find newest datetime & increment counter
        datetime_counter = get_after_newest_song_datetime(&songs);

        // append songs to vector
        for song in songs {
            playhistory_vector.push(song);
        }
    }

    playhistory_vector
}


// takes a vector of playhistory and turns it all into json
pub async fn recently_played_to_json(played_songs: Vec<PlayHistory>) -> String {
    let songs_json = serde_json::to_string_pretty(&played_songs);
    songs_json.expect("unable to unwrap songs and parse to json")
}

// takes in a string of recently_played songs and writes them to a file
pub fn write_json(rec_played_string: String) {
    let file_name = "recent_played_songs.json";
    write(file_name, rec_played_string).unwrap();
}

/* functions to have
 * get all recently played songs
 *      gets from the API
 * write the recently played songs to a json file
 *      rectifies changes between them
 * get lifetime played songs
 *      reads from the json file
 * 
 * */
