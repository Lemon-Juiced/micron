use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;
use regex::Regex;
use crate::error_handler::print_error;

#[derive(Debug)]
pub struct DiscoveryData {
    pub librarray: Vec<String>,
    pub func_map: HashMap<String, String>,
}

impl DiscoveryData {
    /// Function to create a new DiscoveryData instance
    pub fn new() -> Self {
        DiscoveryData {
            librarray: Vec::new(),
            func_map: HashMap::new(),
        }
    }

    /// Function to add library and its associated function
    pub fn add_library(&mut self, lib_name: String, function_name: String) {
        if !self.librarray.contains(&lib_name) {
            self.librarray.push(lib_name.clone());
        }
        self.func_map.insert(function_name, lib_name);
    }
    
    /// Function to print the stored data
    pub fn display(&self) {
        println!("Libraries: {:?}", self.librarray);
        println!("Function Map: {:?}", self.func_map);
    }
}

/// Navigates to the /libs directory and generates:
/// 1. An Array (librarray) - Like lib + array ... I like it.
///    This contains the names of the available library files in this directory.
/// 2. A Map (func_map) 
///    The keys of this map are the function name and the value is the name of the library they map to.
/// These will be stored so that the lib_runner can find where these functions are stored and execute them when needed.
pub fn discover() -> DiscoveryData {
    let mut current_dir = std::env::current_dir().expect("Failed to get current directory");
    
    // Traverse upwards until we find the micron directory
    while !current_dir.ends_with("micron") {
        current_dir.pop();
        if current_dir.as_os_str().is_empty() {
            print_error("Could not find the src directory");
            return DiscoveryData::new();
        }
    }
    
    // Append src/libs to the path
    current_dir.push("src/libs");
    let libs_dir = current_dir; // Path to libraries

    let mut discovery_data = DiscoveryData::new(); // Initializes an empty DiscoveryData struct

    // Read the contents of the /libs directory
    match fs::read_dir(libs_dir) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(lib_name) = path.file_name().and_then(|n| n.to_str()) {
                            // Add library to librarray if not already added
                            if !discovery_data.librarray.contains(&lib_name.to_string()) {
                                discovery_data.librarray.push(lib_name.to_string());
                            }
                            // Read the file contents
                            if let Ok(contents) = fs::read_to_string(&path) {
                                // Use regex to find function names
                                let re = Regex::new(r"def self\.(\w+)\(").unwrap();
                                for cap in re.captures_iter(&contents) {
                                    let function_name = &cap[1];
                                    discovery_data.add_library(lib_name.to_string(), function_name.to_string());
                                }
                            } else {
                                print_error(&format!("Could not read the file: {:?}", path));
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            print_error(&format!("Could not read the /libs directory: {}", e));
        }
    }
    // Return the struct
    discovery_data
}