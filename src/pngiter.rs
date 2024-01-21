// Header and Footer for PNG files
static PNG_HEADER: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
static PNG_FOOTER: [u8; 12] = [
    0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82
];

/// An interator for the found PNGs.
pub struct PngIter<'a> {
    src_data: &'a [u8],
    position: usize,
}

impl<'a> PngIter<'a> {
    pub fn new(src_data: &'a [u8]) -> Self {
        PngIter {
            src_data,
            position: 0,
        }
    }
}

fn find_header(position: &mut usize, src_data: &[u8]) -> Option<usize> {

    while *position < src_data.len() {
        if src_data[*position..].starts_with(&PNG_HEADER) {
            return Some(*position + 8);
        }
        *position += 1;
    }

    None
}

fn find_footer(footer_start: usize, src_data: &[u8]) -> Option<usize> {
    let mut footer_index = 0;

    while footer_start + footer_index < src_data.len() {
        if src_data[footer_start + footer_index..].starts_with(&PNG_FOOTER) {
            footer_index += 12; // Adjust for footer's length
            return Some(footer_index);
        } else {
            footer_index += 1;
        }
    }

    None
}

impl<'a> Iterator for PngIter<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        let src_data = self.src_data;
    
        loop {
            let footer_start = match find_header(&mut self.position, src_data) {
                Some(x) => x,
                None => break,
            };
            self.position += 1;

            if let Some(footer_index) = find_footer(footer_start, src_data) {
                let header_index = footer_start - 8; // Use footer_start for header index
                let footer_index = footer_start + footer_index;

                self.position = footer_index + 12; // Move to the position after the complete carved data

                return Some(&src_data[header_index..footer_index]);
            } else {
                eprintln!("Warning: Header found without a corresponding footer at position {}", self.position - 8);
                self.position = footer_start; // Move to the next potential header position
            }
        }

        None
    }
}

