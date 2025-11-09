use proc_macro::TokenStream;
use quote::quote;
use syn::LitStr;

#[proc_macro]
pub fn gph(input: TokenStream) -> TokenStream {
    let lit = syn::parse_macro_input!(input as LitStr);
    let s = lit.value();

    // Use inkless_core::gph::from_str to check at compile time
    let mut iter = inkless_core::grapheme::gph::from_str(&s);
    let first = iter.next();
    let second = iter.next();

    let err = match (first, second) {
        (None, _) => Some("gph!() requires exactly one grapheme, but the string is empty"),
        (Some(_), Some(_)) => {
            Some("gph!() requires exactly one grapheme, but the string contains multiple")
        }
        _ => None,
    };

    if let Some(msg) = err {
        return syn::Error::new_spanned(lit, msg).to_compile_error().into();
    }

    let root = if std::env::var("CARGO_CRATE_NAME")
        .is_ok_and(|v| v.eq_ignore_ascii_case("inkless-core"))
    {
        quote!(crate)
    } else if std::env::var("CARGO_CRATE_NAME").is_ok_and(|v| v.starts_with("inkless")) {
        quote!(::inkless_core)
    } else {
        quote!(::inkless)
    };

    let expanded = quote! {
        unsafe {
            #root::grapheme::gph::from_single_grapheme_str_unchecked(#lit)
        }
    };

    expanded.into()
}
