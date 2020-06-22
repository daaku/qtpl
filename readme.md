qtpl
====

[![Rust](https://github.com/daaku/qtpl/workflows/Rust/badge.svg)](https://github.com/daaku/qtpl/actions?query=workflow%3ARust)
[![Crates.io](https://img.shields.io/crates/v/qtpl)](https://crates.io/crates/qtpl)
[![Documentation](https://docs.rs/qtpl/badge.svg)](https://docs.rs/qtpl)

Templates in your Rust code.

**NOTE**: This currently works on beta. It will work on stable with Rust 1.45.

This library allows you to write templates, using macros, mixed in with your
Rust code. This allows you to use normal Rust code for the logic, and embed
template code along side it.

See the [documentation](https://docs.rs/qtpl) for how to use this library.
Here's a taste of how it looks:

```rust
use qtpl::{child, render, tpl, tplfn, Render};

#[tplfn]
fn page<B: Render, F: Render>(body: B, footer: F) {
    tpl! {
        <!doctype html>
        <body>
            {!c body}
            <footer>{!c footer}</footer>
        </body>
    }
}

#[tplfn]
fn body(name: String) {
    tpl! {Hello, {&name}!}
}

#[tplfn]
fn footer(company: &str) {
    tpl! {Copyright {company}}
}

#[tplfn]
fn home(name: String, company: &str) {
    let b = child!(body(name));
    let f = child!(footer(company));
    tpl! {
        {!t page(b, f)}
    }
}

let name = String::from("world");
let company = "bigcorp";
let out = render!(home(name, company)).unwrap();

let result = String::from_utf8(out).unwrap();
assert_eq!(
    String::from_utf8(out).unwrap(),
    concat!(
        "<!doctype html>",
        "<body>",
        "Hello, world!",
        "<footer>Copyright bigcorp</footer>",
        "</body>",
    )
);
```

TODO
====

- [ ] Make whitespace handling fully HTML aware and automatically correct
- [ ] Make default escaping fully HTML aware and automatically correct: content, attributes, JS, CSS, etc
- [ ] Support automatic JSON inside <script>
- [ ] Document pattern for passing children to tplfn
- [ ] `child!` should support inline `tpl!` style
- [ ] Support more formatting directives
- [ ] Support methods in addition to functions in `tplfn`
- [ ] Support returning errors (ideally unopinionated)
- [ ] Support `async`/`await` functions (needs anything special?)
- [ ] Support blocks inside string literals (maybe?)
- [ ] Add documentation
- [ ] Monitor test coverage in GitHub actions?
