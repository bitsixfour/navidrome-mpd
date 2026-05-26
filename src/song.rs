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
        let data: = resp.get(io).unwrap();
        let a_resp: Vec<MpdAlbum> = Vec::new();
        (Self {
            full_album: Vec<
        })
    }
    fn to_text(&self) -> &str {

        println!("currentsong");
        
        let mut str: &str = 
            "file: {} \n
             Last-modified: {} \n
             Format: {} \n
             Artist: {} \n
             AlbumArtist: {} \n
             Title: {} \n
             Album: {} \n
             Track: {} \n
             Time: {} \n
             duration: {} \n
             OK";
        str
        
        
    }


}
