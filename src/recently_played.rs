use crate::authorize;
use rspotify::{prelude::*, AuthCodeSpotify};
use rspotify_model::{TimeLimits, PlayHistory};
use chrono::{DateTime, Utc, TimeZone, Duration};

// for turning songs into json
use serde_json::{to_string, to_string_pretty};

// for writing json data to file
use std::fs::{File, write, read_to_string};

// make this a config file thing
// check if the file exists first
const RECENT_SONG_PATH: &str = "Assets/recently_played_songs.json";
const PLAYED_TIMES_REF: &str = "Assets/songs_play_times.json";

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
pub async fn playhistory_to_json(played_songs: Vec<PlayHistory>) -> String {
    let songs_json = serde_json::to_string_pretty(&played_songs);
    songs_json.expect("unable to unwrap songs and parse to json")
}

// takes in a string of recently_played songs and writes them to a file
pub fn write_json(rec_played_string: String) {
    write(RECENT_SONG_PATH, rec_played_string).unwrap();
}

pub async fn update_recently_played_songs(spotify_object: &AuthCodeSpotify) {
    let rec_played_songs = read_songs_from_recent_file();
    let api_recent_songs = get_all_recently_played(spotify_object).await;

    // create a list of times where songs were played
    for song in rec_played_songs {
         
    }

}

// NOTE this does not include the current playing song
pub async fn get_last_songs_played(spotify_object: &AuthCodeSpotify, num_of_songs: i32) -> Vec<PlayHistory> {
    let now_spotify_time: Option<TimeLimits> = Some(TimeLimits::Before(Utc::now()));
    let songs = spotify_object.current_user_recently_played(Some(num_of_songs as u32), now_spotify_time).await;
    songs.expect("Couldn't unwrap cursor based page").items
}

pub fn read_songs_from_recent_file() -> Vec<PlayHistory> {
    let json_data: String = std::fs::read_to_string(RECENT_SONG_PATH).expect("Unable to read songs from recently played file");
    let vec_of_songs: Vec<PlayHistory> = serde_json::from_str(&json_data).unwrap();
    vec_of_songs
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
