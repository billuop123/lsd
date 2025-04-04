use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;
use colored::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let group_dirs_first = args.contains(&"--dir".to_string());
    let recursive = args.contains(&"--recursive".to_string());
    let sort_by_name = args.contains(&"--sort-name".to_string());
    let sort_by_size = args.contains(&"--sort-size".to_string());
    let sort_by_date = args.contains(&"--sort-date".to_string());
    let filter_files = args.contains(&"--files".to_string());
    let filter_dirs = args.contains(&"--dirs".to_string());

    let path = ".";
    list_directory(path, group_dirs_first, recursive, sort_by_name, sort_by_size, sort_by_date, filter_files, filter_dirs, 0);
}

fn list_directory(
    path: &str,
    group_dirs_first: bool,
    recursive: bool,
    sort_by_name: bool,
    sort_by_size: bool,
    sort_by_date: bool,
    filter_files: bool,
    filter_dirs: bool,
    indent_level: usize,
) {
    let entries = fs::read_dir(path).expect("Failed to read directory");

    let mut directories = Vec::new();
    let mut files = Vec::new();
    let mut others = Vec::new();

    for entry in entries {
        match entry {
            Ok(entry) => {
                let entry_path = entry.path();
                let entry_name = entry_path.file_name().unwrap_or_default().to_str().unwrap_or("").to_string();

                // Skip hidden files and directories
                if entry_name.starts_with('.') {
                    continue;
                }

                let metadata = entry.metadata().unwrap();
                let size = metadata.len();
                let modified = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);

                if entry_path.is_dir() {
                    directories.push((entry_name, size, modified));
                } else if entry_path.is_file() {
                    files.push((entry_name, size, modified));
                } else {
                    others.push((entry_name, size, modified));
                }
            }
            Err(e) => println!("Error reading entry: {}", e),
        }
    }

    // Sorting logic
    if sort_by_name {
        directories.sort_by(|a, b| a.0.cmp(&b.0));
        files.sort_by(|a, b| a.0.cmp(&b.0));
        others.sort_by(|a, b| a.0.cmp(&b.0));
    } else if sort_by_size {
        directories.sort_by(|a, b| a.1.cmp(&b.1));
        files.sort_by(|a, b| a.1.cmp(&b.1));
        others.sort_by(|a, b| a.1.cmp(&b.1));
    } else if sort_by_date {
        directories.sort_by(|a, b| a.2.cmp(&b.2));
        files.sort_by(|a, b| a.2.cmp(&b.2));
        others.sort_by(|a, b| a.2.cmp(&b.2));
    }

    // Print directories first if specified
    if group_dirs_first {
        for (dir, size, modified) in &directories {
            if !filter_files {
                print_entry(dir, *size, *modified, "📁", indent_level, Color::Blue);
            }
        }
    }

    // Print files
    for (file, size, modified) in &files {
        if !filter_dirs {
            print_entry(file, *size, *modified, "📄", indent_level, Color::Green);
        }
    }

    // Print other entries (if any)
    for (other, size, modified) in &others {
        print_entry(other, *size, *modified, "🔗", indent_level, Color::Yellow);
    }

    // Print directories last if not grouped first
    if !group_dirs_first {
        for (dir, size, modified) in &directories {
            if !filter_files {
                print_entry(dir, *size, *modified, "📁", indent_level, Color::Blue);
            }
        }
    }

    // Recursive listing
    if recursive {
        for (dir, _, _) in &directories {
            let new_path = Path::new(path).join(dir);
            println!("\n{}Subdirectory: {}", " ".repeat(indent_level * 2), dir);
            list_directory(
                new_path.to_str().unwrap(),
                group_dirs_first,
                recursive,
                sort_by_name,
                sort_by_size,
                sort_by_date,
                filter_files,
                filter_dirs,
                indent_level + 1,
            );
        }
    }
}

fn print_entry(name: &str, size: u64, modified: SystemTime, icon: &str, indent_level: usize, color: Color) {
    let modified_time = modified
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let size_str = format!(" ({} bytes)", size);
    // let time_str = format!(" (modified: {} seconds ago)", modified_time);
    
    println!(
        "{}{} {}{}",
        " ".repeat(indent_level * 2),
        icon,
        name.color(color),
        size_str.color(Color::White),
        // time_str.color(Color::Magenta), 
    );
}