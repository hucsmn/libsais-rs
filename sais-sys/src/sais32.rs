//! 32-bit sais algorithms on u8 array inputs.

use std::ptr::NonNull;

use libc::c_void;

use crate::common::{aux_rate, freq_as_mut_ptr, freq_as_ptr, interpret_return_code_32, same_size, split_size};

extern "C" {
    /// void * libsais_create_ctx(void);
    fn libsais_create_ctx() -> *mut c_void;

    /// void libsais_free_ctx(void * ctx);
    fn libsais_free_ctx(ctx: *mut c_void);

    /// int32_t libsais(const uint8_t * T, int32_t * SA, int32_t n, int32_t fs, int32_t * freq);
    fn libsais(t: *const u8, sa: *mut i32, n: i32, fs: i32, freq: *mut i32) -> i32;

    /// int32_t libsais_int(int32_t * T, int32_t * SA, int32_t n, int32_t k, int32_t fs);
    fn libsais_int(t: *mut i32, sa: *mut i32, n: i32, k: i32, fs: i32) -> i32;

    /// int32_t libsais_ctx(const void * ctx, const uint8_t * T, int32_t * SA, int32_t n, int32_t fs, int32_t * freq);
    fn libsais_ctx(ctx: *mut c_void, t: *const u8, sa: *mut i32, n: i32, fs: i32, freq: *mut i32) -> i32;

    /// int32_t libsais_bwt(const uint8_t * T, uint8_t * U, int32_t * A, int32_t n, int32_t fs, int32_t * freq);
    fn libsais_bwt(t: *const u8, u: *mut u8, a: *mut i32, n: i32, fs: i32, freq: *mut i32) -> i32;

    /// int32_t libsais_bwt_aux(const uint8_t * T, uint8_t * U, int32_t * A, int32_t n, int32_t fs, int32_t * freq, int32_t r, int32_t * I);
    fn libsais_bwt_aux(t: *const u8, u: *mut u8, a: *mut i32, n: i32, fs: i32, freq: *mut i32, r: i32, i: *mut i32) -> i32;

    /// int32_t libsais_bwt_ctx(const void * ctx, const uint8_t * T, uint8_t * U, int32_t * A, int32_t n, int32_t fs, int32_t * freq);
    fn libsais_bwt_ctx(ctx: *mut c_void, t: *const u8, u: *mut u8, a: *mut i32, n: i32, fs: i32, freq: *mut i32) -> i32;

    /// int32_t libsais_bwt_aux_ctx(const void * ctx, const uint8_t * T, uint8_t * U, int32_t * A, int32_t n, int32_t fs, int32_t * freq, int32_t r, int32_t * I);
    fn libsais_bwt_aux_ctx(ctx: *mut c_void, t: *const u8, u: *mut u8, a: *mut i32, n: i32, fs: i32, freq: *mut i32, r: i32, i: *mut i32) -> i32;

    /// void * libsais_unbwt_create_ctx(void);
    fn libsais_unbwt_create_ctx() -> *mut c_void;

    /// void libsais_unbwt_free_ctx(void * ctx);
    fn libsais_unbwt_free_ctx(ctx: *mut c_void);

    /// int32_t libsais_unbwt(const uint8_t * T, uint8_t * U, int32_t * A, int32_t n, const int32_t * freq, int32_t i);
    fn libsais_unbwt(t: *const u8, u: *mut u8, a: *mut i32, n: i32, freq: *const i32, i: i32) -> i32;

    /// int32_t libsais_unbwt_ctx(const void * ctx, const uint8_t * T, uint8_t * U, int32_t * A, int32_t n, const int32_t * freq, int32_t i);
    fn libsais_unbwt_ctx(ctx: *mut c_void, t: *const u8, u: *mut u8, a: *mut i32, n: i32, freq: *const i32, i: i32) -> i32;

    /// int32_t libsais_unbwt_aux(const uint8_t * T, uint8_t * U, int32_t * A, int32_t n, const int32_t * freq, int32_t r, const int32_t * I);
    fn libsais_unbwt_aux(t: *const u8, u: *mut u8, a: *mut i32, n: i32, freq: *const i32, r: i32, i: *const i32) -> i32;

    /// int32_t libsais_unbwt_aux_ctx(const void * ctx, const uint8_t * T, uint8_t * U, int32_t * A, int32_t n, const int32_t * freq, int32_t r, const int32_t * I);
    fn libsais_unbwt_aux_ctx(ctx: *mut c_void, t: *const u8, u: *mut u8, a: *mut i32, n: i32, freq: *const i32, r: i32, i: *const i32) -> i32;

    /// int32_t libsais_plcp(const uint8_t * T, const int32_t * SA, int32_t * PLCP, int32_t n);
    fn libsais_plcp(t: *const u8, sa: *const i32, plcp: *mut i32, n: i32) -> i32;

    /// int32_t libsais_lcp(const int32_t * PLCP, const int32_t * SA, int32_t * LCP, int32_t n);
    fn libsais_lcp(plcp: *const i32, sa: *const i32, lcp: *mut i32, n: i32) -> i32;
}

