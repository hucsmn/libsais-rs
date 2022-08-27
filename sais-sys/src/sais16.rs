//! 32-bit sais algorithms on u16 array inputs.

use std::ptr::NonNull;

use libc::c_void;

use crate::common::{aux_rate, freq_as_mut_ptr, freq_as_ptr, interpret_return_code_32, same_size, split_size};

extern "C" {
    /// void * libsais16_create_ctx(void);
    fn libsais16_create_ctx() -> *mut c_void;

    /// void libsais16_free_ctx(void * ctx);
    fn libsais16_free_ctx(ctx: *mut c_void);

    /// int32_t libsais16(const uint16_t * T, int32_t * SA, int32_t n, int32_t fs, int32_t * freq);
    fn libsais16(t: *const u16, sa: *mut i32, n: i32, fs: i32, freq: *mut i32) -> i32;

    /// int32_t libsais16_ctx(const void * ctx, const uint16_t * T, int32_t * SA, int32_t n, int32_t fs, int32_t * freq);
    fn libsais16_ctx(ctx: *mut c_void, t: *const u16, sa: *mut i32, n: i32, fs: i32, freq: *mut i32) -> i32;

    /// int32_t libsais16_bwt(const uint16_t * T, uint16_t * U, int32_t * A, int32_t n, int32_t fs, int32_t * freq);
    fn libsais16_bwt(t: *const u16, u: *mut u16, a: *mut i32, n: i32, fs: i32, freq: *mut i32) -> i32;

    /// int32_t libsais16_bwt_aux(const uint16_t * T, uint16_t * U, int32_t * A, int32_t n, int32_t fs, int32_t * freq, int32_t r, int32_t * I);
    fn libsais16_bwt_aux(t: *const u16, u: *mut u16, a: *mut i32, n: i32, fs: i32, freq: *mut i32, r: i32, i: *mut i32) -> i32;

    /// int32_t libsais16_bwt_ctx(const void * ctx, const uint16_t * T, uint16_t * U, int32_t * A, int32_t n, int32_t fs, int32_t * freq);
    fn libsais16_bwt_ctx(ctx: *mut c_void, t: *const u16, u: *mut u16, a: *mut i32, n: i32, fs: i32, freq: *mut i32) -> i32;

    /// int32_t libsais16_bwt_aux_ctx(const void * ctx, const uint16_t * T, uint16_t * U, int32_t * A, int32_t n, int32_t fs, int32_t * freq, int32_t r, int32_t * I);
    fn libsais16_bwt_aux_ctx(ctx: *mut c_void, t: *const u16, u: *mut u16, a: *mut i32, n: i32, fs: i32, freq: *mut i32, r: i32, i: *mut i32) -> i32;

    /// void * libsais16_unbwt_create_ctx(void);
    fn libsais16_unbwt_create_ctx() -> *mut c_void;

    /// void libsais16_unbwt_free_ctx(void * ctx);
    fn libsais16_unbwt_free_ctx(ctx: *mut c_void);

    /// int32_t libsais16_unbwt(const uint16_t * T, uint16_t * U, int32_t * A, int32_t n, const int32_t * freq, int32_t i);
    fn libsais16_unbwt(t: *const u16, u: *mut u16, a: *mut i32, n: i32, freq: *const i32, i: i32) -> i32;

    /// int32_t libsais16_unbwt_ctx(const void * ctx, const uint16_t * T, uint16_t * U, int32_t * A, int32_t n, const int32_t * freq, int32_t i);
    fn libsais16_unbwt_ctx(ctx: *mut c_void, t: *const u16, u: *mut u16, a: *mut i32, n: i32, freq: *const i32, i: i32) -> i32;

    /// int32_t libsais16_unbwt_aux(const uint16_t * T, uint16_t * U, int32_t * A, int32_t n, const int32_t * freq, int32_t r, const int32_t * I);
    fn libsais16_unbwt_aux(t: *const u16, u: *mut u16, a: *mut i32, n: i32, freq: *const i32, r: i32, i: *const i32) -> i32;

    /// int32_t libsais16_unbwt_aux_ctx(const void * ctx, const uint16_t * T, uint16_t * U, int32_t * A, int32_t n, const int32_t * freq, int32_t r, const int32_t * I);
    fn libsais16_unbwt_aux_ctx(ctx: *mut c_void, t: *const u16, u: *mut u16, a: *mut i32, n: i32, freq: *const i32, r: i32, i: *const i32) -> i32;

    /// int32_t libsais16_plcp(const uint16_t * T, const int32_t * SA, int32_t * PLCP, int32_t n);
    fn libsais16_plcp(t: *const u16, sa: *const i32, plcp: *mut i32, n: i32) -> i32;

    /// int32_t libsais16_lcp(const int32_t * PLCP, const int32_t * SA, int32_t * LCP, int32_t n);
    fn libsais16_lcp(plcp: *const i32, sa: *const i32, lcp: *mut i32, n: i32) -> i32;
}

