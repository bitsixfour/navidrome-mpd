use clap::Parser;
use reqwest::Client;

mod navi;
use crate::navi::{NaviData, SubsonicResponse};

mod mpd;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short = 'l', long)]
    album: String,

    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

pub struct MpdSong {
    id: String,
    title: String,
    artist: String,
    album: String,
    duration: f32,
}
/* Trait for actual Mpd and
 * the Navidrome api */
pub trait AlbumData {
    fn get_id(var: &str) -> String;
    fn get_dur(var: &str) -> String;
}





#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("starting mpd server....");
    println!("this is a test CLI so I can get Navidrome working properly btw");

    let client: Client = reqwest::Client::new();

    let res: SubsonicResponse = navi::navi_obj(&client).await?;
    let navi: NaviData = NaviData::new(res).await;

//   match mat {
//      Some(value) => println!("the full metadata for that album is {:?}", mat),
//        None => println!("not found"),
//    }
    Ok(())
}
