use libc::c_void;

use sais_sys::sais32::*;

use std::ptr::NonNull;

use crate::common::*;

/// Maximum array length sais algorithms able to cope with.
pub const MAX_LENGTH: usize = i32::MAX as usize;

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
    #[cfg(feature = "parallel")]
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
            let (n, fs) = split_size(t.len(), max_size(sa.len(), MAX_LENGTH)?)?;
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
            let (n, fs) = split_size(same_size(t.len(), u.len())?, max_size(a.len(), MAX_LENGTH)?)?;
            let freq_ptr = freq_as_mut_ptr(freq, FREQ_TABLE_SIZE)?;

            let code = libsais_bwt_ctx(self.0.as_mut(), t_ptr, u_ptr, a_ptr, n, fs, freq_ptr);
            interpret_return_code_32(code)
        }
    }

    pub fn bwt_inplace(&mut self, t: &mut [u8], a: &mut [i32], freq: Option<&mut [i32]>) -> Result<i32> {
        unsafe {
            let t_ptr = t.as_ptr();
            let u_ptr = t.as_mut_ptr();
            let a_ptr = a.as_mut_ptr();
            let (n, fs) = split_size(t.len(), max_size(a.len(), MAX_LENGTH)?)?;
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
            let (n, fs) = split_size(same_size(t.len(), u.len())?, max_size(a.len(), MAX_LENGTH)?)?;
            let freq_ptr = freq_as_mut_ptr(freq, FREQ_TABLE_SIZE)?;
            let r = aux_rate(i.len(), t.len())?;
            let i_ptr = i.as_mut_ptr();

            let code = libsais_bwt_aux_ctx(self.0.as_mut(), t_ptr, u_ptr, a_ptr, n, fs, freq_ptr, r, i_ptr);
            interpret_return_code_32(code).map(|_| ())
        }
    }

    pub fn bwt_aux_inplace(&mut self, t: &mut [u8], a: &mut [i32], freq: Option<&mut [i32]>, i: &mut [i32]) -> Result<()> {
        unsafe {
            let t_ptr = t.as_ptr();
            let u_ptr = t.as_mut_ptr();
            let a_ptr = a.as_mut_ptr();
            let (n, fs) = split_size(t.len(), max_size(a.len(), MAX_LENGTH)?)?;
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
    #[cfg(feature = "parallel")]
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
            let n = unbwt_sufficient_size(same_size(t.len(), u.len())?, max_size(a.len(), MAX_LENGTH)?)?;
            let freq_ptr = freq_as_ptr(freq, FREQ_TABLE_SIZE)?;

            let code = libsais_unbwt_ctx(self.0.as_mut(), t_ptr, u_ptr, a_ptr, n, freq_ptr, i);
            interpret_return_code_32(code).map(|_| ())
        }
    }

    pub fn unbwt_inplace(&mut self, t: &mut [u8], a: &mut [i32], freq: Option<&[i32]>, i: i32) -> Result<()> {
        unsafe {
            let t_ptr = t.as_ptr();
            let u_ptr = t.as_mut_ptr();
            let a_ptr = a.as_mut_ptr();
            let n = unbwt_sufficient_size(t.len(), max_size(a.len(), MAX_LENGTH)?)?;
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
            let n = unbwt_sufficient_size(same_size(t.len(), u.len())?, max_size(a.len(), MAX_LENGTH)?)?;
            let freq_ptr = freq_as_ptr(freq, FREQ_TABLE_SIZE)?;
            let r = aux_rate(i.len(), t.len())?;
            let i_ptr = i.as_ptr();

            let code = libsais_unbwt_aux_ctx(self.0.as_mut(), t_ptr, u_ptr, a_ptr, n, freq_ptr, r, i_ptr);
            interpret_return_code_32(code).map(|_| ())
        }
    }

    pub fn unbwt_aux_inplace(&mut self, t: &mut [u8], a: &mut [i32], freq: Option<&[i32]>, i: &[i32]) -> Result<()> {
        unsafe {
            let t_ptr = t.as_ptr();
            let u_ptr = t.as_mut_ptr();
            let a_ptr = a.as_mut_ptr();
            let n = unbwt_sufficient_size(t.len(), max_size(a.len(), MAX_LENGTH)?)?;
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
        let (n, fs) = split_size(t.len(), max_size(sa.len(), MAX_LENGTH)?)?;
        let freq_ptr = freq_as_mut_ptr(freq, FREQ_TABLE_SIZE)?;

        let code = libsais(t_ptr, sa_ptr, n, fs, freq_ptr);
        interpret_return_code_32(code).map(|_| ())
    }
}

