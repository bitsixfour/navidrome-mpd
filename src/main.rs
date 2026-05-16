use clap::Parser;
use std::collections::HashMap;
use reqwest::Client;
mod navi;


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
    let args: Args = Args::parse();
    
    Ok(())
}
