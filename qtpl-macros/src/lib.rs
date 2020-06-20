mod tpl;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
#[proc_macro_error]
pub fn render_bytes(input: TokenStream) -> TokenStream {
    let mut c = parse_macro_input!(input as syn::ExprCall);
    let arg: syn::Expr = syn::parse_quote!(&mut w);
    c.args.insert(0, arg);
    TokenStream::from(quote! {
        {
            let mut w = Vec::new();
            if let ::std::result::Result::Err(err) = #c {
                ::std::result::Result::Err(err)
            } else {
                ::std::result::Result::Ok(w)
            }
        }
    })
}

#[proc_macro]
#[proc_macro_error]
pub fn render_string(input: TokenStream) -> TokenStream {
    let mut c = parse_macro_input!(input as syn::ExprCall);
    let arg: syn::Expr = syn::parse_quote!(&mut w);
    c.args.insert(0, arg);
    TokenStream::from(quote! {
        {
            let mut w = Vec::new();
            #c.unwrap();
            String::from_utf8(w).unwrap()
        }
    })
}

#[proc_macro]
#[proc_macro_error]
pub fn tpl(input: TokenStream) -> TokenStream {
    let el = parse_macro_input!(input as tpl::Template);
    let result = quote! { #el };
    TokenStream::from(result)
}

#[proc_macro_attribute]
#[proc_macro_error]
pub fn tplfn(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut f = parse_macro_input!(item as syn::ItemFn);
    let arg: syn::FnArg = syn::parse_quote!(w: &mut dyn ::std::io::Write);
    f.sig.inputs.insert(0, arg);
    f.sig.output = syn::parse_quote!(-> ::std::result::Result<(), ::std::io::Error>);
    TokenStream::from(quote!(#f))
}

#[proc_macro]
#[proc_macro_error]
pub fn child(input: TokenStream) -> TokenStream {
    let mut c = parse_macro_input!(input as syn::ExprCall);
    let arg: syn::Expr = syn::parse_quote!(w);
    c.args.insert(0, arg);
    TokenStream::from(quote! {
        |w: &mut dyn ::std::io::Write| -> ::std::io::Result<()> {
            #c
        }
    })
}
