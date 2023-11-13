use anyhow::{ensure, Result};
use iop_keyvault::secp256k1::SecpKeyId;
use iop_sdk::{
    hydra::txtype::{
        hyd_core::Transaction, Aip29Transaction, CommonTransactionFields, OptionalTransactionFields,
    },
    vault::hydra::HydraSigner,
};

use iop_rs_ext::*;

const SENDER: &str = "energy slide remind flip select merge blush clay giraffe doll easy grape";
const RECEIVER: &str = "tfzy3zXYUuEPoX17GkJzQnavdbRmmmFLk6";

#[tokio::main(flavor = "current_thread")]
pub async fn main() -> Result<()> {
    let api = &Api::new("https://test.explorer.hydraledger.io:4705/api/v2/")?;

    let wallet = ArkWallet::new(SENDER, "HYD testnet")?;
    let addr = wallet.addr()?;
    let w = api.wallet(&addr).await?;
    assert_eq!(&addr, &w.address);

    println!("{addr}: balance: {}, nonce: {}", w.balance, w.nonce);

    let common_fields = CommonTransactionFields {
        network: wallet.network(),
        sender_public_key: wallet.public_key()?,
        nonce: w.nonce + 1,
        optional: OptionalTransactionFields {
            amount: 100_000_000,
            manual_fee: None,
            vendor_field: Some("🚀".to_owned()),
        },
    };
    let recipient_id = SecpKeyId::from_p2pkh_addr(RECEIVER, wallet.network())?;
    let unsigned = Transaction::transfer(common_fields, &recipient_id);
    let mut signed = unsigned.to_data();
    wallet.private_key()?.sign_hydra_transaction(&mut signed)?;

    let txids = api.send_txns([&signed]).await?;
    ensure!(txids.len() == 1);
    println!("txid: {}", txids[0]);

    Ok(())
}