#[cfg(feature = "openmp")]
extern "C" {
    /// void * libsais_create_ctx_omp(int32_t threads);
    fn libsais_create_ctx_omp(threads: i32) -> *mut c_void;

    /// int32_t libsais_omp(const uint8_t * T, int32_t * SA, int32_t n, int32_t fs, int32_t * freq, int32_t threads);
    fn libsais_omp(t: *const u8, sa: *mut i32, n: i32, fs: i32, freq: *mut i32, threads: i32) -> i32;

    /// int32_t libsais_int_omp(int32_t * T, int32_t * SA, int32_t n, int32_t k, int32_t fs, int32_t threads);
    fn libsais_int_omp(t: *mut i32, sa: *mut i32, n: i32, k: i32, fs: i32, threads: i32) -> i32;

    /// int32_t libsais_bwt_omp(const uint8_t * T, uint8_t * U, int32_t * A, int32_t n, int32_t fs, int32_t * freq, int32_t threads);
    fn libsais_bwt_omp(t: *const u8, u: *mut u8, a: *mut i32, n: i32, fs: i32, freq: *mut i32, threads: i32) -> i32;

    /// int32_t libsais_bwt_aux_omp(const uint8_t * T, uint8_t * U, int32_t * A, int32_t n, int32_t fs, int32_t * freq, int32_t r, int32_t * I, int32_t threads);
    fn libsais_bwt_aux_omp(t: *const u8, u: *mut u8, a: *mut i32, n: i32, fs: i32, freq: *mut i32, r: i32, i: *mut i32, threads: i32) -> i32;

    /// void * libsais_unbwt_create_ctx_omp(int32_t threads);
    fn libsais_unbwt_create_ctx_omp(threads: i32) -> *mut c_void;

    /// int32_t libsais_unbwt_omp(const uint8_t * T, uint8_t * U, int32_t * A, int32_t n, const int32_t * freq, int32_t i, int32_t threads);
    fn libsais_unbwt_omp(t: *const u8, u: *mut u8, a: *mut i32, n: i32, freq: *const i32, i: i32, threads: i32) -> i32;

    /// int32_t libsais_unbwt_aux_omp(const uint8_t * T, uint8_t * U, int32_t * A, int32_t n, const int32_t * freq, int32_t r, const int32_t * I, int32_t threads);
    fn libsais_unbwt_aux_omp(t: *const u8, u: *mut u8, a: *mut i32, n: i32, freq: *const i32, r: i32, i: *const i32, threads: i32) -> i32;

    /// int32_t libsais_plcp_omp(const uint8_t * T, const int32_t * SA, int32_t * PLCP, int32_t n, int32_t threads);
    fn libsais_plcp_omp(t: *const u8, sa: *const i32, plcp: *mut i32, n: i32, threads: i32) -> i32;

    /// int32_t libsais_lcp_omp(const int32_t * PLCP, const int32_t * SA, int32_t * LCP, int32_t n, int32_t threads);
    fn libsais_lcp_omp(plcp: *const i32, sa: *const i32, lcp: *mut i32, n: i32, threads: i32) -> i32;
}

