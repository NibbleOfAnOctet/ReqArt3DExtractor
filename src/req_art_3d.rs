use std::{fs::File, path::Path, io::{Read, Seek, SeekFrom}};
use byteorder::{ ReadBytesExt, LittleEndian};
use crate::{dcl::{dcl_explode}, bitmap::Bitmap};

pub struct ReqArt3D{
    dat_file: File,
    art_file: File
}

impl ReqArt3D{
    pub fn open(art_file_path:String)->Self{
        let root_path = Path::new(&art_file_path);

        let dat_file = File::open(root_path.join("ReqArt3D.dat")).or_else(|_err|File::open(root_path.join("ReqArtD3.dat"))).expect("Could not open .dat-file");
        let art_file = File::open(root_path.join("ReqArt3D.art")).or_else(|_err|File::open(root_path.join("ReqArtD3.art"))).expect("Could not open .art-file");
        
        Self{ dat_file, art_file }
    }

    fn into_u16(slice: &[u8]) -> [u8; 2] {
        slice.try_into().expect("Invalid slice length")
    }

    pub fn extract(&mut self, output_folder:String){
        let output_path = Path::new(&output_folder);
        println!("Extracting art into '{}' ...", output_path.display());
        let mut offset_buffer = [0u8;4];
        while self.dat_file.read_exact(&mut offset_buffer).is_ok(){
            let offset = u32::from_le_bytes(offset_buffer);
            if offset!=u32::MAX{
                self.art_file.seek(SeekFrom::Start(offset as u64)).expect(format!("Unexpected EOF in .art-file at offset {:08X}",offset).as_str());
                let decompressed_size = self.art_file.read_u64::<LittleEndian>().expect(format!("Expected u64 at offset: {:#08X} in .art-file.",offset).as_str());
                let compressed_size = self.art_file.read_u32::<LittleEndian>().expect(format!("Expected u32 at offset: {:#04X} in .art-file.",offset).as_str());

                
                let mut compressed_data = vec![0u8;(compressed_size-4) as usize];
                self.art_file.read_exact(compressed_data.as_mut_slice()).expect(format!("Expected {} bytes at offset {:#08X} in .art-file.", compressed_size-4, offset+12).as_str());
                
                let mut decompressed_data = vec![0u8;decompressed_size as usize];
                dcl_explode(compressed_data.as_slice(), compressed_size,  decompressed_data.as_mut_slice(), &(decompressed_size as u32));
                
                let width = u16::from_le_bytes(ReqArt3D::into_u16(&decompressed_data[0..2]));
                let height = u16::from_le_bytes(ReqArt3D::into_u16(&decompressed_data[2..4]));
                
                Bitmap::save(output_path.join(format!("{}.bmp", offset).as_str()).as_path(), width, height, &decompressed_data[18..1041], &decompressed_data[1041..]).expect("Could not save bitmap!");
            }
        }
        println!("Done!");
    }
}