#[cfg(feature = "openmp")]
extern "C" {
    /// void * libsais16_create_ctx_omp(int32_t threads);
    fn libsais16_create_ctx_omp(threads: i32) -> *mut c_void;

    /// int32_t libsais16_omp(const uint16_t * T, int32_t * SA, int32_t n, int32_t fs, int32_t * freq, int32_t threads);
    fn libsais16_omp(t: *const u16, sa: *mut i32, n: i32, fs: i32, freq: *mut i32, threads: i32) -> i32;

    /// int32_t libsais16_bwt_omp(const uint16_t * T, uint16_t * U, int32_t * A, int32_t n, int32_t fs, int32_t * freq, int32_t threads);
    fn libsais16_bwt_omp(t: *const u16, u: *mut u16, a: *mut i32, n: i32, fs: i32, freq: *mut i32, threads: i32) -> i32;

    /// int32_t libsais16_bwt_aux_omp(const uint16_t * T, uint16_t * U, int32_t * A, int32_t n, int32_t fs, int32_t * freq, int32_t r, int32_t * I, int32_t threads);
    fn libsais16_bwt_aux_omp(t: *const u16, u: *mut u16, a: *mut i32, n: i32, fs: i32, freq: *mut i32, r: i32, i: *mut i32, threads: i32) -> i32;

    /// void * libsais16_unbwt_create_ctx_omp(int32_t threads);
    fn libsais16_unbwt_create_ctx_omp(threads: i32) -> *mut c_void;

    /// int32_t libsais16_unbwt_omp(const uint16_t * T, uint16_t * U, int32_t * A, int32_t n, const int32_t * freq, int32_t i, int32_t threads);
    fn libsais16_unbwt_omp(t: *const u16, u: *mut u16, a: *mut i32, n: i32, freq: *const i32, i: i32, threads: i32) -> i32;

    /// int32_t libsais16_unbwt_aux_omp(const uint16_t * T, uint16_t * U, int32_t * A, int32_t n, const int32_t * freq, int32_t r, const int32_t * I, int32_t threads);
    fn libsais16_unbwt_aux_omp(t: *const u16, u: *mut u16, a: *mut i32, n: i32, freq: *const i32, r: i32, i: *const i32, threads: i32) -> i32;

    /// int32_t libsais16_plcp_omp(const uint16_t * T, const int32_t * SA, int32_t * PLCP, int32_t n, int32_t threads);
    fn libsais16_plcp_omp(t: *const u16, sa: *const i32, plcp: *mut i32, n: i32, threads: i32) -> i32;

    /// int32_t libsais16_lcp_omp(const int32_t * PLCP, const int32_t * SA, int32_t * LCP, int32_t n, int32_t threads);
    fn libsais16_lcp_omp(plcp: *const i32, sa: *const i32, lcp: *mut i32, n: i32, threads: i32) -> i32;
}

