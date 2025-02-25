# Ensure the environmental variables are set:
export MPESA_MPESA_BASE_URL="https://sandbox.safaricom.co.ke"
export MPESA_MPESA_CONSUMER_KEY="your_consumer_key"
export MPESA_MPESA_CONSUMER_SECRET="your_consumer_secret"
export MPESA_MPESA_PASSKEY="your_passkey"
export MPESA_MPESA_CALLBACK_URL="https://yourcallback.com"
export MPESA_MPESA_ENVIRONMENT="sandbox"
export MPESA_MPESA_CALLBACK_PORT="your listener port"
export MPESA_MPESA_BUSINESS_SHORT_CODE="your short code"
export MPESA_MPESA_PARTY_B="party b, probably similar to short code but ask to confirm"
export MPESA_MPESA_CERTIFICATE_PATH="path to the certificate.pem"
export MPESA_DATABASE_USERNAME="postgres"
export MPESA_DATABASE_PASSWORD="password"
export MPESA_DATABASE_HOST="localhost"
export MPESA_DATABASE_PORT=5432
export MPESA_DATABASE_DATABASE_NAME="mpesa"


# Example Usage of the crate:
use mpesa_rs::{MpesaCrate, client::MpesaClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the crate
    let mpesa_crate = MpesaCrate::new().await?;

    // Initialize the M-PESA client
    let mpesa_client = MpesaClient::new(&mpesa_crate.settings.mpesa);

    // Make a payment request
    let response = mpesa_client
        .lipa_na_mpesa("254712345678", 100.0, "Test", "Payment")
        .await?;

    println!("Payment request successful: {:?}", response);

    Ok(())
}

# Curl Test Example for call backs:
## STK Push call back.
curl -X POST http://127.0.0.1:3030/callback \
  -H "Content-Type: application/json" \
  -d '{
    "merchant_request_id": "test_merchant_request_id",
    "checkout_request_id": "test_checkout_request_id",
    "result_code": "0",
    "result_desc": "Success",
    "amount": 100.0,
    "mpesa_receipt_number": "test_receipt_number",
    "transaction_date": "20231010120000",
    "phone_number": "254712345678"
  }'
  

## C2B Call back.
curl -X POST http://127.0.0.1:3030/c2b_callback \
  -H "Content-Type: application/json" \
  -d '{
    "transaction_type": "PayBill",
    "trans_id": "test_trans_id",
    "trans_time": "20231010120000",
    "trans_amount": 100.0,
    "business_short_code": "174379",
    "bill_ref_number": "test_bill_ref",
    "invoice_number": "test_invoice",
    "org_account_balance": 1000.0,
    "third_party_trans_id": "test_third_party_trans_id",
    "msisdn": "254712345678",
    "first_name": "John",
    "middle_name": "Doe",
    "last_name": "Smith"
  }'