pub fn sais_int(t: &mut [i32], sa: &mut [i32], k: i32) -> Result<()> {
    unsafe {
        let t_ptr = t.as_mut_ptr();
        let sa_ptr = sa.as_mut_ptr();
        let (n, fs) = split_size(t.len(), max_size(sa.len(), MAX_LENGTH)?)?;

        let code = libsais_int(t_ptr, sa_ptr, n, k, fs);
        interpret_return_code_32(code).map(|_| ())
    }
}

pub fn bwt(t: &[u8], u: &mut [u8], a: &mut [i32], freq: Option<&mut [i32]>) -> Result<i32> {
    unsafe {
        let t_ptr = t.as_ptr();
        let u_ptr = u.as_mut_ptr();
        let a_ptr = a.as_mut_ptr();
        let (n, fs) = split_size(same_size(t.len(), u.len())?, max_size(a.len(), MAX_LENGTH)?)?;
        let freq_ptr = freq_as_mut_ptr(freq, FREQ_TABLE_SIZE)?;

        let code = libsais_bwt(t_ptr, u_ptr, a_ptr, n, fs, freq_ptr);
        interpret_return_code_32(code)
    }
}

pub fn bwt_inplace(t: &mut [u8], a: &mut [i32], freq: Option<&mut [i32]>) -> Result<i32> {
    unsafe {
        let t_ptr = t.as_ptr();
        let u_ptr = t.as_mut_ptr();
        let a_ptr = a.as_mut_ptr();
        let (n, fs) = split_size(t.len(), max_size(a.len(), MAX_LENGTH)?)?;
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
        let (n, fs) = split_size(same_size(t.len(), u.len())?, max_size(a.len(), MAX_LENGTH)?)?;
        let freq_ptr = freq_as_mut_ptr(freq, FREQ_TABLE_SIZE)?;
        let r = aux_rate(i.len(), t.len())?;
        let i_ptr = i.as_mut_ptr();

        let code = libsais_bwt_aux(t_ptr, u_ptr, a_ptr, n, fs, freq_ptr, r, i_ptr);
        interpret_return_code_32(code).map(|_| ())
    }
}

pub fn bwt_aux_inplace(t: &mut [u8], a: &mut [i32], freq: Option<&mut [i32]>, i: &mut [i32]) -> Result<()> {
    unsafe {
        let t_ptr = t.as_ptr();
        let u_ptr = t.as_mut_ptr();
        let a_ptr = a.as_mut_ptr();
        let (n, fs) = split_size(t.len(), max_size(a.len(), MAX_LENGTH)?)?;
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
        let n = unbwt_sufficient_size(same_size(t.len(), u.len())?, max_size(a.len(), MAX_LENGTH)?)?;
        let freq_ptr = freq_as_ptr(freq, FREQ_TABLE_SIZE)?;

        let code = libsais_unbwt(t_ptr, u_ptr, a_ptr, n, freq_ptr, i);
        interpret_return_code_32(code).map(|_| ())
    }
}

pub fn unbwt_inplace(t: &mut [u8], a: &mut [i32], freq: Option<&[i32]>, i: i32) -> Result<()> {
    unsafe {
        let t_ptr = t.as_ptr();
        let u_ptr = t.as_mut_ptr();
        let a_ptr = a.as_mut_ptr();
        let n = unbwt_sufficient_size(t.len(), max_size(a.len(), MAX_LENGTH)?)?;
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
        let n = unbwt_sufficient_size(same_size(t.len(), u.len())?, max_size(a.len(), MAX_LENGTH)?)?;
        let freq_ptr = freq_as_ptr(freq, FREQ_TABLE_SIZE)?;
        let r = aux_rate(i.len(), t.len())?;
        let i_ptr = i.as_ptr();

        let code = libsais_unbwt_aux(t_ptr, u_ptr, a_ptr, n, freq_ptr, r, i_ptr);
        interpret_return_code_32(code).map(|_| ())
    }
}

