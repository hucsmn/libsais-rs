#![allow(unused)]

use once_cell::sync::Lazy;

use crate::sais16::*;
use crate::tests::common::*;

static TEXTS: Lazy<Vec<Vec<u16>>> = Lazy::new(|| {
    let text_samples: Vec<&[u8]> = vec![
        b"",
        b"_",
        b"\x00\xff",
        b"mississippi",
        b"the quick brown fox jumps over the lazy dog",
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZ",
        b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. \
            Egestas egestas fringilla phasellus faucibus scelerisque eleifend donec pretium vulputate. Feugiat in fermentum posuere urna. \
            Amet nisl purus in mollis nunc. Tellus orci ac auctor augue mauris augue. Dolor morbi non arcu risus quis varius quam quisque id. \
            Et malesuada fames ac turpis egestas sed tempus. Eget mi proin sed libero enim sed faucibus. Turpis massa sed elementum tempus. \
            Congue eu consequat ac felis donec.",
    ];
    let mut samples: Vec<Vec<u16>> = text_samples
        .into_iter()
        .map(|sample| sample.to_vec().into_iter().map(|ch| ch as u16).collect())
        .collect();
    samples.push(random_text(100..=200, 0..=4));
    samples.push(random_text(100..=200, 0..=16));
    samples.push(random_text(100..=200, 0..=64));
    samples.push(random_text(100..=200, 128..=255));
    samples.push(random_text(100..=200, 15872..=16384));
    samples.push(random_text(100000..=200000, 0..=4));
    samples.push(random_text(100000..=200000, 0..=16));
    samples.push(random_text(100000..=200000, 0..=64));
    samples.push(random_text(100000..=200000, 128..=255));
    samples.push(random_text(100000..=200000, 15872..=16384));
    samples
});

#[test]
fn test_sais_basic() {
    let texts: Vec<&[u16]> = TEXTS.iter().map(|item| item.as_slice()).collect();
    for t in texts {
        for mut sa in allocate_suffix_arrays(t.len()) {
            // sais
            sais(t, sa.as_mut_slice(), None).expect("sais failed");
            check_suffix_array(t, sa.as_slice());

            // sais, w/ output symbol frequency table
            let mut freq = vec![0i32; FREQ_TABLE_SIZE];
            sais(t, sa.as_mut_slice(), Some(freq.as_mut_slice())).expect("sais failed");
            check_frequency_table(t, freq.as_slice(), FREQ_TABLE_SIZE);
            check_suffix_array(t, sa.as_slice());
        }
    }
}

#[test]
#[cfg(feature = "context")]
fn test_sais_context() {
    let texts: Vec<&[u16]> = TEXTS.iter().map(|item| item.as_slice()).collect();
    for t in texts {
        for mut sa in allocate_suffix_arrays(t.len()) {
            // sais, w/ context
            let mut ctx = SaisContext::new().unwrap();
            ctx.sais(t, sa.as_mut_slice(), None).expect("sais failed");
            check_suffix_array(t, sa.as_slice());

            // sais, w/ context, w/ output symbol frequency table
            let mut freq = vec![0i32; FREQ_TABLE_SIZE];
            ctx.sais(t, sa.as_mut_slice(), Some(freq.as_mut_slice()))
                .expect("sais failed");
            check_frequency_table(t, freq.as_slice(), FREQ_TABLE_SIZE);
            check_suffix_array(t, sa.as_slice());
        }
    }
}

#[test]
#[cfg(feature = "parallel")]
fn test_sais_parallel() {
    let texts: Vec<&[u16]> = TEXTS.iter().map(|item| item.as_slice()).collect();
    for t in texts {
        for mut sa in allocate_suffix_arrays(t.len()) {
            // parallel::sais
            parallel::sais(t, sa.as_mut_slice(), None, 0).expect("sais failed");
            check_suffix_array(t, sa.as_slice());

            // parallel::sais, w/ output symbol frequency table
            let mut freq = vec![0i32; FREQ_TABLE_SIZE];
            parallel::sais(t, sa.as_mut_slice(), Some(freq.as_mut_slice()), 0).expect("sais failed");
            check_frequency_table(t, freq.as_slice(), FREQ_TABLE_SIZE);
            check_suffix_array(t, sa.as_slice());
        }
    }
}

