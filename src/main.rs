extern crate hex;
use hex_literal::hex;

use web3::{
    contract::{Contract, Options},
    types::{U256, H160, Bytes},
};

#[tokio::main]
async fn main() -> web3::contract::Result<()> {
    let _ = env_logger::try_init();
    let http = web3::transports::Http::new("https://ropsten.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161")?;
    let web3 = web3::Web3::new(http);

    let addr_u8 = hex::decode("7Ad11de6d4C3DA366BC929377EE2CaFEcC412A10").expect("Decoding failed");
    let addr_h160 = H160::from_slice(&addr_u8);

    let contra = Contract::from_json(
        web3.eth(),
        addr_h160,
        include_bytes!("../contracts/hello_world.json"),
    )?;

    // let acct:[u8; 20] = hex!("f24ff3a9cf04c71dbc94d0b566f7a27b94566cac").into();
    
    let result = contra.query::<String, _, _,_>("get", (), None, Options::default(), None).await?;
    println!("{}", result);

    Ok(())
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}