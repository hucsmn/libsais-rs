use crate::aux_index::aux_rate_exact;
use crate::errors::{Error, ReturnCode};

#[inline]
pub unsafe fn freq_as_mut_ptr<T, EI: ReturnCode>(freq: Option<&mut [T]>, size: usize) -> Result<*mut T, Error<EI>> {
    use std::ptr::null_mut;

    if let Some(slice_mut) = freq {
        same_size(slice_mut.len(), size)?;
        Ok(slice_mut.as_mut_ptr())
    } else {
        Ok(null_mut())
    }
}

#[inline]
pub unsafe fn freq_as_ptr<T, EI: ReturnCode>(freq: Option<&[T]>, size: usize) -> Result<*const T, Error<EI>> {
    use std::ptr::null;

    if let Some(slice) = freq {
        same_size(slice.len(), size)?;
        Ok(slice.as_ptr())
    } else {
        Ok(null())
    }
}

#[inline]
pub fn same_size<T: Copy + Eq, EI: ReturnCode>(one_size: T, another_size: T) -> Result<T, Error<EI>> {
    if one_size == another_size {
        Ok(one_size)
    } else {
        Err(Error::IllegalArguments)
    }
}

#[inline]
pub fn max_size<T: Copy + Ord, EI: ReturnCode>(size: T, max: T) -> Result<T, Error<EI>> {
    if size <= max {
        Ok(size)
    } else {
        Err(Error::IllegalArguments)
    }
}

#[inline]
pub fn split_size<T: TryFrom<usize>, EI: ReturnCode>(small_size: usize, big_size: usize) -> Result<(T, T), Error<EI>> {
    let free_space = if big_size >= small_size {
        big_size - small_size
    } else {
        Err(Error::IllegalArguments)?
    };
    Ok((small_size.try_into().map_err(|_| Error::InternalError)?, free_space.try_into().map_err(|_| Error::InternalError)?))
}

#[inline]
pub fn unbwt_sufficient_size<T: TryFrom<usize>, EI: ReturnCode>(text_size: usize, tmp_size: usize) -> Result<T, Error<EI>> {
    if tmp_size <= text_size {
        Err(Error::IllegalArguments)?
    }
    Ok(text_size.try_into().map_err(|_| Error::InternalError)?)
}

#[inline]
pub fn aux_rate<T: TryFrom<usize>, EI: ReturnCode>(aux_length: usize, text_size: usize) -> Result<T, Error<EI>> {
    if let Some(aux_rate) = aux_rate_exact(text_size, aux_length) {
        aux_rate.try_into().map_err(|_| Error::IllegalArguments)
    } else {
        Err(Error::IllegalArguments)
    }
}

#[inline]
#[cfg(any(feature = "sais16", feature = "sais32"))]
pub fn interpret_return_code_32(code: i32) -> Result<i32, Error<i32>> {
    match code {
        n if n >= 0 => Ok(n),
        -1 => Err(Error::IllegalArguments),
        -2 => Err(Error::InternalError),
        err => Err(Error::Uncategorized(err)),
    }
}

#[inline]
#[cfg(feature = "sais64")]
pub fn interpret_return_code_64(code: i64) -> Result<i64, Error<i64>> {
    match code {
        n if n >= 0 => Ok(n),
        -1 => Err(Error::IllegalArguments),
        -2 => Err(Error::InternalError),
        err => Err(Error::Uncategorized(err)),
    }
}
