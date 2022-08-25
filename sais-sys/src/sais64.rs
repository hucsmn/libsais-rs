//! 64-bit sais algorithms on u8 array inputs.

use std::ptr::null_mut;

extern "C" {
    /// int64_t libsais64(const uint8_t * T, int64_t * SA, int64_t n, int64_t fs, int64_t * freq);
    fn libsais64(t: *const u8, sa: *mut i64, n: i64, fs: i64, freq: *mut i64) -> i64;

    /// int64_t libsais64_bwt(const uint8_t * T, uint8_t * U, int64_t * A, int64_t n, int64_t fs, int64_t * freq);
    fn libsais64_bwt(t: *const u8, u: *mut u8, a: *mut i64, n: i64, fs: i64, freq: *mut i64) -> i64;

    /// int64_t libsais64_bwt_aux(const uint8_t * T, uint8_t * U, int64_t * A, int64_t n, int64_t fs, int64_t * freq, int64_t r, int64_t * I);
    fn libsais64_bwt_aux(t: *const u8, u: *mut u8, a: *mut i64, n: i64, fs: i64, freq: *mut i64, r: i64, i: *mut i64) -> i64;

    /// int64_t libsais64_unbwt(const uint8_t * T, uint8_t * U, int64_t * A, int64_t n, const int64_t * freq, int64_t i);
    fn libsais64_unbwt(t: *const u8, u: *mut u8, a: *mut i64, n: i64, freq: *mut i64, i: i64) -> i64;

    /// int64_t libsais64_unbwt_aux(const uint8_t * T, uint8_t * U, int64_t * A, int64_t n, const int64_t * freq, int64_t r, const int64_t * I);
    fn libsais64_unbwt_aux(t: *const u8, u: *mut u8, a: *mut i64, n: i64, freq: *mut i64, r: i64, i: *mut i64) -> i64;

    /// int64_t libsais64_plcp(const uint8_t * T, const int64_t * SA, int64_t * PLCP, int64_t n);
    fn libsais64_plcp(t: *const u8, sa: *const i64, plcp: *mut i64, n: i64) -> i64;

    /// int64_t libsais64_lcp(const int64_t * PLCP, const int64_t * SA, int64_t * LCP, int64_t n);
    fn libsais64_lcp(plcp: *const i64, sa: *const i64, lcp: *mut i64, n: i64) -> i64;
}

#[cfg(feature = "openmp")]
extern "C" {
    /// int64_t libsais64_omp(const uint8_t * T, int64_t * SA, int64_t n, int64_t fs, int64_t * freq, int64_t threads);
    fn libsais64_omp(t: *const u8, sa: *mut i64, n: i64, fs: i64, freq: *mut i64, threads: i64) -> i64;

    /// int64_t libsais64_bwt_omp(const uint8_t * T, uint8_t * U, int64_t * A, int64_t n, int64_t fs, int64_t * freq, int64_t threads);
    fn libsais64_bwt_omp(t: *const u8, u: *mut u8, a: *mut i64, n: i64, fs: i64, freq: *mut i64, threads: i64) -> i64;

    /// int64_t libsais64_bwt_aux_omp(const uint8_t * T, uint8_t * U, int64_t * A, int64_t n, int64_t fs, int64_t * freq, int64_t r, int64_t * I, int64_t threads);
    fn libsais64_bwt_aux_omp(t: *const u8, u: *mut u8, a: *mut i64, n: i64, fs: i64, freq: *mut i64, r: i64, i: *mut i64, threads: i64) -> i64;

    /// int64_t libsais64_unbwt_omp(const uint8_t * T, uint8_t * U, int64_t * A, int64_t n, const int64_t * freq, int64_t i, int64_t threads);
    fn libsais64_unbwt_omp(t: *const u8, u: *mut u8, a: *mut i64, n: i64, freq: *mut i64, i: i64, threads: i64) -> i64;

    /// int64_t libsais64_unbwt_aux_omp(const uint8_t * T, uint8_t * U, int64_t * A, int64_t n, const int64_t * freq, int64_t r, const int64_t * I, int64_t threads);
    fn libsais64_unbwt_aux_omp(t: *const u8, u: *mut u8, a: *mut i64, n: i64, freq: *mut i64, r: i64, i: *mut i64, threads: i64) -> i64;

    /// int64_t libsais64_plcp_omp(const uint8_t * T, const int64_t * SA, int64_t * PLCP, int64_t n, int64_t threads);
    fn libsais64_plcp_omp(t: *const u8, sa: *const i64, plcp: *mut i64, n: i64, threads: i64) -> i64;

    /// int64_t libsais64_lcp_omp(const int64_t * PLCP, const int64_t * SA, int64_t * LCP, int64_t n, int64_t threads);
    fn libsais64_lcp_omp(plcp: *const i64, sa: *const i64, lcp: *mut i64, n: i64, threads: i64) -> i64;
}

