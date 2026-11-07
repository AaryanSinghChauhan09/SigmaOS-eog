//! eog model — Eye of GNOME image viewer.

#[derive(Debug, Clone)]
pub struct Image {
    pub id: String,
    pub path: String,
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub zoom: f64,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub images: Vec<Image>,
    pub next_id: u64,
}

impl Default for Model {
    fn default() -> Self { Self { images: vec![], next_id: 1 } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn open(&mut self, path: &str, title: &str, width: u32, height: u32) -> String {
        let id = format!("{:x}", self.next_id);
        self.next_id += 1;
        self.images.push(Image { id: id.clone(), path: path.into(), title: title.into(), width, height, zoom: 1.0 });
        id
    }

    pub fn next(&mut self, id: &str) -> bool {
        if let Some(i) = self.images.iter_mut().find(|i| i.id == id) { i.zoom *= 1.1; true } else { false }
    }

    pub fn prev(&mut self, id: &str) -> bool {
        if let Some(i) = self.images.iter_mut().find(|i| i.id == id) { i.zoom /= 1.1; true } else { false }
    }

    pub fn list(&self) -> &[Image] { &self.images }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn image_zoom() {
        let mut e = Model::new();
        let id = e.open("/tmp/test.png", "Test", 800, 600);
        assert!(e.next(&id));
        assert!(e.prev(&id));
    }
}
