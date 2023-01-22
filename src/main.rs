//A command-line tool to play Marco Polo
use clap::Parser;

use std::io::{self, Write};
use std::net::TcpStream;
use websocket::message::Type;
use websocket::sync::Client;
use websocket::ws::dataframe::DataFrame;
use websocket::{ClientBuilder, Message};
use parity_scale_codec_derive::{Decode, Encode};

use serde_json::{Result, Value};

use std::i64;
use sp_application_crypto::Pair;
#[derive(Parser)]
#[clap(version = "1.0", author = "WalquerX", about = "A cli to submit extrinsics to substrate node")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "WalquerX")]
    Play {
        #[clap(short, long)]
        name: String,
        message: String,
    },
    NodeInfo {
        #[clap(short, long)]
        method: String,
    },
    SettingValue {
        #[clap(short, long)]
        number: u128,
    },
    ReadingValue,
    ReadStateBalance {
        #[clap(short, long)]
        text_key: String,
    },
    Mint, 
    //{
    //    #[clap(short, long)]
    //    public_key: u32,
    //    amount: u32,
    //},
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Play { name , message}) => {
            let result = node_cli::marco_polo(&name);
            println!("{}, {}", result, message);
        }
        Some(Commands::NodeInfo { method }) => {
            let result = node_cli::node_info(&method);
            println!("{:?}", result);
        }
        Some(Commands::SettingValue { number }) => {
            let result = node_cli::set_value(number);
            println!("{:?}", result);
        }
        Some(Commands::ReadingValue) => {
            let result_0 = node_cli::read_value();
            let json: serde_json::Value = serde_json::from_str(&result_0).expect("JSON was not well-formatted");
            let result_string = json.clone()["result"].as_str().expect("NO VALUE STORED YET").to_owned();
            let ar: [u8; 16] = hex::decode(&result_string[2..]).unwrap()[0..16].try_into().unwrap();
            let number = u128::from_le_bytes(ar);
            println!("{:?}", number);
        }
        Some(Commands::ReadStateBalance { text_key }) => {
            let result_0 = node_cli::read_balance_with_key(text_key);
            let json: serde_json::Value = serde_json::from_str(&result_0).expect("JSON was not well-formatted");
            let result_string = json.clone()["result"].as_str().expect("NO VALUE STORED YET").to_owned();
            let ar: [u8; 16] = hex::decode(&result_string[2..]).unwrap()[0..16].try_into().unwrap();
            let number = u128::from_le_bytes(ar);
            println!("{:?}", number);
        }
        Some(Commands::Mint) => {
            let result_0 = node_cli::create_extrinsic();
            /*let json: serde_json::Value = serde_json::from_str(&result_0).expect("JSON was not well-formatted");
            let result_string = json.clone()["result"].as_str().expect("NO VALUE STORED YET").to_owned();
            let ar: [u8; 4] = hex::decode(&result_string[2..]).unwrap()[0..4].try_into().unwrap();
            let number = u32::from_le_bytes(ar);*/
            println!("response {:?}", result_0);
        }
        None => println!("No subcommand was used"),
    }
}