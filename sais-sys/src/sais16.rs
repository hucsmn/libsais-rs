use std::ptr::{NonNull, null_mut};

use libc::c_void;

extern "C" {
    /// void * libsais16_create_ctx(void);
    fn libsais16_create_ctx() -> *mut c_void;

    /// void libsais16_free_ctx(void * ctx);
    fn libsais16_free_ctx(ctx: *mut c_void);

    /// int32_t libsais16(const uint16_t * T, int32_t * SA, int32_t n, int32_t fs, int32_t * freq);
    fn libsais16(t: *const u16, sa: *mut i32, n: i32, fs: i32, freq: *const i32) -> i32;

    /// int32_t libsais16_ctx(const void * ctx, const uint16_t * T, int32_t * SA, int32_t n, int32_t fs, int32_t * freq);
    fn libsais16_ctx(ctx: *mut c_void, t: *const u16, sa: *mut i32, n: i32, fs: i32, freq: *const i32) -> i32;

    /// int32_t libsais16_bwt(const uint16_t * T, uint16_t * U, int32_t * A, int32_t n, int32_t fs, int32_t * freq);
    fn libsais16_bwt(t: *const u16, u: *mut u16, a: *mut i32, n: i32, fs: i32, freq: *const i32) -> i32;

    /// int32_t libsais16_bwt_aux(const uint16_t * T, uint16_t * U, int32_t * A, int32_t n, int32_t fs, int32_t * freq, int32_t r, int32_t * I);
    fn libsais16_bwt_aux(t: *const u16, u: *mut u16, a: *mut i32, n: i32, fs: i32, freq: *const i32, r: i32, i: *mut i32) -> i32;

    /// int32_t libsais16_bwt_ctx(const void * ctx, const uint16_t * T, uint16_t * U, int32_t * A, int32_t n, int32_t fs, int32_t * freq);
    fn libsais16_bwt_ctx(ctx: *mut c_void, t: *const u16, u: *mut u16, a: *mut i32, n: i32, fs: i32, freq: *const i32) -> i32;

    /// int32_t libsais16_bwt_aux_ctx(const void * ctx, const uint16_t * T, uint16_t * U, int32_t * A, int32_t n, int32_t fs, int32_t * freq, int32_t r, int32_t * I);
    fn libsais16_bwt_aux_ctx(ctx: *mut c_void, t: *const u16, u: *mut u16, a: *mut i32, n: i32, fs: i32, freq: *const i32, r: i32, i: *mut i32) -> i32;

    /// void * libsais16_unbwt_create_ctx(void);
    fn libsais16_unbwt_create_ctx() -> *mut c_void;

    /// void libsais16_unbwt_free_ctx(void * ctx);
    fn libsais16_unbwt_free_ctx(ctx: *mut c_void);

    /// int32_t libsais16_unbwt(const uint16_t * T, uint16_t * U, int32_t * A, int32_t n, const int32_t * freq, int32_t i);
    fn libsais16_unbwt(t: *const u16, u: *mut u16, a: *mut i32, n: i32, freq: *const i32, i: i32) -> i32;

    /// int32_t libsais16_unbwt_ctx(const void * ctx, const uint16_t * T, uint16_t * U, int32_t * A, int32_t n, const int32_t * freq, int32_t i);
    fn libsais16_unbwt_ctx(ctx: *mut c_void, t: *const u16, u: *mut u16, a: *mut i32, n: i32, freq: *const i32, i: i32) -> i32;

    /// int32_t libsais16_unbwt_aux(const uint16_t * T, uint16_t * U, int32_t * A, int32_t n, const int32_t * freq, int32_t r, const int32_t * I);
    fn libsais16_unbwt_aux(t: *const u16, u: *mut u16, a: *mut i32, n: i32, freq: *const i32, r: i32, i: *mut i32) -> i32;

    /// int32_t libsais16_unbwt_aux_ctx(const void * ctx, const uint16_t * T, uint16_t * U, int32_t * A, int32_t n, const int32_t * freq, int32_t r, const int32_t * I);
    fn libsais16_unbwt_aux_ctx(ctx: *mut c_void, t: *const u16, u: *mut u16, a: *mut i32, n: i32, freq: *const i32, r: i32, i: *mut i32) -> i32;

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
    fn libsais16_omp(t: *const u16, sa: *mut i32, n: i32, fs: i32, freq: *const i32, threads: i32) -> i32;

    /// int32_t libsais16_bwt_omp(const uint16_t * T, uint16_t * U, int32_t * A, int32_t n, int32_t fs, int32_t * freq, int32_t threads);
    fn libsais16_bwt_omp(t: *const u16, u: *mut u16, a: *mut i32, n: i32, fs: i32, freq: *const i32, threads: i32) -> i32;

    /// int32_t libsais16_bwt_aux_omp(const uint16_t * T, uint16_t * U, int32_t * A, int32_t n, int32_t fs, int32_t * freq, int32_t r, int32_t * I, int32_t threads);
    fn libsais16_bwt_aux_omp(t: *const u16, u: *mut u16, a: *mut i32, n: i32, fs: i32, freq: *const i32, r: i32, i: *mut i32, threads: i32) -> i32;

    /// void * libsais16_unbwt_create_ctx_omp(int32_t threads);
    fn libsais16_unbwt_create_ctx_omp(threads: i32) -> *mut c_void;

    /// int32_t libsais16_unbwt_omp(const uint16_t * T, uint16_t * U, int32_t * A, int32_t n, const int32_t * freq, int32_t i, int32_t threads);
    fn libsais16_unbwt_omp(t: *const u16, u: *mut u16, a: *mut i32, n: i32, freq: *const i32, i: i32, threads: i32) -> i32;

    /// int32_t libsais16_unbwt_aux_omp(const uint16_t * T, uint16_t * U, int32_t * A, int32_t n, const int32_t * freq, int32_t r, const int32_t * I, int32_t threads);
    fn libsais16_unbwt_aux_omp(t: *const u16, u: *mut u16, a: *mut i32, n: i32, freq: *const i32, r: i32, i: *mut i32, threads: i32) -> i32;

    /// int32_t libsais16_plcp_omp(const uint16_t * T, const int32_t * SA, int32_t * PLCP, int32_t n, int32_t threads);
    fn libsais16_plcp_omp(t: *const u16, sa: *const i32, plcp: *mut i32, n: i32, threads: i32) -> i32;

    /// int32_t libsais16_lcp_omp(const int32_t * PLCP, const int32_t * SA, int32_t * LCP, int32_t n, int32_t threads);
    fn libsais16_lcp_omp(plcp: *const i32, sa: *const i32, lcp: *mut i32, n: i32, threads: i32) -> i32;
}

