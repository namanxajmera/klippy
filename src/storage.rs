use std::time::SystemTime;

const MAX_ITEMS: usize = 50;

#[derive(Clone, Debug)]
pub enum ClipboardContent {
    Text(String),
    Image(Vec<u8>),
}

#[derive(Clone, Debug)]
pub struct ClipboardItem {
    pub content: ClipboardContent,
    #[allow(dead_code)]
    pub timestamp: SystemTime,
}

pub struct ClipboardStorage {
    items: Vec<ClipboardItem>,
}

impl ClipboardStorage {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add(&mut self, content: ClipboardContent) {
        let item = ClipboardItem {
            content,
            timestamp: SystemTime::now(),
        };

        // Add to front
        self.items.insert(0, item);

        // Keep only last MAX_ITEMS
        if self.items.len() > MAX_ITEMS {
            self.items.truncate(MAX_ITEMS);
        }
    }

    pub fn get_all(&self) -> &[ClipboardItem] {
        &self.items
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}
