use std::fs::{self, File};
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    // Set paths for source and destination files
    let src_file_path = "/home/draco/Documents/Rusty_carvey/t1.dd";
    let dst_directory = "carved_data";
    
    // Define header and footer for PNG files
    let png_header: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    let png_footer: [u8; 12] = [0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82];

    // Read the source file into a byte vector
    let mut src_data = Vec::new();
    File::open(src_file_path)?.read_to_end(&mut src_data)?;

    // Create a directory for the carved data
    fs::create_dir(dst_directory)?;

    // Initialize position and counter for carved files
    let mut position = 0;
    let mut count = 0;

    loop {
        // Find the next header starting from the current position
        if let Some(header_index) = src_data[position..].windows(8).position(|window| window == &png_header) {
            // Find the next footer starting from the header
            if let Some(footer_index) = src_data[(position + header_index)..].windows(12).position(|window| window == &png_footer) {
                let header_index = position + header_index;
                let footer_index = position + header_index + footer_index;

                // Ensure the footer is after the header
                if header_index < footer_index {
                    count += 1;
                    let dst_data = &src_data[header_index..=footer_index];
                    let dst_file_name = format!("{}/{}_{}.png", dst_directory, dst_directory, count);
                    File::create(dst_file_name.clone())?.write_all(dst_data)?;
                    println!("Carved: {}", dst_file_name);
                }

                // Update position to search for the next PNG file
                position = footer_index + 12;
            } else {
                break; // Exit if no footer is found for the current header
            }
        } else {
            break; // Exit if no header is found
        }
    }

    // Print the total number of carved files
    println!("Total carved files: {}", count);

    Ok(())
}
