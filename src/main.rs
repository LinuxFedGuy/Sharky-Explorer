use fltk::*;
use fltk::prelude::*;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

fn main() {
    // Define the window
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut window = window::DoubleWindow::default().with_size(500, 500).with_label("Sharky Explorer || FPS: 0");

    // TextDisplay for file list
    let file_list = Arc::new(Mutex::new(text::TextDisplay::new(10, 10, 480, 450, "")));

    // Dir search
    let input = input::Input::new(10,470, 370, 30, "");

    // Button 
    let mut forward = button::Button::new(390,470, 100, 30, "Open Folder");

    // Clone file_list for the closure
    let file_list_clone = Arc::clone(&file_list);

    // Load the default dir
    clicked(&file_list_clone, "/");

    // Call a new thread when the scan forward is pressed to prevent freezing the window
    forward.set_callback(move |_| {
        let file_list_clone = Arc::clone(&file_list_clone);
        let dir_selection = input.value();
        std::thread::spawn(move || {
            clicked(&file_list_clone, &dir_selection);
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

