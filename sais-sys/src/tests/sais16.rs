use lazy_static::lazy_static;

use crate::sais16::*;

lazy_static! {
    static ref TEXTS: Vec<Vec<u16>> = {
        vec![
            vec![],
            vec![16384],
            vec![0, 65535],
            vec![109, 105, 115, 115, 105, 115, 115, 105, 112, 112, 105],
            vec![
                116, 104, 101, 32, 113, 117, 105, 99, 107, 32, 98, 114, 111, 119, 110, 32, 102,
                111, 120, 32, 106, 117, 109, 112, 115, 32, 111, 118, 101, 114, 32, 116, 104, 101,
                32, 108, 97, 122, 121, 32, 100, 111, 103,
            ],
            vec![
                65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84,
                85, 86, 87, 88, 89, 90,
            ],
        ]
    };
}

#[test]
fn test_sais() {
    let texts: Vec<&[u16]> = TEXTS.iter().map(|item| item.as_slice()).collect();
    for t in texts {
        for mut sa in allocate_suffix_arrays(t.len()) {
            // sais
            sais(t, sa.as_mut_slice(), None)
                .expect("sais failed");
            check_suffix_array(t, sa.as_slice());

            // sais, w/ output symbol frequency table
            let mut freq = vec![0i32; FREQ_TABLE_SIZE];
            sais(t, sa.as_mut_slice(), Some(freq.as_mut_slice()))
                .expect("sais failed");
            check_frequency_table(t, freq.as_slice());
            check_suffix_array(t, sa.as_slice());

            // sais, w/ context
            let mut ctx = SaisContext::new().unwrap();
            ctx
                .sais(t, sa.as_mut_slice(), None)
                .expect("sais failed");
            check_suffix_array(t, sa.as_slice());

            // sais, w/ context, w/ output symbol frequency table
            let mut freq = vec![0i32; FREQ_TABLE_SIZE];
            ctx
                .sais(t, sa.as_mut_slice(), Some(freq.as_mut_slice()))
                .expect("sais failed");
            check_frequency_table(t, freq.as_slice());
            check_suffix_array(t, sa.as_slice());
        }
    }
}

#[test]
#[cfg(feature = "openmp")]
fn test_sais_openmp() {
    let texts: Vec<&[u16]> = TEXTS.iter().map(|item| item.as_slice()).collect();
    for t in texts {
        for mut sa in allocate_suffix_arrays(t.len()) {
            // openmp::sais
            openmp::sais(t, sa.as_mut_slice(), None, 0)
                .expect("sais failed");
            check_suffix_array(t, sa.as_slice());

            // openmp::sais, w/ output symbol frequency table
            let mut freq = vec![0i32; FREQ_TABLE_SIZE];
            openmp::sais(t, sa.as_mut_slice(), Some(freq.as_mut_slice()), 0)
                .expect("sais failed");
            check_frequency_table(t, freq.as_slice());
            check_suffix_array(t, sa.as_slice());
        }
    }
}

#[test]
fn test_bwt_unbwt() {
    let texts: Vec<&[u16]> = TEXTS.iter().map(|item| item.as_slice()).collect();
    for t in texts {
        for mut a in allocate_suffix_arrays(t.len()) {
            let mut u = vec![0u16; t.len()];
            let mut s = vec![0u16; t.len()];

            // bwt + unbwt
            let i = bwt(t, u.as_mut_slice(), a.as_mut_slice(), None)
                .expect("bwt failed");
            unbwt(u.as_slice(), s.as_mut_slice(), a.as_mut_slice(), None, i)
                .expect("unbwt failed");
            assert_eq!(t, s.as_slice());

            // bwt + unbwt, w/ output symbol frequency table
            let mut freq = vec![0i32; FREQ_TABLE_SIZE];
            bwt(t, u.as_mut_slice(), a.as_mut_slice(), Some(freq.as_mut_slice()))
                .expect("bwt failed");
            check_frequency_table(t, freq.as_slice());
            unbwt(u.as_slice(), s.as_mut_slice(), a.as_mut_slice(), Some(freq.as_slice()), i)
                .expect("unbwt failed");
            assert_eq!(t, s.as_slice());

            // bwt + unbwt, w/ contexts
            let mut bwt_ctx = SaisContext::new().unwrap();
            let mut unbwt_ctx = UnbwtContext::new().unwrap();
            let i = bwt_ctx
                .bwt(t, u.as_mut_slice(), a.as_mut_slice(), None)
                .expect("bwt failed");
            unbwt_ctx
                .unbwt(u.as_slice(), s.as_mut_slice(), a.as_mut_slice(), None, i)
                .expect("unbwt failed");
            assert_eq!(t, s.as_slice());

            // bwt + unbwt, w/ contexts, w/ output symbol frequency table
            let mut freq = vec![0i32; FREQ_TABLE_SIZE];
            bwt_ctx
                .bwt(t, u.as_mut_slice(), a.as_mut_slice(), Some(freq.as_mut_slice()))
                .expect("bwt failed");
            check_frequency_table(t, freq.as_slice());
            unbwt_ctx
                .unbwt(u.as_slice(), s.as_mut_slice(), a.as_mut_slice(), Some(freq.as_slice()), i)
                .expect("unbwt failed");
            assert_eq!(t, s.as_slice());
        }
    }
}

