mod api;
mod ark_wallet;

use anyhow::{bail, ensure, Result};
use iop_keyvault::{
    secp256k1::{Secp256k1, SecpKeyId, SecpPrivateKey, SecpPublicKey},
    Network, Networks, PrivateKey as _,
};

pub use api::*;
pub use ark_wallet::*;
