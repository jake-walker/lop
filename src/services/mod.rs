// Copyright (c) 2024 Jake Walker
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::error::Error;

use chrono::{DateTime, Duration, Utc};

#[derive(Debug)]
pub struct ServiceResult {
  pub url: String,
  pub expires: Option<DateTime<Utc>>
}

#[derive(Debug)]
pub struct ServiceOptions {
  pub expiry: Option<Duration>
}

impl ServiceOptions {
  fn expiry_epoch_ms(&self) -> Option<i64> {
    if let Some(expiry_value) = self.expiry {
      let ts = Utc::now() + expiry_value;
      return Some(ts.timestamp_millis())
    }

    None
  }
}

pub trait Service {
  fn new() -> Result<Self, Box<dyn Error>> where Self: Sized;
}

pub trait ShortenService {
  fn shorten(&self, opts: &ServiceOptions, url: &str) -> Result<ServiceResult, Box<dyn Error>>;
}

pub trait UploadService {
  fn upload(&self, opts: &ServiceOptions, data: Vec<u8>, file_name: String, mime_type: String) -> Result<ServiceResult, Box<dyn Error>>;
}

pub trait PasteService {
  fn paste(&self, opts: &ServiceOptions, code: &str, language: &str) -> Result<ServiceResult, Box<dyn Error>>;
}

pub mod vh7;
