use ethereum_types::{H160, U256, H256};
use hex;
use structopt::StructOpt;
use std::fs::File;
use std::io::Read;
use std::str::FromStr; // !!! Necessary for H160::from_str(address).expect("...");
use std::collections::BTreeMap;
use parity_crypto::publickey::{KeyPair, Random, Error as EthkeyError, Generator, sign, verify_public, verify_address};
use rustc_hex::{FromHex, FromHexError};
use std::num::ParseIntError;
use std::{env, fmt, process, io, sync};

use docopt::Docopt;
use parity_crypto::Keccak256;
use parity_wordlist;


/*
target/debug/bloom-cmd contract deploy --from 0000000000000000000000000000000000000001  --value 0 --gas 100000 --gas-price 0 --code-file ./code-file
target/debug/bloom-cmd contract deploy --from 0000000000000000000000000000000000000001  --value 0 --gas 100000 --gas-price 0 --code 000000
target/debug/bloom-cmd contract call --from 0000000000000000000000000000000000000001  --to 0000000000000000000000000000000000000002 --value 0 --gas 100000 --gas-price 0 --data 000000
*/
enum DisplayMode{
    KeyPair,
    Secret,
    Public,
    Address,
}
impl DisplayMode{
    fn new()->Self{
        DisplayMode::KeyPair
    }
}
#[derive(Debug)]
enum Error{
    Ethkey(EthkeyError),
    FromHex(FromHexError),
    ParseInt(ParseIntError),
    Docopt(docopt::Error),
    Io(io::Error),
}
impl From<EthkeyError> for Error {
    fn from(err: EthkeyError) -> Self {
        Error::Ethkey(err)
    }
}

impl From<FromHexError> for Error {
    fn from(err: FromHexError) -> Self {
        Error::FromHex(err)
    }
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Error::ParseInt(err)
    }
}

impl From<docopt::Error> for Error {
    fn from(err: docopt::Error) -> Self {
        Error::Docopt(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Error::Ethkey(ref e) => write!(f, "{}", e),
            Error::FromHex(ref e) => write!(f, "{}", e),
            Error::ParseInt(ref e) => write!(f, "{}", e),
            Error::Docopt(ref e) => write!(f, "{}", e),
            Error::Io(ref e) => write!(f, "{}", e),
        }
    }
}


fn execute(from:String)->Result<String,Error>{
    let display_mode = DisplayMode::new();
    let result = {
        let secret = from.parse().map_err(|_| EthkeyError::InvalidSecretKey)?;
        (KeyPair::from_secret(secret)?, None)
    };
    println!("{:#?}", result);
    Ok(display(result, display_mode))

}
fn display(result: (KeyPair, Option<String>), mode: DisplayMode) -> String {
    let keypair = result.0;
    println!("{:#?}",keypair);
    match mode {
        DisplayMode::KeyPair => match result.1 {
            Some(extra_data) => format!("{}\n{}", extra_data, keypair),
            None => format!("{}", keypair)
        },
        DisplayMode::Secret => format!("{:x}", keypair.secret()),
        DisplayMode::Public => format!("{:x}", keypair.public()),
        DisplayMode::Address => format!("{:x}", keypair.address()),
    }
}

#[derive(Debug, StructOpt, Clone)]
pub struct EthkeyCmd {
    #[structopt(subcommand)]
    cmd: Command
}

#[derive(StructOpt, Debug, Clone)]
enum Command {
    ///Info Eth_keyCmd
    Info{
        tex:String,
        #[structopt(long = "info",default_value="123")]
        info :String,
    },
}

//  ./target/debug/bloom-cmd ethkey info 17d08f5fe8c77af811caa0c9a187e668ce3b74a99acc3f6d976f075fa8e0be55
//  ./target/debug/bloom-cmd ethkey info 59a5208b32e627891c389ebafc644145224006e8
impl EthkeyCmd {
    pub fn run(&self, mut backend: &str) {
        match &self.cmd {
            Command::Info {tex,info} => {
                let from = H256::from_str(tex).expect("From should be a valid address");
                let info = U256::from_str(info).expect("From should be a valid address");
                let result=execute(tex.to_string());
                println!("Deploy {:#?}", backend);
                println!("{:#?}",result.unwrap());
                // println!("tex=={:#?}\ninfo=={:#?}\n",from, info);
            }
        }
    }
}
