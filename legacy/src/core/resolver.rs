// Copyright (c) 2026 Ashutosh Sharma. All rights reserved.

use std::path::{Path, PathBuf};
use std::env;
use std::io;

pub struct Resolver {
    root_dir: PathBuf,
}

impl Resolver {
    pub fn new() -> Self {
        Resolver {
            root_dir: env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        }
    }

    pub fn resolve(&self, import_path: &str, current_file: Option<&Path>) -> io::Result<PathBuf> {
        let path = Path::new(import_path);
        
        let target_path = if path.is_absolute() {
            path.to_path_buf()
        } else if let Some(current) = current_file {
            if let Some(parent) = current.parent() {
                parent.join(path)
            } else {
                self.root_dir.join(path)
            }
        } else {
            self.root_dir.join(path)
        };

        // Canonicalize to resolve .. and . components if file exists
        match target_path.canonicalize() {
            Ok(p) => Ok(p),
            Err(e) => {
                // If file doesn't exist yet (not strictly required for lookup logic but good for runtime),
                // we return error. For generic resolution we might want to just return the path.
                // But for an import system, we generally want the file to exist.
                Err(e)
            }
        }
    }
    
    pub fn read_file(&self, path: &Path) -> io::Result<String> {
        std::fs::read_to_string(path)
    }
}