#[test]
#[cfg(feature = "bwt")]
fn test_bwt_unbwt_basic() {
    let texts: Vec<&[u16]> = TEXTS.iter().map(|item| item.as_slice()).collect();
    for t in texts {
        let mut u = vec![0u16; t.len()];
        let mut s = vec![0u16; t.len()];
        let mut a0 = vec![0i32; t.len()];
        let mut a1 = vec![0i32; t.len().checked_add(1).unwrap()];

        // bwt + unbwt
        let i = bwt(t, u.as_mut_slice(), a0.as_mut_slice(), None).expect("bwt failed");
        unbwt(u.as_slice(), s.as_mut_slice(), a1.as_mut_slice(), None, i).expect("unbwt failed");
        assert_eq!(t, s.as_slice());

        // bwt + unbwt, w/ output symbol frequency table
        let mut freq = vec![0i32; FREQ_TABLE_SIZE];
        let i = bwt(t, u.as_mut_slice(), a0.as_mut_slice(), Some(freq.as_mut_slice())).expect("bwt failed");
        check_frequency_table(t, freq.as_slice(), FREQ_TABLE_SIZE);
        unbwt(u.as_slice(), s.as_mut_slice(), a1.as_mut_slice(), Some(freq.as_slice()), i).expect("unbwt failed");
        assert_eq!(t, s.as_slice());
    }
}

#[test]
#[cfg(all(feature = "bwt", feature = "context"))]
fn test_bwt_unbwt_context() {
    let texts: Vec<&[u16]> = TEXTS.iter().map(|item| item.as_slice()).collect();
    for t in texts {
        let mut u = vec![0u16; t.len()];
        let mut s = vec![0u16; t.len()];
        let mut bwt_ctx = SaisContext::new().unwrap();
        let mut unbwt_ctx = UnbwtContext::new().unwrap();
        let mut a0 = vec![0i32; t.len()];
        let mut a1 = vec![0i32; t.len().checked_add(1).unwrap()];

        // bwt + unbwt, w/ contexts
        let i = bwt_ctx
            .bwt(t, u.as_mut_slice(), a0.as_mut_slice(), None)
            .expect("bwt failed");
        unbwt_ctx
            .unbwt(u.as_slice(), s.as_mut_slice(), a1.as_mut_slice(), None, i)
            .expect("unbwt failed");
        assert_eq!(t, s.as_slice());

        // bwt + unbwt, w/ contexts, w/ output symbol frequency table
        let mut freq = vec![0i32; FREQ_TABLE_SIZE];
        let i = bwt_ctx
            .bwt(t, u.as_mut_slice(), a0.as_mut_slice(), Some(freq.as_mut_slice()))
            .expect("bwt failed");
        check_frequency_table(t, freq.as_slice(), FREQ_TABLE_SIZE);
        unbwt_ctx
            .unbwt(u.as_slice(), s.as_mut_slice(), a1.as_mut_slice(), Some(freq.as_slice()), i)
            .expect("unbwt failed");
        assert_eq!(t, s.as_slice());
    }
}

