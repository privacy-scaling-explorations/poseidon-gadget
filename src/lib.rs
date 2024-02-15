//! This crate provides the poseidon gadget for use with `halo2_proofs`.
//! This gadget has been stracted from zcash's halo2_gadgets:
//! https://github.com/zcash/halo2/tree/main/halo2_gadgets

#![cfg_attr(docsrs, feature(doc_cfg))]
// Temporary until we have more of the crate implemented.
#![allow(dead_code)]
// Catch documentation errors caused by code changes.
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

pub mod poseidon;
pub mod utilities;
