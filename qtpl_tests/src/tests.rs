use pretty_assertions::assert_eq;
use qtpl::{child, render, tpl, tplfn};

#[test]
fn plain_text() {
    #[tplfn]
    fn hello() {
        tpl! { <a>Hello, world!</a> }
    }

    assert_eq!(render!(hello()).unwrap(), b"<a>Hello, world!</a>");
}

#[test]
fn block_with_expr() {
    #[tplfn]
    fn hello(name: &str) {
        tpl! { <a>Hello, {name}!</a> }
    }

    assert_eq!(render!(hello("world")).unwrap(), b"<a>Hello, world!</a>");
}

#[test]
fn spaces_around_block() {
    #[tplfn]
    fn hello(name: &str) {
        tpl! { <a>Hello, {name} - welcome!</a> }
    }

    assert_eq!(
        render!(hello("world")).unwrap(),
        b"<a>Hello, world - welcome!</a>"
    );
}

#[test]
fn no_spaces_around_block() {
    #[tplfn]
    fn hello(name: &str) {
        tpl! { <a>Hello:{name}!</a> }
    }

    assert_eq!(render!(hello("world")).unwrap(), b"<a>Hello:world!</a>");
}

#[test]
fn attrs() {
    #[tplfn]
    fn hello(class: &str) {
        tpl! { <a class={!q class}>Hello!</a> }
    }

    assert_eq!(
        render!(hello("world")).unwrap(),
        b"<a class=\"world\">Hello!</a>"
    );
}

#[test]
fn format_bytes() {
    #[tplfn]
    fn hello(name: &[u8]) {
        tpl! { <a>Hello, {!b name}!</a> }
    }

    assert_eq!(render!(hello(b"world")).unwrap(), b"<a>Hello, world!</a>");
}

#[test]
fn child_elements() {
    trait Render {
        fn render(self, destination: &mut dyn ::std::io::Write) -> ::std::io::Result<()>;
    }

    impl<F> Render for F
    where
        F: FnOnce(&mut dyn ::std::io::Write) -> ::std::io::Result<()>,
    {
        fn render(self, destination: &mut dyn ::std::io::Write) -> ::std::io::Result<()> {
            self(destination)
        }
    }

    #[tplfn]
    fn page<B: Render, F: Render>(body: B, footer: F) {
        tpl! {
            <body>{!c body}</body>
            <footer>{!c footer}</footer>
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
    let result = String::from_utf8(render!(home(name, company)).unwrap()).unwrap();
    assert_eq!(
        result,
        "<body>Hello, world!</body> <footer>Copyright bigcorp</footer>"
    );
}
