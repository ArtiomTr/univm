use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse::Parse, parse_macro_input, spanned::Spanned};

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

fn extract_input_type(item: &ItemFn) -> syn::Result<Box<syn::Type>> {
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
            Err(syn::Error::new(
                receiver.span(),
                "Entrypoint cannot accept self",
            ))
        }
        syn::FnArg::Typed(pat_type) => Ok(pat_type.ty.clone()),
    }
}

fn emit_guest_entrypoint(
    attr: EntrypointAttributes,
    item: ItemFn,
) -> syn::Result<proc_macro2::TokenStream> {
    let input_type = extract_input_type(&item)?;
    let io = attr.io;
    let fn_name = &item.sig.ident;
    let fn_body = &item.block;
    let fn_vis = &item.vis;
    let fn_attrs = &item.attrs;

    // For guest compilation: generate main() that reads input and calls user function
    Ok(quote! {
        // User's original function (preserved for guest)
        #[cfg(target_os = "zkvm")]
        #(#fn_attrs)*
        #fn_vis fn #fn_name(input: #input_type) #fn_body

        // Generated main function for zkvm execution
        #[cfg(target_os = "zkvm")]
        fn main() {
            let input = univm_platform::read::<#input_type>(#io);
            #fn_name(input);
        }
    })
}

fn emit_host_entrypoint(
    _attr: EntrypointAttributes,
    item: ItemFn,
) -> syn::Result<proc_macro2::TokenStream> {
    let input_type = extract_input_type(&item)?;
    let fn_vis = &item.vis;
    let fn_attrs = &item.attrs;

    // For host compilation: include generated constants and re-export Input type
    // Also generate a dummy main for binary crates (since the guest is a binary)
    Ok(quote! {
        // Include the generated guest methods (ELF and ImageID constants)
        #[cfg(not(target_os = "zkvm"))]
        include!(concat!(env!("OUT_DIR"), "/guest_methods.rs"));

        // Re-export the input type for host usage
        #[cfg(not(target_os = "zkvm"))]
        #(#fn_attrs)*
        #fn_vis type Input = #input_type;

        // Dummy main for host compilation (the guest crate is a binary)
        // Users should import this crate as a library dependency, not run it directly
        #[cfg(not(target_os = "zkvm"))]
        fn main() {
            eprintln!("This crate is a zkvm guest and should not be run directly on the host.");
            eprintln!("Instead, use the exported ZKVM_GUEST_ELF and ZKVM_GUEST_ID constants.");
        }
    })
}

/// Marks a function as the entrypoint for a zkvm guest program.
///
/// When compiled for the zkvm target (e.g., risc0), this macro generates:
/// - The original function preserved
/// - A `main()` function that reads input and calls your function
///
/// When compiled for the host target, this macro generates:
/// - `include!` statement to bring in ELF and ImageID constants
/// - Re-exports the Input type for host usage
/// - A dummy `main()` function (for binary crate compatibility)
///
/// # Example
///
/// ```ignore
/// use univm_io::RawIo;
///
/// pub struct Input {
///     pub a: u32,
///     pub b: u32,
/// }
///
/// #[univm_platform::entrypoint(RawIo)]
/// fn guest_main(input: Input) {
///     let sum = input.a + input.b;
/// }
/// ```
#[proc_macro_attribute]
pub fn entrypoint(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as EntrypointAttributes);
    let item = parse_macro_input!(item as ItemFn);

    // Use cfg to determine compilation context at macro expansion time
    // Note: This checks the target at compile time of the crate using the macro
    let guest_stream = match emit_guest_entrypoint(attr.clone(), item.clone()) {
        Ok(v) => v,
        Err(e) => return e.into_compile_error().into(),
    };

    let host_stream = match emit_host_entrypoint(attr, item) {
        Ok(v) => v,
        Err(e) => return e.into_compile_error().into(),
    };

    // Generate code that uses cfg to select appropriate output
    // Note: main() must be at crate level, so we don't wrap it in a module
    // The cfg attributes are already applied in the individual functions
    let stream = quote! {
        #guest_stream

        #host_stream
    };

    stream.into()
}

// Implement Clone for EntrypointAttributes to allow using it twice
impl Clone for EntrypointAttributes {
    fn clone(&self) -> Self {
        Self {
            io: self.io.clone(),
        }
    }
}
