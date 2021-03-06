//! Templates in your Rust code.
//!
//! This library allows you to write HTML templates, using the power of macros,
//! mixed in with your Rust code. This allows you to use normal Rust code for
//! the logic, and embed template code along side it.
//!
//! **Contents:**
//!
//! 1. [Basics](#basics)
//! 1. [Rendering](#rendering)
//! 1. [Escaping](#escaping)
//! 1. [Returning Errors](#returning-errors)
//! 1. [Whitespace](#whitespace)
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
//!     tpl! {Hello, <strong>{name}</strong>!}
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
//! socket. Usually you'll buffer the content entirely, or use a buffered
//! writer at the least.
//!
//! ## To a `File`
//! ```
//! # use qtpl::{tplfn, tpl};
//! #
//! # #[tplfn]
//! # fn hello(name: &str) {
//! #     tpl! {Hello, <strong>{name}</strong>!}
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
//! #     tpl! {Hello, <strong>{name}</strong>!}
//! # }
//! #
//! let mut out = vec![];
//! hello(&mut out, "world")?;
//! assert_eq!(out, b"Hello, <strong>world</strong>!");
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
//! #     tpl! {Hello, <strong>{name}</strong>!}
//! # }
//! #
//! assert_eq!(render_string!(hello("world")), "Hello, <strong>world</strong>!");
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
//! #     tpl! {Hello, <strong>{name}</strong>!}
//! # }
//! #
//! assert_eq!(render_string!(hello("<world>")), "Hello, <strong>&lt;world&gt;</strong>!");
//! #
//! # Ok::<(), std::io::Error>(())
//! ```
//!
//! # Returning Errors
//!
//! The `#[tplfn]` attribute will add a return type of `std::io::Result<()>`,
//! but only if one isn't present. You can customize the return type and take
//! control of the errors being returned. The only requirement is that
//! `std::io::Error` types can be converted into that error using the usual
//! process.
//!
//! A contrived but real example:
//!
//! ```
//! # use qtpl::{tplfn, tpl, render_string};
//! #
//! type BoxError = Box<dyn std::error::Error + Send + Sync>;
//!
//! #[tplfn]
//! fn answer(a: &str) -> Result<(), BoxError> {
//!     let a: i8 = a.parse()?;
//!     tpl! {{&a.to_string()}}
//! }
//!
//! assert_eq!(render_string!(answer("42")), "42");
//!
//! let mut w = vec![];
//! match answer(&mut w, "not a number") {
//!     Result::Err(err) => {
//!         assert_eq!(format!("{}", err), "invalid digit found in string");
//!     },
//!     _ => panic!("expected an error"),
//! };
//! ```
//!
//! # Whitespace
//!
//! The library takes an opinionated stance on whitespace. The rules are as
//! follows:
//!
//! * Whitespace at the begining of the template is stripped.
//! * Whitespace at the end of the template is stripped.
//! * Whitespace around a whitelisted set of elements, where it should be
//!   insignificant is stripped.
//! * All whitespace, including newlines is collapsed into a single space.
//! * Rules only apply to template text, contents of varibles are not modified.
//!
//! This example shows all the rules in action, including how certain tags
//! behave differently, and how multiple spaces including newlines are
//! collapsed:
//!
//! ```
//! # use qtpl::{tplfn, tpl, render_string};
//! #
//! #[tplfn]
//! fn home() {
//!     tpl! {
//!         <div>
//!             <a>Go <i class="icon">   </i></a>
//!             <a>{"   "}</a>
//!         </div>
//!     }
//! }
//!
//! assert_eq!(
//!     render_string!(home()),
//!     concat!(
//!         "<div>",
//!         r#"<a>Go <i class="icon"> </i></a>"#,
//!         " ",
//!         "<a>   </a>",
//!         "</div>",
//!     ),
//! );
//! ```
//!
//! Note how the space inside and around the `<i>` tag is preserved, but the
//! space around the `<div>` tag is stripped. Also notice how the multiple
//! spaces inside the `<i>` are collapsed into a single space.

#![doc(html_favicon_url = "https://raw.githubusercontent.com/daaku/qtpl/master/assets/favicon.png")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/daaku/qtpl/master/assets/logo.png")]

pub use qtpl_macros::{render, render_string, tpl, tplfn};

// This is used internally for escaping in macro output.
#[doc(hidden)]
pub use v_htmlescape::escape;
