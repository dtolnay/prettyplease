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
        indoc! {"
            fn main() {
                if (Struct {}) == (Struct {}) {}
            }
        "},
    );
}

#[test]
fn test_parenthesize_match_guard() {
    let expr_struct = Group::new(Delimiter::None, quote!(Struct {}));
    let expr_binary = Group::new(Delimiter::None, quote!(true && false));
    test(
        quote! {
            fn main() {
                match () {
                    () if let _ = #expr_struct => {}
                    () if let _ = #expr_binary => {}
                }
            }
        },
        indoc! {"
            fn main() {
                match () {
                    () if let _ = Struct {} => {}
                    () if let _ = (true && false) => {}
                }
            }
        "},
    );
}

#[cfg(feature = "verbatim")]
#[test]
fn test_const_impl() {
    let src = "const impl std::ops::Deref for Foo { type Target = Bar; fn deref(&self) -> &Self::Target { &self.x } }";
    let file = syn::parse_file(src).unwrap();
    let pretty = prettyplease::unparse(&file);
    assert_eq!(
        pretty,
        indoc! {"
            const impl std::ops::Deref for Foo {
                type Target = Bar;
                fn deref(&self) -> &Self::Target {
                    &self.x
                }
            }
        "}
    );
}

#[cfg(feature = "verbatim")]
#[test]
fn test_const_unsafe_impl() {
    let src = "const unsafe impl Send for Foo {}";
    let file = syn::parse_file(src).unwrap();
    let pretty = prettyplease::unparse(&file);
    assert_eq!(pretty, "const unsafe impl Send for Foo {}\n");
}
