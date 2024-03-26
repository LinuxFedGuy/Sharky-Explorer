use fltk::*;
use fltk_theme::*;
use fltk::prelude::*;
use walkdir::WalkDir;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

fn main() {
    // Define the window
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut window = window::DoubleWindow::default().with_size(500, 500).with_label("Sharky Explorer || FPS: 0");

    // Theme button
    let mut theme_but = button::Button::new(470, 2, 20, 22, "L");
    
    // TextDisplay for file list
    let file_list = Arc::new(Mutex::new(text::TextDisplay::new(10, 30, 480, 430, "")));

    // Dir search
    let mut dir_search = input::Input::new(10,470, 130, 22, "");
    dir_search.insert("/").expect("Issue inserting into dir search!");

    // File search
    let file_search = input::Input::new(150,470, 130, 22, "");

    // Open folder 
    let mut open_folder = button::Button::new(390,470, 100, 22, "Open Folder");

    // Search for file
    let mut search = button::Button::new(285,470, 100, 22, "Search");

    // Clone file_list for the closure
    let file_list_clone = Arc::clone(&file_list);

    // Load the default dir
    clicked(&file_list_clone, "/");

    // Theme switch 
    let mut dark = true;

    // Theme modes 
    let dark_mode = ColorTheme::new(color_themes::DARK_THEME);
    let light_mode = ColorTheme::new(color_themes::GRAY_THEME);

    // Load dark mode by default 
    dark_mode.apply();

    // Theme Callback
    theme_but.set_callback(move |b| {
        if dark == true{
            // Light mode 
            b.set_label("D");
            light_mode.apply();
            dark = false;
        } else{
            // Dark mode
            b.set_label("L");
            dark_mode.apply();
            dark = true;
        }
    });

    // Call a new thread when the scan open folder is pressed to prevent freezing the window
    open_folder.set_callback(move |_| {
        let file_list_clone = Arc::clone(&file_list_clone);
        let dir_selection = dir_search.value();
        std::thread::spawn(move || {
            clicked(&file_list_clone, &dir_selection);
        });
    });
    
    // Call a new thread when the search for file is pressed to prevent freezing the window
    search.set_callback(move |_| {
        let file_list_clone = Arc::clone(&file_list);
        let file_search_clone = file_search.clone();
        std::thread::spawn(move || {
            search_for_file(&file_search_clone, &file_list_clone);
        });
    });

    // Make the window resizable
    window.make_resizable(true);

    // This adds a close function for when the x is pressed and shows the window
    window.end();
    window.show();

    // These are important for frame counting 
    let mut last_frame_time = Instant::now();
    let mut frame_count = 0;

    // Measure and update the title FPS
    app::add_idle3(move |_| {
        let elapsed = last_frame_time.elapsed();
        if elapsed >= Duration::from_secs(1) {
            // Update the title
            let fps = frame_count;
            window.set_label(&format!("Sharky Explorer || FPS: {}", fps));
            // Clear the frame count
            frame_count = 0;
            last_frame_time = Instant::now();
        } else {
            // Add as many frames as possible to the counter before one second has passed
            frame_count += 1;
        }
    });

    app.run().unwrap();
}

fn search_for_file(f: &input::Input, file_list: &Arc<Mutex<text::TextDisplay>>) {
    let search_text = f.value();

    let mut found_items = String::new();

    for entry in WalkDir::new("/").into_iter().filter_map(|e| e.ok()) {
        let item_name = entry.file_name().to_string_lossy();
        if item_name.contains(&search_text) {
            found_items.push_str(&format!("{}\n", entry.path().display()));
        }
    }

    // Lock the mutex and update the TextDisplay with the found files and directories
    if let Ok(mut guard) = file_list.lock() {
        guard.set_buffer(Some(text::TextBuffer::default()));
        guard.insert(&found_items);
    }
}


fn clicked(file_list: &Arc<Mutex<text::TextDisplay>>, current_dir: &str) {
    let mut file_names = String::new();

    // Get the contents of the current directory
    if let Ok(entries) = std::fs::read_dir(current_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(file_name) = path.file_name() {
                    file_names.push_str(&file_name.to_string_lossy());
                    if path.is_dir() {
                        file_names.push('/');
                    }
                    file_names.push('\n');
                }
            }
        }
    }

    // Lock the mutex and update the TextDisplay with the file list
    if let Ok(mut guard) = file_list.lock() {
        guard.set_buffer(Some(text::TextBuffer::default()));
        guard.insert(&file_names);
    }
}

