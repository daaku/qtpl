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

    assert_eq!(html!(hello(b"world")).unwrap(), b"<a>Hello, world!</a>");
}

#[test]
fn child_elements() {
    #[tplfn]
    fn page(body: &[u8], footer: &[u8]) {
        tpl! {
            <body>{!b body}</body>
            <footer>{!b footer}</footer>
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
        let b = html!(body(name))?;
        let f = html!(footer(company))?;
        tpl! {
            {!c page(&b, &f)}
        }
    }

    let name = String::from("world");
    let company = "bigcorp";
    let result = String::from_utf8(html!(home(name, company)).unwrap()).unwrap();
    assert_eq!(
        result,
        "<body>Hello, world!</body> <footer>Copyright bigcorp</footer>"
    );
}
