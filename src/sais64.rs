use sais_sys::sais64::*;

use crate::common::*;

/// Output symbol frequency table size for u8 strings.
pub const FREQ_TABLE_SIZE: usize = 256;

/// Interpreted error code for 64-bit sais algorithms.
pub type Error = crate::errors::Error<i64>;

/// Interpreted return value for 64-bit sais algorithms.
pub type Result<T> = std::result::Result<T, Error>;

pub fn sais(t: &[u8], sa: &mut [i64], freq: Option<&mut [i64]>) -> Result<()> {
    unsafe {
        let t_ptr = t.as_ptr();
        let sa_ptr = sa.as_mut_ptr();
        let (n, fs) = split_size(t.len(), sa.len())?;
        let freq_ptr = freq_as_mut_ptr(freq, FREQ_TABLE_SIZE)?;

        let code = libsais64(t_ptr, sa_ptr, n, fs, freq_ptr);
        interpret_return_code_64(code).map(|_| ())
    }
}

#[cfg(feature = "bwt")]
pub fn bwt(t: &[u8], u: &mut [u8], a: &mut [i64], freq: Option<&mut [i64]>) -> Result<i64> {
    unsafe {
        let t_ptr = t.as_ptr();
        let u_ptr = u.as_mut_ptr();
        let a_ptr = a.as_mut_ptr();
        let (n, fs) = split_size(same_size(t.len(), u.len())?, a.len())?;
        let freq_ptr = freq_as_mut_ptr(freq, FREQ_TABLE_SIZE)?;

        let code = libsais64_bwt(t_ptr, u_ptr, a_ptr, n, fs, freq_ptr);
        interpret_return_code_64(code)
    }
}

#[cfg(feature = "bwt_aux")]
pub fn bwt_aux(t: &[u8], u: &mut [u8], a: &mut [i64], freq: Option<&mut [i64]>, i: &mut [i64]) -> Result<()> {
    unsafe {
        let t_ptr = t.as_ptr();
        let u_ptr = u.as_mut_ptr();
        let a_ptr = a.as_mut_ptr();
        let (n, fs) = split_size(same_size(t.len(), u.len())?, a.len())?;
        let freq_ptr = freq_as_mut_ptr(freq, FREQ_TABLE_SIZE)?;
        let r = aux_rate(i.len(), t.len())?;
        let i_ptr = i.as_mut_ptr();

        let code = libsais64_bwt_aux(t_ptr, u_ptr, a_ptr, n, fs, freq_ptr, r, i_ptr);
        interpret_return_code_64(code).map(|_| ())
    }
}

#[cfg(feature = "bwt")]
pub fn unbwt(t: &[u8], u: &mut [u8], a: &mut [i64], freq: Option<&[i64]>, i: i64) -> Result<()> {
    unsafe {
        let t_ptr = t.as_ptr();
        let u_ptr = u.as_mut_ptr();
        let a_ptr = a.as_mut_ptr();
        let n = unbwt_sufficient_size(same_size(t.len(), u.len())?, a.len())?;
        let freq_ptr = freq_as_ptr(freq, FREQ_TABLE_SIZE)?;

        let code = libsais64_unbwt(t_ptr, u_ptr, a_ptr, n, freq_ptr, i);
        interpret_return_code_64(code).map(|_| ())
    }
}

#[cfg(feature = "bwt_aux")]
pub fn unbwt_aux(t: &[u8], u: &mut [u8], a: &mut [i64], freq: Option<&[i64]>, i: &[i64]) -> Result<()> {
    unsafe {
        let t_ptr = t.as_ptr();
        let u_ptr = u.as_mut_ptr();
        let a_ptr = a.as_mut_ptr();
        let n = unbwt_sufficient_size(same_size(t.len(), u.len())?, a.len())?;
        let freq_ptr = freq_as_ptr(freq, FREQ_TABLE_SIZE)?;
        let r = aux_rate(i.len(), t.len())?;
        let i_ptr = i.as_ptr();

        let code = libsais64_unbwt_aux(t_ptr, u_ptr, a_ptr, n, freq_ptr, r, i_ptr);
        interpret_return_code_64(code).map(|_| ())
    }
}

#[cfg(feature = "lcp")]
pub fn plcp(t: &[u8], sa: &[i64], plcp: &mut [i64]) -> Result<()> {
    unsafe {
        let t_ptr = t.as_ptr();
        let sa_ptr = sa.as_ptr();
        let plcp_ptr = plcp.as_mut_ptr();
        let (n, _) = split_size(same_size(t.len(), plcp.len())?, sa.len())?;

        let code = libsais64_plcp(t_ptr, sa_ptr, plcp_ptr, n);
        interpret_return_code_64(code).map(|_| ())
    }
}

#[cfg(feature = "lcp")]
pub fn lcp(plcp: &[i64], sa: &[i64], lcp: &mut [i64]) -> Result<()> {
    unsafe {
        let plcp_ptr = plcp.as_ptr();
        let sa_ptr = sa.as_ptr();
        let lcp_ptr = lcp.as_mut_ptr();
        let (n, _) = split_size(same_size(plcp.len(), lcp.len())?, sa.len())?;

        let code = libsais64_lcp(plcp_ptr, sa_ptr, lcp_ptr, n);
        interpret_return_code_64(code).map(|_| ())
    }
}

