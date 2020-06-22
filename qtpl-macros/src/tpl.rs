use proc_macro2::{Delimiter, Span, TokenStream, TokenTree};
use proc_macro_error::emit_error;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result};

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
        w.write_all(#ls)?;
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
        let ts = match self {
            Braced::Default(b) => quote! { write!(w, "{}", ::qtpl::escape(#b))?; },
            Braced::Quote(b) => quote! { write!(w, "\"{}\"", #b)?; },
            Braced::Bytes(b) => quote! { w.write_all(#b)?; },
            Braced::TplFn(b) => {
                let mut c = b.clone();
                let arg: syn::Expr = syn::parse_quote!(w);
                c.args.insert(0, arg);
                quote!{ #c?; }
            },
            Braced::Child(b) => quote!{ #b.render(w)?; },
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
