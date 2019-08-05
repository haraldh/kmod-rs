//! Bindings to libkmod to manage linux kernel modules.
//!
//! # Example
//! ```
//! fn main() {
//!     // create a new kmod context
//!     let ctx = kmod::Context::new().unwrap();
//!
//!     // get a kmod_list of all loaded modules
//!     for module in ctx.modules_loaded().unwrap() {
//!         let name = module.name();
//!         let refcount = module.refcount();
//!         let size = module.size();
//!
//!         let holders: Vec<_> = module.holders()
//!             .map(|x| x.name().to_owned())
//!             .collect();
//!
//!         println!("{:<19} {:8}  {} {:?}", name, size, refcount, holders);
//!     }
//! }
//! ```
#![deny(
    warnings,
    absolute_paths_not_starting_with_crate,
    deprecated_in_future,
    keyword_idents,
    macro_use_extern_crate,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    unused_labels,
    unused_lifetimes,
    unreachable_pub,
    future_incompatible,
    missing_doc_code_examples,
    rust_2018_idioms,
    rust_2018_compatibility
)]

pub use ctx::*;
pub use errors::{Error, ErrorKind, Result};
pub use modules::*;
pub use errno::Errno;

mod errors {
    use std;

    use errno::Errno;

    use error_chain::error_chain;

    error_chain! {
        errors {
            Errno(err: Errno) {
                description("got error")
                display("{}", err)
            }
        }
        foreign_links {
            NulError(std::ffi::NulError);
        }
    }
}

mod ctx;
mod modules;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lsmod() {
        let ctx = Context::new().unwrap();

        for module in ctx.modules_loaded().unwrap() {
            let name = module.name();
            let refcount = module.refcount();
            let size = module.size();

            let holders: Vec<_> = module.holders()
                                    .map(|x| x.name().to_owned())
                                    .collect();

            println!("{:<19} {:8}  {} {:?}", name, size, refcount, holders);
        }
    }
}
