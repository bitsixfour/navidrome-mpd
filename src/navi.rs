use std::collections::HashMap;
use serde::Deserialize;
use reqwest::Client;

const URL: &str = "http://192.168.1.20:8097";
const USR: &str = "nix";

#[derive(Debug, Deserialize)]
pub struct Root {
    #[serde(rename = "subsonic-response")]
    subsonic_response: SubsonicResponse,
}

#[derive(Debug, Deserialize)]
pub struct SubsonicResponse {
    status: String,
    version: String,
    #[serde(rename = "type")]
    kind: String,
    #[serde(rename = "serverVersion")]
    server_version: String,
    #[serde(rename = "openSubsonic")]
    open_subsonic: bool,
    #[serde(rename = "albumList2")]
    album_list_2: AlbumList2,
}

#[derive(Debug, Deserialize, Eq, Hash, PartialEq)]
pub struct AlbumList2 {
    album: Vec<Album>,
}

#[derive(Debug, Deserialize, Eq, Hash, PartialEq)]
pub struct Album {
    pub id: String,
    pub name: String,
    pub artist: String,

    #[serde(rename = "artistId")]
    pub artist_id: String,

    #[serde(rename = "coverArt")]
    pub cover_art: String,

    #[serde(rename = "songCount")]
    pub song_count: u32,

    pub duration: u32,

    pub created: String,

    year: Option<u32>,
    genre: Option<String>,

    #[serde(rename = "userRating")]
    user_rating: Option<u32>,

    genres: Vec<Genre>,

    #[serde(rename = "musicBrainzId")]
    music_brainz_id: Option<String>,

    #[serde(rename = "isCompilation")]
    is_compilation: bool,

    #[serde(rename = "sortName")]
    sort_name: Option<String>,
}

#[derive(Debug, Deserialize, Eq, Hash, PartialEq)]
pub struct Genre {
    name: String,
}

pub async fn navi_obj(client: &Client) -> Result<SubsonicResponse, reqwest::Error> {
    let root = client
        .get("http://127.0.0.1:8097/rest/getAlbumList2.view")
        .query(&[
            ("f", "json"),
            ("type", "alphabeticalByName"),
            ("size", "500"),
            ("offset", "0"),
        ])
        .send()
        .await?
        .error_for_status()?
        .json::<Root>()
        .await?;

    Ok(root.subsonic_response)
}
pub struct NaviData {
    pub data: HashMap<String, Album>,
}

impl NaviData {
    pub async fn new(resp: SubsonicResponse) -> NaviData {
        let mut hmap: HashMap<String, Album> = HashMap::new();
        println!("Mapping Navidrome...");
        let album: Vec<Album> = resp.album_list_2.album;
        
        for i in album {
            println!("adding to hashmap (dbg)");
            let name = i.name.clone();
            hmap.insert(name, i);
 
        }
        NaviData {
            data: hmap,
        }
    }
    pub fn get_url(song_id: &str) -> String {
        format!("{URL}/rest/stream?id={song_id}&u={USR}&v=1.8.0&c=myapp")
    }
}
