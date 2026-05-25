use std::collections::HashMap;


pub struct FullAlbum {
    full_album: Vec<MpdAlbum>,
}


pub struct MpdAlbum {
    file: &str,
    title: &str,
    artist: &str,
    album: &str,
    duration: f32,
    track: i16,
}


impl FullAlbum {
    pub fn get(io: &str, resp: &NaviData ) -> Self {
        println!("finding data");   
        let data: Album = navidata.get(io).unwrap();
        let a_resp: Vec<MpdAlbum> = Vec::new();
        (Self {
            
        })
    }
    fn to_text(&Self) -> &str {

        "currentsong"
    }


}
