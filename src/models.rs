use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub jti: String,
    pub iss: String,
    pub aud: Option<String>,
    pub sub: String,
    pub typ: String,
    pub azp: String,
    pub sid: String,
    pub acr: String,
    #[serde(rename = "allowed-origin")]
    pub allowed_origins: Option<Vec<String>>,
    pub realm_access: Option<RealmAccess>,
    pub resource_access: Option<HashMap<String, ClientRoles>>,
    pub scope: String,
    pub email_verified: bool,
    pub name: String,
    pub preferred_username: String,
    pub given_name: String,
    pub family_name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealmAccess {
    pub roles: Vec<String>,
}

/*

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceAccess {
    #[serde(rename = "realm-management")]
    pub realm_management: Option<ClientRoles>,
    #[serde(rename = "helpdesk-client")]
    pub helpdesk_client: Option<ClientRoles>,
    pub broker: Option<ClientRoles>,
    pub account: Option<ClientRoles>,
} */

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientRoles {
    pub roles: Vec<String>,
}


