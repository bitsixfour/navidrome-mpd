use clap::Parser;
use reqwest::Client;

mod navi;
use crate::navi::SubsonicResponse;


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

    let client: Client = reqwest::Client::new();
    let _res: SubsonicResponse = navi::navi_obj(&client).await?;
    
    let _args: Args = Args::parse();
    
    Ok(())
}
