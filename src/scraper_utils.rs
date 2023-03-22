use std::sync::Mutex;
use std::time::Instant;

use lazy_static::lazy_static;
use reqwest::Error;

pub fn do_throttled_request(url: &str) -> Result<String, Error> {
    let response = reqwest::blocking::get(url)?;
    return response.text();
}