//! Calculation for bwt_aux/unbwt_aux auxiliary indices sampling rate and array length.

pub const AUX_RATE_MIN: usize = 2;

pub const AUX_LENGTH_MIN: usize = 1;

#[inline]
pub fn aux_length_max(text_size: usize, aux_capacity: usize) -> Option<usize> {
    if let Some(rate) = aux_rate_min(text_size, aux_capacity) {
        aux_length_exact(text_size, rate)
    } else {
        None
    }
}

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