#[test]
#[cfg(feature = "openmp")]
fn test_bwt_unbwt_openmp() {
    let texts: Vec<&[u16]> = TEXTS.iter().map(|item| item.as_slice()).collect();
    for t in texts {
        for mut a in allocate_suffix_arrays(t.len()) {
            let mut u = vec![0u16; t.len()];
            let mut s = vec![0u16; t.len()];

            // openmp::bwt + openmp::unbwt
            let i = openmp::bwt(t, u.as_mut_slice(), a.as_mut_slice(), None, 0)
                .expect("bwt failed");
            openmp::unbwt(u.as_slice(), s.as_mut_slice(), a.as_mut_slice(), None, i, 0)
                .expect("unbwt failed");
            assert_eq!(t, s.as_slice());

            // openmp::bwt + openmp::unbwt, w/ output symbol frequency table
            let mut freq = vec![0i32; FREQ_TABLE_SIZE];
            openmp::bwt(t, u.as_mut_slice(), a.as_mut_slice(), Some(freq.as_mut_slice()), 0)
                .expect("bwt failed");
            check_frequency_table(t, freq.as_slice());
            openmp::unbwt(u.as_slice(), s.as_mut_slice(), a.as_mut_slice(), Some(freq.as_slice()), i, 0)
                .expect("unbwt failed");
            assert_eq!(t, s.as_slice());
        }
    }
}

#[test]
fn test_bwt_unbwt_aux() {
    let texts: Vec<&[u16]> = TEXTS.iter().map(|item| item.as_slice()).collect();
    for t in texts {
        for mut a in allocate_suffix_arrays(t.len()) {
            let mut u = vec![0u16; t.len()];
            let mut s = vec![0u16; t.len()];
            let mut i = vec![0i32; Ord::max(t.len() / 4, 1)];

            // bwt_aux + unbwt_aux
            bwt_aux(t, u.as_mut_slice(), a.as_mut_slice(), None, i.as_mut_slice())
                .expect("bwt failed");
            unbwt_aux(u.as_slice(), s.as_mut_slice(), a.as_mut_slice(), None, i.as_slice())
                .expect("unbwt failed");
            assert_eq!(t, s.as_slice());

            // bwt_aux + unbwt_aux, w/ output symbol frequency table
            let mut freq = vec![0i32; FREQ_TABLE_SIZE];
            bwt_aux(t, u.as_mut_slice(), a.as_mut_slice(), Some(freq.as_mut_slice()), i.as_mut_slice())
                .expect("bwt failed");
            check_frequency_table(t, freq.as_slice());
            unbwt_aux(u.as_slice(), s.as_mut_slice(), a.as_mut_slice(), Some(freq.as_slice()), i.as_slice())
                .expect("unbwt failed");
            assert_eq!(t, s.as_slice());

            // bwt_aux + unbwt_aux, w/ contexts
            let mut bwt_ctx = SaisContext::new().unwrap();
            let mut unbwt_ctx = UnbwtContext::new().unwrap();
            bwt_ctx
                .bwt_aux(t, u.as_mut_slice(), a.as_mut_slice(), None, i.as_mut_slice())
                .expect("bwt failed");
            unbwt_ctx
                .unbwt_aux(u.as_slice(), s.as_mut_slice(), a.as_mut_slice(), None, i.as_slice())
                .expect("unbwt failed");
            assert_eq!(t, s.as_slice());

            // bwt_aux + unbwt_aux, w/ contexts, w/ output symbol frequency table
            let mut freq = vec![0i32; FREQ_TABLE_SIZE];
            bwt_ctx
                .bwt_aux(t, u.as_mut_slice(), a.as_mut_slice(), Some(freq.as_mut_slice()), i.as_mut_slice())
                .expect("bwt failed");
            check_frequency_table(t, freq.as_slice());
            unbwt_ctx
                .unbwt_aux(u.as_slice(), s.as_mut_slice(), a.as_mut_slice(), Some(freq.as_slice()), i.as_slice())
                .expect("unbwt failed");
            assert_eq!(t, s.as_slice());
        }
    }
}

