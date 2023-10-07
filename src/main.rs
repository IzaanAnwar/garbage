use clap::Parser;
use dir;
use std::{fs, path, io};

#[derive(Debug, Parser)]
#[command(author = "Izaan Anwar", version = "1.0.0", about = "A Reycle Bin")]
struct CLI {
    /// optional file name to delete
    file_name: Option<String>,

    /// option to empty the garbage dir
    #[arg(long, value_name = "empty")]
    empty: bool,

    /// option to restore files
    #[arg(long, value_name = "restore")]
    restore: bool,
}

fn create_garbage_dir() -> Result<String, std::io::Error> {
    if let Some(home_folder) = dir::home_dir() {
        let garbage_dir = home_folder.join(".local/share/Garbage");
        if !garbage_dir.exists() {
            fs::create_dir_all(&garbage_dir)?;

            fs::create_dir(&garbage_dir.join("garbage"))?;
            fs::create_dir(&garbage_dir.join("garbageInfo"))?;
            println!("Created 'garbage' directory at: {}", garbage_dir.display());
        }
        let garbage_dir_str = garbage_dir.to_str().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to convert path to string",
            )
        })?;
        let garbage_dir_str = garbage_dir_str.to_string();
        Ok(garbage_dir_str)
    } else {
        eprintln!("Failed to determine the user's home directory.");
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Home directory not found",
        ))
    }
}

fn remove_all_file(garbage_files_dir: &path::Path) -> Result<(), io::Error> {
    if !garbage_files_dir.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Not a valid directory path",
        ));                
    }
    for entry in fs::read_dir(garbage_files_dir)? {
        let entry = entry?; 
        let entry_path = entry.path();

        if entry_path.is_file() {
            fs::remove_file(entry_path)?;
        }
    }
    Ok(())

}

fn main() {
    match create_garbage_dir() {
        Ok(dir) => {
            let cli = CLI::parse();

            if let Some(filename) = &cli.file_name {
                let file = match path::Path::new(filename).file_name() {
                    Some(file_os_str) => {
                        match file_os_str.to_str() {
                            Some(file_name) => file_name.to_string(),
                            None => {
                                eprintln!("Invalid File Path");
                                return; // or handle the error in another way
                            }
                        }
                    }
                    None => {
                        eprintln!("Invalid File Path");
                        return; // or handle the error in another way
                    }
                };
                let garbage_file = format!("{}/garbage/{}", dir, file);
                println!("garbage location: {}", garbage_file);
                if path::Path::new(&garbage_file).exists() {
                    println!("file exists");
                    return;
                }
                match fs::rename(&filename, &garbage_file) {
                    Ok(_) => {
                        println!();
                    }
                    Err(e) => {
                        eprint!("Failed to delete the file: {}", e);
                    }
                }
            } else if cli.empty {
                let garbage_files_dir = format!("{}/garbage/", dir);
                let gfd = path::Path::new(&garbage_files_dir);
                if let Err(e) = remove_all_file(gfd) {
                    eprintln!("Error occurred while cleaning the bin: {}", e);
                } else {
                    eprintln!("Cleaned the bin.")
                }
            }

        }
        Err(e) => {
            eprintln!("Error occurred: {}", e);
        }
    }
}
