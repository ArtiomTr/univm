use std::str::FromStr;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, Lit, parse_macro_input, spanned::Spanned};
use thiserror::Error;

#[proc_macro_attribute]
pub fn cfg_zkvm(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as Expr);
    let item: proc_macro2::TokenStream = item.into();

    let transformed = transform_expr(attr).unwrap_or_else(|e| e.into_compile_error());

    quote! {
        #[cfg(#transformed)]
        #item
    }
    .into()
}

#[derive(Debug, Clone, Copy)]
enum ZkvmIdent {
    Risc0,
    Sp1,
    Pico,
    Ziren,
    Zisk,
}

#[derive(Debug, Error)]
enum ParseZkvmIdentError {
    #[error("unknown zkvm {0}")]
    UnknownZkvm(String),
}

impl FromStr for ZkvmIdent {
    type Err = ParseZkvmIdentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "risc0" => Ok(ZkvmIdent::Risc0),
            "sp1" => Ok(ZkvmIdent::Sp1),
            "pico" => Ok(ZkvmIdent::Pico),
            "ziren" => Ok(ZkvmIdent::Ziren),
            "zisk" => Ok(ZkvmIdent::Zisk),
            v => Err(ParseZkvmIdentError::UnknownZkvm(v.to_owned())),
        }
    }
}

impl ZkvmIdent {
    fn cfg_attr(self) -> proc_macro2::TokenStream {
        match self {
            Self::Risc0 => quote!(all(
                target_os = "zkvm",
                target_vendor = "risc0",
                not(zkvm_pico)
            )),
            Self::Sp1 => quote!(all(target_os = "zkvm", target_vendor = "sp1")),
            Self::Pico => quote!(all(target_os = "zkvm", target_vendor = "risc0", zkvm_pico)),
            Self::Ziren => quote!(all(target_os = "zkvm", target_vendor = "zkm")),
            Self::Zisk => quote!(all(target_os = "zkvm", target_vendor = "zisk")),
        }
    }
}

fn transform_expr(attr: Expr) -> syn::Result<proc_macro2::TokenStream> {
    match attr {
        Expr::Path(path) => {
            let ident = path.path.require_ident()?;

            Ok(ident
                .to_string()
                .parse()
                .map(|zkvm: ZkvmIdent| zkvm.cfg_attr())
                .unwrap_or_else(|_| quote!(#ident)))
        }
        Expr::Assign(call) => {
            let Expr::Path(ref ident) = *call.left else {
                return Err(syn::Error::new(
                    call.left.span(),
                    "Only identifier supported in this place",
                ));
            };

            let ident = ident.path.require_ident()?;

            if ident.to_string() != "zkvm" {
                return Ok(quote!(#call));
            }

            let Expr::Lit(lit) = *call.right else {
                return Err(syn::Error::new(
                    call.right.span(),
                    "Only string literal supported in this place",
                ));
            };

            let Lit::Str(str) = lit.lit else {
                return Err(syn::Error::new(
                    lit.lit.span(),
                    "Only string literal supported in this place",
                ));
            };

            let zkvm: ZkvmIdent = str
                .value()
                .parse()
                .map_err(|e| syn::Error::new(str.span(), format!("{e:?}")))?;

            Ok(zkvm.cfg_attr())
        }
        Expr::Call(mut call) => {
            call.args = call
                .args
                .into_iter()
                .map(|v| syn::parse2::<Expr>(transform_expr(v)?))
                .collect::<Result<_, syn::Error>>()?;

            Ok(quote!(#call))
        }
        _ => Err(syn::Error::new(attr.span(), "Not supported expression")),
    }
}
