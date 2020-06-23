use proc_macro2::{Span, TokenStream};
use proc_macro_error::emit_error;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;

const WHITESPACE_INSENSITIVE: &[&str] = &[
    "!doctype",
    "address",
    "article",
    "aside",
    "blockquote",
    "body",
    "br",
    "caption",
    "col",
    "colgroup",
    "dd",
    "div",
    "dl",
    "dt",
    "embed",
    "fieldset",
    "figcaption",
    "figure",
    "footer",
    "form",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "head",
    "header",
    "hgroup",
    "hr",
    "html",
    "li",
    "main",
    "meta",
    "nav",
    "noscript",
    "ol",
    "output",
    "p",
    "pre",
    "section",
    "style",
    "table",
    "tbody",
    "td",
    "tfoot",
    "th",
    "thead",
    "title",
    "tr",
    "ul",
];

// GIANT HACK until the span start/end methods are available in stable
#[derive(Default, Copy, Clone)]
struct SpanPos {
    start: usize,
    end: usize,
}

impl From<Span> for SpanPos {
    fn from(span: Span) -> Self {
        let d = format!("{:?}", span);
        let s = d.find('(').unwrap();
        let e = d.find('.').unwrap();
        Self {
            start: d[s + 1..e].parse().unwrap(),
            end: d[e + 2..d.len() - 1].parse().unwrap(),
        }
    }
}

impl SpanPos {
    fn move_end<T: Into<SpanPos>>(&mut self, end: T) {
        self.end = end.into().end;
    }
}

enum Braced {
    Default(syn::Expr),
    Quote(syn::Expr),
    Bytes(syn::Expr),
    TplFn(syn::ExprCall),
    Child(syn::ExprPath),
}

impl Parse for Braced {
    fn parse(input: ParseStream) -> Result<Self> {
        let fp: Result<syn::Token![!]> = input.parse();
        if fp.is_ok() {
            let modifier: syn::Ident = input.parse()?;
            let ms = modifier.to_string();
            match ms.as_str() {
                "q" => Ok(Self::Quote(input.parse()?)),
                "b" => Ok(Self::Bytes(input.parse()?)),
                "t" => Ok(Self::TplFn(input.parse()?)),
                "c" => Ok(Self::Child(input.parse()?)),
                _ => {
                    emit_error!(modifier.span(), "invalid formatting directive: {}", &ms);
                    Ok(Self::Default(input.parse()?))
                }
            }
        } else {
            Ok(Self::Default(input.parse()?))
        }
    }
}