pub type Error = crate::errors::Error<i32>;

pub type Result<T> = std::result::Result<T, Error>;

pub type FreqTable = [i32; 65536];

pub struct SaisContext(NonNull<c_void>);

impl SaisContext {
    pub fn new() -> Option<Self> {
        unsafe {
            let ctx_ptr = libsais16_create_ctx();
            NonNull::new(ctx_ptr).map(SaisContext)
        }
    }

    #[cfg(feature = "openmp")]
    pub fn new_parallel(threads: i32) -> Option<Self> {
        unsafe {
            let ctx_ptr = libsais16_create_ctx_omp(threads);
            NonNull::new(ctx_ptr).map(SaisContext)
        }
    }

    pub fn sais(&mut self, t: &[u16], sa: &mut [i32], freq: Option<&mut FreqTable>) -> Result<()> {
        unsafe {
            let t_ptr = t.as_ptr();
            let sa_ptr = sa.as_mut_ptr();
            let (n, fs) = length_and_freespace(t.len(), sa.len())?;
            let freq_ptr = freq.map(|ptr| ptr.as_mut_ptr()).unwrap_or_else(null_mut);

            let code = libsais16_ctx(self.0.as_mut(), t_ptr, sa_ptr, n, fs, freq_ptr);
            interpret_code(code).map(|_| ())
        }
    }

    pub fn bwt(&mut self, t: &[u16], u: &mut [u16], a: &mut [i32], freq: Option<&mut FreqTable>) -> Result<i32> {
        unsafe {
            let t_ptr = t.as_ptr();
            let u_ptr = u.as_mut_ptr();
            let a_ptr = a.as_mut_ptr();
            let (n, fs) = length_and_freespace(same_value(t.len(), u.len())?, a.len())?;
            let freq_ptr = freq.map(|ptr| ptr.as_mut_ptr()).unwrap_or_else(null_mut);

            let code = libsais16_bwt_ctx(self.0.as_mut(), t_ptr, u_ptr, a_ptr, n, fs, freq_ptr);
            interpret_code(code)
        }
    }

