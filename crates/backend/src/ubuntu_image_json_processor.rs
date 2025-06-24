use crate::{downloader, json_processor::JsonProcessor};
use serde_json::Value;
pub struct UbuntuImageJsonProcessor {
    json_content_parsed: Value,
}
// pub type Result<T> = std::result::Result<T, Error>;
impl UbuntuImageJsonProcessor {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        return Ok(UbuntuImageJsonProcessor {
            json_content_parsed: serde_json::from_str(&downloader::linear_download()?)?,
        });
    }
}

impl JsonProcessor for UbuntuImageJsonProcessor {
    fn get_supported_ubuntu_releases(&self) -> Option<Vec<String>> {
        let mut release_strs: Vec<String> = vec![];
        let products = self.json_content_parsed.get("products")?.as_object()?;
        for (product_name, val) in products {
            if product_name.ends_with("amd64") {
                let supported = val.get("supported")?.as_bool()?;
                if supported {
                    let version = val.get("version")?.as_str()?;
                    release_strs.push(version.to_owned());
                }
            }
        }

        return Some(release_strs);
    }

    fn get_current_ubuntu_lts(&self) -> Option<f64> {
        let mut current_version: f64 = 0.0;

        let products = self.json_content_parsed.get("products")?.as_object()?;
        for (product_name, val) in products {
            if product_name.ends_with("amd64") {
                let release_title_value: &str = val.get("release_title")?.as_str()?;
                if release_title_value.ends_with("LTS") {
                    let version_value = val.get("version")?.as_str()?.parse::<f64>().ok()?;
                    current_version = f64::max(current_version, version_value);
                }
            }
        }

        return Some(current_version);
    }

    fn get_disk1_img_sha256_of_release(&self, release_version: &str) -> Option<String> {
        let products = self.json_content_parsed.get("products")?.as_object()?;
        for (product_name, val) in products {
            if product_name.ends_with("amd64") {
                let current_release_version: &str = val.get("version")?.as_str()?;
                if current_release_version == release_version {
                    let last_version: (&String, &Value) =
                        val.get("versions")?.as_object()?.iter().last()?;
                    return last_version
                        .1
                        .get("items")?
                        .get("disk1.img")?
                        .get("sha256")?
                        .as_str()
                        .map(|s| s.to_owned());
                }
            }
        }

        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_current_ubuntu_lts() {
        let processor = UbuntuImageJsonProcessor::new().unwrap();
        assert_eq!(processor.get_current_ubuntu_lts().unwrap(), 24.04);
    }

    #[test]
    fn test_get_supported_ubuntu_releases() {
        let processor = UbuntuImageJsonProcessor::new().unwrap();
        processor.get_supported_ubuntu_releases();
    }

    #[test]
    fn test_get_disk1_img_sha256_of_release() {
        let processor = UbuntuImageJsonProcessor::new().unwrap();
        assert_eq!(
            processor.get_disk1_img_sha256_of_release("24.10").unwrap(),
            "8446856f1903fd305a17cfb610bbb6c01e8e2230cdf41d44fc9e3d824f747ff4"
        )
    }

    #[test]
    fn test_get_disk1_img_sha256_of_release_non_exist() {
        let processor = UbuntuImageJsonProcessor::new().unwrap();
        assert!(processor.get_disk1_img_sha256_of_release("24.05").is_none())
    }
}
