// Rust Monero Library
// Written in 2019 by
//   h4sh3d <h4sh3d@protonmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//

//! Utility functions
//!
//! Shared functions needed in different part of the library.
//!

pub mod address;
pub mod key;
pub mod ringct;

use super::network;
use crate::blockdata::transaction;

use thiserror::Error;

/// A general error code, other errors should implement conversions to/from this
/// if appropriate.
#[derive(Error, Debug, PartialEq)]
pub enum Error {
    /// Monero network error
    #[error("Network error: {0}")]
    Network(#[from] network::Error),
    /// Monero address error
    #[error("Address error: {0}")]
    Address(#[from] address::Error),
    /// Monero key error
    #[error("Key error: {0}")]
    Key(#[from] key::Error),
    /// Monero RingCT error
    #[error("RingCT error: {0}")]
    RingCT(#[from] ringct::Error),
    /// Monero transaction error
    #[error("Transaction error: {0}")]
    Transaction(#[from] transaction::Error),
}
