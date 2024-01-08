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

    let mut position = 0;
    let mut count = 0;

    loop {
        let mut header_found = false;
        let mut footer_start = 0;

        while position < src_data.len() && !header_found {
            if src_data[position..].starts_with(&png_header) {
                header_found = true;
                footer_start = position + 8;
            }
            position += 1;
        }

        if !header_found {
            break; // Exit if no header is found
        }

        let mut footer_found = false;
        let mut footer_index = 0;

        while footer_start + footer_index < src_data.len() && !footer_found {
            if src_data[footer_start + footer_index..].starts_with(&png_footer) {
                footer_found = true;
                footer_index += 12; // Adjust for footer's length
            } else {
                footer_index += 1;
            }

            // Break the loop if footer search reaches the end of data
            if footer_start + footer_index >= src_data.len() {
                break;
            }
        }

        if footer_found {
            let header_index = footer_start - 8; // Use footer_start for header index
            let footer_index = footer_start + footer_index;

            // Bounds checks
            if header_index < 0 || footer_index >= src_data.len() {
                println!("Warning: Header or footer found outside data bounds.");
                break; // Exit to prevent potential out-of-bounds access
            }

            if header_index < footer_index {
               
                    count += 1;
                    let dst_data = &src_data[header_index..footer_index + 1]; // Include the entire footer
                    let dst_file_name = format!("{}/{}_{}.png", dst_directory, dst_directory, count);
                    File::create(dst_file_name.clone())?.write_all(dst_data)?;
                    println!("Carved: {}", dst_file_name);
                
            } else {
                println!("Warning: Header found without a corresponding footer at position {}", header_index);
            }

            position = footer_index + 12; // Move to the position after the complete carved data
        } else {
            println!("Warning: Header found without a corresponding footer at position {}", position - 8);
            position = footer_start; // Move to the next potential header position
        }
    }

    println!("Total carved files: {}", count);

    Ok(())
}
