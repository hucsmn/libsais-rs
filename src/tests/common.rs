#![allow(unused)]

use std::fmt::Debug;
use std::ops::RangeInclusive;

use crate::aux_index::{aux_length_exact, AUX_RATE_MIN};
use num_traits::{one, zero};
use num_traits::{AsPrimitive, NumAssignOps, PrimInt};
use rand::distributions::uniform;
use rand::prelude::*;

pub fn random_text<I: PrimInt + uniform::SampleUniform>(text_size: RangeInclusive<usize>, alphabet_range: RangeInclusive<I>) -> Vec<I> {
    let mut rng = thread_rng();
    let mut sample = vec![zero(); rng.gen_range(text_size)];
    for item in sample.iter_mut() {
        *item = rng.gen_range(alphabet_range.clone());
    }
    sample
}

#[inline]
pub fn allocate_suffix_arrays<I: PrimInt>(text_size: usize) -> Vec<Vec<I>> {
    vec![
        // allocates text_len
        vec![zero(); text_size],
        // allocates text_len + 256
        vec![zero(); text_size.saturating_add(256)],
        // allocates text_len * 1.125
        vec![zero(); text_size.saturating_add((text_size as f64 * 0.125) as usize)],
    ]
}

#[inline]
pub fn allocate_aux_arrays<I: PrimInt>(text_size: usize) -> Vec<Vec<I>> {
    let max_rate = Ord::min(text_size.checked_next_power_of_two().unwrap(), AUX_RATE_MIN);
    let mut rates: Vec<usize> = (1..)
        .into_iter()
        .map(|i| 1 << i)
        .take_while(|&r| r <= max_rate)
        .collect();
    if rates.len() > 3 {
        rates = vec![rates[0], rates[rates.len() / 2], rates[rates.len() - 1]];
    }
    rates
        .into_iter()
        .map(|r| vec![zero(); aux_length_exact(text_size, r).unwrap()])
        .collect()
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
