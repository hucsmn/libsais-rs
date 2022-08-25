use libc::c_void;

extern "C" {
    /// void * libsais16_create_ctx(void);
    fn libsais16_create_ctx() -> *mut c_void;

    /// void libsais16_free_ctx(void * ctx);
    fn libsais16_free_ctx(ctx: *mut c_void);

    /// int32_t libsais16(const uint16_t * T, int32_t * SA, int32_t n, int32_t fs, int32_t * freq);
    fn libsais16(t: *const u16, sa: *mut i32, n: i32, fs: i32, freq: *const i32) -> i32;

    /// int32_t libsais16_ctx(const void * ctx, const uint16_t * T, int32_t * SA, int32_t n, int32_t fs, int32_t * freq);
    fn libsais16_ctx(ctx: *mut c_void, t: *const u16, sa: *mut i32, n: i32, fs: i32, freq: *const i32) -> i32;

    /// int32_t libsais16_bwt(const uint16_t * T, uint16_t * U, int32_t * A, int32_t n, int32_t fs, int32_t * freq);
    fn libsais16_bwt(t: *const u16, u: *mut u16, a: *mut i32, n: i32, fs: i32, freq: *const i32) -> i32;

    /// int32_t libsais16_bwt_aux(const uint16_t * T, uint16_t * U, int32_t * A, int32_t n, int32_t fs, int32_t * freq, int32_t r, int32_t * I);
    fn libsais16_bwt_aux(t: *const u16, u: *mut u16, a: *mut i32, n: i32, fs: i32, freq: *const i32, r: i32, i: *mut i32) -> i32;

    /// int32_t libsais16_bwt_ctx(const void * ctx, const uint16_t * T, uint16_t * U, int32_t * A, int32_t n, int32_t fs, int32_t * freq);
    fn libsais16_bwt_ctx(ctx: *mut c_void, t: *const u16, u: *mut u16, a: *mut i32, n: i32, fs: i32, freq: *const i32) -> i32;

    /// int32_t libsais16_bwt_aux_ctx(const void * ctx, const uint16_t * T, uint16_t * U, int32_t * A, int32_t n, int32_t fs, int32_t * freq, int32_t r, int32_t * I);
    fn libsais16_bwt_aux_ctx(ctx: *mut c_void, t: *const u16, u: *mut u16, a: *mut i32, n: i32, fs: i32, freq: *const i32, r: i32, i: *mut i32) -> i32;

    /// void * libsais16_unbwt_create_ctx(void);
    fn libsais16_unbwt_create_ctx() -> *mut c_void;

    /// void libsais16_unbwt_free_ctx(void * ctx);
    fn libsais16_unbwt_free_ctx(ctx: *mut c_void);

    /// int32_t libsais16_unbwt(const uint16_t * T, uint16_t * U, int32_t * A, int32_t n, const int32_t * freq, int32_t i);
    fn libsais16_unbwt(t: *const u16, u: *mut u16, a: *mut i32, n: i32, freq: *const i32, i: i32) -> i32;

    /// int32_t libsais16_unbwt_ctx(const void * ctx, const uint16_t * T, uint16_t * U, int32_t * A, int32_t n, const int32_t * freq, int32_t i);
    fn libsais16_unbwt_ctx(ctx: *mut c_void, t: *const u16, u: *mut u16, a: *mut i32, n: i32, freq: *const i32, i: i32) -> i32;

    /// int32_t libsais16_unbwt_aux(const uint16_t * T, uint16_t * U, int32_t * A, int32_t n, const int32_t * freq, int32_t r, const int32_t * I);
    fn libsais16_unbwt_aux(t: *const u16, u: *mut u16, a: *mut i32, n: i32, freq: *const i32, r: i32, i: *mut i32) -> i32;

    /// int32_t libsais16_unbwt_aux_ctx(const void * ctx, const uint16_t * T, uint16_t * U, int32_t * A, int32_t n, const int32_t * freq, int32_t r, const int32_t * I);
    fn libsais16_unbwt_aux_ctx(ctx: *mut c_void, t: *const u16, u: *mut u16, a: *mut i32, n: i32, freq: *const i32, r: i32, i: *mut i32) -> i32;

    /// int32_t libsais16_plcp(const uint16_t * T, const int32_t * SA, int32_t * PLCP, int32_t n);
    fn libsais16_plcp(t: *const u16, sa: *const i32, plcp: *mut i32, n: i32) -> i32;

    /// int32_t libsais16_lcp(const int32_t * PLCP, const int32_t * SA, int32_t * LCP, int32_t n);
    fn libsais16_lcp(plcp: *const u16, sa: *const i32, lcp: *mut i32, n: i32) -> i32;
}

#[cfg(feature = "openmp")]
extern "C" {
    /// void * libsais16_create_ctx_omp(int32_t threads);
    fn libsais16_create_ctx_omp(threads: i32) -> *mut c_void;

    /// int32_t libsais16_omp(const uint16_t * T, int32_t * SA, int32_t n, int32_t fs, int32_t * freq, int32_t threads);
    fn libsais16_omp(t: *const u16, sa: *mut i32, n: i32, fs: i32, freq: *const i32, threads: i32) -> i32;

    /// int32_t libsais16_bwt_omp(const uint16_t * T, uint16_t * U, int32_t * A, int32_t n, int32_t fs, int32_t * freq, int32_t threads);
    fn libsais16_bwt_omp(t: *const u16, u: *mut u16, a: *mut i32, n: i32, fs: i32, freq: *const i32, threads: i32) -> i32;

    /// int32_t libsais16_bwt_aux_omp(const uint16_t * T, uint16_t * U, int32_t * A, int32_t n, int32_t fs, int32_t * freq, int32_t r, int32_t * I, int32_t threads);
    fn libsais16_bwt_aux_omp(t: *const u16, u: *mut u16, a: *mut i32, n: i32, fs: i32, freq: *const i32, r: i32, i: *mut i32, threads: i32) -> i32;

    /// void * libsais16_unbwt_create_ctx_omp(int32_t threads);
    fn libsais16_unbwt_create_ctx_omp(threads: i32) -> *mut c_void;

    /// int32_t libsais16_unbwt_omp(const uint16_t * T, uint16_t * U, int32_t * A, int32_t n, const int32_t * freq, int32_t i, int32_t threads);
    fn libsais16_unbwt_omp(t: *const u16, u: *mut u16, a: *mut i32, n: i32, freq: *const i32, i: i32, threads: i32) -> i32;

    /// int32_t libsais16_unbwt_aux_omp(const uint16_t * T, uint16_t * U, int32_t * A, int32_t n, const int32_t * freq, int32_t r, const int32_t * I, int32_t threads);
    fn libsais16_unbwt_aux_omp(t: *const u16, u: *mut u16, a: *mut i32, n: i32, freq: *const i32, r: i32, i: *mut i32, threads: i32) -> i32;

    /// int32_t libsais16_plcp_omp(const uint16_t * T, const int32_t * SA, int32_t * PLCP, int32_t n, int32_t threads);
    fn libsais16_plcp_omp(t: *const u16, sa: *const i32, plcp: *mut i32, n: i32, threads: i32) -> i32;

    /// int32_t libsais16_lcp_omp(const int32_t * PLCP, const int32_t * SA, int32_t * LCP, int32_t n, int32_t threads);
    fn libsais16_lcp_omp(plcp: *const u16, sa: *const i32, lcp: *mut i32, n: i32, threads: i32) -> i32;
}
