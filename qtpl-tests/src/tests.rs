use pretty_assertions::assert_eq;
use qtpl::{child, render_string, tpl, tplfn};

#[test]
fn plain_text() {
    #[tplfn]
    fn hello() {
        tpl! { <a>Hello, world!</a> }
    }

    assert_eq!(render_string!(hello()), "<a>Hello, world!</a>");
}

#[test]
fn block_with_expr() {
    #[tplfn]
    fn hello(name: &str) {
        tpl! { <a>Hello, {name}!</a> }
    }

    assert_eq!(render_string!(hello("world")), "<a>Hello, world!</a>");
}

#[test]
fn spaces_around_block() {
    #[tplfn]
    fn hello(name: &str) {
        tpl! { <a>Hello, {name} - welcome!</a> }
    }

    assert_eq!(
        render_string!(hello("world")),
        "<a>Hello, world - welcome!</a>"
    );
}

#[test]
fn no_spaces_around_block() {
    #[tplfn]
    fn hello(name: &str) {
        tpl! { <a>Hello:{name}!</a> }
    }

    assert_eq!(render_string!(hello("world")), "<a>Hello:world!</a>");
}

#[test]
fn attrs() {
    #[tplfn]
    fn hello(class: &str) {
        tpl! { <a class={!q class}>Hello!</a> }
    }

    assert_eq!(
        render_string!(hello("world")),
        "<a class=\"world\">Hello!</a>"
    );
}

#[test]
fn format_bytes() {
    #[tplfn]
    fn hello(name: &[u8]) {
        tpl! { <a>Hello, {!b name}!</a> }
    }

    assert_eq!(render_string!(hello(b"world")), "<a>Hello, world!</a>");
}

#[test]
fn readme_example() {
    use qtpl::{child, render_string, tpl, tplfn, Render};

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

    assert_eq!(
        render_string!(home(name, company)),
        concat!(
            "<!doctype html> ",
            "<body> ",
            "Hello, world! ",
            "<footer>Copyright bigcorp</footer> ",
            "</body>",
        )
    );
}