/// Output symbol frequency table for u16 strings.
pub const FREQ_TABLE_SIZE: usize = 65536;

/// Interpreted error code of 32-bit sais algorithms specialized for u16 strings.
pub type Error = crate::errors::Error<i32>;

/// Interpreted return value of 32-bit sais algorithms specialized for u16 strings.
pub type Result<T> = std::result::Result<T, Error>;

/// Reusable sais/bwt computation context of 32-bit sais algorithms specialized for u16 strings.
pub struct SaisContext(NonNull<c_void>);

impl SaisContext {
    /// Create new single-threaded sais/bwt computation context.
    pub fn new() -> Option<Self> {
        unsafe {
            let ctx_ptr = libsais16_create_ctx();
            NonNull::new(ctx_ptr).map(SaisContext)
        }
    }

    /// Create new multi-threaded sais/bwt computation context.
    #[cfg(feature = "openmp")]
    pub fn new_parallel(threads: i32) -> Option<Self> {
        unsafe {
            let ctx_ptr = libsais16_create_ctx_omp(threads);
            NonNull::new(ctx_ptr).map(SaisContext)
        }
    }

    pub fn sais(&mut self, t: &[u16], sa: &mut [i32], freq: Option<&mut [i32]>) -> Result<()> {
        unsafe {
            let t_ptr = t.as_ptr();
            let sa_ptr = sa.as_mut_ptr();
            let (n, fs) = split_size(t.len(), sa.len())?;
            let freq_ptr = freq_as_mut_ptr(freq, FREQ_TABLE_SIZE)?;

            let code = libsais16_ctx(self.0.as_mut(), t_ptr, sa_ptr, n, fs, freq_ptr);
            interpret_return_code_32(code).map(|_| ())
        }
    }

    pub fn bwt(&mut self, t: &[u16], u: &mut [u16], a: &mut [i32], freq: Option<&mut [i32]>) -> Result<i32> {
        unsafe {
            let t_ptr = t.as_ptr();
            let u_ptr = u.as_mut_ptr();
            let a_ptr = a.as_mut_ptr();
            let (n, fs) = split_size(same_size(t.len(), u.len())?, a.len())?;
            let freq_ptr = freq_as_mut_ptr(freq, FREQ_TABLE_SIZE)?;

            let code = libsais16_bwt_ctx(self.0.as_mut(), t_ptr, u_ptr, a_ptr, n, fs, freq_ptr);
            interpret_return_code_32(code)
        }
    }

    pub fn bwt_aux(&mut self, t: &[u16], u: &mut [u16], a: &mut [i32], freq: Option<&mut [i32]>, i: &mut [i32]) -> Result<()> {
        unsafe {
            let t_ptr = t.as_ptr();
            let u_ptr = u.as_mut_ptr();
            let a_ptr = a.as_mut_ptr();
            let (n, fs) = split_size(same_size(t.len(), u.len())?, a.len())?;
            let freq_ptr = freq_as_mut_ptr(freq, FREQ_TABLE_SIZE)?;
            let r = aux_rate(i.len(), t.len())?;
            let i_ptr = i.as_mut_ptr();

            let code = libsais16_bwt_aux_ctx(self.0.as_mut(), t_ptr, u_ptr, a_ptr, n, fs, freq_ptr, r, i_ptr);
            interpret_return_code_32(code).map(|_| ())
        }
    }
}

impl Drop for SaisContext {
    fn drop(&mut self) {
        unsafe {
            libsais16_free_ctx(self.0.as_ptr());
        }
    }
}

/// Reusable unbwt computation context of 32-bit sais algorithms specialized for u16 strings.
pub struct UnbwtContext(NonNull<c_void>);

impl UnbwtContext {
    /// Create new single-threaded unbwt computation context.
    pub fn new() -> Option<Self> {
        unsafe {
            let ctx_ptr = libsais16_unbwt_create_ctx();
            NonNull::new(ctx_ptr).map(UnbwtContext)
        }
    }

