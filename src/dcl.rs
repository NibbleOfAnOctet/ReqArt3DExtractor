extern crate libc;

extern "C"{
    fn explode(src: *const u8, src_size:u32, dest:*mut u8, dest_size:*const u32)->bool;
    //fn implode(binary_type:i32, dict:i32, src:*const u8, src_size:u32, dest:*mut u8, dest_size:*const u32)->bool;
}
/*pub fn dcl_implode(binary_type:i32, dict:i32, src:&[u8], src_size:u32, dest:&mut [u8], dest_size:&u32)->bool{
    unsafe{
        return implode(binary_type, dict, src.as_ptr(), src_size, dest.as_mut_ptr(), dest_size);
    }
}*/

pub fn dcl_explode(src: &[u8], src_size:u32, dest:&mut [u8], dest_size:&u32)->bool{
    unsafe{
        return explode(src.as_ptr(), src_size, dest.as_mut_ptr(), dest_size);
    }
}