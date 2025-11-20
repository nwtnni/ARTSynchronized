use core::marker::PhantomData;

use cxx::UniquePtr;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("art-sys/include/wrap.h");

        type EpochInfo;

        type Rowex;

        unsafe fn rowex_info(rowex: *mut Rowex) -> UniquePtr<EpochInfo>;

        fn rowex_u64_new() -> UniquePtr<Rowex>;
        unsafe fn rowex_u64_insert(rowex: *mut Rowex, tid: u64, info: *mut EpochInfo);
        unsafe fn rowex_u64_lookup(rowex: *mut Rowex, tid: u64, info: *mut EpochInfo) -> u64;
        unsafe fn rowex_u64_remove(rowex: *mut Rowex, tid: u64, info: *mut EpochInfo);

        fn rowex_string_new() -> UniquePtr<Rowex>;
        unsafe fn rowex_string_insert(rowex: *mut Rowex, tid: u64, info: *mut EpochInfo);
        unsafe fn rowex_string_lookup(rowex: *mut Rowex, tid: u64, info: *mut EpochInfo) -> u64;
        unsafe fn rowex_string_remove(rowex: *mut Rowex, tid: u64, info: *mut EpochInfo);
    }
}

pub struct Rowex<K> {
    inner: UniquePtr<ffi::Rowex>,
    _key: PhantomData<K>,
}

unsafe impl<K> Send for Rowex<K> {}
unsafe impl<K> Sync for Rowex<K> {}

impl Rowex<u64> {
    #[inline]
    pub fn new_u64() -> Self {
        Self::new(ffi::rowex_u64_new())
    }
}

impl Rowex<String> {
    #[inline]
    pub fn new_string() -> Self {
        Self::new(ffi::rowex_string_new())
    }
}

impl<K> Rowex<K> {
    fn new(inner: UniquePtr<ffi::Rowex>) -> Self {
        Self {
            inner,
            _key: PhantomData,
        }
    }

    #[inline]
    pub fn pin(&self) -> RowexRef<K> {
        let epoch = unsafe { ffi::rowex_info(self.inner.as_mut_ptr()) };
        RowexRef {
            rowex: self.inner.as_ref().unwrap(),
            epoch,
            _key: PhantomData,
        }
    }
}

pub struct RowexRef<'a, K> {
    rowex: &'a ffi::Rowex,
    epoch: UniquePtr<ffi::EpochInfo>,
    _key: PhantomData<K>,
}

impl<'a> RowexRef<'a, u64> {
    #[inline]
    pub fn insert_u64(&self, key: u64) {
        unsafe {
            ffi::rowex_u64_insert(
                self.rowex as *const _ as *mut _,
                key,
                self.epoch.as_mut_ptr(),
            )
        }
    }

    #[inline]
    pub fn get_u64(&self, key: u64) -> u64 {
        unsafe {
            ffi::rowex_u64_lookup(
                self.rowex as *const _ as *mut _,
                key,
                self.epoch.as_mut_ptr(),
            )
        }
    }

    #[inline]
    pub fn remove_u64(&self, key: u64) {
        unsafe {
            ffi::rowex_u64_remove(
                self.rowex as *const _ as *mut _,
                key,
                self.epoch.as_mut_ptr(),
            )
        }
    }
}

impl<'a> RowexRef<'a, String> {
    #[inline]
    pub fn insert_string(&self, key: &'static str) {
        unsafe {
            let tid = Self::tid_from_string(key);
            ffi::rowex_string_insert(
                self.rowex as *const _ as *mut _,
                tid,
                self.epoch.as_mut_ptr(),
            )
        }
    }

    #[inline]
    pub fn get_string(&self, key: &'static str) -> bool {
        unsafe {
            let tid = Self::tid_from_string(key);
            dbg!(ffi::rowex_string_lookup(
                self.rowex as *const _ as *mut _,
                tid,
                self.epoch.as_mut_ptr(),
            )) == tid
        }
    }

    #[inline]
    pub fn remove_string(&self, key: &'static str) {
        unsafe {
            let tid = Self::tid_from_string(key);
            ffi::rowex_string_remove(
                self.rowex as *const _ as *mut _,
                tid,
                self.epoch.as_mut_ptr(),
            )
        }
    }

    fn tid_from_string(key: &'static str) -> u64 {
        let len = key.len() as u64;
        debug_assert!(len <= u16::MAX as u64);
        let mut tid = key.as_ptr() as u64;
        debug_assert_eq!(tid & !((1 << 48) - 1), 0);
        tid |= len << 48;
        tid
    }
}

#[cfg(test)]
mod tests {
    use crate::Rowex;

    #[test]
    fn u64() {
        const COUNT: u64 = 10_000;

        let rowex = Rowex::new_u64();
        let map = rowex.pin();

        for i in (1..COUNT).step_by(3) {
            map.insert_u64(i);
        }

        for i in (1..COUNT).step_by(3) {
            assert_eq!(map.get_u64(i), i);
        }

        for i in (1..COUNT).step_by(3) {
            map.remove_u64(i);
        }

        for i in 1..COUNT {
            assert_eq!(map.get_u64(i), 0);
        }
    }

    #[test]
    fn string() {
        const DATA: [&str; 5] = ["1\n", "12\n", "123\n", "1234\n", "12345\n"];

        let rowex = Rowex::new_string();
        let map = rowex.pin();

        for string in DATA {
            map.insert_string(string);
        }

        for string in DATA {
            assert!(map.get_string(string));
        }

        for string in DATA {
            map.remove_string(string);
        }

        for string in DATA {
            assert!(!map.get_string(string));
        }
    }
}
