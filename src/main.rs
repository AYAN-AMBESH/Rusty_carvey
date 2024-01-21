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

#[cfg(test)]
mod tests {
    use crate::pngiter::*;
    use std::fs::File;
    use std::io::prelude::*;
    
    static HASHES: [&'static str; 10] = [
        "6d064466ccb4016905bba4d306b590dc71a04f7abce9abc0aad37de11271f9f0",
        "fed0a48e48a1cd62a532b6021104cd50dfd36a3cac591a3c87df98148a5aeae4",
        "ed36115b9a469beedf8e27508022a6295de1dc9514be3b39c124d9d4f14d7e88",
        "d6864217e6a7d0279070d00cb8fed75be989f06ee01a7888674304a086aa6438",
        "73bd5fd675d98432014a21365247d7c333b0f714e615b28d8f70feaaf9ad7a38",
        "7df7563ce68ed355ac82af7e08e2a713cc8c8827fbb7b017ea6afc5de857af02",
        "2fe41be9ad0fdc12b552bdc8db07a1b6db526abcf8dc9daede71158da445f485",
        "1350c075215cf3dcbbf9d4df6cb58ac28a0f2e741f010cb5f72e1c73e32decfe",
        "3bda1c6ae5766e5cd767ef54c505f3b8adfdafaa0db02e6c74de50343fdbb2e7",
        "04fd8a8d7d621f082a393022163e89f59b852aa65499a7fbd477c6deaae84eaf",
    ];

    #[test]
    fn t1dd() {
        let mut src_data = Vec::new();
        File::open("t1.dd")
            .expect("Couldn't open ./t1.dd")
            .read_to_end(&mut src_data)
            .expect("Couldn't read ./t1.dd");
        
        for (count, dst_data) in PngIter::new(&src_data).enumerate() {
            let hash = sha256::digest(dst_data);
            println!("{} is {hash}", count + 1);
            assert_eq!(HASHES[count], hash);
        }
    }
}

