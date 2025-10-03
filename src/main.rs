mod clipboard;
mod storage;

use clipboard::ClipboardMonitor;
use storage::{ClipboardContent, ClipboardStorage};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tray_icon::{
    menu::{Menu, MenuEvent, MenuId, MenuItem, PredefinedMenuItem, accelerator::{Accelerator, Modifiers, Code}},
    TrayIconBuilder,
};
use winit::event::Event;
use winit::event_loop::{ControlFlow, EventLoopBuilder};
use global_hotkey::{GlobalHotKeyManager, hotkey::{HotKey, Modifiers as HKModifiers, Code as HKCode}, GlobalHotKeyEvent};

const MENU_ITEMS_LIMIT: usize = 25;

#[derive(Debug, Clone)]
enum AppEvent {
    UpdateMenu,
    PasteItem(usize),
    ClearAll,
    Quit,
}

#[derive(Default)]
struct MenuState {
    clear_all: MenuId,
    quit: MenuId,
    item_ids: HashMap<MenuId, usize>, // menu id -> clipboard item index
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let storage = Arc::new(Mutex::new(ClipboardStorage::new()));
    let storage_clone = Arc::clone(&storage);

    // Build winit event loop with user events
    let event_loop = EventLoopBuilder::with_user_event().build().unwrap();
    let proxy = event_loop.create_proxy();

    // Spawn clipboard monitoring thread
    let monitor_proxy = proxy.clone();
    thread::spawn(move || {
        let mut monitor = ClipboardMonitor::new().expect("Failed to create clipboard monitor");
        loop {
            if let Ok(mut storage) = storage_clone.lock() {
                let old_len = storage.len();
                let _ = monitor.check_and_store(&mut storage);
                let new_len = storage.len();

                // If new item was added, update menu
                if new_len > old_len {
                    let _ = monitor_proxy.send_event(AppEvent::UpdateMenu);
                }
            }
            thread::sleep(Duration::from_millis(500));
        }
    });

    // Create initial tray menu
    let menu = Menu::new();
    let mut tray = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_tooltip("Klippy - Clipboard Manager")
        .with_title("📎")
        .build()?;

    // Shared state for menu item IDs
    let menu_state = Arc::new(Mutex::new(MenuState::default()));

    // Initial menu build
    rebuild_menu(&mut tray, &storage, &menu_state);

    // Register global hotkeys for Cmd+0-9
    let manager = GlobalHotKeyManager::new().expect("Failed to create hotkey manager");
    let mut hotkeys = Vec::new();

    for i in 0..=9 {
        let code = match i {
            0 => HKCode::Digit0,
            1 => HKCode::Digit1,
            2 => HKCode::Digit2,
            3 => HKCode::Digit3,
            4 => HKCode::Digit4,
            5 => HKCode::Digit5,
            6 => HKCode::Digit6,
            7 => HKCode::Digit7,
            8 => HKCode::Digit8,
            9 => HKCode::Digit9,
            _ => continue,
        };
        let hotkey = HotKey::new(Some(HKModifiers::SUPER), code);
        manager.register(hotkey).expect("Failed to register hotkey");
        hotkeys.push((hotkey.id(), i));
    }


    // Spawn global hotkey event listener
    let hotkey_proxy = proxy.clone();
    thread::spawn(move || {
        let hotkey_rx = GlobalHotKeyEvent::receiver();
        while let Ok(event) = hotkey_rx.recv() {
            for (id, index) in &hotkeys {
                if event.id == *id {
                    let _ = hotkey_proxy.send_event(AppEvent::PasteItem(*index));
                    break;
                }
            }
        }
    });

    // Spawn menu event listener thread
    let menu_proxy = proxy.clone();
    let menu_state_clone = Arc::clone(&menu_state);
    thread::spawn(move || {
        let menu_rx = MenuEvent::receiver();
        while let Ok(event) = menu_rx.recv() {
            if let Ok(state) = menu_state_clone.lock() {
                if event.id == state.quit {
                    let _ = menu_proxy.send_event(AppEvent::Quit);
                } else if event.id == state.clear_all {
                    let _ = menu_proxy.send_event(AppEvent::ClearAll);
                } else if let Some(&index) = state.item_ids.get(&event.id) {
                    let _ = menu_proxy.send_event(AppEvent::PasteItem(index));
                }
            }
        }
    });