    /// Create new multi-threaded unbwt computation context.
    #[cfg(feature = "openmp")]
    pub fn new_parallel(threads: i32) -> Option<Self> {
        unsafe {
            let ctx_ptr = libsais16_unbwt_create_ctx_omp(threads);
            NonNull::new(ctx_ptr).map(UnbwtContext)
        }
    }

    pub fn unbwt(&mut self, t: &[u16], u: &mut [u16], a: &mut [i32], freq: Option<&[i32]>, i: i32) -> Result<()> {
        unsafe {
            let t_ptr = t.as_ptr();
            let u_ptr = u.as_mut_ptr();
            let a_ptr = a.as_mut_ptr();
            let (n, _) = split_size(same_size(t.len(), u.len())?, a.len())?;
            let freq_ptr = freq_as_ptr(freq, FREQ_TABLE_SIZE)?;

            let code = libsais16_unbwt_ctx(self.0.as_mut(), t_ptr, u_ptr, a_ptr, n, freq_ptr, i);
            interpret_return_code_32(code).map(|_| ())
        }
    }

    pub fn unbwt_aux(&mut self, t: &[u16], u: &mut [u16], a: &mut [i32], freq: Option<&[i32]>, i: &[i32]) -> Result<()> {
        unsafe {
            let t_ptr = t.as_ptr();
            let u_ptr = u.as_mut_ptr();
            let a_ptr = a.as_mut_ptr();
            let (n, _) = split_size(same_size(t.len(), u.len())?, a.len())?;
            let freq_ptr = freq_as_ptr(freq, FREQ_TABLE_SIZE)?;
            let r = aux_rate(i.len(), t.len())?;
            let i_ptr = i.as_ptr();

            let code = libsais16_unbwt_aux_ctx(self.0.as_mut(), t_ptr, u_ptr, a_ptr, n, freq_ptr, r, i_ptr);
            interpret_return_code_32(code).map(|_| ())
        }
    }
}

impl Drop for UnbwtContext {
    fn drop(&mut self) {
        unsafe {
            libsais16_unbwt_free_ctx(self.0.as_ptr());
        }
    }
}

pub fn sais(t: &[u16], sa: &mut [i32], freq: Option<&mut [i32]>) -> Result<()> {
    unsafe {
        let t_ptr = t.as_ptr();
        let sa_ptr = sa.as_mut_ptr();
        let (n, fs) = split_size(t.len(), sa.len())?;
        let freq_ptr = freq_as_mut_ptr(freq, FREQ_TABLE_SIZE)?;

        let code = libsais16(t_ptr, sa_ptr, n, fs, freq_ptr);
        interpret_return_code_32(code).map(|_| ())
    }
}

pub fn bwt(t: &[u16], u: &mut [u16], a: &mut [i32], freq: Option<&mut [i32]>) -> Result<i32> {
    unsafe {
        let t_ptr = t.as_ptr();
        let u_ptr = u.as_mut_ptr();
        let a_ptr = a.as_mut_ptr();
        let (n, fs) = split_size(same_size(t.len(), u.len())?, a.len())?;
        let freq_ptr = freq_as_mut_ptr(freq, FREQ_TABLE_SIZE)?;

        let code = libsais16_bwt(t_ptr, u_ptr, a_ptr, n, fs, freq_ptr);
        interpret_return_code_32(code)
    }
}

pub fn bwt_aux(t: &[u16], u: &mut [u16], a: &mut [i32], freq: Option<&mut [i32]>, i: &mut [i32]) -> Result<()> {
    unsafe {
        let t_ptr = t.as_ptr();
        let u_ptr = u.as_mut_ptr();
        let a_ptr = a.as_mut_ptr();
        let (n, fs) = split_size(same_size(t.len(), u.len())?, a.len())?;
        let freq_ptr = freq_as_mut_ptr(freq, FREQ_TABLE_SIZE)?;
        let r = aux_rate(i.len(), t.len())?;
        let i_ptr = i.as_mut_ptr();

        let code = libsais16_bwt_aux(t_ptr, u_ptr, a_ptr, n, fs, freq_ptr, r, i_ptr);
        interpret_return_code_32(code).map(|_| ())
    }
}