pub fn unbwt_aux_inplace(t: &mut [u8], a: &mut [i32], freq: Option<&[i32]>, i: &[i32]) -> Result<()> {
    unsafe {
        let t_ptr = t.as_ptr();
        let u_ptr = t.as_mut_ptr();
        let a_ptr = a.as_mut_ptr();
        let n = unbwt_sufficient_size(t.len(), max_size(a.len(), MAX_LENGTH)?)?;
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
        let (n, _) = split_size(same_size(t.len(), plcp.len())?, max_size(sa.len(), MAX_LENGTH)?)?;

        let code = libsais_plcp(t_ptr, sa_ptr, plcp_ptr, n);
        interpret_return_code_32(code).map(|_| ())
    }
}

pub fn lcp(plcp: &[i32], sa: &[i32], lcp: &mut [i32]) -> Result<()> {
    unsafe {
        let plcp_ptr = plcp.as_ptr();
        let sa_ptr = sa.as_ptr();
        let lcp_ptr = lcp.as_mut_ptr();
        let (n, _) = split_size(same_size(plcp.len(), lcp.len())?, max_size(sa.len(), MAX_LENGTH)?)?;

        let code = libsais_lcp(plcp_ptr, sa_ptr, lcp_ptr, n);
        interpret_return_code_32(code).map(|_| ())
    }
}

#[cfg(feature = "parallel")]
pub mod parallel {
    //! Multi-threaded 32-bit sais algorithms on u8 array inputs.

    use super::*;

    pub fn sais(t: &[u8], sa: &mut [i32], freq: Option<&mut [i32]>, threads: i32) -> Result<()> {
        unsafe {
            let t_ptr = t.as_ptr();
            let sa_ptr = sa.as_mut_ptr();
            let (n, fs) = split_size(t.len(), max_size(sa.len(), MAX_LENGTH)?)?;
            let freq_ptr = freq_as_mut_ptr(freq, FREQ_TABLE_SIZE)?;

            let code = libsais_omp(t_ptr, sa_ptr, n, fs, freq_ptr, threads);
            interpret_return_code_32(code).map(|_| ())
        }
    }

    pub fn sais_int(t: &mut [i32], sa: &mut [i32], k: i32, threads: i32) -> Result<()> {
        unsafe {
            let t_ptr = t.as_mut_ptr();
            let sa_ptr = sa.as_mut_ptr();
            let (n, fs) = split_size(t.len(), max_size(sa.len(), MAX_LENGTH)?)?;

            let code = libsais_int_omp(t_ptr, sa_ptr, n, k, fs, threads);
            interpret_return_code_32(code).map(|_| ())
        }
    }

    pub fn bwt(t: &[u8], u: &mut [u8], a: &mut [i32], freq: Option<&mut [i32]>, threads: i32) -> Result<i32> {
        unsafe {
            let t_ptr = t.as_ptr();
            let u_ptr = u.as_mut_ptr();
            let a_ptr = a.as_mut_ptr();
            let (n, fs) = split_size(same_size(t.len(), u.len())?, max_size(a.len(), MAX_LENGTH)?)?;
            let freq_ptr = freq_as_mut_ptr(freq, FREQ_TABLE_SIZE)?;

            let code = libsais_bwt_omp(t_ptr, u_ptr, a_ptr, n, fs, freq_ptr, threads);
            interpret_return_code_32(code)
        }
    }

