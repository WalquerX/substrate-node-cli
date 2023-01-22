/* A Marco Polo game. */

/* Accepts a string with a name.
If the name is "Marco", returns "Polo".
If the name is "any other value", it returns "Marco".
*/

use websocket::{ClientBuilder, Message};
use sp_core::hexdisplay::HexDisplay;

use parity_scale_codec::{Decode, Encode};

use sp_runtime::traits::Extrinsic;

use hex::decode;

use websocket_base::message::OwnedMessage;


//use sp_core::sr25519::Pair;

//use sp_std::prelude::*;
use sp_application_crypto::Pair;
//use sp_block_builder::runtime_decl_for_BlockBuilder::BlockBuilderV6;


//[cfg_attr(feature = "std", derive(Serialize, Deserialize, parity_util_mem::MallocSizeOf))]
#[derive(Debug, Encode, Decode, PartialEq, Eq, Clone)]
pub struct BasicExtrinsic {
	call: crate::Call,
	signature: Option<SignaturePayload>,
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize, parity_util_mem::MallocSizeOf))]
#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone)]
pub struct SignaturePayload {
	pub_key: sp_core::H256,
	signature: sp_core::H512,
}

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

//#[cfg(test)]
impl BasicExtrinsic {
	fn new_unsigned(call: Call) -> Self {
		<Self as Extrinsic>::new(call, None).unwrap()
	}

	fn new_signed(call: Call, payload: Option<SignaturePayload>) -> Self {
		<Self as Extrinsic>::new(call, payload).unwrap()
	}
}

impl sp_runtime::traits::Extrinsic for BasicExtrinsic {
	type Call = Call;
	type SignaturePayload = SignaturePayload;

	fn new(data: Self::Call, sig: Option<Self::SignaturePayload>) -> Option<Self> {
		Some(Self{call: data, signature: sig})
	}
}


//#[cfg_attr(feature = "std", derive(Serialize, Deserialize, parity_util_mem::MallocSizeOf))]
#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone)]
pub enum Call {
	SetValue(u128),
	Mint([u8; 32], u128),
	Burn([u8; 32], u128),
	// ***we can mint or hardcode some amount in accounts
	// ***Transfer sender account, recipient account, nonce, amount, fee
	Transfer([u8; 32], [u8; 32], u128, u128, u128),
	Upgrade(Vec<u8>),
}





pub fn marco_polo(name: &str) -> String {
    if name == "Marco" {
        "Polo".to_string()
    } else {
        "Marco".to_string()
    }
}

pub fn call_rpc(message: String) -> String {

    let mut client = ClientBuilder::new("ws://127.0.0.1:9944")
        .unwrap()
        .connect_insecure()
        .unwrap();
  
    client.send_message(&Message::text(message)).unwrap();
    let mut response = client.recv_message().unwrap();

    while !response.is_data() {
        response = client.recv_message().unwrap();
    }

    match response {
        OwnedMessage::Text(t) => return t,
        /// A message containing binary data
        OwnedMessage::Binary(t) => return "is bin".to_string(),
        /// A message which indicates closure of the WebSocket connection.
        /// This message may or may not contain data.
        OwnedMessage::Close(t)  => return "close data".to_string(),
        /// A ping message - should be responded to with a pong message.
        /// Usually the pong message will be sent with the same data as the
        /// received ping message.
        OwnedMessage::Ping(t)  => return "ping".to_string(),
        /// A pong message, sent in response to a Ping message, usually
        /// containing the same data as the received ping message.
        OwnedMessage::Pong(t)  => return "pong".to_string(),
    };

    "end".to_string()
}


pub fn node_info(method: &str) -> String {

    let str = "{\"jsonrpc\":\"2.0\", \"id\":1, \"method\":\"".to_owned() + method + "\"}";
    println!("{}", &str);
    call_rpc(str);
    "end".to_string()
}



pub fn set_value(value: u128) -> String {

    // curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d   '{
    //"jsonrpc":"2.0",
    //"id":1,
    //"method":"author_submitExtrinsic",
    //"params": ["0x002a000000"]
    //}'
    // 002a000000

    // Temporaly since i don´t use unsigned with the signatures
    let extrinsic = BasicExtrinsic::new_unsigned(Call::SetValue(value));
    let binding = extrinsic.encode();
    let hex_extrinsic = HexDisplay::from(&binding).to_string();

    let str = "{\"jsonrpc\":\"2.0\", \"id\":1, \"method\":\"author_submitExtrinsic\", \"params\": [\"".to_owned() + &hex_extrinsic + "\"]}";

    let response = call_rpc(str);
    response
}

const VALUE_KEY: &[u8] = b"VALUE_KEY"; 
const SYSTEM_KEY: &[u8] = b"SYSTEM_KEY";

pub fn from_text_to_key(text: String) -> String {
    //let binding = HexDisplay::from(&VALUE_KEY);
    let key: &[u8] = text.as_bytes();
    let hex_key = HexDisplay::from(&key);
    hex_key.to_string()
}

pub fn read_value() -> String {
    // curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d   '{
    //"jsonrpc":"2.0",
    // "id":1,
    // "method":"state_getStorage",
    // "params": ["56414c55455f4b4559"]
    // }'
    let bites_key = from_text_to_key("VALUE_KEY".to_string());
    let str = "{\"jsonrpc\":\"2.0\", \"id\":1, \"method\":\"state_getStorage\", \"params\": [\"".to_owned() + &bites_key + "\"]}";
    //println!("{}", &str);
    let response = call_rpc(str);
    response
}

pub fn read_balance_with_key(key: String) -> String {
    // curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d   '{
    //"jsonrpc":"2.0",
    // "id":1,
    // "method":"state_getStorage",
    // "params": ["56414c55455f4b4559"]
    // }'
    let bites_key = from_text_to_key(key);
    let str = "{\"jsonrpc\":\"2.0\", \"id\":1, \"method\":\"state_getStorage\", \"params\": [\"".to_owned() + &bites_key + "\"]}";
    //println!("{}", &str);
    let response = call_rpc(str);
    response
}

pub fn generate_key_pair(passphrase: String) -> sp_core::sr25519::Pair {

    let pair: sp_core::sr25519::Pair = sp_core::Pair::generate_with_phrase(Some(&passphrase)).0;
    pair

}

pub fn create_extrinsic() -> String {

    const TEST_KEY: &str = "key for testing purposes";
	let pair: sp_core::sr25519::Pair = sp_core::Pair::generate_with_phrase(Some(TEST_KEY)).0;

    let amount: u128 = 100;
	let call = Call::Mint(pair.clone().clone().public().0, amount);

    let extrinsic = BasicExtrinsic { call, signature: None};

    let binding = extrinsic.encode();
    let hex_extrinsic = HexDisplay::from(&binding);

    let str_hex_extrinsic = hex_extrinsic.to_string();

    let str = "{\"jsonrpc\":\"2.0\", \"id\":1, \"method\":\"author_submitExtrinsic\", \"params\": [\"".to_owned() + &str_hex_extrinsic + "\"]}";
    println!("cx {}", &str);
    let response = call_rpc(str);
    response
}