pub fn unbwt(t: &[u16], u: &mut [u16], a: &mut [i32], freq: Option<&[i32]>, i: i32) -> Result<()> {
    unsafe {
        let t_ptr = t.as_ptr();
        let u_ptr = u.as_mut_ptr();
        let a_ptr = a.as_mut_ptr();
        let (n, _) = split_size(same_size(t.len(), u.len())?, a.len())?;
        let freq_ptr = freq_as_ptr(freq, FREQ_TABLE_SIZE)?;

        let code = libsais16_unbwt(t_ptr, u_ptr, a_ptr, n, freq_ptr, i);
        interpret_return_code_32(code).map(|_| ())
    }
}

pub fn unbwt_aux(t: &[u16], u: &mut [u16], a: &mut [i32], freq: Option<&[i32]>, i: &[i32]) -> Result<()> {
    unsafe {
        let t_ptr = t.as_ptr();
        let u_ptr = u.as_mut_ptr();
        let a_ptr = a.as_mut_ptr();
        let (n, _) = split_size(same_size(t.len(), u.len())?, a.len())?;
        let freq_ptr = freq_as_ptr(freq, FREQ_TABLE_SIZE)?;
        let r = aux_rate(i.len(), t.len())?;
        let i_ptr = i.as_ptr();

        let code = libsais16_unbwt_aux(t_ptr, u_ptr, a_ptr, n, freq_ptr, r, i_ptr);
        interpret_return_code_32(code).map(|_| ())
    }
}

pub fn plcp(t: &[u16], sa: &[i32], plcp: &mut [i32]) -> Result<()> {
    unsafe {
        let t_ptr = t.as_ptr();
        let sa_ptr = sa.as_ptr();
        let plcp_ptr = plcp.as_mut_ptr();
        let (n, _) = split_size(same_size(t.len(), plcp.len())?, sa.len())?;

        let code = libsais16_plcp(t_ptr, sa_ptr, plcp_ptr, n);
        interpret_return_code_32(code).map(|_| ())
    }
}

pub fn lcp(plcp: &[i32], sa: &[i32], lcp: &mut [i32]) -> Result<()> {
    unsafe {
        let plcp_ptr = plcp.as_ptr();
        let sa_ptr = sa.as_ptr();
        let lcp_ptr = lcp.as_mut_ptr();
        let (n, _) = split_size(same_size(plcp.len(), lcp.len())?, sa.len())?;

        let code = libsais16_lcp(plcp_ptr, sa_ptr, lcp_ptr, n);
        interpret_return_code_32(code).map(|_| ())
    }
}

#[cfg(feature = "openmp")]
pub mod openmp {
    //! Multi-threaded 32-bit sais algorithms on u16 array inputs.

    use super::*;

    pub fn sais(t: &[u16], sa: &mut [i32], freq: Option<&mut [i32]>, threads: i32) -> Result<()> {
        unsafe {
            let t_ptr = t.as_ptr();
            let sa_ptr = sa.as_mut_ptr();
            let (n, fs) = split_size(t.len(), sa.len())?;
            let freq_ptr = freq_as_mut_ptr(freq, FREQ_TABLE_SIZE)?;

            let code = libsais16_omp(t_ptr, sa_ptr, n, fs, freq_ptr, threads);
            interpret_return_code_32(code).map(|_| ())
        }
    }

    pub fn bwt(t: &[u16], u: &mut [u16], a: &mut [i32], freq: Option<&mut [i32]>, threads: i32) -> Result<i32> {
        unsafe {
            let t_ptr = t.as_ptr();
            let u_ptr = u.as_mut_ptr();
            let a_ptr = a.as_mut_ptr();
            let (n, fs) = split_size(same_size(t.len(), u.len())?, a.len())?;
            let freq_ptr = freq_as_mut_ptr(freq, FREQ_TABLE_SIZE)?;

            let code = libsais16_bwt_omp(t_ptr, u_ptr, a_ptr, n, fs, freq_ptr, threads);
            interpret_return_code_32(code)
        }
    }

