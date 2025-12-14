// src-tauri/src/lib.rs (or main.rs for Tauri v1)
use enigo::{Enigo, Keyboard, Settings};
use std::{thread, time};

// 1. Create the command
#[tauri::command]
fn type_text(text: String) {
    println!("Rust received: {}", text); // This prints to your terminal for debugging
    
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    // Loop through every character in the string
    for char in text.chars() {
        // Type the single character
        let _ = enigo.text(&char.to_string());
        
        // Wait 20ms between keys (mimics fast human typing)
        thread::sleep(time::Duration::from_millis(20));
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // 2. Register the command here so Frontend can find it
        .invoke_handler(tauri::generate_handler![type_text])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}