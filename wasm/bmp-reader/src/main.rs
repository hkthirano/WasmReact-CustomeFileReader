use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::mem;

#[repr(C, packed)]
struct BmpHeader {
    // BMP file header
    bfType: u16,      // File type. Should be 'BM' (0x42, 0x4D)
    bfSize: u32,      // File size in bytes
    bfReserved1: u16, // Reserved. Should be 0
    bfReserved2: u16, // Reserved. Should be 0
    bfOffBits: u32,   // Offset to the start of image data
    // BMP info header
    biSize: u32,          // Size of BITMAPINFOHEADER structure. Should be 40
    biWidth: i32,         // Image width in pixels
    biHeight: i32,        // Image height in pixels
    biPlanes: u16,        // Number of color planes. Should be 1
    biBitCount: u16,      // Number of bits per pixel. Should be 1, 4, 8, 16, 24, or 32
    biCompression: u32, // Compression method being used. 0 for no compression, 1 for RLE-8, 2 for RLE-4, or 3 for bitfields
    biSizeImage: u32,   // Size of image data in bytes
    biXPelsPerMeter: i32, // Horizontal resolution in pixels per meter
    biYPelsPerMeter: i32, // Vertical resolution in pixels per meter
    biClrUsed: u32,     // Number of colors in the color palette, or 0 to default to 2^n
    biClrImportant: u32, // Number of important colors used, or 0 when every color is important
}

fn main() -> io::Result<()> {
    // Read BMP file
    let mut file = File::open("src/sample-2.bmp")?;

    // Read BMP header
    let mut header = BmpHeader {
        bfType: 0,
        bfSize: 0,
        bfReserved1: 0,
        bfReserved2: 0,
        bfOffBits: 0,
        biSize: 0,
        biWidth: 0,
        biHeight: 0,
        biPlanes: 0,
        biBitCount: 0,
        biCompression: 0,
        biSizeImage: 0,
        biXPelsPerMeter: 0,
        biYPelsPerMeter: 0,
        biClrUsed: 0,
        biClrImportant: 0,
    };

    let header_size = mem::size_of::<BmpHeader>();
    let header_buf: &mut [u8] =
        unsafe { std::slice::from_raw_parts_mut(&mut header as *mut _ as *mut u8, header_size) };
    file.read_exact(header_buf)?;

    // Check if the file is a BMP file
    if header.bfType != 0x4D42 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Not a BMP file"));
    }

    // Move the file cursor to the start of the image data
    file.seek(SeekFrom::Start(header.bfOffBits as u64))?;

    // Get the image data size
    let pixel_data_size = if header.biSizeImage == 0 {
        (header.biWidth * header.biHeight * (header.biBitCount as i32 / 8)) as usize
    } else {
        header.biSizeImage as usize
    };

    // Read the image data
    let mut pixel_data = vec![0u8; pixel_data_size];
    file.read_exact(&mut pixel_data)?;

    // Print the first 10 rows of pixel data
    for (i, chunk) in pixel_data.chunks_exact(3).enumerate() {
        if i >= 10 {
            break;
        }
        let r = chunk[2];
        let g = chunk[1];
        let b = chunk[0];
        println!("R: {}, G: {}, B: {}", r, g, b);
    }

    Ok(())
}
