use std::ptr::{null, null_mut};

use crate::errors::{Error, ReturnCode};

#[inline]
pub unsafe fn freq_as_mut_ptr<T, EI: ReturnCode>(freq: Option<&mut [T]>, size: usize) -> Result<*mut T, Error<EI>> {
    if let Some(slice_mut) = freq {
        same_size(slice_mut.len(), size)?;
        Ok(slice_mut.as_mut_ptr())
    } else {
        Ok(null_mut())
    }
}

#[inline]
pub unsafe fn freq_as_ptr<T, EI: ReturnCode>(freq: Option<&[T]>, size: usize) -> Result<*const T, Error<EI>> {
    if let Some(slice) = freq {
        same_size(slice.len(), size)?;
        Ok(slice.as_ptr())
    } else {
        Ok(null())
    }
}

#[inline]
pub fn same_size<T: Copy + Eq, EI: ReturnCode>(one_size: T, another_size: T) -> Result<T, Error<EI>> {
    if one_size != another_size {
        Err(Error::IllegalArguments)
    } else {
        Ok(one_size)
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
pub fn aux_rate<T: TryFrom<usize>, EI: ReturnCode>(aux_cap: usize, text_size: usize) -> Result<T, Error<EI>> {
    if aux_cap == 0 {
        return Err(Error::IllegalArguments);
    }

    // calculate minimum rate
    let mut rate = if text_size % aux_cap != 0 {
        text_size / aux_cap + 1
    } else {
        text_size / aux_cap
    };
    rate = Ord::max(rate, 2);

    // try to find a minimum power of two rate value
    if (rate & (rate - 1)) != 0 {
        rate = rate.next_power_of_two();
        if rate == 0 {
            return Err(Error::IllegalArguments);
        }
    }

    // convert usize
    rate.try_into().map_err(|_| Error::IllegalArguments)
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
