#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("art-sys/include/rowex.h");

        type EpochInfo;

        type Rowex;

        fn rowex_new() -> UniquePtr<Rowex>;

        unsafe fn rowex_info(rowex: *mut Rowex) -> UniquePtr<EpochInfo>;

        unsafe fn rowex_insert(rowex: *mut Rowex, key: u64, info: *mut EpochInfo);
    }
}

fn main() {
    let mut rowex = ffi::rowex_new();
    let mut info = unsafe { ffi::rowex_info(rowex.as_mut_ptr()) };
    unsafe { ffi::rowex_insert(rowex.as_mut_ptr(), 0, info.as_mut_ptr()) };
}
