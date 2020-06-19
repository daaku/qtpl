use pretty_assertions::assert_eq;
use qtpl::{html, tpl, tplfn};

#[test]
fn plain_text() {
    #[tplfn]
    fn hello() {
        tpl! { <a>Hello, world!</a> }
    }

    assert_eq!(html!(hello()).unwrap(), b"<a>Hello, world!</a>");
}

#[test]
fn block_with_expr() {
    #[tplfn]
    fn hello(name: &str) {
        tpl! { <a>Hello, {name}!</a> }
    }

    assert_eq!(html!(hello("world")).unwrap(), b"<a>Hello, world!</a>");
}

#[test]
fn spaces_around_block() {
    #[tplfn]
    fn hello(name: &str) {
        tpl! { <a>Hello, {name} - welcome!</a> }
    }

    assert_eq!(
        html!(hello("world")).unwrap(),
        b"<a>Hello, world - welcome!</a>"
    );
}

#[test]
fn no_spaces_around_block() {
    #[tplfn]
    fn hello(name: &str) {
        tpl! { <a>Hello:{name}!</a> }
    }

    assert_eq!(html!(hello("world")).unwrap(), b"<a>Hello:world!</a>");
}

#[test]
fn attrs() {
    #[tplfn]
    fn hello(class: &str) {
        tpl! { <a class={!q class}>Hello!</a> }
    }

    assert_eq!(
        html!(hello("world")).unwrap(),
        b"<a class=\"world\">Hello!</a>"
    );
}

#[test]
fn format_bytes() {
    #[tplfn]
    fn hello(name: &[u8]) {
        tpl! { <a>Hello, {!b name}!</a> }
    }

    assert_eq!(
        html!(hello(b"world")).unwrap(),
        b"<a>Hello, world!</a>"
    );
}
