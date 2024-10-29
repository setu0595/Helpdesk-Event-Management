use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, TokenData};
use reqwest::Client;
use crate::models::Claims;
use std::error::Error;
use serde_json::Value;
use std::fs::File;
use std::io::Read;

pub async fn authenticate_user(client: &Client, username: &str, password: &str) -> Result<String, Box<dyn Error>> {
    //println!("{}", username);
    let params = [
        ("client_id", "helpdesk-client"),
        ("client_secret", "YOUR_SECRET_KEY"),
        ("grant_type", "password"),
        ("username", username),
        ("password", password),
    ];
    //println!("Here in auth...");
    let res = client
        .post("http://localhost:8080/auth/realms/helpdesk-realm/protocol/openid-connect/token")
        .form(&params)
        .send()
        .await?;

    // Print the raw response text for debugging
    let raw_body = res.text().await?;
    //println!("Raw response: {}", raw_body);

    // Try to parse the JSON response
    let json_body: Value = serde_json::from_str(&raw_body)?;

    // Check if the "access_token" field exists
    if let Some(access_token) = json_body.get("access_token") {
        if let Some(access_token_str) = access_token.as_str() {
            return Ok(access_token_str.to_string());
        }
    }

    // If no access token is found, return an error
    Err("Missing access token in response".into())
}


pub fn decode_token(token: &str) -> Result<TokenData<Claims>, Box<dyn Error>> {
    //println!("in decode token");
    // Load the RSA public key from a PEM file (you can load it from a string too)
    let mut pem_file = File::open("public_key.pem")?;
    let mut pem = String::new();
    pem_file.read_to_string(&mut pem)?;

    // Create the DecodingKey from the RSA public key
    let decoding_key = DecodingKey::from_rsa_pem(pem.as_bytes())?;

    // Define validation criteria for RS256
    let validation = Validation::new(Algorithm::RS256);

    // Decode the token
    let token_data = decode::<Claims>(token, &decoding_key, &validation)?;
    //println!("token data {:?}", token_data);
    Ok(token_data)
}