#[test]
#[cfg(all(feature = "bwt", feature = "parallel"))]
fn test_bwt_unbwt_parallel() {
    let texts: Vec<&[u16]> = TEXTS.iter().map(|item| item.as_slice()).collect();
    for t in texts {
        let mut u = vec![0u16; t.len()];
        let mut s = vec![0u16; t.len()];
        let mut a0 = vec![0i32; t.len()];
        let mut a1 = vec![0i32; t.len().checked_add(1).unwrap()];

        // parallel::bwt + parallel::unbwt
        let i = parallel::bwt(t, u.as_mut_slice(), a0.as_mut_slice(), None, 0).expect("bwt failed");
        parallel::unbwt(u.as_slice(), s.as_mut_slice(), a1.as_mut_slice(), None, i, 0).expect("unbwt failed");
        assert_eq!(t, s.as_slice());

        // parallel::bwt + parallel::unbwt, w/ output symbol frequency table
        let mut freq = vec![0i32; FREQ_TABLE_SIZE];
        let i = parallel::bwt(t, u.as_mut_slice(), a0.as_mut_slice(), Some(freq.as_mut_slice()), 0).expect("bwt failed");
        check_frequency_table(t, freq.as_slice(), FREQ_TABLE_SIZE);
        parallel::unbwt(u.as_slice(), s.as_mut_slice(), a1.as_mut_slice(), Some(freq.as_slice()), i, 0).expect("unbwt failed");
        assert_eq!(t, s.as_slice());
    }
}

#[test]
#[cfg(feature = "bwt_aux")]
fn test_bwt_unbwt_aux_basic() {
    let texts: Vec<&[u16]> = TEXTS.iter().map(|item| item.as_slice()).collect();
    for t in texts {
        let mut u = vec![0u16; t.len()];
        let mut s = vec![0u16; t.len()];
        let mut i = vec![0i32; Ord::max(t.len() / 4, 1)];
        let mut a0 = vec![0i32; t.len()];
        let mut a1 = vec![0i32; t.len().checked_add(1).unwrap()];

        // bwt_aux + unbwt_aux
        bwt_aux(t, u.as_mut_slice(), a0.as_mut_slice(), None, i.as_mut_slice()).expect("bwt failed");
        unbwt_aux(u.as_slice(), s.as_mut_slice(), a1.as_mut_slice(), None, i.as_slice()).expect("unbwt failed");
        assert_eq!(t, s.as_slice());

        // bwt_aux + unbwt_aux, w/ output symbol frequency table
        let mut freq = vec![0i32; FREQ_TABLE_SIZE];
        bwt_aux(t, u.as_mut_slice(), a0.as_mut_slice(), Some(freq.as_mut_slice()), i.as_mut_slice()).expect("bwt failed");
        check_frequency_table(t, freq.as_slice(), FREQ_TABLE_SIZE);
        unbwt_aux(u.as_slice(), s.as_mut_slice(), a1.as_mut_slice(), Some(freq.as_slice()), i.as_slice()).expect("unbwt failed");
        assert_eq!(t, s.as_slice());
    }
}

#[test]
#[cfg(all(feature = "bwt_aux", feature = "context"))]
fn test_bwt_unbwt_aux_context() {
    let texts: Vec<&[u16]> = TEXTS.iter().map(|item| item.as_slice()).collect();
    for t in texts {
        let mut u = vec![0u16; t.len()];
        let mut s = vec![0u16; t.len()];
        let mut i = vec![0i32; Ord::max(t.len() / 4, 1)];
        let mut bwt_ctx = SaisContext::new().unwrap();
        let mut unbwt_ctx = UnbwtContext::new().unwrap();
        let mut a0 = vec![0i32; t.len()];
        let mut a1 = vec![0i32; t.len().checked_add(1).unwrap()];

        // bwt_aux + unbwt_aux, w/ contexts
        bwt_ctx
            .bwt_aux(t, u.as_mut_slice(), a0.as_mut_slice(), None, i.as_mut_slice())
            .expect("bwt failed");
        unbwt_ctx
            .unbwt_aux(u.as_slice(), s.as_mut_slice(), a1.as_mut_slice(), None, i.as_slice())
            .expect("unbwt failed");
        assert_eq!(t, s.as_slice());

        // bwt_aux + unbwt_aux, w/ contexts, w/ output symbol frequency table
        let mut freq = vec![0i32; FREQ_TABLE_SIZE];
        bwt_ctx
            .bwt_aux(t, u.as_mut_slice(), a0.as_mut_slice(), Some(freq.as_mut_slice()), i.as_mut_slice())
            .expect("bwt failed");
        check_frequency_table(t, freq.as_slice(), FREQ_TABLE_SIZE);
        unbwt_ctx
            .unbwt_aux(u.as_slice(), s.as_mut_slice(), a1.as_mut_slice(), Some(freq.as_slice()), i.as_slice())
            .expect("unbwt failed");
        assert_eq!(t, s.as_slice());
    }
}

