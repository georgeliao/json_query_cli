pub trait JsonProcessor {
    fn get_supported_ubuntu_releases(&self) -> Option<Vec<String>>;
    fn get_current_ubuntu_lts(&self) -> Option<f64>;
    fn get_disk1_img_sha256_of_release(&self, release_version: &str) -> Option<String>;
}