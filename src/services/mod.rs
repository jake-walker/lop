// Copyright (c) 2024 Jake Walker
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::{error::Error, str::Bytes};

use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct ServiceResult {
  pub url: String,
  pub expires: Option<DateTime<Utc>>
}

pub trait Service {
  fn new() -> Result<Self, Box<dyn Error>> where Self: Sized;
}

pub trait ShortenService {
  fn shorten(&self, url: &str) -> Result<ServiceResult, Box<dyn Error>>;
}

pub trait UploadService {
  fn upload(&self, data: Vec<u8>, file_name: String, mime_type: String) -> Result<ServiceResult, Box<dyn Error>>;
}

pub trait PasteService {
  fn paste(&self, code: &str, language: &str) -> Result<ServiceResult, Box<dyn Error>>;
}

pub mod vh7;
