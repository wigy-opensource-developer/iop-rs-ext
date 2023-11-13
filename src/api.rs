use iop_sdk::hydra::TransactionData;
use reqwest::{get, Client, Url};
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Deserialize)]
#[serde(untagged)]
enum Response<T> {
    Success { data: T },
    Failure { errors: String },
}

pub struct WalletResponse {
    pub address: String,
    pub public_key: SecpPublicKey,
    pub balance: u64,
    pub nonce: u64,
    pub is_delegate: bool,
    pub is_resigned: bool,
    pub vote: Option<SecpPublicKey>,
    pub username: Option<String>,
}

pub struct Api {
    base: Url,
}

impl Api {
    pub fn new(base: &str) -> Result<Self> {
        let base = Url::parse(base)?;
        Ok(Self { base })
    }

    pub async fn wallet(&self, addr: &str) -> Result<WalletResponse> {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct WalletDto {
            address: String,
            public_key: String,
            balance: String,
            nonce: String,
            is_delegate: bool,
            is_resigned: bool,
            vote: Option<String>,
            username: Option<String>,
        }

        let url = self.base.join(&format!("wallets/{addr}"))?;

        let response = get(url).await?;
        let result: Response<WalletDto> = response.json().await?;
        let dto = match result {
            Response::Success { data } => data,
            Response::Failure { errors } => bail!("Could not get balance {addr}: {errors}"),
        };

        let address = dto.address;
        let public_key = dto.public_key.parse()?;
        let balance = dto.balance.parse()?;
        let nonce = dto.nonce.parse()?;
        let is_delegate = dto.is_delegate;
        let is_resigned = dto.is_resigned;
        let vote = dto.vote.map(|s| s.parse()).transpose()?;
        let username = dto.username;

        Ok(WalletResponse {
            address,
            public_key,
            balance,
            nonce,
            is_delegate,
            is_resigned,
            vote,
            username,
        })
    }

    pub async fn send_txns(
        &self,
        txns: impl IntoIterator<Item = &TransactionData>,
    ) -> Result<Vec<String>> {
        #[derive(Serialize)]
        struct SendTxnsReq<'a> {
            transactions: Vec<&'a TransactionData>,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct SendTxnsRes {
            data: SendTxnsResData,
            errors: Option<serde_json::Value>,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct SendTxnsResData {
            accept: Vec<String>,
            // broadcast: Vec<String>,
            // excess: Vec<String>,
            // invalid: Vec<String>,
            // error_response: Option<String>,
        }

        let transactions = txns.into_iter().collect::<Vec<_>>();
        let count = transactions.len();

        let url = self.base.join("transactions")?;

        let response = Client::new()
            .post(url)
            .json(&SendTxnsReq { transactions })
            .send()
            .await?;
        let r: String = response.text().await?;
        let result: SendTxnsRes = serde_json::from_str(&r)
            .map_err(|e| anyhow::anyhow!("Could not parse {}: {}", &r, e))?;

        ensure!(
            result.errors.is_none(),
            "Could not send transactions: {}",
            serde_json::to_string(&result.errors.expect("Checked for None"))?
        );
        ensure!(
            count == result.data.accept.len(),
            "Some transactions were not accepted: {}",
            &r
        );
        Ok(result.data.accept)
    }
}
