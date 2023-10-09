use clap::Parser;
use dir;
use std::{fs, path::{self, PathBuf}, io::{self, Write}, env};

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

impl CLI {
    fn get_pwd(&self) -> Option<(String, String)> {
        if let Some(ref user_file) = self.file_name {
            let path = path::Path::new(user_file);
            match path.canonicalize() {
                Ok(abs_path) => {
                    let file_name = path.file_name()?.to_string_lossy().to_string();
                    return Some((file_name, abs_path.to_string_lossy().to_string()));
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                    return None;
                }
            }
        } 
        return None;

    }
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
    let garbage_info = garbage_files_dir.join("garbage");
    let garbage_files = garbage_files_dir.join("garbageInfo");
    for entry in fs::read_dir(garbage_files)? {
        let entry = entry?; 
        let entry_path = entry.path();

        if entry_path.is_file() {
            fs::remove_file(entry_path)?;
        }
    }
    for entry in fs::read_dir(garbage_info)? {
        let entry = entry?; 
        let entry_path = entry.path();

        if entry_path.is_file() {
            fs::remove_file(entry_path)?;
        }
    }
    Ok(())

}

fn info_file_config(info_file: String, path: String) -> io::Result<()> {
    let garbage_info_file = fs::File::create(info_file)?;
    let garbage_info = format!("[Garbage Information]\nPath={}", path);
    let mut buf_writer = io::BufWriter::new(garbage_info_file);
    buf_writer.write_all(garbage_info.as_bytes())?;
    buf_writer.flush()?;
    Ok(())
                            

}

fn main() {
    match create_garbage_dir() {
        Ok(dir) => {
            let cli = CLI::parse();

            if let Some(filename) = &cli.file_name {
                let (file, file_path) = match cli.get_pwd() {
                    Some((file_name, path)) => (file_name, path),
                    None => {
                        eprintln!("Invalid File Path");
                        return; 
                    }
                };
                let garbage_file = format!("{}/garbage/{}", dir, file);
                println!("garbage location: {}", file);
                if path::Path::new(&garbage_file).exists() {
                    println!("file exists");
                    return;
                }
                match fs::rename(&filename, &garbage_file) {
                    Ok(_) => {
                        let info_file = format!("{}/garbageInfo/{}.garbageInfo", dir, file);
                        match info_file_config(info_file, file_path) {
                            Ok(_) => (),
                            Err(e) => {
                                eprintln!("Error: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        eprint!("Failed to delete the file: {}", e);
                    }
                }
            } else if cli.empty {
                let gfd = path::Path::new(&dir);
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
