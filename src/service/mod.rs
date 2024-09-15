use std::u64;

use reqwest::{Response, StatusCode};
use serde_json::{from_value, Value};

use crate::{
    config::BASE_URL,
    model::{person_info::PersonInfo, playlist::Playlist, song::Song},
};

pub async fn send_captcha(phone: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let url: String = format!("{}/captcha/sent?phone={}", BASE_URL, phone);
    let r: Response = reqwest::get(url).await?;

    if r.status() == StatusCode::OK {
        Ok(true)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to send captcha",
        )))
    }
}

pub async fn get_cookie(phone: &str, captcha: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url: String = format!(
        "{}/login/cellphone?phone={}&captcha={}",
        BASE_URL, phone, captcha
    );
    let r: PersonInfo = reqwest::get(url).await?.json::<PersonInfo>().await?;
    Ok(r.cookie)
}

pub async fn get_playlist(uid: &str) -> Result<Vec<Playlist>, Box<dyn std::error::Error>> {
    let url: String = format!("{}/user/playlist?uid={}", BASE_URL, uid);
    let r: Value = reqwest::get(url).await?.json::<Value>().await?;
    let mut playlists: Vec<Playlist> = vec![];
    let pls: &Value = &r["playlist"];
    if let Some(pl) = pls.as_array() {
        for p in pl {
            playlists.push(Playlist {
                id: from_value(p["id"].clone())?,
                name: from_value(p["name"].clone())?,
            })
        }
    }

    Ok(playlists)
}

pub async fn get_playlist_info(id: &str) -> Result<Vec<Song>, Box<dyn std::error::Error>> {
    let url: String = format!("{}/playlist/track/all?id={}", BASE_URL, id);
    let r: Value = reqwest::get(url).await?.json::<Value>().await?;
    let mut songs: Vec<Song> = vec![];
    let v_songs: &Value = &r["songs"];
    if let Some(a_songs) = v_songs.as_array() {
        for song in a_songs {
            songs.push(Song {
                id: from_value(song["id"].clone())?,
                name: from_value(song["name"].clone())?,
            })
        }
    }

    Ok(songs)
}

pub async fn get_song_url(id: u64) -> Result<String, Box<dyn std::error::Error>> {
    let url: String = format!("{}/song/url/v1?id={}&level=exhigh", BASE_URL, id);
    let r: Value = reqwest::get(url).await?.json::<Value>().await?;
    let song_url: String = from_value(r["data"][0]["url"].clone())?;

    Ok(song_url)
}

pub async fn get_recommand_songs(cookie: &str) -> Result<Vec<Song>, Box<dyn std::error::Error>> {
    let url: String = format!("{}/recommend/songs?cookie={}", BASE_URL, cookie);
    let r: Value = reqwest::get(url).await?.json::<Value>().await?;
    let mut songs: Vec<Song> = vec![];
    let v_songs: &Value = &r["data"]["dailySongs"];
    if let Some(a_songs) = v_songs.as_array() {
        for song in a_songs {
            songs.push(Song {
                id: from_value(song["id"].clone())?,
                name: from_value(song["name"].clone())?,
            })
        }
    }

    Ok(songs)
}