/// Interpreted error code for 64-bit sais algorithms.
pub type Error = crate::errors::Error<i64>;

/// Interpreted return value for 64-bit sais algorithms.
pub type Result<T> = std::result::Result<T, Error>;

/// Output symbol frequency table for u8 strings.
pub type FreqTable = [i64; 256];

pub fn sais(t: &[u8], sa: &mut [i64], freq: Option<&mut FreqTable>) -> Result<()> {
    unsafe {
        let t_ptr = t.as_ptr();
        let sa_ptr = sa.as_mut_ptr();
        let (n, fs) = length_and_freespace(t.len(), sa.len())?;
        let freq_ptr = freq.map(|ptr| ptr.as_mut_ptr()).unwrap_or_else(null_mut);

        let code = libsais64(t_ptr, sa_ptr, n, fs, freq_ptr);
        interpret_code(code).map(|_| ())
    }
}

pub fn bwt(t: &[u8], u: &mut [u8], a: &mut [i64], freq: Option<&mut FreqTable>) -> Result<i64> {
    unsafe {
        let t_ptr = t.as_ptr();
        let u_ptr = u.as_mut_ptr();
        let a_ptr = a.as_mut_ptr();
        let (n, fs) = length_and_freespace(same_value(t.len(), u.len())?, a.len())?;
        let freq_ptr = freq.map(|ptr| ptr.as_mut_ptr()).unwrap_or_else(null_mut);

        let code = libsais64_bwt(t_ptr, u_ptr, a_ptr, n, fs, freq_ptr);
        interpret_code(code)
    }
}

pub fn bwt_aux(t: &[u8], u: &mut [u8], a: &mut [i64], freq: Option<&mut FreqTable>, i: &mut [i64]) -> Result<i64> {
    unsafe {
        let t_ptr = t.as_ptr();
        let u_ptr = u.as_mut_ptr();
        let a_ptr = a.as_mut_ptr();
        let (n, fs) = length_and_freespace(same_value(t.len(), u.len())?, a.len())?;
        let freq_ptr = freq.map(|ptr| ptr.as_mut_ptr()).unwrap_or_else(null_mut);
        let r = aux_rate(i.len(), t.len())?;
        let i_ptr = i.as_mut_ptr();

        let code = libsais64_bwt_aux(t_ptr, u_ptr, a_ptr, n, fs, freq_ptr, r, i_ptr);
        interpret_code(code)
    }
}

pub fn unbwt(t: &[u8], u: &mut [u8], a: &mut [i64], freq: Option<&mut FreqTable>, i: i64) -> Result<()> {
    unsafe {
        let t_ptr = t.as_ptr();
        let u_ptr = u.as_mut_ptr();
        let a_ptr = a.as_mut_ptr();
        let (n, _) = length_and_freespace(same_value(t.len(), u.len())?, a.len())?;
        let freq_ptr = freq.map(|ptr| ptr.as_mut_ptr()).unwrap_or_else(null_mut);

        let code = libsais64_unbwt(t_ptr, u_ptr, a_ptr, n, freq_ptr, i);
        interpret_code(code).map(|_| ())
    }
}

