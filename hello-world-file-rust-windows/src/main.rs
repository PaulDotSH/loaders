use std::env;
use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let dir = if cfg!(windows) {
        home_dir().unwrap_or_else(|| env::var("USERPROFILE").expect("Could not determine home directory").into())
            .join("Documents")
    } else {
        home_dir().unwrap_or_else(|| env::var("HOME").expect("Could not determine home directory").into())
    };

    let file_path = dir.join("foo.txt");
    let content = "Hello, world!";

    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

fn home_dir() -> Option<std::path::PathBuf> {
    #[cfg(windows)]
    return env::var("USERPROFILE")
        .ok()
        .and_then(|p| Some(std::path::PathBuf::from(p)));

    #[cfg(not(windows))]
    return env::var("HOME")
        .ok()
        .and_then(|p| Some(std::path::PathBuf::from(p)));
}