#[test]
#[cfg(all(feature = "bwt_aux", feature = "parallel"))]
fn test_bwt_unbwt_aux_parallel() {
    let texts: Vec<&[u16]> = TEXTS.iter().map(|item| item.as_slice()).collect();
    for t in texts {
        let mut u = vec![0u16; t.len()];
        let mut s = vec![0u16; t.len()];
        let mut i = vec![0i32; Ord::max(t.len() / 4, 1)];
        let mut a0 = vec![0i32; t.len()];
        let mut a1 = vec![0i32; t.len().checked_add(1).unwrap()];

        // parallel::bwt_aux + parallel::unbwt_aux
        parallel::bwt_aux(t, u.as_mut_slice(), a0.as_mut_slice(), None, i.as_mut_slice(), 0).expect("bwt failed");
        parallel::unbwt_aux(u.as_slice(), s.as_mut_slice(), a1.as_mut_slice(), None, i.as_slice(), 0).expect("unbwt failed");
        assert_eq!(t, s.as_slice());

        // parallel::bwt_aux + parallel::unbwt_aux, w/ output symbol frequency table
        let mut freq = vec![0i32; FREQ_TABLE_SIZE];
        parallel::bwt_aux(t, u.as_mut_slice(), a0.as_mut_slice(), Some(freq.as_mut_slice()), i.as_mut_slice(), 0).expect("bwt failed");
        check_frequency_table(t, freq.as_slice(), FREQ_TABLE_SIZE);
        parallel::unbwt_aux(u.as_slice(), s.as_mut_slice(), a1.as_mut_slice(), Some(freq.as_slice()), i.as_slice(), 0).expect("unbwt failed");
        assert_eq!(t, s.as_slice());
    }
}

#[test]
#[cfg(feature = "lcp")]
fn test_plcp_lcp_basic() {
    let texts: Vec<&[u16]> = TEXTS.iter().map(|item| item.as_slice()).collect();
    for t in texts {
        let mut sa = vec![0i32; t.len()];
        let mut plcp_array = vec![0i32; t.len()];
        let mut lcp_array = vec![0i32; t.len()];

        sais(t, sa.as_mut_slice(), None).expect("sais failed");

        // plcp + lcp
        plcp(t, sa.as_slice(), plcp_array.as_mut_slice()).expect("plcp failed");
        lcp(plcp_array.as_slice(), sa.as_slice(), lcp_array.as_mut_slice()).expect("lcp failed");
        check_lcp_array(t, sa.as_slice(), lcp_array.as_slice());
    }
}

#[test]
#[cfg(all(feature = "lcp", feature = "parallel"))]
fn test_plcp_lcp_parallel() {
    let texts: Vec<&[u16]> = TEXTS.iter().map(|item| item.as_slice()).collect();
    for t in texts {
        let mut sa = vec![0i32; t.len()];
        let mut plcp_array = vec![0i32; t.len()];
        let mut lcp_array = vec![0i32; t.len()];

        parallel::sais(t, sa.as_mut_slice(), None, 0).expect("sais failed");

        // parallel::plcp + parallel::lcp
        parallel::plcp(t, sa.as_slice(), plcp_array.as_mut_slice(), 0).expect("plcp failed");
        parallel::lcp(plcp_array.as_slice(), sa.as_slice(), lcp_array.as_mut_slice(), 0).expect("lcp failed");
        check_lcp_array(t, sa.as_slice(), lcp_array.as_slice());
    }
}
