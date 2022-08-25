extern "C" {
    /// int64_t libsais64(const uint8_t * T, int64_t * SA, int64_t n, int64_t fs, int64_t * freq);
    fn libsais64(t: *const u8, sa: *mut i64, n: i64, fs: i64, freq: *mut i64) -> i64;

    /// int64_t libsais64_bwt(const uint8_t * T, uint8_t * U, int64_t * A, int64_t n, int64_t fs, int64_t * freq);
    fn libsais64_bwt(t: *const u8, u: *mut u8, a: *mut i64, n: i64, fs: i64, freq: *mut i64) -> i64;

    /// int64_t libsais64_bwt_aux(const uint8_t * T, uint8_t * U, int64_t * A, int64_t n, int64_t fs, int64_t * freq, int64_t r, int64_t * I);
    fn libsais64_bwt_aux(t: *const u8, u: *mut u8, a: *mut i64, n: i64, fs: i64, freq: *mut i64, r: i64, i: *mut i64) -> i64;

    /// int64_t libsais64_unbwt(const uint8_t * T, uint8_t * U, int64_t * A, int64_t n, const int64_t * freq, int64_t i);
    fn libsais64_unbwt(t: *const u8, u: *mut u8, a: *mut i64, n: i64, freq: *mut i64, i: i64) -> i64;

    /// int64_t libsais64_unbwt_aux(const uint8_t * T, uint8_t * U, int64_t * A, int64_t n, const int64_t * freq, int64_t r, const int64_t * I);
    fn libsais64_unbwt_aux(t: *const u8, u: *mut u8, a: *mut i64, n: i64, freq: *mut i64, r: i64, i: *mut i64) -> i64;

    /// int64_t libsais64_plcp(const uint8_t * T, const int64_t * SA, int64_t * PLCP, int64_t n);
    fn libsais64_plcp(t: *const u8, sa: *const i64, plcp: *mut i64, n: i64) -> i64;

    /// int64_t libsais64_lcp(const int64_t * PLCP, const int64_t * SA, int64_t * LCP, int64_t n);
    fn libsais64_lcp(plcp: *const u8, sa: *const i64, lcp: *mut i64, n: i64) -> i64;
}

#[cfg(feature = "openmp")]
extern "C" {
    /// int64_t libsais64_omp(const uint8_t * T, int64_t * SA, int64_t n, int64_t fs, int64_t * freq, int64_t threads);
    fn libsais64_omp(t: *const u8, sa: *mut i64, n: i64, fs: i64, freq: *mut i64, threads: i64) -> i64;

    /// int64_t libsais64_bwt_omp(const uint8_t * T, uint8_t * U, int64_t * A, int64_t n, int64_t fs, int64_t * freq, int64_t threads);
    fn libsais64_bwt_omp(t: *const u8, u: *mut u8, a: *mut i64, n: i64, fs: i64, freq: *mut i64, threads: i64) -> i64;

    /// int64_t libsais64_bwt_aux_omp(const uint8_t * T, uint8_t * U, int64_t * A, int64_t n, int64_t fs, int64_t * freq, int64_t r, int64_t * I, int64_t threads);
    fn libsais64_bwt_aux_omp(t: *const u8, u: *mut u8, a: *mut i64, n: i64, fs: i64, freq: *mut i64, r: i64, i: *mut i64, threads: i64) -> i64;

    /// int64_t libsais64_unbwt_omp(const uint8_t * T, uint8_t * U, int64_t * A, int64_t n, const int64_t * freq, int64_t i, int64_t threads);
    fn libsais64_unbwt_omp(t: *const u8, u: *mut u8, a: *mut i64, n: i64, freq: *mut i64, i: i64, threads: i64) -> i64;

    /// int64_t libsais64_unbwt_aux_omp(const uint8_t * T, uint8_t * U, int64_t * A, int64_t n, const int64_t * freq, int64_t r, const int64_t * I, int64_t threads);
    fn libsais64_unbwt_aux_omp(t: *const u8, u: *mut u8, a: *mut i64, n: i64, freq: *mut i64, r: i64, i: *mut i64, threads: i64) -> i64;

    /// int64_t libsais64_plcp_omp(const uint8_t * T, const int64_t * SA, int64_t * PLCP, int64_t n, int64_t threads);
    fn libsais64_plcp_omp(t: *const u8, sa: *const i64, plcp: *mut i64, n: i64, threads: i64) -> i64;

    /// int64_t libsais64_lcp_omp(const int64_t * PLCP, const int64_t * SA, int64_t * LCP, int64_t n, int64_t threads);
    fn libsais64_lcp_omp(plcp: *const u8, sa: *const i64, lcp: *mut i64, n: i64, threads: i64) -> i64;
}