pub fn unbwt_aux(t: &[u8], u: &mut [u8], a: &mut [i64], freq: Option<&mut FreqTable>, i: &mut [i64]) -> Result<()> {
    unsafe {
        let t_ptr = t.as_ptr();
        let u_ptr = u.as_mut_ptr();
        let a_ptr = a.as_mut_ptr();
        let (n, _) = length_and_freespace(same_value(t.len(), u.len())?, a.len())?;
        let freq_ptr = freq.map(|ptr| ptr.as_mut_ptr()).unwrap_or_else(null_mut);
        let r = aux_rate(i.len(), t.len())?;
        let i_ptr = i.as_mut_ptr();

        let code = libsais64_unbwt_aux(t_ptr, u_ptr, a_ptr, n, freq_ptr, r, i_ptr);
        interpret_code(code).map(|_| ())
    }
}

pub fn plcp(t: &[u8], sa: &[i64], plcp: &mut [i64]) -> Result<()> {
    unsafe {
        let t_ptr = t.as_ptr();
        let sa_ptr = sa.as_ptr();
        let plcp_ptr = plcp.as_mut_ptr();
        let (n, _) = length_and_freespace(same_value(t.len(), plcp.len())?, sa.len())?;

        let code = libsais64_plcp(t_ptr, sa_ptr, plcp_ptr, n);
        interpret_code(code).map(|_| ())
    }
}

pub fn lcp(plcp: &[i64], sa: &[i64], lcp: &mut [i64]) -> Result<()> {
    unsafe {
        let plcp_ptr = plcp.as_ptr();
        let sa_ptr = sa.as_ptr();
        let lcp_ptr = lcp.as_mut_ptr();
        let (n, _) = length_and_freespace(same_value(plcp.len(), lcp.len())?, sa.len())?;

        let code = libsais64_lcp(plcp_ptr, sa_ptr, lcp_ptr, n);
        interpret_code(code).map(|_| ())
    }
}

#[cfg(feature = "openmp")]
pub mod openmp {
    //! Multi-threaded 64-bit sais algorithms on u8 array inputs.

    use super::*;

    pub fn sais(t: &[u8], sa: &mut [i64], freq: Option<&mut FreqTable>, threads: i64) -> Result<()> {
        unsafe {
            let t_ptr = t.as_ptr();
            let sa_ptr = sa.as_mut_ptr();
            let (n, fs) = length_and_freespace(t.len(), sa.len())?;
            let freq_ptr = freq.map(|ptr| ptr.as_mut_ptr()).unwrap_or_else(null_mut);

            let code = libsais64_omp(t_ptr, sa_ptr, n, fs, freq_ptr, threads);
            interpret_code(code).map(|_| ())
        }
    }

    pub fn bwt(t: &[u8], u: &mut [u8], a: &mut [i64], freq: Option<&mut FreqTable>, threads: i64) -> Result<i64> {
        unsafe {
            let t_ptr = t.as_ptr();
            let u_ptr = u.as_mut_ptr();
            let a_ptr = a.as_mut_ptr();
            let (n, fs) = length_and_freespace(same_value(t.len(), u.len())?, a.len())?;
            let freq_ptr = freq.map(|ptr| ptr.as_mut_ptr()).unwrap_or_else(null_mut);

            let code = libsais64_bwt_omp(t_ptr, u_ptr, a_ptr, n, fs, freq_ptr, threads);
            interpret_code(code)
        }
    }

