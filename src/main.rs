use std::env;

use ethers::abi::{ParamType, Tokenizable};
use ethers::types::{H160, U256};

enum Query {
    Proof,
    Active,
}

fn main() {
    let mut args = env::args().skip(1);
    let maybe_query = args.next().as_ref().and_then(|s| match s.as_ref() {
        "active" => Some(Query::Active),
        "proof" => Some(Query::Proof),
        _ => None,
    });
    let maybe_abi_data = args.next().and_then(|abi| {
        let prefix = abi.get(..2)?;
        if prefix != "0x" {
            println!("no");
            return None;
        }
        let abi = abi.get(2..)?;
        hex::decode(abi).ok()
    });

    let Some(query) = maybe_query else {
        eprintln!("First arg must be either `proof` or `active`");
        return;
    };
    let Some(abi_data) = maybe_abi_data else {
        eprintln!("Invalid or no ABI data arg provided, after the mode of operation");
        return;
    };

    match query {
        Query::Active => {
            let params = [ParamType::Tuple(vec![
                ParamType::Array(Box::new(ParamType::Address)),
                ParamType::Array(Box::new(ParamType::Uint(256))),
                ParamType::Uint(256),
            ])];
            let Ok(Some(token)) = ethers::abi::decode(&params, &abi_data).map(|mut t| t.pop()) else {
                eprintln!("Invalid active valset ABI encoded data");
                return;
            };
            let decoded: (Vec<H160>, Vec<U256>, U256) = Tokenizable::from_token(token).unwrap();
            println!("{decoded:#?}");
        }
        Query::Proof => {
            let params = [ParamType::Tuple(vec![
                ParamType::FixedBytes(32),
                ParamType::FixedBytes(32),
                ParamType::Array(Box::new(ParamType::Tuple(vec![
                    ParamType::FixedBytes(32),
                    ParamType::FixedBytes(32),
                    ParamType::Uint(8),
                ]))),
            ])];
            let Ok(Some(token)) = ethers::abi::decode(&params, &abi_data).map(|mut t| t.pop()) else {
                eprintln!("Invalid valset proof ABI encoded data");
                return;
            };
            let decoded: ([u8; 32], [u8; 32], Vec<([u8; 32], [u8; 32], u8)>) =
                Tokenizable::from_token(token).unwrap();
            println!("{decoded:#?}");
        }
    }
}
