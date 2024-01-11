#![windows_subsystem = "windows"]

use std::fs;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

fn main() {
    // Specify the names of the shortcuts you want to remove
    let shortcut_names = ["Roblox Player.lnk", "Roblox Studio.lnk"];

    // Get the desktop directory path
    let desktop_path = get_desktop_path();

    // Start a thread to continuously check for shortcuts
    thread::spawn(move || loop {
        for &shortcut_name in &shortcut_names {
            // Construct the full path of the shortcut
            let shortcut_path = desktop_path.join(shortcut_name);

            // Check if the shortcut exists
            if shortcut_path.exists() {
                // Remove the shortcut
                if let Err(err) = fs::remove_file(&shortcut_path) {
                    eprintln!("Error removing shortcut '{}': {}", shortcut_name, err);
                } else {
                    println!("Shortcut '{}' removed successfully.", shortcut_name);
                }
            }
        }

        // Sleep for a few seconds before checking again
        thread::sleep(Duration::from_secs(5));
    });

    // Keep the main thread running
    loop {
        // You can add other background tasks or just sleep
        thread::sleep(Duration::from_secs(60));
    }
}

fn get_desktop_path() -> PathBuf {
    // Get the user's home directory
    let home_dir = match dirs::home_dir() {
        Some(path) => path,
        None => {
            eprintln!("Error getting home directory.");
            std::process::exit(1);
        }
    };

    // Construct the desktop path
    let desktop_path = home_dir.join("Desktop");

    desktop_path
}