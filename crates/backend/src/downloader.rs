use std::error;

pub fn linear_download() -> Result<String, Box<dyn error::Error>> {
    // TODO, mock the downloader to facilitate unit testing.
    const URL: &str = "https://cloud-images.ubuntu.com/releases/streams/v1/com.ubuntu.cloud:released:download.json";
    let response: String = reqwest::blocking::get(URL)?.text()?;

    return Ok(response);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_download() {
        assert!(linear_download().is_ok());
    }
}
