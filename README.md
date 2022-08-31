libsais-rs
==========

Rust bindings to [libsais](https://github.com/IlyaGrebnov/libsais), a library for linear time suffix array,
longest common prefix array and burrows wheeler transform construction
based on induced sorting algorithm.

status: **work in progress**

todo:

* [ ] write documents
* [ ] input size limit check
* [ ] more tests on edge cases, e.g. `sais32::sais` text ~ 2 GiB
* [ ] build script tweaks, based on benchmark
* [ ] more platforms, e.g. aarch64, macos, cross compile (?)
