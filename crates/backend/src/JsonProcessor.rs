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
        return vec![String::from("")];
    }

    pub fn get_current_ubuntu_lts(&self) -> f64 {
        let mut current_version = 0.0;

        let products = self
            .json_content_parsed
            .get("products")
            .unwrap()
            .as_object()
            .unwrap();
        for (key, val) in products {
            if key.ends_with("amd64") {
                let release_title_value: &str = val.get("release_title").unwrap().as_str().unwrap();
                if release_title_value.ends_with("LTS") {
                    let version_value = val
                        .get("version")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .parse::<f64>()
                        .unwrap();
                    current_version = f64::max(current_version, version_value);
                }
            }
        }

        return current_version;
    }

    pub fn get_sha256(&self, ubuntu_release: String) -> String {
        return String::new();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_current_ubuntu_lts() {
        let processor = JsonProcessor::new();
        assert_eq!(processor.get_current_ubuntu_lts(), 24.04);
    }
}
