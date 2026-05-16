use clap::Parser;
use std::collections::HashMap;
use reqwest::Client;

mod navi;
use navi::navi_obj;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'l', long)]
    album: String,

    #[arg(short = 'a', long)]
    artist: String,

    #[arg(short, long)]
    song: String,

    #[arg(short, long, default_value_t = 1)]
    count: u8,
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("starting mpd server....");
    println!("this is a test CLI so I can get Navidrome working properly btw");


    let client = reqwest::Client::new();
    let res: SubsonicResponse = client
        .get("https://clawd.rip");
        .send()
        .await.ok()?
        .error_for_status().ok()?
        .json::<SubsonicResponse>().await.ok?;

    let args: Args = Args::parse();
    
    Ok(())
}
