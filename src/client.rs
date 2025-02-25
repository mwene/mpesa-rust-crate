use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio_rustls::rustls::{ClientConfig, RootCertStore};
use tokio_rustls::rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::fs::File;
use std::io::{BufReader, Error as IoError};
use std::sync::Arc;

#[derive(Error, Debug)]
pub enum MpesaError {
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),
    #[error("Authentication failed")]
    AuthenticationFailed,
    #[error("API error: {0}")]
    ApiError(String),
    #[error("Failed to load certificate: {0}")]
    CertificateError(String),
    #[error("IO error: {0}")]
    IoError(#[from] IoError),
}

#[derive(Serialize)]
struct LipaNaMpesaRequest {
    BusinessShortCode: String,
    Password: String,
    Timestamp: String,
    TransactionType: String,
    Amount: String,
    PartyA: String,
    PartyB: String,
    PhoneNumber: String,
    CallBackURL: String,
    AccountReference: String,
    TransactionDesc: String,
}

#[derive(Deserialize)]
struct LipaNaMpesaResponse {
    MerchantRequestID: String,
    CheckoutRequestID: String,
    ResponseCode: String,
    ResponseDescription: String,
}

pub struct MpesaClient {
    client: Client,
    base_url: String,
    consumer_key: String,
    consumer_secret: String,
    passkey: String,
    callback_url: String,
    business_short_code: String,
    party_b: String,
}

impl MpesaClient {
    pub async fn new(settings: &crate::settings::MpesaSettings) -> Result<Self, MpesaError> {
        // Load the client certificate
        let cert_file = File::open(&settings.certificate_path)?;
        let mut cert_reader = BufReader::new(cert_file);

        // Load certificates and private key
        let certs = certs(&mut cert_reader)
            .collect::<Result<Vec<CertificateDer>, _>>()
            .map_err(|e| MpesaError::CertificateError(e.to_string()))?;

        let key_file = File::open(&settings.certificate_path)?;
        let mut key_reader = BufReader::new(key_file);

        let keys = pkcs8_private_keys(&mut key_reader)
            .collect::<Result<Vec<PrivateKeyDer>, _>>()
            .map_err(|e| MpesaError::CertificateError(e.to_string()))?;

        let key = keys.into_iter().next().ok_or_else(|| {
            MpesaError::CertificateError("No private key found in certificate file".to_string())
        })?;

        // Configure TLS
        let mut root_store = RootCertStore::empty();
        let config = ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_client_auth_cert(certs, key)
            .map_err(|e| MpesaError::CertificateError(e.to_string()))?;

        let connector = reqwest::tls::rustls::ClientConfig::from(Arc::new(config));
        let client = Client::builder()
            .use_preconfigured_tls(connector)
            .build()?;

        Ok(Self {
            client,
            base_url: settings.base_url.clone(),
            consumer_key: settings.consumer_key.clone(),
            consumer_secret: settings.consumer_secret.clone(),
            passkey: settings.passkey.clone(),
            callback_url: settings.callback_url.clone(),
            business_short_code: settings.business_short_code.clone(),
            party_b: settings.party_b.clone(),
        })
    }

    pub async fn lipa_na_mpesa(
        &self,
        phone_number: &str,
        amount: f64,
        account_reference: &str,
        transaction_desc: &str,
    ) -> Result<LipaNaMpesaResponse, MpesaError> {
        let password = self.generate_password();
        let request = LipaNaMpesaRequest {
            BusinessShortCode: self.business_short_code.clone(),
            Password: password,
            Timestamp: chrono::Utc::now().format("%Y%m%d%H%M%S").to_string(),
            TransactionType: "CustomerPayBillOnline".to_string(),
            Amount: amount.to_string(),
            PartyA: phone_number.to_string(),
            PartyB: self.party_b.clone(),
            PhoneNumber: phone_number.to_string(),
            CallBackURL: self.callback_url.clone(),
            AccountReference: account_reference.to_string(),
            TransactionDesc: transaction_desc.to_string(),
        };

        let response = self
            .client
            .post(&format!("{}/mpesa/stkpush/v1/processrequest", self.base_url))
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let response_body: LipaNaMpesaResponse = response.json().await?;
            Ok(response_body)
        } else {
            Err(MpesaError::ApiError(response.status().to_string()))
        }
    }

    fn generate_password(&self) -> String {
        let timestamp = chrono::Utc::now().format("%Y%m%d%H%M%S").to_string();
        let data = format!("{}{}{}", self.business_short_code, self.passkey, timestamp);
        base64::encode(data)
    }
}
