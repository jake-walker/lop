// Copyright (c) 2024 Jake Walker
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::{collections::HashMap, error::Error};

use super::{PasteService, Service, ServiceOptions, ShortenService, UploadService};
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
  fn shorten(&self, opts: &ServiceOptions, url: &str) -> Result<super::ServiceResult, Box<dyn Error>> {
    let mut form_data: HashMap<String, String> = HashMap::from([(String::from("url"), String::from(url))]);

    if let Some(expiry_epoch) = opts.expiry_epoch_ms() {
      form_data.insert(String::from("expires"), expiry_epoch.to_string());
    } else {
      form_data.insert(String::from("expires"), String::from("null"));
    }

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
  fn paste(&self, opts: &ServiceOptions, code: &str, _language: &str) -> Result<super::ServiceResult, Box<dyn Error>> {
    let mut form_data: HashMap<String, String> = HashMap::from([
      (String::from("code"), String::from(code)),
      // ("language", language)
    ]);

    if let Some(expiry_epoch) = opts.expiry_epoch_ms() {
      form_data.insert(String::from("expires"), expiry_epoch.to_string());
    } else {
      form_data.insert(String::from("expires"), String::from("null"));
    }

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
  fn upload<'a>(&self, opts: &ServiceOptions, data: Vec<u8>, file_name: String, mime_type: String) -> Result<super::ServiceResult, Box<dyn Error>> {
    let file_part = reqwest::blocking::multipart::Part::bytes(data)
      .file_name(file_name)
      .mime_str(&mime_type)?;
    let mut form = reqwest::blocking::multipart::Form::new()
      .part("file", file_part);

    if let Some(expiry_epoch) = opts.expiry_epoch_ms() {
      form = form.part("expires", reqwest::blocking::multipart::Part::text(expiry_epoch.to_string()));
    } else {
      form = form.part("expires", reqwest::blocking::multipart::Part::text(String::from("null")));
    }

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
