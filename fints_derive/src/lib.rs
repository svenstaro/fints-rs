extern crate proc_macro;
use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Message)]
pub fn message_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_message_macro(&ast)
}

fn impl_message_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Message for #name {
            /// Take care of everything the message needs for sending:
            /// 1. serialize
            /// 2. encode as base64
            /// 3. take byte length of resulting encoded message
            /// 4. write back byte length to original message
            /// 5. serialize
            /// 6. encode as base64
            /// 7. return
            fn prepare_message_for_sending(&self) -> String {
                let serialized = to_string(&self).unwrap();
                dbg!(&serialized);
                base64::encode(&serialized)
            }
        }
    };
    gen.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
