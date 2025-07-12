use std::{io, marker::PhantomData, ops};

use memmap2::{MmapMut, MmapOptions};

pub struct MmapMutVal<T>(pub(in crate::set) MmapMut, PhantomData<T>);

impl<T> MmapMutVal<T> {
    pub fn new<F: memmap2::MmapAsRawDesc>(file: F, len: usize, offset: u64) -> io::Result<Self> {
        let mmap = unsafe {
            MmapOptions::new()
                .len(len)
                .offset(offset)
                .populate()
                .map_mut(file)?
        };
        Ok(Self(mmap, PhantomData::<T>))
    }
}

impl<T> AsRef<T> for MmapMutVal<T> {
    fn as_ref(&self) -> &T {
        let data = self.0.as_ref();
        let len = util::func::align_to_offsets::<T, u8>(data);
        unsafe { core::slice::from_raw_parts(data.as_ptr() as *const T, len) }
            .first()
            .unwrap()
    }
}

impl<T> AsMut<T> for MmapMutVal<T> {
    fn as_mut(&mut self) -> &mut T {
        let data = self.0.as_mut();
        let len = util::func::align_to_offsets::<T, u8>(data);
        unsafe { core::slice::from_raw_parts_mut(data.as_mut_ptr() as *mut T, len) }
            .first_mut()
            .unwrap()
    }
}

impl<T> ops::Deref for MmapMutVal<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> ops::DerefMut for MmapMutVal<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}
