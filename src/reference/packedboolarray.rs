#[derive(Clone)]
pub struct PackedBoolArray {
    m_array: Vec<u8>,
    m_true_size: usize,
}

impl PackedBoolArray {
    pub fn new(size: usize) -> PackedBoolArray {
        PackedBoolArray {
            m_array: Vec::with_capacity(((size - 1) / 8) + 1 as usize ),
            m_true_size: size
        }
    }
    pub fn len(&self) -> usize {
        self.m_true_size
    }
    // We can't use indexmut because we can't express the idea of the subregion of a byte. 
    // Therefore, we have to have an explicit set. 
    pub fn set(&mut self, index: usize, value: bool) {
        self.m_array[index / 8 as usize] &= 255 as u8 - ((value as u8) << index % 8); // 255 = 0b11111111, aka an u8 with all bits set.
    }
}

// In order to cast to bool, we compare the value with zero. 
// This is actually the same speed as using an unsafe bool cast, because if we cast to bool, we have to rshift the bits. 


impl PackedBoolArray {
    pub fn get(&self, index: usize) -> bool {
        if index >= self.m_true_size {
            panic!("Index out of range for PackedBoolArray! Index: {}, Size: {}", index, self.m_true_size);
        }
        (self.m_array[index / 8 as usize] & (1 << index % 8)) != 0
    }
}

