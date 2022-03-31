//! Internals needed by macros. These have to be exported for the macros to work
pub use crate::context::internal::{initialize_module, Env};

// An alias for neon_runtime so macros can refer to it.
pub mod runtime {
    pub use neon_runtime::*;
}