    // Run event loop
    let _ = event_loop.run(move |event, elwt| {
        elwt.set_control_flow(ControlFlow::Wait);
        match event {
            Event::UserEvent(AppEvent::UpdateMenu) => {
                rebuild_menu(&mut tray, &storage, &menu_state);
            }
            Event::UserEvent(AppEvent::PasteItem(index)) => {
                if let Ok(storage) = storage.lock() {
                    let items = storage.get_all();
                    if let Some(item) = items.get(index) {
                        // Set clipboard to selected item
                        if let Ok(mut monitor) = ClipboardMonitor::new() {
                            let _ = monitor.set_clipboard(&item.content);
                        }
                    }
                }
            }
            Event::UserEvent(AppEvent::ClearAll) => {
                if let Ok(mut storage) = storage.lock() {
                    storage.clear();
                }
                rebuild_menu(&mut tray, &storage, &menu_state);
            }
            Event::UserEvent(AppEvent::Quit) => {
                elwt.exit();
            }
            _ => {}
        }
    });

    Ok(())
}

fn rebuild_menu(
    tray: &mut tray_icon::TrayIcon,
    storage: &Arc<Mutex<ClipboardStorage>>,
    menu_state: &Arc<Mutex<MenuState>>,
) {
    let menu = Menu::new();

    if let Ok(storage) = storage.lock() {
        let items = storage.get_all();
        let display_count = items.len().min(MENU_ITEMS_LIMIT);

        if display_count == 0 {
            let empty_item = MenuItem::new("No clipboard history", false, None);
            let _ = menu.append(&empty_item);
        } else {
            // Reset menu state
            if let Ok(mut state) = menu_state.lock() {
                state.item_ids.clear();
            }

            // Add clipboard items (up to 25)
            for (i, item) in items.iter().take(display_count).enumerate() {
                let label = format_item_label(item, i);

                // Add native keyboard accelerator for items 0-9
                let accelerator = if i < 10 {
                    // Create Cmd+number shortcut (Cmd+0, Cmd+1, etc.)
                    let code = match i {
                        0 => Code::Digit0,
                        1 => Code::Digit1,
                        2 => Code::Digit2,
                        3 => Code::Digit3,
                        4 => Code::Digit4,
                        5 => Code::Digit5,
                        6 => Code::Digit6,
                        7 => Code::Digit7,
                        8 => Code::Digit8,
                        9 => Code::Digit9,
                        _ => Code::Digit0,
                    };
                    Some(Accelerator::new(Some(Modifiers::SUPER), code))
                } else {
                    None
                };

                let menu_item = MenuItem::new(&label, true, accelerator);
                let _ = menu.append(&menu_item);

                // Store mapping
                if let Ok(mut state) = menu_state.lock() {
                    state.item_ids.insert(menu_item.id().clone(), i);
                }
            }
        }

        let _ = menu.append(&PredefinedMenuItem::separator());

        // Clear All
        let clear_item = MenuItem::new("Clear All", true, None);
        let _ = menu.append(&clear_item);

        // Quit
        let quit_item = MenuItem::new("Quit", true, None);
        let _ = menu.append(&quit_item);

        // Update menu state IDs
        if let Ok(mut state) = menu_state.lock() {
            state.clear_all = clear_item.id().clone();
            state.quit = quit_item.id().clone();
        }
    }

    let _ = tray.set_menu(Some(Box::new(menu)));
}

fn format_item_label(item: &storage::ClipboardItem, _index: usize) -> String {
    match &item.content {
        ClipboardContent::Text(text) => {
            // Show first 50 chars, respecting UTF-8 boundaries
            let preview = if text.chars().count() > 50 {
                let truncated: String = text.chars().take(50).collect();
                format!("{}...", truncated)
            } else {
                text.clone()
            };
            // Replace newlines with spaces
            preview.replace('\n', " ").replace('\r', " ")
        }
        ClipboardContent::Image(_) => "[Image]".to_string(),
    }
}
