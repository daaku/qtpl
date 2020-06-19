use proc_macro2::{Delimiter, Span, TokenStream, TokenTree};
use proc_macro_error::emit_error;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;

// GIANT HACK until the span start/end methods are available in stable
fn span_pos(s: &Span) -> (usize, usize) {
    let d = format!("{:?}", s);
    let s = d.find('(').unwrap();
    let e = d.find('.').unwrap();
    (
        d[s + 1..e].parse().unwrap(),
        d[e + 2..d.len() - 1].parse().unwrap(),
    )
}

fn literal_bytes(s: &str) -> TokenStream {
    let ls = syn::LitByteStr::new(s.as_bytes(), Span::call_site());
    quote! {
        w.write(#ls)?;
    }
}

enum Format {
    Raw,
    Quote,
    Bytes,
    Child,
}

impl Parse for Format {
    fn parse(input: ParseStream) -> Result<Self> {
        let fp: Result<syn::Token![!]> = input.parse();
        if fp.is_ok() {
            let modifier: syn::Ident = input.parse()?;
            let ms = modifier.to_string();
            match ms.as_str() {
                "q" => Ok(Self::Quote),
                "b" => Ok(Self::Bytes),
                "c" => Ok(Self::Child),
                _ => {
                    emit_error!(modifier.span(), "invalid formatting directive: {}", &ms);
                    Ok(Self::Raw)
                }
            }
        } else {
            Ok(Self::Raw)
        }
    }
}

struct Braced {
    format: Format,
    expr: syn::Expr,
}

fn child_call(expr: &syn::Expr) -> TokenStream {
    match expr {
        syn::Expr::Call(c) => {
            let mut c = c.clone();
            // let arg: syn::Expr = syn::parse_quote!(&mut w);
            let arg: syn::Expr = syn::parse_quote!(w);
            c.args.insert(0, arg);
            quote!(#c?;)
        }
        _ => {
            emit_error!(expr.span(), "expected call expression here");
            quote!()
        }
    }
}

impl Parse for Braced {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Braced {
            format: input.parse()?,
            expr: input.parse()?,
        })
    }
}

impl ToTokens for Braced {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let b = &self.expr;
        let ts = match self.format {
            Format::Raw => quote! { write!(w, "{}", #b)?; },
            Format::Quote => quote! { write!(w, "\"{}\"", #b)?; },
            Format::Bytes => quote! { w.write(#b)?; },
            Format::Child => child_call(b),
        };
        ts.to_tokens(tokens);
    }
}

fn parse_braced(g: proc_macro2::Group) -> Result<TokenStream> {
    let braced: Braced = syn::parse2(g.stream())?;
    Ok(quote!(#braced))
}

fn is_braced_group(tt: &TokenTree) -> bool {
    if let TokenTree::Group(g) = tt {
        g.delimiter() == Delimiter::Brace
    } else {
        false
    }
}

pub struct Template {
    items: Vec<TokenStream>,
}

impl Parse for Template {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut items = vec![];
        input.step(|cursor| {
            let mut s = String::new();
            let mut prev_end = 0;
            let mut skip_space = true;
            let mut rest = *cursor;
            while let Some((tt, next)) = rest.token_tree() {
                let (span_start, span_end) = span_pos(&tt.span());
                if !skip_space && prev_end != span_start {
                    s.push_str(" ");
                }
                skip_space = false;
                prev_end = span_end;

                if is_braced_group(&tt) {
                    if let TokenTree::Group(g) = tt {
                        items.push(literal_bytes(&s));
                        s.truncate(0);
                        items.push(parse_braced(g)?);
                        rest = next;
                        continue;
                    }
                    panic!("unexpected");
                }

                s.push_str(&tt.to_string());
                rest = next
            }
            items.push(literal_bytes(&s));
            Ok(((), rest))
        })?;
        items.push(quote!(Ok(())));
        Ok(Template { items })
    }
}

impl ToTokens for Template {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let items = self.items.iter();
        quote!(#(#items)*).to_tokens(tokens);
    }
}
