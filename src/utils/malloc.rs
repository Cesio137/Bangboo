#[cfg(target_env = "gnu")]
pub mod malloc {
    use std::os::raw::c_int;
    const M_MMAP_THRESHOLD: c_int = -3;

    unsafe extern "C" {
        fn malloc_trim(pad: usize);
        fn mallopt(param: c_int, value: c_int) -> c_int;
    }

    /// Prevents glibc from hoarding memory via memory fragmentation.
    pub fn limit_mmap_threshold() {
        unsafe {
            mallopt(M_MMAP_THRESHOLD, 65536);
        }
    }

    /// Asks glibc to trim malloc arenas.
    pub fn trim() {
        unsafe {
            malloc_trim(0);
        }
    }
}