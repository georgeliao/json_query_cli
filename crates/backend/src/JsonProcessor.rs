use crate::downloader;

// #[derive(Default)]
struct JsonProcessor {
    json_content : String,
}

impl JsonProcessor {
    pub fn new() -> Self {
        //TODO replace unwrap to ? later maybe
        return JsonProcessor {json_content : downloader::linear_download().unwrap()};
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
