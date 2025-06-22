use std::error;

pub fn linear_download() -> Result<String, Box<dyn error::Error>> {
    let download_url: &str = "https://cloud-images.ubuntu.com/releases/streams/v1/com.ubuntu.cloud:released:download.json";
    let response: String = reqwest::blocking::get(download_url)?.text()?;

    return Ok(response);
}