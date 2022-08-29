//! 32-bit sais algorithms on u8 array inputs.

use libc::c_void;

extern "C" {
    /// void * libsais_create_ctx(void);
    pub fn libsais_create_ctx() -> *mut c_void;

    /// void libsais_free_ctx(void * ctx);
    pub fn libsais_free_ctx(ctx: *mut c_void);

    /// int32_t libsais(const uint8_t * T, int32_t * SA, int32_t n, int32_t fs, int32_t * freq);
    pub fn libsais(t: *const u8, sa: *mut i32, n: i32, fs: i32, freq: *mut i32) -> i32;

    /// int32_t libsais_int(int32_t * T, int32_t * SA, int32_t n, int32_t k, int32_t fs);
    pub fn libsais_int(t: *mut i32, sa: *mut i32, n: i32, k: i32, fs: i32) -> i32;

    /// int32_t libsais_ctx(const void * ctx, const uint8_t * T, int32_t * SA, int32_t n, int32_t fs, int32_t * freq);
    pub fn libsais_ctx(ctx: *mut c_void, t: *const u8, sa: *mut i32, n: i32, fs: i32, freq: *mut i32) -> i32;

    /// int32_t libsais_bwt(const uint8_t * T, uint8_t * U, int32_t * A, int32_t n, int32_t fs, int32_t * freq);
    pub fn libsais_bwt(t: *const u8, u: *mut u8, a: *mut i32, n: i32, fs: i32, freq: *mut i32) -> i32;

    /// int32_t libsais_bwt_aux(const uint8_t * T, uint8_t * U, int32_t * A, int32_t n, int32_t fs, int32_t * freq, int32_t r, int32_t * I);
    pub fn libsais_bwt_aux(t: *const u8, u: *mut u8, a: *mut i32, n: i32, fs: i32, freq: *mut i32, r: i32, i: *mut i32) -> i32;

    /// int32_t libsais_bwt_ctx(const void * ctx, const uint8_t * T, uint8_t * U, int32_t * A, int32_t n, int32_t fs, int32_t * freq);
    pub fn libsais_bwt_ctx(ctx: *mut c_void, t: *const u8, u: *mut u8, a: *mut i32, n: i32, fs: i32, freq: *mut i32) -> i32;

    /// int32_t libsais_bwt_aux_ctx(const void * ctx, const uint8_t * T, uint8_t * U, int32_t * A, int32_t n, int32_t fs, int32_t * freq, int32_t r, int32_t * I);
    pub fn libsais_bwt_aux_ctx(ctx: *mut c_void, t: *const u8, u: *mut u8, a: *mut i32, n: i32, fs: i32, freq: *mut i32, r: i32, i: *mut i32) -> i32;

    /// void * libsais_unbwt_create_ctx(void);
    pub fn libsais_unbwt_create_ctx() -> *mut c_void;

    /// void libsais_unbwt_free_ctx(void * ctx);
    pub fn libsais_unbwt_free_ctx(ctx: *mut c_void);

    /// int32_t libsais_unbwt(const uint8_t * T, uint8_t * U, int32_t * A, int32_t n, const int32_t * freq, int32_t i);
    pub fn libsais_unbwt(t: *const u8, u: *mut u8, a: *mut i32, n: i32, freq: *const i32, i: i32) -> i32;

    /// int32_t libsais_unbwt_ctx(const void * ctx, const uint8_t * T, uint8_t * U, int32_t * A, int32_t n, const int32_t * freq, int32_t i);
    pub fn libsais_unbwt_ctx(ctx: *mut c_void, t: *const u8, u: *mut u8, a: *mut i32, n: i32, freq: *const i32, i: i32) -> i32;

    /// int32_t libsais_unbwt_aux(const uint8_t * T, uint8_t * U, int32_t * A, int32_t n, const int32_t * freq, int32_t r, const int32_t * I);
    pub fn libsais_unbwt_aux(t: *const u8, u: *mut u8, a: *mut i32, n: i32, freq: *const i32, r: i32, i: *const i32) -> i32;

    /// int32_t libsais_unbwt_aux_ctx(const void * ctx, const uint8_t * T, uint8_t * U, int32_t * A, int32_t n, const int32_t * freq, int32_t r, const int32_t * I);
    pub fn libsais_unbwt_aux_ctx(ctx: *mut c_void, t: *const u8, u: *mut u8, a: *mut i32, n: i32, freq: *const i32, r: i32, i: *const i32) -> i32;

    /// int32_t libsais_plcp(const uint8_t * T, const int32_t * SA, int32_t * PLCP, int32_t n);
    pub fn libsais_plcp(t: *const u8, sa: *const i32, plcp: *mut i32, n: i32) -> i32;

    /// int32_t libsais_lcp(const int32_t * PLCP, const int32_t * SA, int32_t * LCP, int32_t n);
    pub fn libsais_lcp(plcp: *const i32, sa: *const i32, lcp: *mut i32, n: i32) -> i32;
}

