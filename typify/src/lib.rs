// Copyright 2021 Oxide Computer Company

//! Typify lets you convert JSON Schema documents into Rust types. It can be
//! used via a macro [`import_types!`] or a `build.rs` file.
//!
//! A typical use looks like this:
//! ```
//! # use typify_macro::import_types;
//! # use serde::{Deserialize,Serialize};
//! import_types!("../example.json");
//! ```
//!
//! This expands to type definitions corresponding to the types from the file
//! `example.json`. The types are `pub` and have a number of useful associated
//! `impl`s including [Debug], [Clone],
//! [Serialize](https://docs.rs/serde/latest/serde/trait.Serialize.html), and
//! [Deserialize](https://docs.rs/serde/latest/serde/trait.Deserialize.html).
//!
//! #### Macro vs. `build.rs`
//!
//! While using the [`import_types!`] macro is quite a bit simpler, you can
//! also construct output in a `build.rs` script. Doing so requires a little
//! more work do process the JSON Schema document and write out the file to
//! your intended location. The significant benefit is that the generated type
//! definitions are significantly easier to inspect. The macro-generated types
//! can be viewed with `cargo expand` and they (like `build.rs`-derived types)
//! have generated documentation, but if you find that you'd like to see the
//! actual code generated you may prefer a `build.rs`.
//!
//! ## Builder interface
//!
//! Typify exports a [TypeSpace] interface that is intended for programmatic
//! construction of types. This can be for something simple like a `build.rs`
//! script or something more complex like a generated that includes types as
//! part of its definition.
//!
//! # Mapping JSON Schema to Rust
//!
//! JSON Schema allows for extreme flexibility. As such, there are some schemas
//! that Typify isn't able to interpret (please file an issue!). In general,
//! though, Typify does a pretty job of mapping JSON Schema types to Rust. For
//! more information, see the project's
//! [README.md](https://github.com/oxidecomputer/typify).

pub use typify_impl::Error;
pub use typify_impl::Type;
pub use typify_impl::TypeSpace;
pub use typify_macro::import_types;
