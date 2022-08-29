#[cfg(feature = "context")]
use libc::c_void;

use sais_sys::sais16::*;

#[cfg(feature = "context")]
use std::ptr::NonNull;

use crate::common::*;

/// Output symbol frequency table for u16 strings.
pub const FREQ_TABLE_SIZE: usize = 65536;

/// Interpreted error code of 32-bit sais algorithms specialized for u16 strings.
pub type Error = crate::errors::Error<i32>;

/// Interpreted return value of 32-bit sais algorithms specialized for u16 strings.
pub type Result<T> = std::result::Result<T, Error>;

/// Reusable sais/bwt computation context of 32-bit sais algorithms specialized for u16 strings.
#[cfg(feature = "context")]
pub struct SaisContext(NonNull<c_void>);

#[cfg(feature = "context")]
impl SaisContext {
    /// Create new single-threaded sais/bwt computation context.
    pub fn new() -> Option<Self> {
        unsafe {
            let ctx_ptr = libsais16_create_ctx();
            NonNull::new(ctx_ptr).map(SaisContext)
        }
    }

    /// Create new multi-threaded sais/bwt computation context.
    #[cfg(feature = "parallel")]
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

    #[cfg(feature = "bwt")]
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

    #[cfg(feature = "bwt_aux")]
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

#[cfg(feature = "context")]
impl Drop for SaisContext {
    fn drop(&mut self) {
        unsafe {
            libsais16_free_ctx(self.0.as_ptr());
        }
    }
}

/// Reusable unbwt computation context of 32-bit sais algorithms specialized for u16 strings.
#[cfg(all(feature = "context", any(feature = "bwt", feature = "bwt_aux")))]
pub struct UnbwtContext(NonNull<c_void>);

#[cfg(all(feature = "context", any(feature = "bwt", feature = "bwt_aux")))]
impl UnbwtContext {
    /// Create new single-threaded unbwt computation context.
    pub fn new() -> Option<Self> {
        unsafe {
            let ctx_ptr = libsais16_unbwt_create_ctx();
            NonNull::new(ctx_ptr).map(UnbwtContext)
        }
    }

    /// Create new multi-threaded unbwt computation context.
    #[cfg(feature = "parallel")]
    pub fn new_parallel(threads: i32) -> Option<Self> {
        unsafe {
            let ctx_ptr = libsais16_unbwt_create_ctx_omp(threads);
            NonNull::new(ctx_ptr).map(UnbwtContext)
        }
    }

    #[cfg(feature = "bwt")]
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

    #[cfg(feature = "bwt_aux")]
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

#[cfg(all(feature = "context", any(feature = "bwt", feature = "bwt_aux")))]
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

#[cfg(feature = "bwt")]
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

#[cfg(feature = "bwt_aux")]
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

#[cfg(feature = "bwt")]
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

#[cfg(feature = "bwt_aux")]
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

#[cfg(feature = "lcp")]
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

#[cfg(feature = "lcp")]
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

#[cfg(feature = "parallel")]
pub mod parallel {
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

    #[cfg(feature = "bwt")]
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

    #[cfg(feature = "bwt_aux")]
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

    #[cfg(feature = "bwt")]
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

    #[cfg(feature = "bwt_aux")]
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

    #[cfg(feature = "lcp")]
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

    #[cfg(feature = "lcp")]
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
