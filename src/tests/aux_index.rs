use crate::aux_index::*;
use std::iter::once;

const MAX_SAMPLE_TEXT_SIZE: usize = 130;
const TEXT_SIZE_THRESHOLD: usize = 4096;

#[test]
fn test_aux_index_known_capacity() {
    for text_size in text_size_samples() {
        for aux_capacity in aux_capacity_samples(text_size) {
            let optional_length = aux_length_max(text_size, aux_capacity);
            let optional_rate = aux_rate_min(text_size, aux_capacity);
            let is_invalid = aux_capacity == 0
                || text_size != 0
                    && ((text_size - 1) / aux_capacity + 1)
                        .checked_next_power_of_two()
                        .is_none();
            assert_eq!(is_invalid, optional_length.is_none());
            assert_eq!(is_invalid, optional_rate.is_none());
            if is_invalid {
                continue;
            }

            let length = optional_length.unwrap();
            let rate = optional_rate.unwrap();
            assert!(length >= AUX_LENGTH_MIN);
            assert!(rate >= AUX_RATE_MIN);
            if text_size > 0 {
                assert_eq!(length, (text_size - 1) / rate + 1);
            }
        }
    }
}

#[test]
fn test_aux_index_conversions() {
    for text_size in text_size_samples() {
        for aux_rate in aux_rate_samples() {
            let length = aux_length_exact(text_size, aux_rate).unwrap();
            assert!(length >= AUX_LENGTH_MIN);
            if text_size != 0 {
                assert!(length <= (text_size - 1) / AUX_RATE_MIN + 1)
            }

            let rate = aux_rate_exact(text_size, length).unwrap();
            assert!(rate >= AUX_RATE_MIN);
            assert!(rate <= aux_rate);
            if length > AUX_LENGTH_MIN {
                assert_eq!(rate, aux_rate);
            }
        }
    }
}

fn text_size_samples() -> impl Iterator<Item = usize> {
    (0..MAX_SAMPLE_TEXT_SIZE).chain(once(usize::MAX))
}

fn aux_rate_samples() -> impl Iterator<Item = usize> {
    (0..usize::BITS)
        .map(|n| 1 << n)
        .filter(|&r| r >= AUX_RATE_MIN)
}

fn aux_capacity_samples(text_size: usize) -> Box<dyn Iterator<Item = usize>> {
    if text_size < TEXT_SIZE_THRESHOLD {
        Box::new(0..text_size.saturating_add(1))
    } else {
        Box::new(
            once(0)
                .chain((0..usize::BITS).flat_map(|n| {
                    [
                        (1usize << n).saturating_sub(1),
                        1usize << n,
                        (1usize << n).saturating_add(1),
                    ]
                    .into_iter()
                }))
                .chain(once(usize::MAX)),
        )
    }
}
