use indoc::indoc;
use proc_macro2::{Delimiter, Group, TokenStream};
use quote::quote;

#[track_caller]
fn test(tokens: TokenStream, expected: &str) {
    let syntax_tree: syn::File = syn::parse2(tokens).unwrap();
    let pretty = prettyplease::unparse(&syntax_tree);
    assert_eq!(pretty, expected);
}

#[test]
fn test_parenthesize_cond() {
    let s = Group::new(Delimiter::None, quote!(Struct {}));
    test(
        quote! {
            fn main() {
                if #s == #s {}
            }
        },
        // FIXME this is not valid Rust syntax. It needs to be either:
        //
        //     if (Struct {}) == (Struct {}) {}
        //
        // or:
        //
        //     if (Struct {} == Struct {}) {}
        indoc! {"
            fn main() {
                if Struct {} == Struct {} {}
            }
        "},
    );
}
