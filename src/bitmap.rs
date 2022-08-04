use std::{path::Path, fs::File, io::{Write, Error}};
use byteorder::{WriteBytesExt, LittleEndian};

pub struct Bitmap{}
impl Bitmap{
    pub fn save(output_path: &Path, width:u16, height:u16, palette_bytes:&[u8], pixel_bytes:&[u8])->Result<bool, Error>{

        let mut file = File::create(output_path).expect("Could not open bitmap output path.");      
        file.write_u16::<LittleEndian>(0x4d42)?;
        file.write_u32::<LittleEndian>((54+palette_bytes.len()+pixel_bytes.len()) as u32)?;
        file.write_u32::<LittleEndian>(0)?;
        file.write_u32::<LittleEndian>(309)?;
        file.write_u32::<LittleEndian>(40)?;
        file.write_u32::<LittleEndian>(width as u32)?;
        file.write_u32::<LittleEndian>(height as u32)?;
        file.write_u16::<LittleEndian>(1)?; //Planes
        file.write_u16::<LittleEndian>(8)?; //Bits per pixel
        file.write_u32::<LittleEndian>(0)?; //Compression
        file.write_u32::<LittleEndian>(0)?; // Image size, 0 is okay if not compressed.
        file.write_u32::<LittleEndian>(1)?; //x pixel per meter
        file.write_u32::<LittleEndian>(1)?; //y pixel per meter
        file.write_u32::<LittleEndian>(256)?; //Colors used
        file.write_u32::<LittleEndian>(0)?; //Important colors

        file.write_all(palette_bytes)?;
        file.write_all(pixel_bytes)?;
        Ok(true)
    }
}