#![recursion_limit = "128"]
extern crate proc_macro;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;
extern crate unique_type_id;
extern crate uuid;

struct UuidString {
    value: String
}

impl syn::parse::Parse for UuidString {
    fn parse(input: &syn::parse::ParseBuffer<'_>) -> Result<Self, syn::Error> {
        let str = input.parse::<syn::LitStr>()?;

        Ok(Self {
            value: str.value()
        })
    }
}
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn uuid(attr: TokenStream, item: TokenStream) -> TokenStream {
    let uuid = parse_macro_input!(attr as UuidString);
    let derive = parse_macro_input!(item as syn::DeriveInput);

    let name = &derive.ident;
    let (impl_generics, ty_generics, where_clause) = derive.generics.split_for_impl();
    let uuid = uuid.value;

    TokenStream::from(quote! {
        #derive
        impl #impl_generics unique_type_id::StaticUuid for #name #ty_generics #where_clause {
            const UUID: uuid::Uuid = uuid::uuid!(#uuid);
            fn uuid() -> uuid::Uuid {
                Self::UUID
            }
        }
    })
}