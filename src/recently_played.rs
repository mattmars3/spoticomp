// for authrorization
use crate::authorize;
use rspotify::{prelude::*, AuthCodeSpotify};
use rspotify_model::{TimeLimits, PlayHistory};
use chrono::{DateTime, Utc, TimeZone, Duration};

// for turning songs into json
use serde_json::{to_string, to_string_pretty};

// for writing json data to file
use std::fs::{File, write, read_to_string};

use crate::configuration::recently_played_path;

// make this a config file thing
// check if the file exists first

// NOTE this does not include the current playing song
// gets last x songs
pub async fn get_x_recent_songs(spotify_object: &AuthCodeSpotify, num_of_songs: i32) -> Vec<PlayHistory> {
    let now_spotify_time: Option<TimeLimits> = Some(TimeLimits::Before(Utc::now()));
    let songs = spotify_object.current_user_recently_played(Some(num_of_songs as u32), now_spotify_time).await;
    songs.expect("Couldn't unwrap cursor based page").items
}

pub async fn get_all_recent_songs(spotify_object: &AuthCodeSpotify) -> Vec<PlayHistory> {
    fn get_after_newest_song_datetime(songs: &Vec<PlayHistory>) -> DateTime<Utc> {
        let mut newest_song = songs.get(0).unwrap();
        for song in songs {
            if song.played_at > newest_song.played_at {newest_song = song;}
        }

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

    // amount of songs to request per req
    let limit = 4u32;
    // old datetime counter so that all songs in recent played will be after it
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

// takes in a string of recently_played songs and writes them to a file
pub fn write_to_file(played_songs: Vec<PlayHistory>) {
    let songs_json = serde_json::to_string_pretty(&played_songs);
    let rec_played_string = songs_json.expect("unable to unwrap songs and parse to json");
    write(recently_played_path(), rec_played_string).unwrap();
}

pub fn read_from_file() -> Vec<PlayHistory> {
    let json_data: String = std::fs::read_to_string(recently_played_path()).expect("Unable to read songs from recently played file");
    let vec_of_songs: Vec<PlayHistory> = serde_json::from_str(&json_data).unwrap();
    vec_of_songs
}

// gets all songs from the recently played 
pub async fn file_and_api_recently_played(spotify_object: &AuthCodeSpotify) -> Vec<PlayHistory> {
    let mut file_recent_songs: Vec<PlayHistory> = read_from_file();
    let api_recent_songs = get_all_recent_songs(spotify_object).await;

    let mut file_played_at_times: Vec<DateTime<Utc>> = vec![];

    // create a list of times where songs were played
    for song in &file_recent_songs {
        file_played_at_times.push(song.played_at);
    }


    // if a song has the same DateTime, you know it is a duplicate because there's no way it would
    // be the exact same time
    for song in api_recent_songs {
        // if it is not already in the file
        if !file_played_at_times.contains(&song.played_at) {
            // append it to the vector 
            // there is probably an ownership bug here lmao
            file_recent_songs.push(song);
        }
    }

    // return updated list
    file_recent_songs
}

pub async fn update_recently_played(spotify_object: &AuthCodeSpotify) {
    // get the file and api recently played songs
    let file_and_recent_played = file_and_api_recently_played(spotify_object).await;
    println!("{} songs recorded in history.", file_and_recent_played.len());
    write_to_file(file_and_recent_played);
}
