use proc_macro::TokenStream;

#[proc_macro]
pub fn impl_cin_type(t: TokenStream) -> TokenStream {
    return format!("impl Parseable for {} {{
        type Ret = {};
        fn parse(sc: &mut impl Scanner) -> Self::Ret {{
            sc.read_token().unwrap()
        }}
    }}", t.to_string(), t.to_string()).parse().unwrap()
}
