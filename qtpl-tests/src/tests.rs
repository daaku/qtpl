use pretty_assertions::assert_eq;
use qtpl::{render_string, tpl, tplfn};

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
fn format_bytes() {
    #[tplfn]
    fn hello(name: &[u8]) {
        tpl! { <a>Hello, {!b name}!</a> }
    }

    assert_eq!(render_string!(hello(b"world")), "<a>Hello, world!</a>");
}

#[test]
fn whitespace_sensitive() {
    #[tplfn]
    fn hello() {
        tpl! { <a>Hello, <i>  world </i> !</a> }
    }

    assert_eq!(render_string!(hello()), "<a>Hello, <i> world </i> !</a>");
}

#[test]
fn whitespace_insensitive() {
    #[tplfn]
    fn hello() {
        tpl! { <a>Hello, <div> world </div> !</a> }
    }

    assert_eq!(render_string!(hello()), "<a>Hello,<div>world</div>!</a>");
}

const XSS: &str = r#"You're <script>alert("pawned")</script>!"#;

#[test]
fn escape_content() {
    #[tplfn]
    fn t(v: &str) {
        tpl! { <a>{v}</a> }
    }
    assert_eq!(
        render_string!(t(XSS)),
        "<a>You&#x27;re &lt;script&gt;alert(&quot;pawned&quot;)&lt;&#x2f;script&gt;!</a>",
    );
}

#[test]
fn escape_attr() {
    #[tplfn]
    fn t(v: &str) {
        tpl! { <a id={v}> }
    }
    assert_eq!(render_string!(t("me")), r#"<a id="me">"#);
    assert_eq!(
        render_string!(t(XSS)),
        r#"<a id="You&#x27;re &lt;script&gt;alert(&quot;pawned&quot;)&lt;&#x2f;script&gt;!">"#,
    );
}

#[test]
fn readme_example() {
    use qtpl::{render, render_string, tpl, tplfn};

    #[tplfn]
    fn page(body: &[u8], footer: &[u8]) {
        tpl! {
            <!doctype html>
            <body>
                {!b body}
                <footer>{!b footer}</footer>
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
        let b = render!(body(name));
        let f = render!(footer(company));
        tpl! {
            {!t page(&b, &f)}
        }
    }

    let name = String::from("world");
    let company = "bigcorp";

    assert_eq!(
        render_string!(home(name, company)),
        concat!(
            "<!doctype html>",
            "<body>",
            "Hello, world!",
            "<footer>Copyright bigcorp</footer>",
            "</body>",
        )
    );
}
