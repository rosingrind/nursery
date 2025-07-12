use std::{io, marker::PhantomData, ops};

use memmap2::{MmapMut, MmapOptions};

pub struct MmapMutBuf<T>(pub(in crate::set) MmapMut, PhantomData<T>);

impl<T> MmapMutBuf<T> {
    pub fn new<F: memmap2::MmapAsRawDesc>(file: F, len: usize, offset: u64) -> io::Result<Self> {
        let mmap = unsafe { MmapOptions::new().len(len).offset(offset).map_mut(file)? };
        Ok(Self(mmap, PhantomData::<T>))
    }
}

impl<T> AsRef<[T]> for MmapMutBuf<T> {
    fn as_ref(&self) -> &[T] {
        let data = self.0.as_ref();
        let len = util::func::align_to_offsets::<T, u8>(data);
        unsafe { core::slice::from_raw_parts(data.as_ptr() as *const T, len) }
    }
}

impl<T> AsMut<[T]> for MmapMutBuf<T> {
    fn as_mut(&mut self) -> &mut [T] {
        let data = self.0.as_mut();
        let len = util::func::align_to_offsets::<T, u8>(data);
        unsafe { core::slice::from_raw_parts_mut(data.as_mut_ptr() as *mut T, len) }
    }
}

impl<T> ops::Deref for MmapMutBuf<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> ops::DerefMut for MmapMutBuf<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}