/// Output symbol frequency table size for u8 strings.
pub const FREQ_TABLE_SIZE: usize = 256;

/// Interpreted error code for 32-bit sais algorithms.
pub type Error = crate::errors::Error<i32>;

/// Interpreted return value for 32-bit sais algorithms.
pub type Result<T> = std::result::Result<T, Error>;

/// Reusable sais/bwt computation context for 32-bit sais algorithms.
pub struct SaisContext(NonNull<c_void>);

impl SaisContext {
    /// Create new single-threaded sais/bwt computation context.
    pub fn new() -> Option<Self> {
        unsafe {
            let ctx_ptr = libsais_create_ctx();
            NonNull::new(ctx_ptr).map(SaisContext)
        }
    }

    /// Create new multi-threaded sais/bwt computation context.
    #[cfg(feature = "openmp")]
    pub fn new_parallel(threads: i32) -> Option<Self> {
        unsafe {
            let ctx_ptr = libsais_create_ctx_omp(threads);
            NonNull::new(ctx_ptr).map(SaisContext)
        }
    }

    pub fn sais(&mut self, t: &[u8], sa: &mut [i32], freq: Option<&mut [i32]>) -> Result<()> {
        unsafe {
            let t_ptr = t.as_ptr();
            let sa_ptr = sa.as_mut_ptr();
            let (n, fs) = split_size(t.len(), sa.len())?;
            let freq_ptr = freq_as_mut_ptr(freq, FREQ_TABLE_SIZE)?;

            let code = libsais_ctx(self.0.as_mut(), t_ptr, sa_ptr, n, fs, freq_ptr);
            interpret_return_code_32(code).map(|_| ())
        }
    }

    pub fn bwt(&mut self, t: &[u8], u: &mut [u8], a: &mut [i32], freq: Option<&mut [i32]>) -> Result<i32> {
        unsafe {
            let t_ptr = t.as_ptr();
            let u_ptr = u.as_mut_ptr();
            let a_ptr = a.as_mut_ptr();
            let (n, fs) = split_size(same_size(t.len(), u.len())?, a.len())?;
            let freq_ptr = freq_as_mut_ptr(freq, FREQ_TABLE_SIZE)?;

            let code = libsais_bwt_ctx(self.0.as_mut(), t_ptr, u_ptr, a_ptr, n, fs, freq_ptr);
            interpret_return_code_32(code)
        }
    }

    pub fn bwt_aux(&mut self, t: &[u8], u: &mut [u8], a: &mut [i32], freq: Option<&mut [i32]>, i: &mut [i32]) -> Result<()> {
        unsafe {
            let t_ptr = t.as_ptr();
            let u_ptr = u.as_mut_ptr();
            let a_ptr = a.as_mut_ptr();
            let (n, fs) = split_size(same_size(t.len(), u.len())?, a.len())?;
            let freq_ptr = freq_as_mut_ptr(freq, FREQ_TABLE_SIZE)?;
            let r = aux_rate(i.len(), t.len())?;
            let i_ptr = i.as_mut_ptr();

            let code = libsais_bwt_aux_ctx(self.0.as_mut(), t_ptr, u_ptr, a_ptr, n, fs, freq_ptr, r, i_ptr);
            interpret_return_code_32(code).map(|_| ())
        }
    }
}

impl Drop for SaisContext {
    fn drop(&mut self) {
        unsafe {
            libsais_free_ctx(self.0.as_ptr());
        }
    }
}

/// Reusable unbwt computation context for 32-bit sais algorithms.
pub struct UnbwtContext(NonNull<c_void>);

impl UnbwtContext {
    /// Create new single-threaded unbwt computation context.
    pub fn new() -> Option<Self> {
        unsafe {
            let ctx_ptr = libsais_unbwt_create_ctx();
            NonNull::new(ctx_ptr).map(UnbwtContext)
        }
    }

    /// Create new multi-threaded unbwt computation context.
    #[cfg(feature = "openmp")]
    pub fn new_parallel(threads: i32) -> Option<Self> {
        unsafe {
            let ctx_ptr = libsais_unbwt_create_ctx_omp(threads);
            NonNull::new(ctx_ptr).map(UnbwtContext)
        }
    }

