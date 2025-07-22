use std::{io, marker::PhantomData, ops};

use memmap2::{MmapMut, MmapOptions};

pub struct ValMut<T>(pub MmapMut, PhantomData<T>);

impl<T> ValMut<T> {
    pub fn new<F: memmap2::MmapAsRawDesc>(
        file: F,
        mode: super::Mode,
        offset: u64,
    ) -> io::Result<Self> {
        const {
            assert!(size_of::<T>() != 0);
        };

        let mut opts = MmapOptions::new();
        opts.offset(offset).len(size_of::<T>()).populate();

        Ok(Self(
            match mode {
                super::Mode::Shared => unsafe { opts.map_mut(file) },
                super::Mode::Private => unsafe { opts.map_copy(file) },
            }?,
            PhantomData::<T>,
        ))
    }
}

impl<T> AsRef<T> for ValMut<T> {
    fn as_ref(&self) -> &T {
        let data = self.0.as_ref();
        let len = util::func::align_to_offsets::<T, u8>(data);
        unsafe { &core::slice::from_raw_parts(data.as_ptr() as *const T, len)[0] }
    }
}

impl<T> AsMut<T> for ValMut<T> {
    fn as_mut(&mut self) -> &mut T {
        let data = self.0.as_mut();
        let len = util::func::align_to_offsets::<T, u8>(data);
        unsafe { &mut core::slice::from_raw_parts_mut(data.as_mut_ptr() as *mut T, len)[0] }
    }
}

impl<T> ops::Deref for ValMut<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> ops::DerefMut for ValMut<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}
