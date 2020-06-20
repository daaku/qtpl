//! Templates in your Rust code.
//!
//! This library allows you to write templates, using macros, mixed in with your
//! Rust code. This allows you to use normal Rust code for the logic, and embed
//! template code along side it.
//!
//! **Caveat**: This library is going to be mostly useful for HTML, primarily
//! because it collapses whitespace. It not that it is tied to HTML in anyway
//! though. This whitespace management is an artificat of how Rust macros work.
//! One can inject newlines manually, but it will feel cumbersome. The rule is
//! that all whitespace, including newlines are collapsed into a single space.
//!
//! ### Basics
//!
//! The most basic template is this:
//!
//! ```
//! use qtpl::{tplfn, tpl};
//!
//! #[tplfn]
//! fn hello(name: &str) {
//!     tpl! {Hello, {name}!}
//! }
//! ```
//!
//! There are a few things going on here -- first, we're adding the `#[tlpfn]`
//! attribute to our template function. This makes it possible to use this
//! function as a template. Second, we're using the `tpl!` macro inside the body
//! and embedding some textual content. Lastly, we're putting the variable
//! `name` inside another block.
//!
//! ### Rendering
//!
//! Fundamentally rendering happens to something that implements
//! `std::io::Write`. This means you could potentially write directly to a
//! socket. Usually you'll buffer the content entirely, or use a buffered socket
//! at the least.
//!
//! For example, if you wanted to write the above template to a file:
//!
//! ```
//! # use qtpl::{tplfn, tpl};
//! #
//! # #[tplfn]
//! # fn hello(name: &str) {
//! #     tpl! {Hello, {name}!}
//! # }
//! #
//! let mut file = std::fs::File::create("/tmp/qtpl.txt")?;
//! hello(&mut file, "world")?;
//! # Ok::<(), std::io::Error>(())
//! ```

pub use qtpl_macros::{child, render, tpl, tplfn};
use std::io::{Result, Write};

pub trait Render {
    fn render(self, destination: &mut dyn Write) -> Result<()>;
}

impl<F> Render for F
where
    F: FnOnce(&mut dyn Write) -> Result<()>,
{
    fn render(self, destination: &mut dyn Write) -> Result<()> {
        self(destination)
    }
}
