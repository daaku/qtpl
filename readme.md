qtpl
====

Templates in your Rust code.

At a high level, this crate provides a nicer way of embedding textual templates
in your Rust code. This allows you to use normal Rust code for the logic, and
embed template code along side it. The templates are writing to
`std::io::Write` behind the scenes using direct `write` calls with bytes from
literal text, or `write!` when using dynamic content.


TODO
====

- [ ] Support more formatting directives
- [ ] Support methods in addition to functions in tplfn
- [ ] Support returning errors (ideally unopinionated)
- [ ] Support async/await functions (needs anything special?)
- [ ] Support blocks inside string literals (maybe?)
- [ ] Add documentation
- [ ] Monitor test coverage in GitHub actions?
