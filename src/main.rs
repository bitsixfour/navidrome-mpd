use std::{collections::HashMap, error::Error};
use mpd::Client;
use std::net::TcpStream;
use clap::Parser;
use std::error::Error;
use anyhow::Error;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    
    #[arg(short, long)]
    album: String,

    #[arg(short,long)]
    artist: String,

    #[arg(short, long)]
    song: String, 

    #[arg(short, long, default_value_t = 1)]
    count: u8,
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("starting mpd server....");
    println!("this is a test CLI so I can get Navidrome working");
    let args: Args = Args::parse();
    
    Ok(())
}
