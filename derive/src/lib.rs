synstructure::decl_derive!([JSTraceable] => js_traceable_derive);

fn js_traceable_derive(_s: synstructure::Structure) -> proc_macro2::TokenStream {
    synstructure::quote! {
        impl crate::JSTraceable for S {}
    }
}