#[cfg(feature = "parallel")]
pub mod parallel {
    //! Multi-threaded 64-bit sais algorithms on u8 array inputs.

    use super::*;

    pub fn sais(t: &[u8], sa: &mut [i64], freq: Option<&mut [i64]>, threads: i64) -> Result<()> {
        unsafe {
            let t_ptr = t.as_ptr();
            let sa_ptr = sa.as_mut_ptr();
            let (n, fs) = split_size(t.len(), sa.len())?;
            let freq_ptr = freq_as_mut_ptr(freq, FREQ_TABLE_SIZE)?;

            let code = libsais64_omp(t_ptr, sa_ptr, n, fs, freq_ptr, threads);
            interpret_return_code_64(code).map(|_| ())
        }
    }

    #[cfg(feature = "bwt")]
    pub fn bwt(t: &[u8], u: &mut [u8], a: &mut [i64], freq: Option<&mut [i64]>, threads: i64) -> Result<i64> {
        unsafe {
            let t_ptr = t.as_ptr();
            let u_ptr = u.as_mut_ptr();
            let a_ptr = a.as_mut_ptr();
            let (n, fs) = split_size(same_size(t.len(), u.len())?, a.len())?;
            let freq_ptr = freq_as_mut_ptr(freq, FREQ_TABLE_SIZE)?;

            let code = libsais64_bwt_omp(t_ptr, u_ptr, a_ptr, n, fs, freq_ptr, threads);
            interpret_return_code_64(code)
        }
    }

    #[cfg(feature = "bwt_aux")]
    pub fn bwt_aux(t: &[u8], u: &mut [u8], a: &mut [i64], freq: Option<&mut [i64]>, i: &mut [i64], threads: i64) -> Result<()> {
        unsafe {
            let t_ptr = t.as_ptr();
            let u_ptr = u.as_mut_ptr();
            let a_ptr = a.as_mut_ptr();
            let (n, fs) = split_size(same_size(t.len(), u.len())?, a.len())?;
            let freq_ptr = freq_as_mut_ptr(freq, FREQ_TABLE_SIZE)?;
            let r = aux_rate(i.len(), t.len())?;
            let i_ptr = i.as_mut_ptr();

            let code = libsais64_bwt_aux_omp(t_ptr, u_ptr, a_ptr, n, fs, freq_ptr, r, i_ptr, threads);
            interpret_return_code_64(code).map(|_| ())
        }
    }

    #[cfg(feature = "bwt")]
    pub fn unbwt(t: &[u8], u: &mut [u8], a: &mut [i64], freq: Option<&[i64]>, i: i64, threads: i64) -> Result<()> {
        unsafe {
            let t_ptr = t.as_ptr();
            let u_ptr = u.as_mut_ptr();
            let a_ptr = a.as_mut_ptr();
            let n = unbwt_sufficient_size(same_size(t.len(), u.len())?, a.len())?;
            let freq_ptr = freq_as_ptr(freq, FREQ_TABLE_SIZE)?;

            let code = libsais64_unbwt_omp(t_ptr, u_ptr, a_ptr, n, freq_ptr, i, threads);
            interpret_return_code_64(code).map(|_| ())
        }
    }

    #[cfg(feature = "bwt_aux")]
    pub fn unbwt_aux(t: &[u8], u: &mut [u8], a: &mut [i64], freq: Option<&[i64]>, i: &[i64], threads: i64) -> Result<()> {
        unsafe {
            let t_ptr = t.as_ptr();
            let u_ptr = u.as_mut_ptr();
            let a_ptr = a.as_mut_ptr();
            let n = unbwt_sufficient_size(same_size(t.len(), u.len())?, a.len())?;
            let freq_ptr = freq_as_ptr(freq, FREQ_TABLE_SIZE)?;
            let r = aux_rate(i.len(), t.len())?;
            let i_ptr = i.as_ptr();

            let code = libsais64_unbwt_aux_omp(t_ptr, u_ptr, a_ptr, n, freq_ptr, r, i_ptr, threads);
            interpret_return_code_64(code).map(|_| ())
        }
    }

    #[cfg(feature = "lcp")]
    pub fn plcp(t: &[u8], sa: &[i64], plcp: &mut [i64], threads: i64) -> Result<()> {
        unsafe {
            let t_ptr = t.as_ptr();
            let sa_ptr = sa.as_ptr();
            let plcp_ptr = plcp.as_mut_ptr();
            let (n, _) = split_size(same_size(t.len(), plcp.len())?, sa.len())?;

            let code = libsais64_plcp_omp(t_ptr, sa_ptr, plcp_ptr, n, threads);
            interpret_return_code_64(code).map(|_| ())
        }
    }

    #[cfg(feature = "lcp")]
    pub fn lcp(plcp: &[i64], sa: &[i64], lcp: &mut [i64], threads: i64) -> Result<()> {
        unsafe {
            let plcp_ptr = plcp.as_ptr();
            let sa_ptr = sa.as_ptr();
            let lcp_ptr = lcp.as_mut_ptr();
            let (n, _) = split_size(same_size(plcp.len(), lcp.len())?, sa.len())?;

            let code = libsais64_lcp_omp(plcp_ptr, sa_ptr, lcp_ptr, n, threads);
            interpret_return_code_64(code).map(|_| ())
        }
    }
}
