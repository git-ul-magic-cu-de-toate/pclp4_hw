use std::env;                                       // pentru argumentele din linia de comanda,pwd
use std::fs;                                        // pentru cat, mkdir, ln (hard link), chmod
use std::path::Path;                                // pentru a verifica existenta unui fisier / director, chmod
use std::os::unix::fs::symlink as symbolic_link;    // pentru ln -s
use std::os::unix::fs::PermissionsExt;              // pentru chmod

use regex::Regex;                                   // pentru grep

// use chrono::Local;                               // pentru touch


fn main() {
    // TODO: Read the command line arguments
    let args: Vec<String> = env::args().collect();
}