#[test]
#[cfg(feature = "openmp")]
fn test_bwt_unbwt_aux_openmp() {
    let texts: Vec<&[u16]> = TEXTS.iter().map(|item| item.as_slice()).collect();
    for t in texts {
        for mut a in allocate_suffix_arrays(t.len()) {
            let mut u = vec![0u16; t.len()];
            let mut s = vec![0u16; t.len()];
            let mut i = vec![0i32; Ord::max(t.len() / 4, 1)];

            // openmp::bwt_aux + openmp::unbwt_aux
            openmp::bwt_aux(t, u.as_mut_slice(), a.as_mut_slice(), None, i.as_mut_slice(), 0)
                .expect("bwt failed");
            openmp::unbwt_aux(u.as_slice(), s.as_mut_slice(), a.as_mut_slice(), None, i.as_slice(), 0)
                .expect("unbwt failed");
            assert_eq!(t, s.as_slice());

            // openmp::bwt_aux + openmp::unbwt_aux, w/ output symbol frequency table
            let mut freq = vec![0i32; FREQ_TABLE_SIZE];
            openmp::bwt_aux(t, u.as_mut_slice(), a.as_mut_slice(), Some(freq.as_mut_slice()), i.as_mut_slice(), 0)
                .expect("bwt failed");
            check_frequency_table(t, freq.as_slice());
            openmp::unbwt_aux(u.as_slice(), s.as_mut_slice(), a.as_mut_slice(), Some(freq.as_slice()), i.as_slice(), 0)
                .expect("unbwt failed");
            assert_eq!(t, s.as_slice());
        }
    }
}

#[test]
fn test_plcp_lcp() {
    let texts: Vec<&[u16]> = TEXTS.iter().map(|item| item.as_slice()).collect();
    for t in texts {
        let mut sa = vec![0i32; t.len()];
        let mut plcp_array = vec![0i32; t.len()];
        let mut lcp_array = vec![0i32; t.len()];

        sais(t, sa.as_mut_slice(), None)
            .expect("sais failed");

        // plcp + lcp
        plcp(t, sa.as_slice(), plcp_array.as_mut_slice())
            .expect("plcp failed");
        lcp(plcp_array.as_slice(), sa.as_slice(), lcp_array.as_mut_slice())
            .expect("lcp failed");
        check_lcp_array(t, sa.as_slice(), lcp_array.as_slice());
    }
}

#[test]
#[cfg(feature = "openmp")]
fn test_plcp_lcp_openmp() {
    let texts: Vec<&[u16]> = TEXTS.iter().map(|item| item.as_slice()).collect();
    for t in texts {
        let mut sa = vec![0i32; t.len()];
        let mut plcp_array = vec![0i32; t.len()];
        let mut lcp_array = vec![0i32; t.len()];

        openmp::sais(t, sa.as_mut_slice(), None, 0)
            .expect("sais failed");

        // openmp::plcp + openmp::lcp
        openmp::plcp(t, sa.as_slice(), plcp_array.as_mut_slice(), 0)
            .expect("plcp failed");
        openmp::lcp(plcp_array.as_slice(), sa.as_slice(), lcp_array.as_mut_slice(), 0)
            .expect("lcp failed");
        check_lcp_array(t, sa.as_slice(), lcp_array.as_slice());
    }
}

fn allocate_suffix_arrays(len: usize) -> Vec<Vec<i32>> {
    vec![
        vec![0i32; len],
        vec![0i32; len.saturating_mul(2)],
    ]
}

fn check_suffix_array(t: &[u16], sa: &[i32]) {
    if t.len() > 0 {
        assert!(sa.len() >= t.len());
        for i in 0..t.len() - 1 {
            assert!(t[sa[i] as usize..] < t[sa[i + 1] as usize..]);
        }
    }
}

fn check_lcp_array(t: &[u16], sa: &[i32], lcp: &[i32]) {
    if t.len() > 0 {
        assert!(sa.len() >= t.len());
        assert!(lcp.len() >= t.len());
        assert_eq!(lcp[0], 0);
        for i in 1..t.len() {
            let common = Iterator::zip(
                t[sa[i - 1] as usize..].iter(),
                t[sa[i] as usize..].iter(),
            )
                .take_while(|(&x, &y)| x == y)
                .count();
            assert_eq!(lcp[i], common.try_into().unwrap());
        }
    }
}

fn check_frequency_table(t: &[u16], freq: &[i32]) {
    let mut rust_freq = [0i32; FREQ_TABLE_SIZE];
    for &ch in t {
        rust_freq[ch as usize] += 1;
    }
    assert_eq!(freq, rust_freq.as_slice());
}
