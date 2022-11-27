use std::{alloc::{Layout, self, Allocator, Global}, mem::size_of};
use std::ptr::NonNull;

pub fn alloc_box_buffer(len: usize, size: usize) -> Box<[u8]> {
    if len == 0 {
        return <Box<[u8]>>::default();
    }
    let layout = Layout::array::<u8>(len * size).unwrap();
    let layout = layout.align_to(size).unwrap();
    let ptr = unsafe { alloc::alloc_zeroed(layout) };
    let slice_ptr = core::ptr::slice_from_raw_parts_mut(ptr, len * size);
    unsafe { Box::from_raw(slice_ptr) }
}

pub struct MultiTypeBox {
    inner: Option<Box<[u8]>>,
    offsets: [usize; 5],
}

// FIXME: Needs to support 32-byte types (Reference).
impl MultiTypeBox {
    fn size_from_offset_idx(idx: usize) -> usize {
        return 1 << (4 - idx);
    }
    pub fn offset_idx_from_size(mut size: usize) -> Option<usize> {
        if !size.is_power_of_two() || size > 16 {
            return None;
        }
        let mut idx = 4;
        while size != 1 {
            idx -= 1;
            size >>= 1;
        }
        return Some(idx);
    }


    pub fn new(num_words: [usize; 5]) -> Option<MultiTypeBox> {
        let mut offsets = [0; 5];
        let mut base_idx = 0;
        while num_words[base_idx] == 0 {
            offsets[base_idx] = usize::MAX;
            base_idx += 1;
            if base_idx > 4 {
                return None;
            }
        }
        base_idx += 1;
        for idx in base_idx..5 {
            offsets[idx] = offsets[idx - 1] + num_words[idx - 1] * MultiTypeBox::size_from_offset_idx(idx - 1);
        }

        Some(MultiTypeBox {inner: Some(alloc_box_buffer(MultiTypeBox::total_allocated_size_from_offsets(offsets), 
            MultiTypeBox::max_stored_size_from_offsets(offsets))), offsets})

    }
    pub fn as_box(&mut self) -> &mut Box<[u8]> {
        self.inner.as_mut().unwrap()
    }
    pub fn get<U>(&self, idx: usize) -> Option<&U> {
        if size_of::<U>() > 16 {
            None
        } else {
            let mut i = 0;
            while MultiTypeBox::size_from_offset_idx(i) > size_of::<U>() {
                i += 1;
                if i > 4 {
                    return None;
                }
            }
            if self.offsets[i] == usize::MAX {
                None 
            } else {
                if self.offsets[i] + idx * size_of::<U>() >= self.inner.as_ref().unwrap().len() || 
                    self.offsets[i] + (idx + 1) * size_of::<U>() >= self.inner.as_ref().unwrap().len() {
                    return None;
                }
                unsafe {
                    Some((self.inner.as_ref().unwrap()[self.offsets[i] + idx * size_of::<U>() .. 
                    self.offsets[i] + (idx + 1) * size_of::<U>()]
                    .as_ptr() as *const U).as_ref().unwrap())
                }
            }   
        }
    }
    pub fn get_mut<U>(&mut self, idx: usize) -> Option<&mut U> {
        if size_of::<U>() > 16 {
            None
        } else {
            let mut i = 0;
            while MultiTypeBox::size_from_offset_idx(i) > size_of::<U>() {
                i += 1;
                if i > 4 {
                    return None;
                }
            }
            if self.offsets[i] == usize::MAX {
                None 
            } else {
                if self.offsets[i] + idx * size_of::<U>() >= self.inner.as_ref().unwrap().len() || 
                    self.offsets[i] + (idx + 1) * size_of::<U>() >= self.inner.as_ref().unwrap().len() {
                    return None;
                }
                unsafe {
                    Some((self.inner.as_mut().unwrap()[self.offsets[i] + idx * size_of::<U>() .. 
                        self.offsets[i] + (idx + 1) * size_of::<U>()]
                    .as_ptr() as *mut U).as_mut().unwrap())
                }
            }   
        }
    }
    pub fn get_from_start<U>(&self, idx: usize) -> Option<&U> {
        if size_of::<U>() > 16 {
            None
        } else {
            let mut i = 0;
            while self.offsets[i] == usize::MAX {
                i += 1;
                if i > 4 {
                    return None;
                }
            }       
            let mut base = 0;
            while self.offsets[i] < idx * MultiTypeBox::size_from_offset_idx(i) + base {
                base = self.offsets[i];
                i += 1;
                if i > 4 {
                    return None;
                }
            }
            unsafe {
                Some((self.inner.as_ref().unwrap()[self.offsets[i] + idx .. self.offsets[i] + idx + 1]
                .as_ptr() as *const U).as_ref().unwrap())
            } 
        }
    }
}

impl MultiTypeBox {
    fn total_allocated_size_from_offsets(offsets: [usize; 5]) -> usize {
        let mut size = 0;
        let mut base_idx = 0;
        while offsets[base_idx] == usize::MAX {
            base_idx += 1;
        }
        for idx in (base_idx + 1)..5 {
            size += (offsets[idx] - offsets[idx - 1]) * MultiTypeBox::size_from_offset_idx(idx - 1);
        }
        size
    }
    fn max_stored_size_from_offsets(offsets: [usize; 5]) -> usize{
        let mut idx = 0;
        while offsets[idx] == usize::MAX {
            idx += 1;
        }
        MultiTypeBox::size_from_offset_idx(idx)
    }

    fn total_allocated_size(&self) -> usize {
        MultiTypeBox::total_allocated_size_from_offsets(self.offsets)
    }
    fn max_stored_size(&self) -> usize {
        MultiTypeBox::max_stored_size_from_offsets(self.offsets)
    }
}

impl Drop for MultiTypeBox {
    fn drop(&mut self) {
        unsafe {
            let inner = self.inner.take().unwrap();
            let c: NonNull<[u8]> = NonNull::new_unchecked(Box::into_raw(inner));
            let layout = Layout::array::<u8>(self.total_allocated_size()).unwrap();
            let layout = layout.align_to(self.max_stored_size()).unwrap();
            Global.deallocate(c.cast(), layout);
        }
    }
}







