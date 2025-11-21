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
        unsafe fn rowex_u64_insert(
            rowex: *mut Rowex,
            key: u64,
            value: u64,
            info: *mut EpochInfo,
        ) -> bool;
        unsafe fn rowex_u64_lookup(
            rowex: *mut Rowex,
            key: u64,
            value: *mut u64,
            info: *mut EpochInfo,
        ) -> bool;

        fn rowex_string_new() -> UniquePtr<Rowex>;
        unsafe fn rowex_string_insert(
            rowex: *mut Rowex,
            kbuf: *const c_char,
            klen: usize,
            value: u64,
            info: *mut EpochInfo,
        ) -> bool;
        unsafe fn rowex_string_lookup(
            rowex: *mut Rowex,
            kbuf: *const c_char,
            klen: usize,
            value: *mut u64,
            info: *mut EpochInfo,
        ) -> bool;
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
    pub fn insert_u64(&self, key: u64, value: u64) -> bool {
        unsafe {
            ffi::rowex_u64_insert(
                self.rowex as *const _ as *mut _,
                key,
                value,
                self.epoch.as_mut_ptr(),
            )
        }
    }

    #[inline]
    pub fn get_u64(&self, key: u64) -> Option<u64> {
        unsafe {
            let mut value = 0u64;
            ffi::rowex_u64_lookup(
                self.rowex as *const _ as *mut _,
                key,
                &mut value,
                self.epoch.as_mut_ptr(),
            )
            .then_some(value)
        }
    }
}

impl<'a> RowexRef<'a, String> {
    #[inline]
    pub fn insert_string(&self, key: &str, value: u64) -> bool {
        unsafe {
            ffi::rowex_string_insert(
                self.rowex as *const _ as *mut _,
                key.as_ptr().cast(),
                key.len(),
                value,
                self.epoch.as_mut_ptr(),
            )
        }
    }

    #[inline]
    pub fn get_string(&self, key: &str) -> Option<u64> {
        unsafe {
            let mut value = 0u64;
            ffi::rowex_string_lookup(
                self.rowex as *const _ as *mut _,
                key.as_ptr().cast(),
                key.len(),
                &mut value,
                self.epoch.as_mut_ptr(),
            )
            .then_some(value)
        }
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
            assert!(map.insert_u64(i, i));
        }

        for i in (1..COUNT).step_by(3) {
            assert_eq!(map.get_u64(i), Some(i));
        }

        for i in (1..COUNT).skip(1).step_by(3) {
            assert_eq!(map.get_u64(i), None);
        }
    }

    #[test]
    fn duplicate_u64() {
        let rowex = Rowex::new_u64();
        let map = rowex.pin();

        const COUNT: u64 = 10_000;

        for i in 0..COUNT {
            assert!(map.insert_u64(i, i));
        }

        for i in 0..COUNT {
            assert!(!map.insert_u64(i, i + 1));
        }
    }

    #[test]
    fn string() {
        const DATA: [&str; 5] = ["1\n", "12\n", "123\n", "1234\n", "12345\n"];

        let rowex = Rowex::new_string();
        let map = rowex.pin();

        for (i, string) in DATA.iter().enumerate() {
            assert!(map.insert_string(string, i as u64));
        }

        for (i, string) in DATA.iter().enumerate() {
            assert_eq!(map.get_string(string), Some(i as u64));
        }
    }

    #[test]
    fn long_strings() {
        let rowex = Rowex::new_string();
        let map = rowex.pin();

        let keys = (1..1000)
            .map(|len| {
                let mut key = "a".repeat(len);
                key.push('\n');
                key
            })
            .collect::<Vec<_>>();

        for (i, string) in keys.iter().enumerate() {
            assert!(map.insert_string(string, i as u64));
        }

        for (i, string) in keys.iter().enumerate() {
            assert_eq!(map.get_string(string), Some(i as u64));
        }
    }
}
