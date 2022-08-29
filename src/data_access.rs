pub unsafe fn read_u8(data: *const u8, loc: &mut isize) -> u8 {
    *loc += 1;
    u8::from_be(*data.offset(*loc - 1)) 
}
pub unsafe fn read_u16(data: *const u8, loc: &mut isize) -> u16 {
    *loc += 2; 
    let new_data = data.offset(*loc - 2) as *const u16;
    u16::from_be(new_data.read_unaligned()) 
}
pub unsafe fn read_u32(data: *const u8, loc: &mut isize) -> u32 {
    *loc += 4;
    let new_data = data.offset(*loc - 4) as *const u32;
    u32::from_be(new_data.read_unaligned()) 
}
pub unsafe fn read_u64(data: *const u8, loc: &mut isize) -> u64 {
    *loc += 8;
    let new_data = data.offset(*loc - 8) as *const u64;
    u64::from_be(new_data.read_unaligned()) 
}
pub unsafe fn read_i8(data: *const u8, loc: &mut isize) -> i8 {
    *loc += 1;
    let new_data = data as *const i8;
    i8::from_be(*new_data.offset(*loc - 1)) 
}
pub unsafe fn read_i16(data: *const u8, loc: &mut isize) -> i16 {
    *loc += 2; 
    let new_data = data.offset(*loc - 2) as *const i16;
    i16::from_be(new_data.read_unaligned()) 
}
pub unsafe fn read_i32(data: *const u8, loc: &mut isize) -> i32 {
    *loc += 4;
    let new_data = data.offset(*loc - 4) as *const i32;
    i32::from_be(new_data.read_unaligned()) 
}
pub unsafe fn read_i64(data: *const u8, loc: &mut isize) -> i64 {
    *loc += 8;
    let new_data = data.offset(*loc - 8) as *const i64;
    i64::from_be(new_data.read_unaligned()) 
}