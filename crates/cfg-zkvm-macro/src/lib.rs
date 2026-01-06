use std::str::FromStr;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, Lit, parse_macro_input, spanned::Spanned};
use thiserror::Error;

#[proc_macro_attribute]
pub fn cfg_zkvm(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as Expr);
    let item: proc_macro2::TokenStream = item.into();

    let transformed = match transform_expr(attr) {
        Ok(v) => v,
        Err(e) => return e.into_compile_error().into(),
    };

    quote! {
        #[cfg(#transformed)]
        #item
    }
    .into()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ZkvmIdent {
    Risc0,
    Sp1,
    Pico,
    Ziren,
    Zisk,
}

fn format_suggestion(suggestion: &Option<String>) -> String {
    suggestion
        .as_ref()
        .map(|v| format!(" - maybe you wanted to use `{v}`?"))
        .unwrap_or(" - valid options are `risc0`, `sp1`, `pico`, `ziren` and `zisk`.".to_owned())
}

#[derive(Debug, Error)]
#[cfg_attr(test, derive(PartialEq, Eq))]
enum ParseZkvmIdentError {
    #[error("unknown zkvm \"{input}\"{}", format_suggestion(suggestion))]
    UnknownZkvm {
        input: String,
        suggestion: Option<String>,
    },
}

impl ParseZkvmIdentError {
    fn unknown(vm: String) -> Self {
        Self::UnknownZkvm {
            input: vm,
            suggestion: None,
        }
    }

    fn suggestion(vm: String, suggestion: String) -> Self {
        Self::UnknownZkvm {
            input: vm,
            suggestion: Some(suggestion),
        }
    }
}