    pub fn bwt_inplace(t: &mut [u8], a: &mut [i32], freq: Option<&mut [i32]>, threads: i32) -> Result<i32> {
        unsafe {
            let t_ptr = t.as_ptr();
            let u_ptr = t.as_mut_ptr();
            let a_ptr = a.as_mut_ptr();
            let (n, fs) = split_size(t.len(), max_size(a.len(), MAX_LENGTH)?)?;
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
            let (n, fs) = split_size(same_size(t.len(), u.len())?, max_size(a.len(), MAX_LENGTH)?)?;
            let freq_ptr = freq_as_mut_ptr(freq, FREQ_TABLE_SIZE)?;
            let r = aux_rate(i.len(), t.len())?;
            let i_ptr = i.as_mut_ptr();

            let code = libsais_bwt_aux_omp(t_ptr, u_ptr, a_ptr, n, fs, freq_ptr, r, i_ptr, threads);
            interpret_return_code_32(code).map(|_| ())
        }
    }

    pub fn bwt_aux_inplace(t: &mut [u8], a: &mut [i32], freq: Option<&mut [i32]>, i: &mut [i32], threads: i32) -> Result<()> {
        unsafe {
            let t_ptr = t.as_ptr();
            let u_ptr = t.as_mut_ptr();
            let a_ptr = a.as_mut_ptr();
            let (n, fs) = split_size(t.len(), max_size(a.len(), MAX_LENGTH)?)?;
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
            let n = unbwt_sufficient_size(same_size(t.len(), u.len())?, max_size(a.len(), MAX_LENGTH)?)?;
            let freq_ptr = freq_as_ptr(freq, FREQ_TABLE_SIZE)?;

            let code = libsais_unbwt_omp(t_ptr, u_ptr, a_ptr, n, freq_ptr, i, threads);
            interpret_return_code_32(code).map(|_| ())
        }
    }

    pub fn unbwt_inplace(t: &mut [u8], a: &mut [i32], freq: Option<&[i32]>, i: i32, threads: i32) -> Result<()> {
        unsafe {
            let t_ptr = t.as_ptr();
            let u_ptr = t.as_mut_ptr();
            let a_ptr = a.as_mut_ptr();
            let n = unbwt_sufficient_size(t.len(), max_size(a.len(), MAX_LENGTH)?)?;
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
            let n = unbwt_sufficient_size(same_size(t.len(), u.len())?, max_size(a.len(), MAX_LENGTH)?)?;
            let freq_ptr = freq_as_ptr(freq, FREQ_TABLE_SIZE)?;
            let r = aux_rate(i.len(), t.len())?;
            let i_ptr = i.as_ptr();

            let code = libsais_unbwt_aux_omp(t_ptr, u_ptr, a_ptr, n, freq_ptr, r, i_ptr, threads);
            interpret_return_code_32(code).map(|_| ())
        }
    }

    pub fn unbwt_aux_inplace(t: &mut [u8], a: &mut [i32], freq: Option<&[i32]>, i: &[i32], threads: i32) -> Result<()> {
        unsafe {
            let t_ptr = t.as_ptr();
            let u_ptr = t.as_mut_ptr();
            let a_ptr = a.as_mut_ptr();
            let n = unbwt_sufficient_size(t.len(), max_size(a.len(), MAX_LENGTH)?)?;
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
            let (n, _) = split_size(same_size(t.len(), plcp.len())?, max_size(sa.len(), MAX_LENGTH)?)?;

            let code = libsais_plcp_omp(t_ptr, sa_ptr, plcp_ptr, n, threads);
            interpret_return_code_32(code).map(|_| ())
        }
    }

    pub fn lcp(plcp: &[i32], sa: &[i32], lcp: &mut [i32], threads: i32) -> Result<()> {
        unsafe {
            let plcp_ptr = plcp.as_ptr();
            let sa_ptr = sa.as_ptr();
            let lcp_ptr = lcp.as_mut_ptr();
            let (n, _) = split_size(same_size(plcp.len(), lcp.len())?, max_size(sa.len(), MAX_LENGTH)?)?;

            let code = libsais_lcp_omp(plcp_ptr, sa_ptr, lcp_ptr, n, threads);
            interpret_return_code_32(code).map(|_| ())
        }
    }
}
