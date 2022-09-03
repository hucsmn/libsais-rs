//! Calculation for bwt_aux/unbwt_aux auxiliary indices sampling rate and array length.

/// Minimum auxiliary indices array sampling rate.
pub const AUX_RATE_MIN: usize = 2;

/// Minimum auxiliary indices array length.
pub const AUX_LENGTH_MIN: usize = 1;

/// Calculates the maximum auxiliary indices array length, given limited array capacity.
///
/// Returns `Some(length)` where `length >= AUX_LENGTH_MIN`.
///
/// Returns `None` iff `aux_capacity == 0` or `aux_capacity` is too small to compute a proper sampling rate.
///
/// # Arguments
///
/// * `text_size` - Input text size of bwt/unbwt.
/// * `aux_capacity` - Capacity limit of auxiliary indices array.
///
/// # Examples
///
/// ```
/// use sais::sais32::bwt_aux;
/// use sais::aux_index::aux_length_max;
///
/// let input = b"mississippi";
/// let mut output = vec![0; input.len()];
/// let mut temporary = vec![0; input.len()];
/// let mut aux = vec![0; aux_length_max(input.len(), usize::MAX).unwrap()];
///
/// bwt_aux(input, &mut output[..], &mut temporary[..], None, &mut aux).unwrap();
/// println!("bwt_aux(input = {:?}) => (output = {:?}, aux = {:?})", input, output, aux);
/// ```
#[inline]
pub fn aux_length_max(text_size: usize, aux_capacity: usize) -> Option<usize> {
    if let Some(rate) = aux_rate_min(text_size, aux_capacity) {
        aux_length_exact(text_size, rate)
    } else {
        None
    }
}

/// Calculates the minimum auxiliary indices sampling rate, given limited array capacity.
///
/// Returns `Some(rate)` where `rate` is power of two and `rate >= AUX_RATE_MIN`.
///
/// Returns `None` iff `aux_capacity == 0` or `aux_capacity` is too small to compute a proper sampling rate.
///
/// # Arguments
///
/// * `text_size` - Input text size of bwt/unbwt.
/// * `aux_capacity` - Capacity limit of auxiliary indices array.
///
/// # Examples
///
/// ```
/// use sais::sais32::bwt_aux;
/// use sais::aux_index::{aux_length_max, aux_rate_min};
///
/// let input = b"mississippi";
/// let mut output = vec![0; input.len()];
/// let mut temporary = vec![0; input.len()];
/// let rate = aux_rate_min(input.len(), usize::MAX).unwrap();
/// let mut aux = vec![0; aux_length_max(input.len(), usize::MAX).unwrap()];
///
/// bwt_aux(input, &mut output[..], &mut temporary[..], None, &mut aux).unwrap();
/// println!("bwt_aux(input = {:?}) => (output = {:?}, aux = {:?}), sampling rate is {}", input, output, aux, rate);
/// ```
#[inline]
pub fn aux_rate_min(text_size: usize, aux_capacity: usize) -> Option<usize> {
    if aux_capacity == 0 {
        None
    } else if let Some(aux_rate) = div_ceil(text_size, aux_capacity).checked_next_power_of_two() {
        Some(Ord::max(aux_rate, AUX_RATE_MIN))
    } else {
        None
    }
}

/// Calculates proper sampling rate from a valid auxiliary indices array length.
///
/// Returns `Some(rate)` where `rate` is power of two and `rate >= AUX_RATE_MIN`.
///
/// Returns `None` iff `exact_aux_length` is not a valid auxiliary indices array length for `text_size`.
///
/// # Arguments
///
/// * `text_size` - Input text size of bwt/unbwt.
/// * `exact_aux_length` - Valid auxiliary indices array length for the input text size.
///
/// # Examples
///
/// ```
/// use sais::sais32::bwt_aux;
/// use sais::aux_index::{aux_length_max, aux_rate_exact};
///
/// let input = b"mississippi";
/// let mut output = vec![0; input.len()];
/// let mut temporary = vec![0; input.len()];
/// let mut aux = vec![0; aux_length_max(input.len(), usize::MAX).unwrap()];
/// let rate = aux_rate_exact(input.len(), aux.len()).unwrap();
///
/// bwt_aux(input, &mut output[..], &mut temporary[..], None, &mut aux).unwrap();
/// println!("bwt_aux(input = {:?}) => (output = {:?}, aux = {:?}), sampling rate is {}", input, output, aux, rate);
/// ```
#[inline]
pub fn aux_rate_exact(text_size: usize, exact_aux_length: usize) -> Option<usize> {
    if let Some(aux_rate) = aux_rate_min(text_size, exact_aux_length) {
        if let Some(max_aux_length) = aux_length_exact(text_size, aux_rate) {
            if exact_aux_length == max_aux_length {
                return Some(aux_rate);
            }
        }
    }
    None
}

/// Calculates auxiliary indices array length from a valid sampling rate.
///
/// Returns `Some(length)` where `length >= AUX_LENGTH_MIN`.
///
/// Returns `None` iff `aux_rate` is not a valid auxiliary sampling rate,
/// i.e. `aux_rate < AUX_RATE_MIN || !aux_rate.is_power_of_two()`.
///
/// # Arguments
///
/// * `text_size` - Input text size of bwt/unbwt.
/// * `aux_rate` - Valid auxiliary indices sampling rate, must be power of two and no less than `AUX_RATE_MIN`.
///
/// # Examples
///
/// ```
/// use sais::sais32::bwt_aux;
/// use sais::aux_index::{aux_rate_min, aux_length_exact};
///
/// let input = b"mississippi";
/// let mut output = vec![0; input.len()];
/// let mut temporary = vec![0; input.len()];
/// let rate = aux_rate_min(input.len(), usize::MAX).unwrap();
/// let mut aux = vec![0; aux_length_exact(input.len(), rate).unwrap()];
///
/// bwt_aux(input, &mut output[..], &mut temporary[..], None, &mut aux).unwrap();
/// println!("bwt_aux(input = {:?}) => (output = {:?}, aux = {:?}), sampling rate is {}", input, output, aux, rate);
/// ```
#[inline]
pub fn aux_length_exact(text_size: usize, aux_rate: usize) -> Option<usize> {
    if aux_rate >= AUX_RATE_MIN && aux_rate.is_power_of_two() {
        Some(Ord::max(div_ceil(text_size, aux_rate), AUX_LENGTH_MIN))
    } else {
        None
    }
}

#[inline]
fn div_ceil(x: usize, y: usize) -> usize {
    // use usize::div_ceil when it comes to stable rust
    // do not check y != 0
    if x % y == 0 {
        x / y
    } else {
        x / y + 1
    }
}
