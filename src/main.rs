extern crate google_sheets4 as sheets4;

use sheets4::{Sheets, oauth2, oauth2::ServiceAccountAuthenticator};
use sheets4::api::ValueRange;
use sheets4::{Result, Error};
use std::default::Default;

#[tokio::main]
async fn main() {
    let creds = oauth2::read_service_account_key("serviceaccount.json")
        .await
        .unwrap();
    let sa = ServiceAccountAuthenticator::builder(creds)
        .build()
        .await
        .unwrap();
    let scopes = &["https://www.googleapis.com/auth/pubsub"];

    let tok = sa.token(scopes).await.unwrap();
    println!("token is: {:?}", tok);
    let tok = sa.token(scopes).await.unwrap();
    println!("cached token is {:?} and should be identical", tok);

    let hub = Sheets::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().enable_http2().build()), sa);

    let req = ValueRange::default();

    // You can configure optional parameters by calling the respective setters at will, and
    // execute the final call using `doit()`.
    // Values shown here are possibly random and not representative !
    let result = hub.spreadsheets().values_append(req, "1Wzp7fWqcgQNQsv7MxAj5wrPm7JrFstFP0RBSoAje8QI", "A1:D10")
                 .value_input_option("no")
                 .response_value_render_option("ipsum")
                 .response_date_time_render_option("voluptua.")
                 .insert_data_option("At")
                 .include_values_in_response(false)
                 .doit().await;
    
    match result {
        Err(e) => match e {
            // The Error enum provides details about what exactly happened.
            // You can also just use its `Debug`, `Display` or `Error` traits
             Error::HttpError(_)
            |Error::Io(_)
            |Error::MissingAPIKey
            |Error::MissingToken(_)
            |Error::Cancelled
            |Error::UploadSizeLimitExceeded(_, _)
            |Error::Failure(_)
            |Error::BadRequest(_)
            |Error::FieldClash(_)
            |Error::JsonDecodeError(_, _) => println!("{}", e),
        },
        Ok(res) => println!("Success: {:?}", res),
    }
}