    pub fn bwt_aux(t: &[u8], u: &mut [u8], a: &mut [i64], freq: Option<&mut FreqTable>, i: &mut [i64], threads: i64) -> Result<i64> {
        unsafe {
            let t_ptr = t.as_ptr();
            let u_ptr = u.as_mut_ptr();
            let a_ptr = a.as_mut_ptr();
            let (n, fs) = length_and_freespace(same_value(t.len(), u.len())?, a.len())?;
            let freq_ptr = freq.map(|ptr| ptr.as_mut_ptr()).unwrap_or_else(null_mut);
            let r = aux_rate(i.len(), t.len())?;
            let i_ptr = i.as_mut_ptr();

            let code = libsais64_bwt_aux_omp(t_ptr, u_ptr, a_ptr, n, fs, freq_ptr, r, i_ptr, threads);
            interpret_code(code)
        }
    }

    pub fn unbwt(t: &[u8], u: &mut [u8], a: &mut [i64], freq: Option<&mut FreqTable>, i: i64, threads: i64) -> Result<()> {
        unsafe {
            let t_ptr = t.as_ptr();
            let u_ptr = u.as_mut_ptr();
            let a_ptr = a.as_mut_ptr();
            let (n, _) = length_and_freespace(same_value(t.len(), u.len())?, a.len())?;
            let freq_ptr = freq.map(|ptr| ptr.as_mut_ptr()).unwrap_or_else(null_mut);

            let code = libsais64_unbwt_omp(t_ptr, u_ptr, a_ptr, n, freq_ptr, i, threads);
            interpret_code(code).map(|_| ())
        }
    }

    pub fn unbwt_aux(t: &[u8], u: &mut [u8], a: &mut [i64], freq: Option<&mut FreqTable>, i: &mut [i64], threads: i64) -> Result<()> {
        unsafe {
            let t_ptr = t.as_ptr();
            let u_ptr = u.as_mut_ptr();
            let a_ptr = a.as_mut_ptr();
            let (n, _) = length_and_freespace(same_value(t.len(), u.len())?, a.len())?;
            let freq_ptr = freq.map(|ptr| ptr.as_mut_ptr()).unwrap_or_else(null_mut);
            let r = aux_rate(i.len(), t.len())?;
            let i_ptr = i.as_mut_ptr();

            let code = libsais64_unbwt_aux_omp(t_ptr, u_ptr, a_ptr, n, freq_ptr, r, i_ptr, threads);
            interpret_code(code).map(|_| ())
        }
    }

    pub fn plcp(t: &[u8], sa: &[i64], plcp: &mut [i64], threads: i64) -> Result<()> {
        unsafe {
            let t_ptr = t.as_ptr();
            let sa_ptr = sa.as_ptr();
            let plcp_ptr = plcp.as_mut_ptr();
            let (n, _) = length_and_freespace(same_value(t.len(), plcp.len())?, sa.len())?;

            let code = libsais64_plcp_omp(t_ptr, sa_ptr, plcp_ptr, n, threads);
            interpret_code(code).map(|_| ())
        }
    }

    pub fn lcp(plcp: &[i64], sa: &[i64], lcp: &mut [i64], threads: i64) -> Result<()> {
        unsafe {
            let plcp_ptr = plcp.as_ptr();
            let sa_ptr = sa.as_ptr();
            let lcp_ptr = lcp.as_mut_ptr();
            let (n, _) = length_and_freespace(same_value(plcp.len(), lcp.len())?, sa.len())?;

            let code = libsais64_lcp_omp(plcp_ptr, sa_ptr, lcp_ptr, n, threads);
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

fn length_and_freespace(n: usize, m: usize) -> Result<(i64, i64)> {
    let p: i64 = n.try_into().map_err(|_| Error::IllegalArguments)?;
    let q: i64 = m.try_into().map_err(|_| Error::IllegalArguments)?;
    let fs = if q >= p { q - p } else { Err(Error::IllegalArguments)? };
    Ok((p, fs))
}

fn aux_rate(cap: usize, n: usize) -> Result<i64> {
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

fn interpret_code(code: i64) -> Result<i64> {
    match code {
        n if n >= 0 => Ok(n),
        -1 => Err(Error::IllegalArguments),
        -2 => Err(Error::InternalError),
        err => Err(Error::Uncategorized(err)),
    }
}
