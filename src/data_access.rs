use paste::paste;

macro_rules! accessor {
    ($num_type:ident, $width:expr, $nwidth:expr) => {    
        paste! {     
            /// # Safety
            /// 
            #[doc = "This function is unsafe because it requires that there are "]
            #[doc = $nwidth ]
            #[doc = "valid bytes at `data`"]
            pub unsafe fn [< read_$num_type >](data: *const u8, loc: &mut isize) -> $num_type {
                *loc += $width;
                $num_type::from_be_bytes(std::slice::from_raw_parts(data.add((*loc - $width) as usize), $width).try_into().unwrap())
            }
        }
    };
}

/// # Safety
/// 
/// This function is unsafe because it requires that there is a valid byte at `data`
pub unsafe fn read_u8(data: *const u8, loc: &mut isize) -> u8 {
    *loc += 1;
    *data.add((*loc - 1) as usize)
}

accessor!(u16, 2, "2");
accessor!(u32, 4, "4");
accessor!(u64, 8, "8");

/// # Safety
/// 
/// This function is unsafe because it requires that there is a valid byte at `data`
pub unsafe fn read_i8(data: *const u8, loc: &mut isize) -> i8 {
    *loc += 1;
    *data.add((*loc - 1) as usize) as i8
}

accessor!(i16, 2, "2");
accessor!(i32, 4, "4");
accessor!(i64, 8, "8");