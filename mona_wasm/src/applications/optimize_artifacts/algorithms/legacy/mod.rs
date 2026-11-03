//! Frozen optimizer baseline from e9543ad10f3eb41298781e2c939e429ca96ef1fa.
//!
//! These six files preserve the original implementations, including known bugs.
//! Only internal module paths were relocated here. Do not silently fix this
//! baseline; put corrected strategies alongside it and compare their outputs.
//! Enable `optimizer-legacy` to compile. Production builds exclude it by default.
pub mod common;
pub mod cutoff_a_star;
pub mod cutoff_algo2;
pub mod cutoff_heu_plus_a_star;
pub mod cutoff_heuristic;
pub mod weight_heuristic;
