#![feature(specialization)]

pub mod mutator;
pub mod optimistic;
mod runtime_config;

pub use mutagen_transform::mutate;

pub use runtime_config::MutagenRuntimeConfig;
