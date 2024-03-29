use std::error::Error;

use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct ShortenResult {
  pub url: String,
  pub expires: Option<DateTime<Utc>>
}

#[derive(Debug)]
pub struct UploadResult {
  pub url: String,
  pub expires: Option<DateTime<Utc>>
}

#[derive(Debug)]
pub struct PasteResult {
  pub url: String,
  pub expires: Option<DateTime<Utc>>
}

pub trait Service {
  fn new() -> Result<Self, Box<dyn Error>> where Self: Sized;
}

pub trait ShortenService {
  fn shorten(&self, url: &str) -> Result<ShortenResult, Box<dyn Error>>;
}

pub trait UploadService {
  fn upload(&self, data: &[u8]) -> Result<UploadResult, Box<dyn Error>>;
}

pub trait PasteService {
  fn paste(&self, code: &str, language: &str) -> Result<PasteResult, Box<dyn Error>>;
}

pub mod vh7;
