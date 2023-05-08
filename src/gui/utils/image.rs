use std::path::Path;

pub struct Image {
    name: String,
    path: String,
}

impl Image {
    pub fn new(name: &str, path: &str) -> Image {
        Image {
            name: String::from(name),
            path: String::from(path),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn load(&self) -> Result<tetra::graphics::Texture, tetra::TetraError> {
        tetra::graphics::Texture::new(Path::new(&self.path))
    }
}
