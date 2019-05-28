// crates
extern crate chrono;

// system items
use std::process;
use std::env;
use std::fs;
use std::path::Path;
use std::os::unix::fs::MetadataExt;

// externals
use chrono::prelude::*;

fn format_unix_mode(mode: u32) -> String {
    let mut formatted = vec!['-'; 10];
    let file_type = mode & 0xF000;
    formatted[0] = match file_type {
        0x8000 => '-',
        0x4000 => 'd',
        0xA000 => 'l',
        _ => 'z' // because I literally cannot find the numerical values for the rest of the types anywhere...
    };

    let mut n: u32 = 1;
    for i in 1..10 {
        // bit is flagged
        if mode & n > 0 {
            // cute switch case
            // bits are in reverse order from display (other is actually the far left in binary)
            formatted[10 - i] = match i % 3 {
                0 => 'r',
                1 => 'x',
                2 => 'w',
                _ => '-' // literally can't get here but rust syntax
            };
        }
        n = n << 1; // shift n over to check the next bit (binary operation)
    }
    formatted.into_iter().collect() // convert vec of chars to string
}

fn main() {
    let path = Path::new(".");

    let directory = fs::read_dir(path).unwrap_or_else(|err| {
        eprintln!("Problem opening: `{}`, {}", path.to_str().unwrap(), err);
        process::exit(1);
    });

    for entry in directory {
        if let Ok(entry) = entry {
            // name
            let name = entry.file_name().into_string().unwrap();
            
            if let Ok(metadata) = entry.metadata() {
                // mode
                let mode = format_unix_mode(metadata.mode());
                // links
                let link_count = metadata.nlink();
                // owner
                let uid = metadata.uid();
                // group
                let gid = metadata.gid();
                // size
                let size = metadata.size();
                // write date (local time)
                let time = Local.timestamp(metadata.mtime(), 0);

                println!(
                "{:10}       {:19}       {:10}      {}",
                mode,
                time.format("%m/%d/%Y  %l:%M %p"),
                size,
                name,
            );
            }
        }
    }
}
