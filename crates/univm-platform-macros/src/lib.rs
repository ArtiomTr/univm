use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, ItemFn, parse::Parse, parse_macro_input, spanned::Spanned};

struct EntrypointAttributes {
    io: syn::Ident,
}

impl Parse for EntrypointAttributes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let io = input.parse::<syn::Ident>().map_err(|e| {
            syn::Error::new(
                e.span(),
                "Entrypoint requires at least one attribute - IO kind",
            )
        })?;

        Ok(EntrypointAttributes { io })
    }
}

fn emit_entrypoint(
    attr: EntrypointAttributes,
    item: ItemFn,
) -> syn::Result<proc_macro2::TokenStream> {
    let input = {
        if item.sig.inputs.len() != 1 {
            return Err(syn::Error::new(
                item.sig.paren_token.span.span(),
                "Entrypoint must accept exactly one argument",
            ));
        }

        let first_arg = item
            .sig
            .inputs
            .first()
            .expect("Precondition checked before");

        match first_arg {
            syn::FnArg::Receiver(receiver) => {
                return Err(syn::Error::new(
                    receiver.span(),
                    "Entrypoint cannot accept self",
                ));
            }
            syn::FnArg::Typed(pat_type) => &pat_type.ty,
        }
    };

    let io = attr.io;

    let fn_name = &item.sig.ident;
    let fn_body = &item.block;
    let fn_vis = &item.vis;
    let fn_attrs = &item.attrs;
    let fn_sig = &item.sig;

    let cloned_sig = {
        let mut s = item.sig.clone();

        s.ident = Ident::new(format!("__{}", s.ident.to_string()).as_str(), s.span());

        s
    };

    let cloned_ident = &cloned_sig.ident;

    Ok(quote! {
        #[cfg(target_os = "zkvm")]
        #(#fn_attrs)*
        #cloned_sig {
            #fn_body
        }

        #[cfg(target_os = "zkvm")]
        #fn_vis #fn_sig {
            let input = univm_platform::read::<#input>(#io);

            let output = #cloned_ident(input);
        }

        #[cfg(not(target_os = "zkvm"))]
        #fn_vis fn #fn_name<T: univm_interface::Zkvm>() {}

        #[cfg(not(target_os = "zkvm"))]
        fn main() {
            println!("This crate must be used only for running from zkvm.");
        }
    })
}

#[proc_macro_attribute]
pub fn function(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as EntrypointAttributes);
    let item = parse_macro_input!(item as ItemFn);

    let stream = match emit_entrypoint(attr, item) {
        Ok(v) => v,
        Err(e) => e.into_compile_error(),
    };

    stream.into()
}
