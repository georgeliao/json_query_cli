use crate::downloader;

// #[derive(Default)]
struct JsonProcessor {
    json_content: String,
}

impl JsonProcessor {
    pub fn new() -> Self {
        //TODO replace unwrap to ? later maybe
        return JsonProcessor {
            json_content: downloader::linear_download().unwrap(),
        };
    }

    pub fn get_supported_ubuntu_releases() -> Vec<String> {
        return vec![];
    }

    pub fn get_current_ubuntu_lts() -> String {
        return String::new();
    }

    pub fn get_sha256(ubuntu_release : String) -> String {
        return String::new();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_download() {
        let processor = JsonProcessor::new();
    }
}
