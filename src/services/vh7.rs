// Copyright (c) 2024 Jake Walker
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::error::Error;

use super::{PasteService, Service, ShortenService, UploadService};
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Vh7UploadResult {
  id: String,
  r#type: String,
  created_at: String,
  expires_at: Option<String>,
  updated_at: Option<String>,
  filename: String,
  hash: String,
  size: i64
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

impl UploadService for Vh7Service {
  fn upload<'a>(&self, data: Vec<u8>, file_name: String, mime_type: String) -> Result<super::ServiceResult, Box<dyn Error>> {
    let file_part = reqwest::blocking::multipart::Part::bytes(data)
      .file_name(file_name)
      .mime_str(&mime_type)?;
    let form = reqwest::blocking::multipart::Form::new()
      .part("file", file_part);

    let upload_route = self.api_base.join("upload")?;
    let res = self.client.post(upload_route)
      .multipart(form)
      .send()?;

    let res_data = res.json::<Vh7UploadResult>()?;

    Ok(super::ServiceResult {
      url: self.api_base.join("..")?.join(&res_data.id)?.to_string(),
      expires: res_data.expires_at.and_then(|d| Some(DateTime::parse_from_rfc3339(&d).ok()?.to_utc()))
    })
  }
}
