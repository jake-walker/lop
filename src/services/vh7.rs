use std::error::Error;

use super::{PasteService, Service, ShortenService};
use chrono::DateTime;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use url::Url;

static DEFAULT_API_URL: &'static str = "https://vh7.uk/api/";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Vh7ShortenResult {
  id: String,
  r#type: String,
  created_at: String,
  expires_at: Option<String>,
  updated_at: Option<String>,
  url: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Vh7PasteResult {
  id: String,
  r#type: String,
  created_at: String,
  expires_at: Option<String>,
  updated_at: Option<String>,
  code: String,
  language: Option<String>
}

pub struct Vh7Service {
  api_base: Url,
  client: Client
}

impl Service for Vh7Service {
  fn new() -> Result<Self, Box<dyn Error>> {
    let client = Client::new();

    Ok(Vh7Service {
      api_base: Url::parse(&DEFAULT_API_URL)?,
      client
    })
  }
}

impl ShortenService for Vh7Service {
  fn shorten(&self, url: &str) -> Result<super::ServiceResult, Box<dyn Error>> {
    let form_data = [("url", url)];

    let shorten_route = self.api_base.join("shorten")?;
    let res = self.client.post(shorten_route)
      .form(&form_data)
      .send()?;

    let res_data = res.json::<Vh7ShortenResult>()?;

    Ok(super::ServiceResult {
      url: self.api_base.join("..")?.join(&res_data.id)?.to_string(),
      expires: res_data.expires_at.and_then(|d| Some(DateTime::parse_from_rfc3339(&d).ok()?.to_utc()))
    })
  }
}

impl PasteService for Vh7Service {
  fn paste(&self, code: &str, _language: &str) -> Result<super::ServiceResult, Box<dyn Error>> {
    let form_data = [
      ("code", code),
      // ("language", language)
    ];

    let paste_route = self.api_base.join("paste")?;
    let res = self.client.post(paste_route)
      .form(&form_data)
      .send()?;

    let res_data = res.json::<Vh7PasteResult>()?;

    Ok(super::ServiceResult {
      url: self.api_base.join("..")?.join(&res_data.id)?.to_string(),
      expires: res_data.expires_at.and_then(|d| Some(DateTime::parse_from_rfc3339(&d).ok()?.to_utc()))
    })
  }
}
