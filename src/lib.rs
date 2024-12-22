#![doc(
    html_logo_url = "https://github.com/opensass/accordion-rs/blob/main/assets/logo.webp",
    html_favicon_url = "https://github.com/opensass/accordion-rs/blob/main/assets/favicon.png"
)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

pub mod common;

#[cfg(feature = "yew")]
pub mod yew;

pub use common::{Align, Size};