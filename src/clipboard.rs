use arboard::{Clipboard, ImageData};
use crate::storage::{ClipboardContent, ClipboardStorage};

pub struct ClipboardMonitor {
    clipboard: Clipboard,
    last_text: Option<String>,
    last_image_hash: Option<u64>,
}

impl ClipboardMonitor {
    pub fn new() -> Result<Self, arboard::Error> {
        Ok(Self {
            clipboard: Clipboard::new()?,
            last_text: None,
            last_image_hash: None,
        })
    }

    pub fn check_and_store(&mut self, storage: &mut ClipboardStorage) -> Result<(), arboard::Error> {
        // Try text first
        if let Ok(text) = self.clipboard.get_text() {
            if Some(&text) != self.last_text.as_ref() {
                // Check if this text is already in storage (avoid duplicates from paste)
                let items = storage.get_all();
                let already_exists = items.iter().any(|item| {
                    matches!(&item.content, ClipboardContent::Text(t) if t == &text)
                });

                self.last_text = Some(text.clone());

                if !already_exists {
                    storage.add(ClipboardContent::Text(text));
                }
                return Ok(());
            }
        }

        // Try image
        if let Ok(image) = self.clipboard.get_image() {
            let hash = Self::hash_image(&image);
            if Some(hash) != self.last_image_hash {
                self.last_image_hash = Some(hash);
                let rgba_data = image.bytes.to_vec();
                storage.add(ClipboardContent::Image(rgba_data));
                return Ok(());
            }
        }

        Ok(())
    }

    pub fn set_clipboard(&mut self, content: &ClipboardContent) -> Result<(), arboard::Error> {
        match content {
            ClipboardContent::Text(text) => {
                self.clipboard.set_text(text)?;
                self.last_text = Some(text.clone());
            }
            ClipboardContent::Image(_data) => {
                // For now, skip image pasting - we'll add this later if needed
                // Images are complex to paste back correctly
            }
        }
        Ok(())
    }

    fn hash_image(image: &ImageData) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        image.bytes.hash(&mut hasher);
        hasher.finish()
    }
}
