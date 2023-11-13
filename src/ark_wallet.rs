use super::*;

pub struct ArkWallet {
    passphrase: String,
    network: &'static dyn Network<Suite = Secp256k1>,
}

impl ArkWallet {
    pub fn new(passphrase: &str, network: &str) -> Result<Self> {
        let passphrase = passphrase.to_owned();
        let network = Networks::by_name(network)?;
        Ok(Self {
            passphrase,
            network,
        })
    }

    pub fn network(&self) -> &dyn Network<Suite = Secp256k1> {
        self.network
    }

    pub fn private_key(&self) -> Result<SecpPrivateKey> {
        SecpPrivateKey::from_ark_passphrase(&self.passphrase)
    }

    pub fn public_key(&self) -> Result<SecpPublicKey> {
        Ok(self.private_key()?.public_key())
    }

    pub fn key_id(&self) -> Result<SecpKeyId> {
        Ok(self.public_key()?.ark_key_id())
    }

    pub fn addr(&self) -> Result<String> {
        Ok(self.key_id()?.to_p2pkh_addr(self.network.p2pkh_addr()))
    }
}