    pub fn unbwt(&mut self, t: &[u8], u: &mut [u8], a: &mut [i32], freq: Option<&[i32]>, i: i32) -> Result<()> {
        unsafe {
            let t_ptr = t.as_ptr();
            let u_ptr = u.as_mut_ptr();
            let a_ptr = a.as_mut_ptr();
            let (n, _) = split_size(same_size(t.len(), u.len())?, a.len())?;
            let freq_ptr = freq_as_ptr(freq, FREQ_TABLE_SIZE)?;

            let code = libsais_unbwt_ctx(self.0.as_mut(), t_ptr, u_ptr, a_ptr, n, freq_ptr, i);
            interpret_return_code_32(code).map(|_| ())
        }
    }

    pub fn unbwt_aux(&mut self, t: &[u8], u: &mut [u8], a: &mut [i32], freq: Option<&[i32]>, i: &[i32]) -> Result<()> {
        unsafe {
            let t_ptr = t.as_ptr();
            let u_ptr = u.as_mut_ptr();
            let a_ptr = a.as_mut_ptr();
            let (n, _) = split_size(same_size(t.len(), u.len())?, a.len())?;
            let freq_ptr = freq_as_ptr(freq, FREQ_TABLE_SIZE)?;
            let r = aux_rate(i.len(), t.len())?;
            let i_ptr = i.as_ptr();

            let code = libsais_unbwt_aux_ctx(self.0.as_mut(), t_ptr, u_ptr, a_ptr, n, freq_ptr, r, i_ptr);
            interpret_return_code_32(code).map(|_| ())
        }
    }
}

impl Drop for UnbwtContext {
    fn drop(&mut self) {
        unsafe {
            libsais_unbwt_free_ctx(self.0.as_ptr());
        }
    }
}

pub fn sais(t: &[u8], sa: &mut [i32], freq: Option<&mut [i32]>) -> Result<()> {
    unsafe {
        let t_ptr = t.as_ptr();
        let sa_ptr = sa.as_mut_ptr();
        let (n, fs) = split_size(t.len(), sa.len())?;
        let freq_ptr = freq_as_mut_ptr(freq, FREQ_TABLE_SIZE)?;

        let code = libsais(t_ptr, sa_ptr, n, fs, freq_ptr);
        interpret_return_code_32(code).map(|_| ())
    }
}

pub fn sais_int(t: &mut [i32], sa: &mut [i32], k: i32) -> Result<()> {
    unsafe {
        let t_ptr = t.as_mut_ptr();
        let sa_ptr = sa.as_mut_ptr();
        let (n, fs) = split_size(t.len(), sa.len())?;

        let code = libsais_int(t_ptr, sa_ptr, n, k, fs);
        interpret_return_code_32(code).map(|_| ())
    }
}

pub fn bwt(t: &[u8], u: &mut [u8], a: &mut [i32], freq: Option<&mut [i32]>) -> Result<i32> {
    unsafe {
        let t_ptr = t.as_ptr();
        let u_ptr = u.as_mut_ptr();
        let a_ptr = a.as_mut_ptr();
        let (n, fs) = split_size(same_size(t.len(), u.len())?, a.len())?;
        let freq_ptr = freq_as_mut_ptr(freq, FREQ_TABLE_SIZE)?;

        let code = libsais_bwt(t_ptr, u_ptr, a_ptr, n, fs, freq_ptr);
        interpret_return_code_32(code)
    }
}

pub fn bwt_aux(t: &[u8], u: &mut [u8], a: &mut [i32], freq: Option<&mut [i32]>, i: &mut [i32]) -> Result<()> {
    unsafe {
        let t_ptr = t.as_ptr();
        let u_ptr = u.as_mut_ptr();
        let a_ptr = a.as_mut_ptr();
        let (n, fs) = split_size(same_size(t.len(), u.len())?, a.len())?;
        let freq_ptr = freq_as_mut_ptr(freq, FREQ_TABLE_SIZE)?;
        let r = aux_rate(i.len(), t.len())?;
        let i_ptr = i.as_mut_ptr();

        let code = libsais_bwt_aux(t_ptr, u_ptr, a_ptr, n, fs, freq_ptr, r, i_ptr);
        interpret_return_code_32(code).map(|_| ())
    }
}

