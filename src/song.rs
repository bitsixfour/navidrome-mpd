use std::collections::HashMap;


pub struct Song {

}


pub struct MpdAlbum {
    file: &str,
    title: &str,
    artist: &str,
    album: &str,
    duration: f32,
    track: i16,
}


impl Album {
    pub fn get(io: &str, resp: &NaviData ) -> Self {
        println!("finding data");   
        let data: Album = navidata.get(io).unwrap();
        

        (Self {
            file: data.id,
            title: data.name,
            artist: data.artist,
            album: data.name;
            duration:
            track:
        })
    }
    fn to_text(&Self) -> &str {

        "currentsong"
    }


}
