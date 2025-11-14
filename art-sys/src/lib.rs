use cxx::UniquePtr;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("art-sys/include/rowex.h");

        type EpochInfo;

        type Rowex;

        fn rowex_new() -> UniquePtr<Rowex>;

        unsafe fn rowex_info(rowex: *mut Rowex) -> UniquePtr<EpochInfo>;

        unsafe fn rowex_insert(rowex: *mut Rowex, key: u64, info: *mut EpochInfo);

        unsafe fn rowex_lookup(rowex: *mut Rowex, key: u64, info: *mut EpochInfo) -> u64;

        unsafe fn rowex_remove(rowex: *mut Rowex, key: u64, info: *mut EpochInfo);
    }
}

pub struct Rowex(UniquePtr<ffi::Rowex>);

impl Default for Rowex {
    fn default() -> Self {
        Self(ffi::rowex_new())
    }
}

unsafe impl Send for Rowex {}
unsafe impl Sync for Rowex {}

impl Rowex {
    #[inline]
    pub fn pin(&self) -> RowexRef {
        let epoch = unsafe { ffi::rowex_info(self.0.as_mut_ptr()) };
        RowexRef {
            rowex: self.0.as_ref().unwrap(),
            epoch,
        }
    }
}

pub struct RowexRef<'a> {
    rowex: &'a ffi::Rowex,
    epoch: UniquePtr<ffi::EpochInfo>,
}

impl<'a> RowexRef<'a> {
    #[inline]
    pub fn insert(&self, key: u64) {
        unsafe {
            ffi::rowex_insert(
                self.rowex as *const _ as *mut _,
                key,
                self.epoch.as_mut_ptr(),
            )
        }
    }

    #[inline]
    pub fn get(&self, key: u64) -> u64 {
        unsafe {
            ffi::rowex_lookup(
                self.rowex as *const _ as *mut _,
                key,
                self.epoch.as_mut_ptr(),
            )
        }
    }

    #[inline]
    pub fn remove(&self, key: u64) {
        unsafe {
            ffi::rowex_remove(
                self.rowex as *const _ as *mut _,
                key,
                self.epoch.as_mut_ptr(),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Rowex;

    #[test]
    fn smoke() {
        const COUNT: u64 = 10_000;

        let rowex = Rowex::default();
        let map = rowex.pin();

        for i in (1..COUNT).step_by(3) {
            map.insert(i);
        }

        for i in (1..COUNT).step_by(3) {
            assert_eq!(map.get(i), i);
        }

        for i in (1..COUNT).step_by(3) {
            map.remove(i);
        }

        for i in 1..COUNT {
            assert_eq!(map.get(i), 0);
        }
    }
}
