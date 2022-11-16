/* 
use std::{alloc::{Layout, self, Allocator, Global}, mem::size_of, ops::Index};
use std::ptr::NonNull;
use std::marker;

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

pub struct SafeBox{
    inner: Option<Box<[u8]>>,
    offsets: [usize; 5],
}

impl SafeBox {
    pub fn new(num_words: (usize, usize, usize, usize, usize)) -> SafeBox {
        let offsets;
        let max_size = if num_words.0 > 0 {
            offsets = [0, num_words.0 * 16, num_words.0 * 16 + num_words.1 * 8, 
            num_words.0 * 16 + num_words.1 * 8 + num_words.2 * 4, 
            num_words.0 * 16 + num_words.1 * 8 + num_words.2 * 4 + num_words.3 * 2];
            16 
        } else if num_words.1 > 0 {
            offsets = [usize::MAX, 0, num_words.1 * 8, num_words.1 * 8 + num_words.2 * 4, 
            num_words.1 * 8 + num_words.2 * 4 + num_words.3 * 2];
            8
        } else if num_words.2 > 0 {
            offsets = [usize::MAX, usize::MAX, 0, num_words.2 * 4, num_words.2 * 4 + num_words.3 * 2];
            4
        } else if num_words.3 > 0 {
            offsets = [usize::MAX, usize::MAX, usize::MAX, 0, num_words.3 * 2];
            2
        } else if num_words.4 > 0 {
            offsets = [usize::MAX, usize::MAX, usize::MAX, usize::MAX, 0];
            1
        } else {0};
        assert!(max_size > 0);

        let len = offsets.iter().filter(|off| off != usize::MAX).sum() + num_words.4;

        SafeBox {inner: Some(alloc_box_buffer(len, max_size)), offsets}

    }
    pub fn as_box(&mut self) -> &mut Box<[u8]> {
        self.inner.as_mut().unwrap()
    }
    pub unsafe fn get<U>(&self, mut idx: usize) -> Option<&U> {
        if size_of::<U>() > 16 {
            None
        } else {
            let i = 0;
            while (16 - size_of::<U>()) > 2 << i {
                i += 1;
            }
            if self.offsets[i] != usize::MAX {
                None 
            } else {
                unsafe {
                    Some((self.inner.as_ref().unwrap()[self.offsets[i] + idx * size_of::<U>() .. self.offsets[i] + (idx + 1) * size_of::<U>()]
                    .as_ptr() as *const U).as_ref().unwrap())
                }
            }   
        }
    }
    pub unsafe fn get_mut<U>(&mut self, idx: usize) -> &mut U {
        if size_of::<U>() > 16 {
            None
        } else {
            let i = 0;
            while (16 - size_of::<U>()) > 1 << i {
                i += 1;
            }
            if self.offsets[i] != usize::MAX {
                None 
            } else {
                unsafe {
                    Some((self.inner.as_mut().unwrap()[self.offsets[i] + idx * size_of::<U>() .. self.offsets[i] + (idx + 1) * size_of::<U>()]
                    .as_ptr() as *const U).as_mut().unwrap())
                }
            }   
        }
    }
    pub unsafe fn get_from_start<U>(&self, mut idx: usize) -> &U {
        let mut i = 0;
        while self.offsets[i] == usize::MAX {
            i += 1;
        }       
        let mut base = 0;
        while self.offsets[i] < idx * 1 << (4 - i) + base {
            base = self.offsets[i];
            i += 1;
        }
        unsafe {
            (self.inner.as_ref().unwrap()[self.offsets[i] + idx .. self.offsets[i] + idx + 1]
            .as_ptr() as *const U).as_ref().unwrap()
        } 
    }
}

impl Drop for SafeBox {
    fn drop(&mut self) {
        unsafe {
            let inner = self.inner.take().unwrap();
            let c: NonNull<[u8]> = NonNull::new_unchecked(Box::into_raw(inner));
            let layout = Layout::array::<u8>(len * size).unwrap();
            let layout = layout.align_to(size).unwrap();
            Global.deallocate(c.cast(), layout);
        }
    }
}
*/






