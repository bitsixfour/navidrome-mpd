use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)] struct Root {
      #[serde(rename = "subsonic-response")]
      subsonic_response: SubsonicResponse,
  }

  #[derive(Debug, Deserialize)]
  struct SubsonicResponse {
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

  #[derive(Debug, Deserialize)]
  struct AlbumList2 {
      album: Vec<Album>,
  }

  #[derive(Debug, Deserialize)]
  struct Album {
      id: String,
      name: String,
      artist: String,

      #[serde(rename = "artistId")]
      artist_id: String,

      #[serde(rename = "coverArt")]
      cover_art: String,

      #[serde(rename = "songCount")]
      song_count: u32,

      duration: u32,

      created: String,

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

  #[derive(Debug, Deserialize)]
  struct Genre {
      name: String,
  }
pub struct NaviData { /* may change later */
    data: HashMap<Album, String>
}   


pub async fn navi_obj() -> SubsonicResponse {
    


}



impl Album {
    
    pub async fn new(str: &String) -> NaviData {
        for i in &map {



        }

}