    pub fn bwt_aux(t: &[u16], u: &mut [u16], a: &mut [i32], freq: Option<&mut [i32]>, i: &mut [i32], threads: i32) -> Result<()> {
        unsafe {
            let t_ptr = t.as_ptr();
            let u_ptr = u.as_mut_ptr();
            let a_ptr = a.as_mut_ptr();
            let (n, fs) = split_size(same_size(t.len(), u.len())?, a.len())?;
            let freq_ptr = freq_as_mut_ptr(freq, FREQ_TABLE_SIZE)?;
            let r = aux_rate(i.len(), t.len())?;
            let i_ptr = i.as_mut_ptr();

            let code = libsais16_bwt_aux_omp(t_ptr, u_ptr, a_ptr, n, fs, freq_ptr, r, i_ptr, threads);
            interpret_return_code_32(code).map(|_| ())
        }
    }

    pub fn unbwt(t: &[u16], u: &mut [u16], a: &mut [i32], freq: Option<&[i32]>, i: i32, threads: i32) -> Result<()> {
        unsafe {
            let t_ptr = t.as_ptr();
            let u_ptr = u.as_mut_ptr();
            let a_ptr = a.as_mut_ptr();
            let (n, _) = split_size(same_size(t.len(), u.len())?, a.len())?;
            let freq_ptr = freq_as_ptr(freq, FREQ_TABLE_SIZE)?;

            let code = libsais16_unbwt_omp(t_ptr, u_ptr, a_ptr, n, freq_ptr, i, threads);
            interpret_return_code_32(code).map(|_| ())
        }
    }

    pub fn unbwt_aux(t: &[u16], u: &mut [u16], a: &mut [i32], freq: Option<&[i32]>, i: &[i32], threads: i32) -> Result<()> {
        unsafe {
            let t_ptr = t.as_ptr();
            let u_ptr = u.as_mut_ptr();
            let a_ptr = a.as_mut_ptr();
            let (n, _) = split_size(same_size(t.len(), u.len())?, a.len())?;
            let freq_ptr = freq_as_ptr(freq, FREQ_TABLE_SIZE)?;
            let r = aux_rate(i.len(), t.len())?;
            let i_ptr = i.as_ptr();

            let code = libsais16_unbwt_aux_omp(t_ptr, u_ptr, a_ptr, n, freq_ptr, r, i_ptr, threads);
            interpret_return_code_32(code).map(|_| ())
        }
    }

    pub fn plcp(t: &[u16], sa: &[i32], plcp: &mut [i32], threads: i32) -> Result<()> {
        unsafe {
            let t_ptr = t.as_ptr();
            let sa_ptr = sa.as_ptr();
            let plcp_ptr = plcp.as_mut_ptr();
            let (n, _) = split_size(same_size(t.len(), plcp.len())?, sa.len())?;

            let code = libsais16_plcp_omp(t_ptr, sa_ptr, plcp_ptr, n, threads);
            interpret_return_code_32(code).map(|_| ())
        }
    }

    pub fn lcp(plcp: &[i32], sa: &[i32], lcp: &mut [i32], threads: i32) -> Result<()> {
        unsafe {
            let plcp_ptr = plcp.as_ptr();
            let sa_ptr = sa.as_ptr();
            let lcp_ptr = lcp.as_mut_ptr();
            let (n, _) = split_size(same_size(plcp.len(), lcp.len())?, sa.len())?;

            let code = libsais16_lcp_omp(plcp_ptr, sa_ptr, lcp_ptr, n, threads);
            interpret_return_code_32(code).map(|_| ())
        }
    }
}