pub fn unbwt(t: &[u8], u: &mut [u8], a: &mut [i32], freq: Option<&[i32]>, i: i32) -> Result<()> {
    unsafe {
        let t_ptr = t.as_ptr();
        let u_ptr = u.as_mut_ptr();
        let a_ptr = a.as_mut_ptr();
        let (n, _) = split_size(same_size(t.len(), u.len())?, a.len())?;
        let freq_ptr = freq_as_ptr(freq, FREQ_TABLE_SIZE)?;

        let code = libsais_unbwt(t_ptr, u_ptr, a_ptr, n, freq_ptr, i);
        interpret_return_code_32(code).map(|_| ())
    }
}

pub fn unbwt_aux(t: &[u8], u: &mut [u8], a: &mut [i32], freq: Option<&[i32]>, i: &[i32]) -> Result<()> {
    unsafe {
        let t_ptr = t.as_ptr();
        let u_ptr = u.as_mut_ptr();
        let a_ptr = a.as_mut_ptr();
        let (n, _) = split_size(same_size(t.len(), u.len())?, a.len())?;
        let freq_ptr = freq_as_ptr(freq, FREQ_TABLE_SIZE)?;
        let r = aux_rate(i.len(), t.len())?;
        let i_ptr = i.as_ptr();

        let code = libsais_unbwt_aux(t_ptr, u_ptr, a_ptr, n, freq_ptr, r, i_ptr);
        interpret_return_code_32(code).map(|_| ())
    }
}

pub fn plcp(t: &[u8], sa: &[i32], plcp: &mut [i32]) -> Result<()> {
    unsafe {
        let t_ptr = t.as_ptr();
        let sa_ptr = sa.as_ptr();
        let plcp_ptr = plcp.as_mut_ptr();
        let (n, _) = split_size(same_size(t.len(), plcp.len())?, sa.len())?;

        let code = libsais_plcp(t_ptr, sa_ptr, plcp_ptr, n);
        interpret_return_code_32(code).map(|_| ())
    }
}

pub fn lcp(plcp: &[i32], sa: &[i32], lcp: &mut [i32]) -> Result<()> {
    unsafe {
        let plcp_ptr = plcp.as_ptr();
        let sa_ptr = sa.as_ptr();
        let lcp_ptr = lcp.as_mut_ptr();
        let (n, _) = split_size(same_size(plcp.len(), lcp.len())?, sa.len())?;

        let code = libsais_lcp(plcp_ptr, sa_ptr, lcp_ptr, n);
        interpret_return_code_32(code).map(|_| ())
    }
}

#[cfg(feature = "openmp")]
pub mod openmp {
    //! Multi-threaded 32-bit sais algorithms on u8 array inputs.

    use super::*;

    pub fn sais(t: &[u8], sa: &mut [i32], freq: Option<&mut [i32]>, threads: i32) -> Result<()> {
        unsafe {
            let t_ptr = t.as_ptr();
            let sa_ptr = sa.as_mut_ptr();
            let (n, fs) = split_size(t.len(), sa.len())?;
            let freq_ptr = freq_as_mut_ptr(freq, FREQ_TABLE_SIZE)?;

            let code = libsais_omp(t_ptr, sa_ptr, n, fs, freq_ptr, threads);
            interpret_return_code_32(code).map(|_| ())
        }
    }

    pub fn sais_int(t: &mut [i32], sa: &mut [i32], k: i32, threads: i32) -> Result<()> {
        unsafe {
            let t_ptr = t.as_mut_ptr();
            let sa_ptr = sa.as_mut_ptr();
            let (n, fs) = split_size(t.len(), sa.len())?;

            let code = libsais_int_omp(t_ptr, sa_ptr, n, k, fs, threads);
            interpret_return_code_32(code).map(|_| ())
        }
    }

