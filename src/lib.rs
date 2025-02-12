#![feature(map_first_last)]
#![feature(entry_insert)]
#![feature(bound_map)]
#![feature(once_cell)]
#![feature(associated_type_defaults)]
#![feature(trivial_bounds)]
#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(generic_associated_types)]
#![feature(const_generics_defaults)]
#![feature(try_trait_v2)]
#![feature(never_type)]

extern crate self as orga;

/// Integration with ABCI (gated by `abci` feature).
#[cfg(feature = "abci")]
pub mod abci;

pub mod call;

pub mod client;

/// Data structures which implement the [`state::State`](state/trait.State.html)
/// trait.
pub mod collections;

/// Traits for deterministic encoding and decoding.
///
/// This module is actually just a re-export of the [ed](https://docs.rs/ed)
/// crate.
pub mod encoding;

/// Integration with [merk](https://docs.rs/merk) (gated by `merk` feature).
#[cfg(feature = "merk")]
pub mod merk;

pub mod query;

/// High-level abstractions for state data.
pub mod state;

/// Helpers for executing state machine logic.
pub mod state_machine;

/// Low-level key/value store abstraction.
pub mod store;

/// Tendermint process handler.
pub mod tendermint;

pub mod plugins;

#[cfg(feature = "abci")]
pub mod coins;

pub mod context;

mod error;

// re-exports
pub use async_trait::async_trait;
pub use error::*;
pub use futures_lite::future::Boxed as BoxFuture;
pub use orga_macros as macros;

pub mod prelude {
    #[cfg(feature = "abci")]
    pub use crate::abci::*;
    pub use crate::call::*;
    pub use crate::client::{AsyncCall, Client};
    #[cfg(feature = "abci")]
    pub use crate::coins::*;
    pub use crate::collections::*;
    pub use crate::context::*;
    pub use crate::encoding::*;
    pub use crate::plugins::*;
    pub use crate::query::*;
    pub use crate::state::*;
    pub use crate::store::*;
    pub use crate::Result;
}