    pub fn bwt_aux(&mut self, t: &[u16], u: &mut [u16], a: &mut [i32], freq: Option<&mut FreqTable>, i: &mut [i32]) -> Result<i32> {
        unsafe {
            let t_ptr = t.as_ptr();
            let u_ptr = u.as_mut_ptr();
            let a_ptr = a.as_mut_ptr();
            let (n, fs) = length_and_freespace(same_value(t.len(), u.len())?, a.len())?;
            let freq_ptr = freq.map(|ptr| ptr.as_mut_ptr()).unwrap_or_else(null_mut);
            let r = aux_rate(i.len(), t.len())?;
            let i_ptr = i.as_mut_ptr();

            let code = libsais16_bwt_aux_ctx(self.0.as_mut(), t_ptr, u_ptr, a_ptr, n, fs, freq_ptr, r, i_ptr);
            interpret_code(code)
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

pub struct UnbwtContext(NonNull<c_void>);

impl UnbwtContext {
    pub fn new() -> Option<Self> {
        unsafe {
            let ctx_ptr = libsais16_unbwt_create_ctx();
            NonNull::new(ctx_ptr).map(UnbwtContext)
        }
    }

    #[cfg(feature = "openmp")]
    pub fn new_parallel(threads: i32) -> Option<Self> {
        unsafe {
            let ctx_ptr = libsais16_unbwt_create_ctx_omp(threads);
            NonNull::new(ctx_ptr).map(UnbwtContext)
        }
    }

    pub fn unbwt(&mut self, t: &[u16], u: &mut [u16], a: &mut [i32], freq: Option<&mut FreqTable>, i: i32) -> Result<()> {
        unsafe {
            let t_ptr = t.as_ptr();
            let u_ptr = u.as_mut_ptr();
            let a_ptr = a.as_mut_ptr();
            let (n, _) = length_and_freespace(same_value(t.len(), u.len())?, a.len())?;
            let freq_ptr = freq.map(|ptr| ptr.as_mut_ptr()).unwrap_or_else(null_mut);

            let code = libsais16_unbwt_ctx(self.0.as_mut(), t_ptr, u_ptr, a_ptr, n, freq_ptr, i);
            interpret_code(code).map(|_| ())
        }
    }

    pub fn unbwt_aux(&mut self, t: &[u16], u: &mut [u16], a: &mut [i32], freq: Option<&mut FreqTable>, i: &mut [i32]) -> Result<()> {
        unsafe {
            let t_ptr = t.as_ptr();
            let u_ptr = u.as_mut_ptr();
            let a_ptr = a.as_mut_ptr();
            let (n, _) = length_and_freespace(same_value(t.len(), u.len())?, a.len())?;
            let freq_ptr = freq.map(|ptr| ptr.as_mut_ptr()).unwrap_or_else(null_mut);
            let r = aux_rate(i.len(), t.len())?;
            let i_ptr = i.as_mut_ptr();

            let code = libsais16_unbwt_aux_ctx(self.0.as_mut(), t_ptr, u_ptr, a_ptr, n, freq_ptr, r, i_ptr);
            interpret_code(code).map(|_| ())
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

pub fn sais(t: &[u16], sa: &mut [i32], freq: Option<&mut FreqTable>) -> Result<()> {
    unsafe {
        let t_ptr = t.as_ptr();
        let sa_ptr = sa.as_mut_ptr();
        let (n, fs) = length_and_freespace(t.len(), sa.len())?;
        let freq_ptr = freq.map(|ptr| ptr.as_mut_ptr()).unwrap_or_else(null_mut);

        let code = libsais16(t_ptr, sa_ptr, n, fs, freq_ptr);
        interpret_code(code).map(|_| ())
    }
}

pub fn bwt(t: &[u16], u: &mut [u16], a: &mut [i32], freq: Option<&mut FreqTable>) -> Result<i32> {
    unsafe {
        let t_ptr = t.as_ptr();
        let u_ptr = u.as_mut_ptr();
        let a_ptr = a.as_mut_ptr();
        let (n, fs) = length_and_freespace(same_value(t.len(), u.len())?, a.len())?;
        let freq_ptr = freq.map(|ptr| ptr.as_mut_ptr()).unwrap_or_else(null_mut);

        let code = libsais16_bwt(t_ptr, u_ptr, a_ptr, n, fs, freq_ptr);
        interpret_code(code)
    }
}

pub fn bwt_aux(t: &[u16], u: &mut [u16], a: &mut [i32], freq: Option<&mut FreqTable>, i: &mut [i32]) -> Result<i32> {
    unsafe {
        let t_ptr = t.as_ptr();
        let u_ptr = u.as_mut_ptr();
        let a_ptr = a.as_mut_ptr();
        let (n, fs) = length_and_freespace(same_value(t.len(), u.len())?, a.len())?;
        let freq_ptr = freq.map(|ptr| ptr.as_mut_ptr()).unwrap_or_else(null_mut);
        let r = aux_rate(i.len(), t.len())?;
        let i_ptr = i.as_mut_ptr();

        let code = libsais16_bwt_aux(t_ptr, u_ptr, a_ptr, n, fs, freq_ptr, r, i_ptr);
        interpret_code(code)
    }
}

pub fn unbwt(t: &[u16], u: &mut [u16], a: &mut [i32], freq: Option<&mut FreqTable>, i: i32) -> Result<()> {
    unsafe {
        let t_ptr = t.as_ptr();
        let u_ptr = u.as_mut_ptr();
        let a_ptr = a.as_mut_ptr();
        let (n, _) = length_and_freespace(same_value(t.len(), u.len())?, a.len())?;
        let freq_ptr = freq.map(|ptr| ptr.as_mut_ptr()).unwrap_or_else(null_mut);

        let code = libsais16_unbwt(t_ptr, u_ptr, a_ptr, n, freq_ptr, i);
        interpret_code(code).map(|_| ())
    }
}

pub fn unbwt_aux(t: &[u16], u: &mut [u16], a: &mut [i32], freq: Option<&mut FreqTable>, i: &mut [i32]) -> Result<()> {
    unsafe {
        let t_ptr = t.as_ptr();
        let u_ptr = u.as_mut_ptr();
        let a_ptr = a.as_mut_ptr();
        let (n, _) = length_and_freespace(same_value(t.len(), u.len())?, a.len())?;
        let freq_ptr = freq.map(|ptr| ptr.as_mut_ptr()).unwrap_or_else(null_mut);
        let r = aux_rate(i.len(), t.len())?;
        let i_ptr = i.as_mut_ptr();

        let code = libsais16_unbwt_aux(t_ptr, u_ptr, a_ptr, n, freq_ptr, r, i_ptr);
        interpret_code(code).map(|_| ())
    }
}

pub fn plcp(t: &[u16], sa: &[i32], plcp: &mut [i32]) -> Result<()> {
    unsafe {
        let t_ptr = t.as_ptr();
        let sa_ptr = sa.as_ptr();
        let plcp_ptr = plcp.as_mut_ptr();
        let (n, _) = length_and_freespace(same_value(t.len(), plcp.len())?, sa.len())?;

        let code = libsais16_plcp(t_ptr, sa_ptr, plcp_ptr, n);
        interpret_code(code).map(|_| ())
    }
}

pub fn lcp(plcp: &[i32], sa: &[i32], lcp: &mut [i32]) -> Result<()> {
    unsafe {
        let plcp_ptr = plcp.as_ptr();
        let sa_ptr = sa.as_ptr();
        let lcp_ptr = lcp.as_mut_ptr();
        let (n, _) = length_and_freespace(same_value(plcp.len(), lcp.len())?, sa.len())?;

        let code = libsais16_lcp(plcp_ptr, sa_ptr, lcp_ptr, n);
        interpret_code(code).map(|_| ())
    }
}

#[cfg(feature = "openmp")]
pub mod openmp {
    use super::*;

    pub fn sais(t: &[u16], sa: &mut [i32], freq: Option<&mut FreqTable>, threads: i32) -> Result<()> {
        unsafe {
            let t_ptr = t.as_ptr();
            let sa_ptr = sa.as_mut_ptr();
            let (n, fs) = length_and_freespace(t.len(), sa.len())?;
            let freq_ptr = freq.map(|ptr| ptr.as_mut_ptr()).unwrap_or_else(null_mut);

            let code = libsais16_omp(t_ptr, sa_ptr, n, fs, freq_ptr, threads);
            interpret_code(code).map(|_| ())
        }
    }

    pub fn bwt(t: &[u16], u: &mut [u16], a: &mut [i32], freq: Option<&mut FreqTable>, threads: i32) -> Result<i32> {
        unsafe {
            let t_ptr = t.as_ptr();
            let u_ptr = u.as_mut_ptr();
            let a_ptr = a.as_mut_ptr();
            let (n, fs) = length_and_freespace(same_value(t.len(), u.len())?, a.len())?;
            let freq_ptr = freq.map(|ptr| ptr.as_mut_ptr()).unwrap_or_else(null_mut);

            let code = libsais16_bwt_omp(t_ptr, u_ptr, a_ptr, n, fs, freq_ptr, threads);
            interpret_code(code)
        }
    }

    pub fn bwt_aux(t: &[u16], u: &mut [u16], a: &mut [i32], freq: Option<&mut FreqTable>, i: &mut [i32], threads: i32) -> Result<i32> {
        unsafe {
            let t_ptr = t.as_ptr();
            let u_ptr = u.as_mut_ptr();
            let a_ptr = a.as_mut_ptr();
            let (n, fs) = length_and_freespace(same_value(t.len(), u.len())?, a.len())?;
            let freq_ptr = freq.map(|ptr| ptr.as_mut_ptr()).unwrap_or_else(null_mut);
            let r = aux_rate(i.len(), t.len())?;
            let i_ptr = i.as_mut_ptr();

            let code = libsais16_bwt_aux_omp(t_ptr, u_ptr, a_ptr, n, fs, freq_ptr, r, i_ptr, threads);
            interpret_code(code)
        }
    }

    pub fn unbwt(t: &[u16], u: &mut [u16], a: &mut [i32], freq: Option<&mut FreqTable>, i: i32, threads: i32) -> Result<()> {
        unsafe {
            let t_ptr = t.as_ptr();
            let u_ptr = u.as_mut_ptr();
            let a_ptr = a.as_mut_ptr();
            let (n, _) = length_and_freespace(same_value(t.len(), u.len())?, a.len())?;
            let freq_ptr = freq.map(|ptr| ptr.as_mut_ptr()).unwrap_or_else(null_mut);

            let code = libsais16_unbwt_omp(t_ptr, u_ptr, a_ptr, n, freq_ptr, i, threads);
            interpret_code(code).map(|_| ())
        }
    }

    pub fn unbwt_aux(t: &[u16], u: &mut [u16], a: &mut [i32], freq: Option<&mut FreqTable>, i: &mut [i32], threads: i32) -> Result<()> {
        unsafe {
            let t_ptr = t.as_ptr();
            let u_ptr = u.as_mut_ptr();
            let a_ptr = a.as_mut_ptr();
            let (n, _) = length_and_freespace(same_value(t.len(), u.len())?, a.len())?;
            let freq_ptr = freq.map(|ptr| ptr.as_mut_ptr()).unwrap_or_else(null_mut);
            let r = aux_rate(i.len(), t.len())?;
            let i_ptr = i.as_mut_ptr();

            let code = libsais16_unbwt_aux_omp(t_ptr, u_ptr, a_ptr, n, freq_ptr, r, i_ptr, threads);
            interpret_code(code).map(|_| ())
        }
    }

    pub fn plcp(t: &[u16], sa: &[i32], plcp: &mut [i32], threads: i32) -> Result<()> {
        unsafe {
            let t_ptr = t.as_ptr();
            let sa_ptr = sa.as_ptr();
            let plcp_ptr = plcp.as_mut_ptr();
            let (n, _) = length_and_freespace(same_value(t.len(), plcp.len())?, sa.len())?;

            let code = libsais16_plcp_omp(t_ptr, sa_ptr, plcp_ptr, n, threads);
            interpret_code(code).map(|_| ())
        }
    }

    pub fn lcp(plcp: &[i32], sa: &[i32], lcp: &mut [i32], threads: i32) -> Result<()> {
        unsafe {
            let plcp_ptr = plcp.as_ptr();
            let sa_ptr = sa.as_ptr();
            let lcp_ptr = lcp.as_mut_ptr();
            let (n, _) = length_and_freespace(same_value(plcp.len(), lcp.len())?, sa.len())?;

            let code = libsais16_lcp_omp(plcp_ptr, sa_ptr, lcp_ptr, n, threads);
            interpret_code(code).map(|_| ())
        }
    }
}

fn same_value<T: Copy + Eq>(a: T, b: T) -> Result<T> {
    if a != b {
        Err(Error::IllegalArguments)
    } else {
        Ok(a)
    }
}

fn length_and_freespace(n: usize, m: usize) -> Result<(i32, i32)> {
    let p: i32 = n.try_into().map_err(|_| Error::IllegalArguments)?;
    let q: i32 = m.try_into().map_err(|_| Error::IllegalArguments)?;
    let fs = if q >= p { q - p } else { Err(Error::IllegalArguments)? };
    Ok((p, fs))
}

fn aux_rate(cap: usize, n: usize) -> Result<i32> {
    if cap == 0 || n == 0 {
        Err(Error::IllegalArguments)
    } else {
        let mut r = n / cap;
        if n % cap != 0 {
            r = n / cap + 1;
        }
        if (r & (r - 1)) != 0 {
            r = r.next_power_of_two();
        }
        if r != 0 {
            r.try_into().map_err(|_| Error::IllegalArguments)
        } else {
            Err(Error::IllegalArguments)
        }
    }
}

fn interpret_code(code: i32) -> Result<i32> {
    match code {
        n if n >= 0 => Ok(n),
        -1 => Err(Error::IllegalArguments),
        -2 => Err(Error::InternalError),
        err => Err(Error::Uncategorized(err)),
    }
}
