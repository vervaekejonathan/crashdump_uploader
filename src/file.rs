use crate::mail;

use aws_sdk_sesv2::Error;
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs;
use std::sync::mpsc::channel;
use std::time::Duration;

// loop over all files in the directory and handle each file found
pub async fn handle_all_files_in_folder(email: &str, subject: &str, path: &str) {
    let paths = fs::read_dir(path).unwrap();
    for path in paths {
        match path {
            Ok(dir_entry) => {
                // retrieve filename from entry
                let path = dir_entry.path();
                let filename = path.to_str().unwrap();
                println!("Name: {}", filename);

                // e-mail file content and delete it
                handle_file(&email, &subject, &filename).await;
            }
            Err(_) => {}
        }
    }
}

pub async fn watch_all_files_in_folder(
    email: &str,
    subject: &str,
    path: &str,
) -> Result<(), Error> {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2)).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path, RecursiveMode::Recursive).unwrap();

    // Wait until a monitored folder notifies a change
    loop {
        match rx.recv() {
            Ok(event) => {
                //println!(">>> {:?}", event);
                match event {
                    // Hits after a file is created
                    DebouncedEvent::Create(file_path) => {
                        let filename = file_path.as_path().to_str().unwrap();
                        handle_file(&email, &subject, filename).await;
                    }
                    // Hits after a file is overwritten
                    DebouncedEvent::Write(file_path) => {
                        let filename = file_path.as_path().to_str().unwrap();
                        handle_file(&email, &subject, filename).await;
                    }
                    // Event not useful for our cause
                    _ => {}
                }
            }
            Err(e) => println!("Watcher error: {:?}", e),
        }
    }
}

// read the content of a file, e-mail it and remove the file
async fn handle_file(email: &str, serialnumber: &str, filename: &str) {
    println!("\t{}", filename);
    // read the files content
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    // e-mail the files content through AWS SES
    match mail::mail_support(email, serialnumber, &contents).await {
        Ok(_) => {
            // remove the e-mailed file
            // CAREFUL ENABLING THIS FEATURE
            // fs::remove_file(filename).unwrap()
        }
        Err(err) => {
            println!("Error sending mail: {}", err)
        }
    }
}
