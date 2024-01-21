use std::fs::{self, File};
use std::io::prelude::*;
use std::env;

mod pngiter;
use pngiter::*;

fn main() -> std::io::Result<()> {
    // Set paths for source and destination files
    let src_file_path = env::args().nth(1)
        .expect("Usage: r u s t y c a r v e r   f i l e");
    let dst_directory = "carved_data";


    // Read the source file into a byte vector
    let mut src_data = Vec::new();
    File::open(src_file_path)?.read_to_end(&mut src_data)?;

    // Create a directory for the carved data
    fs::create_dir(dst_directory)?;


    for (count, dst_data) in PngIter::new(&src_data).enumerate() {
        let dst_file_name = format!("{}/{}_{}.png", dst_directory, dst_directory, count + 1);
        File::create(dst_file_name.clone())?.write_all(dst_data)?;
        eprintln!("Carved: {}", dst_file_name);
    }

    Ok(())
}