#[cfg(feature = "openmp")]
extern "C" {
    /// void * libsais_create_ctx_omp(int32_t threads);
    pub fn libsais_create_ctx_omp(threads: i32) -> *mut c_void;

    /// int32_t libsais_omp(const uint8_t * T, int32_t * SA, int32_t n, int32_t fs, int32_t * freq, int32_t threads);
    pub fn libsais_omp(t: *const u8, sa: *mut i32, n: i32, fs: i32, freq: *mut i32, threads: i32) -> i32;

    /// int32_t libsais_int_omp(int32_t * T, int32_t * SA, int32_t n, int32_t k, int32_t fs, int32_t threads);
    pub fn libsais_int_omp(t: *mut i32, sa: *mut i32, n: i32, k: i32, fs: i32, threads: i32) -> i32;

    /// int32_t libsais_bwt_omp(const uint8_t * T, uint8_t * U, int32_t * A, int32_t n, int32_t fs, int32_t * freq, int32_t threads);
    pub fn libsais_bwt_omp(t: *const u8, u: *mut u8, a: *mut i32, n: i32, fs: i32, freq: *mut i32, threads: i32) -> i32;

    /// int32_t libsais_bwt_aux_omp(const uint8_t * T, uint8_t * U, int32_t * A, int32_t n, int32_t fs, int32_t * freq, int32_t r, int32_t * I, int32_t threads);
    pub fn libsais_bwt_aux_omp(t: *const u8, u: *mut u8, a: *mut i32, n: i32, fs: i32, freq: *mut i32, r: i32, i: *mut i32, threads: i32) -> i32;

    /// void * libsais_unbwt_create_ctx_omp(int32_t threads);
    pub fn libsais_unbwt_create_ctx_omp(threads: i32) -> *mut c_void;

    /// int32_t libsais_unbwt_omp(const uint8_t * T, uint8_t * U, int32_t * A, int32_t n, const int32_t * freq, int32_t i, int32_t threads);
    pub fn libsais_unbwt_omp(t: *const u8, u: *mut u8, a: *mut i32, n: i32, freq: *const i32, i: i32, threads: i32) -> i32;

    /// int32_t libsais_unbwt_aux_omp(const uint8_t * T, uint8_t * U, int32_t * A, int32_t n, const int32_t * freq, int32_t r, const int32_t * I, int32_t threads);
    pub fn libsais_unbwt_aux_omp(t: *const u8, u: *mut u8, a: *mut i32, n: i32, freq: *const i32, r: i32, i: *const i32, threads: i32) -> i32;

    /// int32_t libsais_plcp_omp(const uint8_t * T, const int32_t * SA, int32_t * PLCP, int32_t n, int32_t threads);
    pub fn libsais_plcp_omp(t: *const u8, sa: *const i32, plcp: *mut i32, n: i32, threads: i32) -> i32;

    /// int32_t libsais_lcp_omp(const int32_t * PLCP, const int32_t * SA, int32_t * LCP, int32_t n, int32_t threads);
    pub fn libsais_lcp_omp(plcp: *const i32, sa: *const i32, lcp: *mut i32, n: i32, threads: i32) -> i32;
}
