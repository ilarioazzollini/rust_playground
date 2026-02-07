//! The `///` syntax is used to document the item present after it.
//! That's why it is called an outer documentation. There is another syntax: `//!`,
//! which is used to document the item it is present inside. It is called an inner documentation.
//! It is often used when documenting the entire crate, because nothing comes before it:
//! it is the root of the crate. So in order to document an entire crate, you need to use //! syntax.
//!
//! 📘 For guides and design docs, see the
//! [project book](../book/index.html).

/// A utilities module.
///
/// See the [crate-level documentation] for details.
///
///   [crate-level documentation]: ../index.html
pub mod utilities;
