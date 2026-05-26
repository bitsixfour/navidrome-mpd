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
        let data = resp.get(io).unwrap();
        let a_resp: Vec<MpdAlbum> = Vec::new();
        Self {
            full_album: a_resp
        }

    }
    async fn to_obj(&self) -> Vec<MpdAlbum> {
        let vec: Vec<MpdAlbum> = Vec::new();
        println!("currentsong");
        let root = client
            .get("http://192.168.1.20:8097/rest/getAlbumList2?u=nix&p=2008&v=1.16.1&c=test&f=json&type=alphabeticalByName&size=500")
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

        vec
        
    }


}
