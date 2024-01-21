// Header and Footer for PNG files
static PNG_HEADER: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
static PNG_FOOTER: [u8; 12] = [
    0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82
];

/// An interator for the found PNGs.
pub struct PngIter<'a> {
    src_data: &'a Vec<u8>,
    position: usize,
}

impl<'a> PngIter<'a> {
    pub fn new(src_data: &'a Vec<u8>) -> Self {
        PngIter {
            src_data,
            position: 0,
        }
    }
}

impl<'a> Iterator for PngIter<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        let src_data = self.src_data;
    
        loop {
            let mut header_found = false;
            let mut footer_start = 0;

            while self.position < src_data.len() && !header_found {
                if src_data[self.position..].starts_with(&PNG_HEADER) {
                    header_found = true;
                    footer_start = self.position + 8;
                }
                self.position += 1;
            }

            if !header_found {
                break; // Exit if no header is found
            }

            let mut footer_found = false;
            let mut footer_index = 0;

            while footer_start + footer_index < src_data.len() && !footer_found {
                if src_data[footer_start + footer_index..].starts_with(&PNG_FOOTER) {
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
                if  footer_index >= src_data.len() {
                    eprintln!("Warning: Header or footer found outside data bounds.");
                    break; // Exit to prevent potential out-of-bounds access
                }

                if header_index < footer_index {
                    // Include the entire footer
                    return Some(&src_data[header_index..footer_index + 1]);
                } else {
                    eprintln!("Warning: Header found without a corresponding footer at position {}", header_index);
                }

                self.position = footer_index + 12; // Move to the position after the complete carved data
            } else {
                eprintln!("Warning: Header found without a corresponding footer at position {}", self.position - 8);
                self.position = footer_start; // Move to the next potential header position
            }
        }

        None
    }
}