    pub fn bwt(t: &[u8], u: &mut [u8], a: &mut [i32], freq: Option<&mut [i32]>, threads: i32) -> Result<i32> {
        unsafe {
            let t_ptr = t.as_ptr();
            let u_ptr = u.as_mut_ptr();
            let a_ptr = a.as_mut_ptr();
            let (n, fs) = split_size(same_size(t.len(), u.len())?, a.len())?;
            let freq_ptr = freq_as_mut_ptr(freq, FREQ_TABLE_SIZE)?;

            let code = libsais_bwt_omp(t_ptr, u_ptr, a_ptr, n, fs, freq_ptr, threads);
            interpret_return_code_32(code)
        }
    }

    pub fn bwt_aux(t: &[u8], u: &mut [u8], a: &mut [i32], freq: Option<&mut [i32]>, i: &mut [i32], threads: i32) -> Result<()> {
        unsafe {
            let t_ptr = t.as_ptr();
            let u_ptr = u.as_mut_ptr();
            let a_ptr = a.as_mut_ptr();
            let (n, fs) = split_size(same_size(t.len(), u.len())?, a.len())?;
            let freq_ptr = freq_as_mut_ptr(freq, FREQ_TABLE_SIZE)?;
            let r = aux_rate(i.len(), t.len())?;
            let i_ptr = i.as_mut_ptr();

            let code = libsais_bwt_aux_omp(t_ptr, u_ptr, a_ptr, n, fs, freq_ptr, r, i_ptr, threads);
            interpret_return_code_32(code).map(|_| ())
        }
    }

    pub fn unbwt(t: &[u8], u: &mut [u8], a: &mut [i32], freq: Option<&[i32]>, i: i32, threads: i32) -> Result<()> {
        unsafe {
            let t_ptr = t.as_ptr();
            let u_ptr = u.as_mut_ptr();
            let a_ptr = a.as_mut_ptr();
            let (n, _) = split_size(same_size(t.len(), u.len())?, a.len())?;
            let freq_ptr = freq_as_ptr(freq, FREQ_TABLE_SIZE)?;

            let code = libsais_unbwt_omp(t_ptr, u_ptr, a_ptr, n, freq_ptr, i, threads);
            interpret_return_code_32(code).map(|_| ())
        }
    }

    pub fn unbwt_aux(t: &[u8], u: &mut [u8], a: &mut [i32], freq: Option<&[i32]>, i: &[i32], threads: i32) -> Result<()> {
        unsafe {
            let t_ptr = t.as_ptr();
            let u_ptr = u.as_mut_ptr();
            let a_ptr = a.as_mut_ptr();
            let (n, _) = split_size(same_size(t.len(), u.len())?, a.len())?;
            let freq_ptr = freq_as_ptr(freq, FREQ_TABLE_SIZE)?;
            let r = aux_rate(i.len(), t.len())?;
            let i_ptr = i.as_ptr();

            let code = libsais_unbwt_aux_omp(t_ptr, u_ptr, a_ptr, n, freq_ptr, r, i_ptr, threads);
            interpret_return_code_32(code).map(|_| ())
        }
    }

    pub fn plcp(t: &[u8], sa: &[i32], plcp: &mut [i32], threads: i32) -> Result<()> {
        unsafe {
            let t_ptr = t.as_ptr();
            let sa_ptr = sa.as_ptr();
            let plcp_ptr = plcp.as_mut_ptr();
            let (n, _) = split_size(same_size(t.len(), plcp.len())?, sa.len())?;

            let code = libsais_plcp_omp(t_ptr, sa_ptr, plcp_ptr, n, threads);
            interpret_return_code_32(code).map(|_| ())
        }
    }

    pub fn lcp(plcp: &[i32], sa: &[i32], lcp: &mut [i32], threads: i32) -> Result<()> {
        unsafe {
            let plcp_ptr = plcp.as_ptr();
            let sa_ptr = sa.as_ptr();
            let lcp_ptr = lcp.as_mut_ptr();
            let (n, _) = split_size(same_size(plcp.len(), lcp.len())?, sa.len())?;

            let code = libsais_lcp_omp(plcp_ptr, sa_ptr, lcp_ptr, n, threads);
            interpret_return_code_32(code).map(|_| ())
        }
    }
}
