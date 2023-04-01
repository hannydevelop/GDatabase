use google_sheets4 as sheets4;
use rocket::{post, serde::json::Json};
use serde_derive::{Deserialize, Serialize};

use hyper;
use hyper_rustls;
use sheets4::{api::ValueRange, oauth2, oauth2::ServiceAccountAuthenticator, Error, Sheets};

#[derive(Serialize, Deserialize)]
pub struct Information {
    first_name: String,
    last_name: String,
    email: String,
    phone_num: String,
    occupation: String,
    country: String,
}

#[post("/", format = "json", data = "<info>")]

pub async fn post_data(info: Json<Information>) {
    let creds = oauth2::read_service_account_key("serviceaccount.json")
        .await
        .expect("Can't read credential, an error occurred");

    let sa = ServiceAccountAuthenticator::builder(creds)
        .build()
        .await
        .expect("There was an error, trying to build connection with authenticator");

    let hub = Sheets::new(
        hyper::Client::builder().build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_or_http()
                .enable_http1()
                .enable_http2()
                .build(),
        ),
        sa,
    );

    let info = info.into_inner();

    let req = ValueRange {
        major_dimension: None,
        range: None,
        values: Some(vec![vec![
            info.first_name,
            info.last_name,
            info.email,
            info.phone_num,
            info.occupation,
            info.country,
        ]]),
    };

    // You can configure optional parameters by calling the respective setters at will, and
    // execute the final call using `doit()`.
    // Values shown here are possibly random and not representative !
    let result = hub
        .spreadsheets()
        .values_append(req, "1Wzp7fWqcgQNQsv7MxAj5wrPm7JrFstFP0RBSoAje8QI", "A2:F2")
        .value_input_option("USER_ENTERED")
        .doit()
        .await;

    match result {
        Err(e) => match e {
            // The Error enum provides details about what exactly happened.
            // You can also just use its `Debug`, `Display` or `Error` traits
            Error::HttpError(_)
            | Error::Io(_)
            | Error::MissingAPIKey
            | Error::MissingToken(_)
            | Error::Cancelled
            | Error::UploadSizeLimitExceeded(_, _)
            | Error::Failure(_)
            | Error::BadRequest(_)
            | Error::FieldClash(_)
            | Error::JsonDecodeError(_, _) => println!("{}", e),
        },
        Ok(res) => println!("Success: {:?}", res),
    }
}
