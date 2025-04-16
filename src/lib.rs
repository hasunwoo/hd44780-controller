#![no_std]
#![deny(unsafe_code)]
#![allow(async_fn_in_trait)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod charset;
pub mod command;
pub mod controller;
pub mod device;
