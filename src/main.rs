use clap::Parser;
use reqwest::Client;

mod navi;
use crate::navi::{NaviData, SubsonicResponse};

mod album;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short = 'l', long)]
    album: String,

    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

pub trait Album {
    pub fn sigma(var: &str) -> String;
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