impl FromStr for ZkvmIdent {
    type Err = ParseZkvmIdentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lowercase = s.trim().to_lowercase();
        match (s, lowercase.as_str()) {
            ("risc0", _) => Ok(ZkvmIdent::Risc0),
            ("sp1", _) => Ok(ZkvmIdent::Sp1),
            ("pico", _) => Ok(ZkvmIdent::Pico),
            ("ziren", _) => Ok(ZkvmIdent::Ziren),
            ("zisk", _) => Ok(ZkvmIdent::Zisk),

            (v, "r0" | "risc0") => Err(ParseZkvmIdentError::suggestion(
                v.to_owned(),
                "risc0".to_owned(),
            )),
            (v, "succinct" | "sp1") => Err(ParseZkvmIdentError::suggestion(
                v.to_owned(),
                "sp1".to_owned(),
            )),
            (v, "brevis" | "brevis-pico" | "picovm" | "pico-vm" | "pico") => Err(
                ParseZkvmIdentError::suggestion(v.to_owned(), "pico".to_owned()),
            ),
            (v, "zkm" | "ziren") => Err(ParseZkvmIdentError::suggestion(
                v.to_owned(),
                "ziren".to_owned(),
            )),
            (v, "zisk") => Err(ParseZkvmIdentError::suggestion(
                v.to_owned(),
                "zisk".to_owned(),
            )),

            (v, _) => Err(ParseZkvmIdentError::unknown(v.to_owned())),
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
            Self::Sp1 => quote!(all(target_os = "zkvm", target_vendor = "succinct")),
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
                .map_err(|e: ParseZkvmIdentError| syn::Error::new(str.span(), e.to_string()))?;

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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use quote::quote;
    use syn::{Expr, parse_quote};

    use crate::{ZkvmIdent, transform_expr};

    #[test]
    fn correct_zkvm_identifiers_must_be_parsed() {
        assert_eq!(ZkvmIdent::from_str("risc0"), Ok(ZkvmIdent::Risc0));
        assert_eq!(ZkvmIdent::from_str("sp1"), Ok(ZkvmIdent::Sp1));
        assert_eq!(ZkvmIdent::from_str("pico"), Ok(ZkvmIdent::Pico));
        assert_eq!(ZkvmIdent::from_str("ziren"), Ok(ZkvmIdent::Ziren));
        assert_eq!(ZkvmIdent::from_str("zisk"), Ok(ZkvmIdent::Zisk));
    }

    #[test]
    fn incorrect_zkvm_identifiers_must_be_rejected() {
        const INCORRECT_IDENTIFIERS: &[&str] = &[
            "risc",
            "SP1",
            "risczero",
            "R0",
            "r0",
            "succinct",
            "picovm",
            "zkpico",
            "zkvm-pico",
            "pico-zkvm",
            "pico ",
            "brevis",
            "brevis-pico",
            " pico ",
            "ziren ",
            "Ziren",
            "zkm",
            "zkm-zkvm",
            "Zisk",
            "zIsk",
            "zisK",
        ];

        for id in INCORRECT_IDENTIFIERS {
            assert!(
                ZkvmIdent::from_str(id).is_err(),
                r#"zkvm identifier "{id}" must be incorrect"#
            )
        }
    }

    macro_rules! assert_expr_eq {
        ($received: expr, $expected: expr) => {
            let received_raw = $received;
            let received = syn::parse2::<Expr>(received_raw.clone()).unwrap();
            let expected = parse_quote!($expected);
            assert!(
                received == expected,
                "assertion `left == right` failed\n   left: {}\n  right: {}",
                received_raw,
                quote!($expected)
            )
        };
    }

    #[test]
    fn shorthand_syntax_is_correctly_transformed() {
        assert_expr_eq!(
            transform_expr(parse_quote!(risc0)).unwrap(),
            all(target_os = "zkvm", target_vendor = "risc0", not(zkvm_pico))
        );
        assert_expr_eq!(
            transform_expr(parse_quote!(sp1)).unwrap(),
            all(target_os = "zkvm", target_vendor = "succinct")
        );
        assert_expr_eq!(
            transform_expr(parse_quote!(pico)).unwrap(),
            all(target_os = "zkvm", target_vendor = "risc0", zkvm_pico)
        );
        assert_expr_eq!(
            transform_expr(parse_quote!(ziren)).unwrap(),
            all(target_os = "zkvm", target_vendor = "zkm")
        );
        assert_expr_eq!(
            transform_expr(parse_quote!(zisk)).unwrap(),
            all(target_os = "zkvm", target_vendor = "zisk")
        );
    }

    #[test]
    fn full_form_syntax_is_correctly_transformed() {
        assert_expr_eq!(
            transform_expr(parse_quote!(zkvm = "risc0")).unwrap(),
            all(target_os = "zkvm", target_vendor = "risc0", not(zkvm_pico))
        );
        assert_expr_eq!(
            transform_expr(parse_quote!(zkvm = "sp1")).unwrap(),
            all(target_os = "zkvm", target_vendor = "succinct")
        );
        assert_expr_eq!(
            transform_expr(parse_quote!(zkvm = "pico")).unwrap(),
            all(target_os = "zkvm", target_vendor = "risc0", zkvm_pico)
        );
        assert_expr_eq!(
            transform_expr(parse_quote!(zkvm = "ziren")).unwrap(),
            all(target_os = "zkvm", target_vendor = "zkm")
        );
        assert_expr_eq!(
            transform_expr(parse_quote!(zkvm = "zisk")).unwrap(),
            all(target_os = "zkvm", target_vendor = "zisk")
        );
    }

    #[test]
    fn parameters_in_function_calls_are_correctly_transformed() {
        assert_expr_eq!(
            transform_expr(parse_quote!(any(zkvm = "risc0", zkvm = "pico"))).unwrap(),
            any(
                all(target_os = "zkvm", target_vendor = "risc0", not(zkvm_pico)),
                all(target_os = "zkvm", target_vendor = "risc0", zkvm_pico)
            )
        );
        assert_expr_eq!(
            transform_expr(parse_quote!(any(sp1, ziren))).unwrap(),
            any(
                all(target_os = "zkvm", target_vendor = "succinct"),
                all(target_os = "zkvm", target_vendor = "zkm")
            )
        );
        assert_expr_eq!(
            transform_expr(parse_quote!(all(
                feature = "dummy-feature",
                any(zisk, ziren)
            )))
            .unwrap(),
            all(
                feature = "dummy-feature",
                any(
                    all(target_os = "zkvm", target_vendor = "zisk"),
                    all(target_os = "zkvm", target_vendor = "zkm")
                )
            )
        );
    }

    #[test]
    fn any_with_mixed_syntax_is_correctly_transformed() {
        assert_expr_eq!(
            transform_expr(parse_quote!(any(risc0, zkvm = "sp1"))).unwrap(),
            any(
                all(target_os = "zkvm", target_vendor = "risc0", not(zkvm_pico)),
                all(target_os = "zkvm", target_vendor = "succinct")
            )
        );
        assert_expr_eq!(
            transform_expr(parse_quote!(any(zkvm = "pico", zisk))).unwrap(),
            any(
                all(target_os = "zkvm", target_vendor = "risc0", zkvm_pico),
                all(target_os = "zkvm", target_vendor = "zisk")
            )
        );
    }

    #[test]
    fn all_with_feature_flags_is_correctly_transformed() {
        assert_expr_eq!(
            transform_expr(parse_quote!(all(risc0, feature = "std"))).unwrap(),
            all(
                all(target_os = "zkvm", target_vendor = "risc0", not(zkvm_pico)),
                feature = "std"
            )
        );
        assert_expr_eq!(
            transform_expr(parse_quote!(all(zkvm = "sp1", feature = "experimental"))).unwrap(),
            all(
                all(target_os = "zkvm", target_vendor = "succinct"),
                feature = "experimental"
            )
        );
    }

    #[test]
    fn not_is_correctly_transformed() {
        assert_expr_eq!(
            transform_expr(parse_quote!(not(risc0))).unwrap(),
            not(all(
                target_os = "zkvm",
                target_vendor = "risc0",
                not(zkvm_pico)
            ))
        );
        assert_expr_eq!(
            transform_expr(parse_quote!(not(zkvm = "pico"))).unwrap(),
            not(all(target_os = "zkvm", target_vendor = "risc0", zkvm_pico))
        );
    }

    #[test]
    fn nested_function_calls_are_correctly_transformed() {
        // any inside all
        assert_expr_eq!(
            transform_expr(parse_quote!(all(feature = "std", any(risc0, sp1)))).unwrap(),
            all(
                feature = "std",
                any(
                    all(target_os = "zkvm", target_vendor = "risc0", not(zkvm_pico)),
                    all(target_os = "zkvm", target_vendor = "succinct")
                )
            )
        );
        // all inside any
        assert_expr_eq!(
            transform_expr(parse_quote!(any(
                all(risc0, feature = "foo"),
                all(sp1, feature = "bar")
            )))
            .unwrap(),
            any(
                all(
                    all(target_os = "zkvm", target_vendor = "risc0", not(zkvm_pico)),
                    feature = "foo"
                ),
                all(
                    all(target_os = "zkvm", target_vendor = "succinct"),
                    feature = "bar"
                )
            )
        );
    }

    #[test]
    fn deeply_nested_calls_are_correctly_transformed() {
        assert_expr_eq!(
            transform_expr(parse_quote!(all(
                not(zisk),
                any(zkvm = "risc0", zkvm = "ziren")
            )))
            .unwrap(),
            all(
                not(all(target_os = "zkvm", target_vendor = "zisk")),
                any(
                    all(target_os = "zkvm", target_vendor = "risc0", not(zkvm_pico)),
                    all(target_os = "zkvm", target_vendor = "zkm")
                )
            )
        );
    }

    #[test]
    fn any_with_all_zkvm_variants_is_correctly_transformed() {
        assert_expr_eq!(
            transform_expr(parse_quote!(any(risc0, sp1, pico, ziren, zisk))).unwrap(),
            any(
                all(target_os = "zkvm", target_vendor = "risc0", not(zkvm_pico)),
                all(target_os = "zkvm", target_vendor = "succinct"),
                all(target_os = "zkvm", target_vendor = "risc0", zkvm_pico),
                all(target_os = "zkvm", target_vendor = "zkm"),
                all(target_os = "zkvm", target_vendor = "zisk")
            )
        );
    }

    #[test]
    fn complex_nested_expression_is_correctly_transformed() {
        assert_expr_eq!(
            transform_expr(parse_quote!(all(
                feature = "default",
                any(
                    all(risc0, not(feature = "no-risc0")),
                    all(zkvm = "sp1", feature = "sp1-extras")
                )
            )))
            .unwrap(),
            all(
                feature = "default",
                any(
                    all(
                        all(target_os = "zkvm", target_vendor = "risc0", not(zkvm_pico)),
                        not(feature = "no-risc0")
                    ),
                    all(
                        all(target_os = "zkvm", target_vendor = "succinct"),
                        feature = "sp1-extras"
                    )
                )
            )
        );
    }

    #[test]
    fn not_any_combination_is_correctly_transformed() {
        assert_expr_eq!(
            transform_expr(parse_quote!(not(any(ziren, zisk)))).unwrap(),
            not(any(
                all(target_os = "zkvm", target_vendor = "zkm"),
                all(target_os = "zkvm", target_vendor = "zisk")
            ))
        );
    }

    #[test]
    fn build_tests() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/fail/*.rs");
        t.pass("tests/pass/*.rs");
    }
}
