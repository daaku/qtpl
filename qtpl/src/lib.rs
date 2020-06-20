//! Templates in your Rust code.
//!
//! This library allows you to write templates, using macros, mixed in with your
//! Rust code. This allows you to use normal Rust code for the logic, and embed
//! template code along side it.
//!
//! **Caveat**: This library is going to be mostly useful for HTML, primarily
//! because it collapses whitespace (whitespace, including newlines are
//! collapsed into a single space). It also defaults to HTML escaping, though
//! that can be altered by using directives.
//!
//! # Basics
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
//! # Rendering
//!
//! Fundamentally rendering happens to something that implements
//! `std::io::Write`. This means you could potentially write directly to a
//! socket. Usually you'll buffer the content entirely, or use a buffered socket
//! at the least.
//!
//! ## To a `File`
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
//! #
//! # Ok::<(), std::io::Error>(())
//! ```
//!
//! ## To a `Vec<u8>`
//! ```
//! # use qtpl::{tplfn, tpl};
//! #
//! # #[tplfn]
//! # fn hello(name: &str) {
//! #     tpl! {Hello, {name}!}
//! # }
//! #
//! let mut out = vec![];
//! hello(&mut out, "world")?;
//! assert_eq!(out, b"Hello, world!");
//! #
//! # Ok::<(), std::io::Error>(())
//! ```
//!
//! ## To a `String` for Testing
//! Purely as a convinience, a `render_string!` macro is provided which
//! panics on errors, and returns a `String`. Remember, this is useful for
//! testing and documentation, but you shouldn't be using this in production
//! code, because it involves unnecessary copies and conversions.
//!
//! ```
//! # use qtpl::{tplfn, tpl, render_string};
//! #
//! # #[tplfn]
//! # fn hello(name: &str) {
//! #     tpl! {Hello, {name}!}
//! # }
//! #
//! assert_eq!(render_string!(hello("world")), "Hello, world!");
//! #
//! # Ok::<(), std::io::Error>(())
//! ```
//!
//! # Escaping
//!
//! The default escaping used by the library is geared towards HTML. Using the
//! same example above:
//!
//! ```
//! # use qtpl::{tplfn, tpl, render_string};
//! #
//! # #[tplfn]
//! # fn hello(name: &str) {
//! #     tpl! {Hello, {name}!}
//! # }
//! #
//! assert_eq!(render_string!(hello("<world>")), "Hello, &lt;world&gt;!");
//! #
//! # Ok::<(), std::io::Error>(())
//! ```

pub use qtpl_macros::{child, render_string, tpl, tplfn};
use std::io::{Result, Write};
pub use v_htmlescape::escape;

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
