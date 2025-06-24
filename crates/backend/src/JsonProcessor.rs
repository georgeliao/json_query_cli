use crate::downloader;
use serde_json::Value;
struct JsonProcessor {
    json_content_parsed: Value,
}

impl JsonProcessor {
    pub fn new() -> Self {
        //TODO replace unwrap to ? later maybe
        return JsonProcessor {
            json_content_parsed: serde_json::from_str(&downloader::linear_download().unwrap())
                .unwrap(),
        };
    }

    pub fn get_supported_ubuntu_releases(&self) -> Vec<String> {
        let mut release_strs: Vec<String> = vec![];
        let products = self
            .json_content_parsed
            .get("products")
            .unwrap()
            .as_object()
            .unwrap();
        for (product_name, val) in products {
            if product_name.ends_with("amd64") {
                let supported = val.get("supported").unwrap().as_bool().unwrap();
                if (supported) {
                    let version = val.get("version").unwrap().as_str().unwrap();
                    release_strs.push(version.to_owned());
                }
            }
        }
        for (ele) in &release_strs {
            println!("{}", ele);
        }
        return release_strs;
    }

    pub fn get_current_ubuntu_lts(&self) -> Option<f64> {
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

    pub fn get_disk1_img_sha256_of_release(
        &self,
        product_name: &str,
        version_name: &str,
    ) -> Option<String> {
        return self
            .json_content_parsed
            .get("products")?
            .get(product_name)?
            .get("versions")?
            .get(version_name)?
            .get("items")?
            .get("disk1.img")?
            .get("sha256")?
            .as_str()
            .map(|s| s.to_owned());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_current_ubuntu_lts() {
        let processor = JsonProcessor::new();
        assert_eq!(processor.get_current_ubuntu_lts().unwrap(), 24.04);
    }

    #[test]
    fn test_get_supported_ubuntu_releases() {
        let processor = JsonProcessor::new();
        processor.get_supported_ubuntu_releases();
    }

    #[test]
    fn test_get_disk1_img_sha256_of_release() {
        let processor = JsonProcessor::new();
        let sha256 = processor
            .get_disk1_img_sha256_of_release("com.ubuntu.cloud:server:21.10:amd64", "20220708")
            .unwrap();
        println!("The sha256 value is {}", sha256);
    }
}
