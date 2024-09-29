use js_sys::Uint8Array;
use std::mem;
use wasm_bindgen::prelude::*;
use web_sys::{self, File};

#[wasm_bindgen]
pub struct Bmp {
    file: File,
    header: Option<BmpHeader>,
}

#[wasm_bindgen]
#[repr(C, packed)]
#[derive(Clone)] // for the getter
pub struct BmpHeader {
    // BMP file header
    bf_type: u16,      // File type. Should be 'BM' (0x42, 0x4D)
    bf_size: u32,      // File size in bytes
    bf_reserved1: u16, // Reserved. Should be 0
    bf_reserved2: u16, // Reserved. Should be 0
    bf_off_bits: u32,  // Offset to the start of image data
    // BMP info header
    bi_size: u32,             // Size of BITMAPINFOHEADER structure. Should be 40
    bi_width: i32,            // Image width in pixels
    bi_height: i32,           // Image height in pixels
    bi_planes: u16,           // Number of color planes. Should be 1
    bi_bit_count: u16,        // Number of bits per pixel. Should be 1, 4, 8, 16, 24, or 32
    bi_compression: u32, // Compression method being used. 0 for no compression, 1 for RLE-8, 2 for RLE-4, or 3 for bitfields
    bi_size_image: u32,  // Size of image data in bytes
    bi_x_pels_per_meter: i32, // Horizontal resolution in pixels per meter
    bi_y_pels_per_meter: i32, // Vertical resolution in pixels per meter
    bi_clr_used: u32,    // Number of colors in the color palette, or 0 to default to 2^n
    bi_clr_important: u32, // Number of important colors used, or 0 when every color is important
}

#[wasm_bindgen]
impl BmpHeader {
    #[wasm_bindgen(getter)]
    pub fn bi_width(&self) -> i32 {
        self.bi_width
    }

    #[wasm_bindgen(getter)]
    pub fn bi_height(&self) -> i32 {
        self.bi_height
    }
}

#[wasm_bindgen]
impl Bmp {
    #[wasm_bindgen(constructor)]
    pub fn new(file: File) -> Bmp {
        Bmp { file, header: None }
    }

    pub async fn open(&mut self) -> Result<(), JsValue> {
        let start = 0f64;
        let header_size = mem::size_of::<BmpHeader>() as f64;

        // Get a slice of the file
        let slice = self
            .file
            .slice_with_f64_and_f64(start as f64, header_size as f64)?;

        // Read the slice as an ArrayBuffer
        let array_buffer_promise = slice.array_buffer();
        let array_buffer = wasm_bindgen_futures::JsFuture::from(array_buffer_promise).await?;

        // Convert the ArrayBuffer to a Uint8Array
        let uint8_array = js_sys::Uint8Array::new(&array_buffer);

        // Check if the file is a BMP file (lettel endian)
        let bf_type: u16 = u16::from_le_bytes([uint8_array.get_index(0), uint8_array.get_index(1)]);
        if bf_type != 0x4D42 {
            return Err("Not a BMP file".into());
        }

        // Read the BMP header
        let bf_type = u16::from_le_bytes([uint8_array.get_index(0), uint8_array.get_index(1)]);
        let bf_size = u32::from_le_bytes([
            uint8_array.get_index(2),
            uint8_array.get_index(3),
            uint8_array.get_index(4),
            uint8_array.get_index(5),
        ]);
        let bf_reserved1 = u16::from_le_bytes([uint8_array.get_index(6), uint8_array.get_index(7)]);
        let bf_reserved2 = u16::from_le_bytes([uint8_array.get_index(8), uint8_array.get_index(9)]);
        let bf_off_bits = u32::from_le_bytes([
            uint8_array.get_index(10),
            uint8_array.get_index(11),
            uint8_array.get_index(12),
            uint8_array.get_index(13),
        ]);
        let bi_size = u32::from_le_bytes([
            uint8_array.get_index(14),
            uint8_array.get_index(15),
            uint8_array.get_index(16),
            uint8_array.get_index(17),
        ]);
        let bi_width = i32::from_le_bytes([
            uint8_array.get_index(18),
            uint8_array.get_index(19),
            uint8_array.get_index(20),
            uint8_array.get_index(21),
        ]);
        let bi_height = i32::from_le_bytes([
            uint8_array.get_index(22),
            uint8_array.get_index(23),
            uint8_array.get_index(24),
            uint8_array.get_index(25),
        ]);
        let bi_planes = u16::from_le_bytes([uint8_array.get_index(26), uint8_array.get_index(27)]);
        let bi_bit_count =
            u16::from_le_bytes([uint8_array.get_index(28), uint8_array.get_index(29)]);
        let bi_compression = u32::from_le_bytes([
            uint8_array.get_index(30),
            uint8_array.get_index(31),
            uint8_array.get_index(32),
            uint8_array.get_index(33),
        ]);
        let bi_size_image = u32::from_le_bytes([
            uint8_array.get_index(34),
            uint8_array.get_index(35),
            uint8_array.get_index(36),
            uint8_array.get_index(37),
        ]);
        let bi_x_pels_per_meter = i32::from_le_bytes([
            uint8_array.get_index(38),
            uint8_array.get_index(39),
            uint8_array.get_index(40),
            uint8_array.get_index(41),
        ]);
        let bi_y_pels_per_meter = i32::from_le_bytes([
            uint8_array.get_index(42),
            uint8_array.get_index(43),
            uint8_array.get_index(44),
            uint8_array.get_index(45),
        ]);
        let bi_clr_used = u32::from_le_bytes([
            uint8_array.get_index(46),
            uint8_array.get_index(47),
            uint8_array.get_index(48),
            uint8_array.get_index(49),
        ]);
        let bi_clr_important = u32::from_le_bytes([
            uint8_array.get_index(50),
            uint8_array.get_index(51),
            uint8_array.get_index(52),
            uint8_array.get_index(53),
        ]);

        self.header = Some(BmpHeader {
            bf_type,
            bf_size,
            bf_reserved1,
            bf_reserved2,
            bf_off_bits,
            bi_size,
            bi_width,
            bi_height,
            bi_planes,
            bi_bit_count,
            bi_compression,
            bi_size_image,
            bi_x_pels_per_meter,
            bi_y_pels_per_meter,
            bi_clr_used,
            bi_clr_important,
        });

        Ok(())
    }

    #[wasm_bindgen(getter)]
    pub fn header(&self) -> Option<BmpHeader> {
        self.header.clone()
    }

    pub async fn get_pixel_data(&self) -> Result<Uint8Array, JsValue> {
        if let Some(header) = &self.header {
            let start = header.bf_off_bits as f64;
            let pixel_data_slice = self.file.slice_with_f64(start)?;

            let array_buffer_promise = pixel_data_slice.array_buffer();
            let array_buffer = wasm_bindgen_futures::JsFuture::from(array_buffer_promise).await?;
            let uint8_array = Uint8Array::new(&array_buffer);

            let width = header.bi_width as usize;
            let height = header.bi_height as usize;
            let row_size = ((header.bi_bit_count as usize * width + 31) / 32) * 4;
            let image_size = row_size * height;

            let mut flipped_data = vec![0; image_size];
            let data: Vec<u8> = uint8_array.to_vec();

            for y in 0..height {
                let src_index = y * row_size;
                let dest_index = (height - 1 - y) * row_size;
                flipped_data[dest_index..dest_index + row_size]
                    .copy_from_slice(&data[src_index..src_index + row_size]);
            }

            Ok(Uint8Array::from(flipped_data.as_slice()))
        } else {
            Err("BMP header not found".into())
        }
    }
}