impl ToTokens for Braced {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Default(b) => quote! { write!(w, "{}", ::qtpl::escape(#b))?; },
            Self::Quote(b) => quote! { write!(w, "\"{}\"", #b)?; },
            Self::Bytes(b) => quote! { w.write_all(#b)?; },
            Self::TplFn(b) => {
                let mut c = b.clone();
                let arg: syn::Expr = syn::parse_quote!(w);
                c.args.insert(0, arg);
                quote! { #c?; }
            }
            Self::Child(b) => quote! { #b.render(w)?; },
        }
        .to_tokens(tokens);
    }
}

struct Name {
    value: String,
    span_pos: SpanPos,
}

impl Parse for Name {
    fn parse(input: ParseStream) -> Result<Self> {
        input.step(|cursor| {
            let mut value = String::new();
            let mut span_pos = SpanPos::default();
            let mut rest = *cursor;
            let mut first = true;
            while let Some((tt, next)) = rest.token_tree() {
                let tts = tt.to_string();
                if first {
                    span_pos = SpanPos::from(tt.span());
                    first = false;
                } else {
                    let cur_span = SpanPos::from(tt.span());
                    if span_pos.end != cur_span.start {
                        break;
                    }
                    let first = tts.chars().next().unwrap();
                    if !first.is_alphabetic() && first != '/' {
                        break;
                    }
                    span_pos.move_end(cur_span);
                }
                value.push_str(&tts);
                rest = next;
            }
            Ok((Self { value, span_pos }, rest))
        })
    }
}

enum ItemElement {
    Literal(String),
    Braced(Braced),
    StartOpenTag(Name),
    StartCloseTag(Name),
    EndTag,
}

struct Item {
    element: ItemElement,
    span_pos: SpanPos,
}

impl Item {
    fn new(span_pos: SpanPos, element: ItemElement) -> Self {
        Item { span_pos, element }
    }
}

impl Parse for Item {
    fn parse(input: ParseStream) -> Result<Self> {
        // we get the starting position from the first span, and then we'll move
        // the end once we figure out where that is exactly.
        let mut span_pos = SpanPos::from(input.span());
        if input.peek(syn::Token!(<)) {
            input.parse::<syn::Token!(<)>()?;
            if input.peek(syn::Token!(/)) {
                input.parse::<syn::Token!(/)>()?;
                let name = input.parse::<Name>()?;
                span_pos.move_end(name.span_pos);
                Ok(Self::new(span_pos, ItemElement::StartCloseTag(name)))
            } else {
                let name = input.parse::<Name>()?;
                span_pos.move_end(name.span_pos);
                Ok(Self::new(span_pos, ItemElement::StartOpenTag(name)))
            }
        } else if input.peek(syn::Token!(>)) {
            let angle = input.parse::<syn::Token!(>)>()?;
            span_pos.move_end(angle.span());
            Ok(Self::new(span_pos, ItemElement::EndTag))
        } else if input.peek(syn::token::Brace) {
            let content;
            let braced = syn::braced!(content in input);
            span_pos.move_end(braced.span);
            Ok(Self::new(span_pos, ItemElement::Braced(content.parse()?)))
        } else {
            Ok(Self::new(
                span_pos,
                ItemElement::Literal(input.step(|cursor| {
                    if let Some((tt, next)) = cursor.token_tree() {
                        span_pos.move_end(tt.span());
                        Ok((tt.to_string(), next))
                    } else {
                        panic!("unexpected internal error: was expecting some tokens");
                    }
                })?),
            ))
        }
    }
}

impl ToTokens for Item {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match &self.element {
            ItemElement::Literal(l) => {
                let ls = syn::LitByteStr::new(l.as_bytes(), Span::call_site());
                quote! {
                    w.write_all(#ls)?;
                }
            }
            ItemElement::Braced(b) => quote! { #b },
            _ => panic!("unexpected ToTokens for item besides Literal or Braced"),
        }
        .to_tokens(tokens)
    }
}

pub struct Template {
    items: Vec<Item>,
}

impl Parse for Template {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut items = vec![];
        let mut literal = String::new();
        let mut literal_start_pos = SpanPos::default();
        let mut prev_span_pos = SpanPos::default();
        let mut skip_space = true;
        let mut current_tag = String::new();
        while !input.is_empty() {
            let item = Item::parse(input)?;
            let span_pos = item.span_pos;

            if literal.is_empty() {
                literal_start_pos = span_pos;
            }

            if !skip_space && prev_span_pos.end != span_pos.start {
                literal.push_str(" ");
            }
            prev_span_pos = span_pos;
            skip_space = false;

            match item.element {
                ItemElement::Literal(l) => literal.push_str(&l),
                ItemElement::Braced(_) => {
                    if !literal.is_empty() {
                        let mut span_pos = literal_start_pos;
                        span_pos.move_end(prev_span_pos);
                        items.push(Item::new(span_pos, ItemElement::Literal(literal)));
                        literal = String::new();
                    }
                    items.push(item);
                }
                ItemElement::StartOpenTag(n) => {
                    current_tag = n.value;
                    if WHITESPACE_INSENSITIVE.contains(&current_tag.as_str()) {
                        literal = literal.trim_end().to_owned();
                    }
                    literal.push_str(&format!("<{}", current_tag));
                }
                ItemElement::StartCloseTag(n) => {
                    current_tag = n.value;
                    if WHITESPACE_INSENSITIVE.contains(&current_tag.as_str()) {
                        literal = literal.trim_end().to_owned();
                    }
                    literal.push_str(&format!("</{}", current_tag));
                }
                ItemElement::EndTag => {
                    if WHITESPACE_INSENSITIVE.contains(&current_tag.as_str()) {
                        skip_space = true;
                    }
                    literal.push_str(">");
                }
            }
        }
        if !literal.is_empty() {
            let mut span_pos = literal_start_pos;
            span_pos.move_end(prev_span_pos);
            items.push(Item::new(span_pos, ItemElement::Literal(literal)));
        }
        Ok(Self { items })
    }
}

impl ToTokens for Template {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let items = self.items.iter();
        let q = quote! {
            #(#items)*
            Ok(())
        };
        q.to_tokens(tokens);
    }
}
