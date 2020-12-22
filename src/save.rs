use std::path::{Path, PathBuf};

pub trait Savable {
    fn save(&self, save_path: PathBuf);
    fn load(&mut self, save_path: PathBuf);
}
