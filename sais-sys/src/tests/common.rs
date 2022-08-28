use std::fmt::Debug;

use num_traits::{one, zero};
use num_traits::{AsPrimitive, NumAssignOps, PrimInt};

#[inline]
pub fn allocate_suffix_arrays<I: PrimInt>(len: usize) -> Vec<Vec<I>> {
    vec![vec![zero(); len], vec![zero(); len.saturating_mul(2)]]
}

#[inline]
pub fn check_suffix_array<TI, AI>(t: &[TI], sa: &[AI])
where
    TI: PrimInt + AsPrimitive<usize>,
    AI: PrimInt + AsPrimitive<usize>,
{
    if t.len() > 0 {
        assert!(sa.len() >= t.len());
        for i in 0..t.len() - 1 {
            assert!(t[sa[i].as_()..] < t[sa[i + 1].as_()..]);
        }
    }
}

#[inline]
pub fn check_lcp_array<TI, AI>(t: &[TI], sa: &[AI], lcp: &[AI])
where
    TI: PrimInt + AsPrimitive<usize> + Debug,
    AI: PrimInt + AsPrimitive<usize> + TryFrom<usize> + Debug,
    <AI as TryFrom<usize>>::Error: Debug,
{
    if t.len() > 0 {
        assert!(sa.len() >= t.len());
        assert!(lcp.len() >= t.len());
        assert_eq!(lcp[0], zero());
        for i in 1..t.len() {
            let common = Iterator::zip(t[sa[i - 1].as_()..].iter(), t[sa[i].as_()..].iter())
                .take_while(|(&x, &y)| x == y)
                .count();
            assert_eq!(lcp[i], common.try_into().unwrap());
        }
    }
}

#[inline]
pub fn check_frequency_table<TI, AI>(t: &[TI], freq: &[AI], freq_size: usize)
where
    TI: PrimInt + AsPrimitive<usize> + Debug,
    AI: PrimInt + NumAssignOps + AsPrimitive<usize> + Debug,
{
    let mut rust_freq = vec![zero::<AI>(); freq_size];
    for &ch in t {
        rust_freq[ch.as_()] += one();
    }
    assert_eq!(freq, rust_freq.as_slice());
}
