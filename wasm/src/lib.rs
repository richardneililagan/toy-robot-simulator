// :: Allow dead code and unused imports in unoptimized builds (e.g. dev environment),
//    but will still warn / fail in release builds.
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

pub mod components;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
