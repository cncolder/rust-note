//! procedural macro 宏编程
//!
//! 输入输出语法流, 中间操作语法树 AST.
//!
//! 目前需要特定类型的 crate. 未来可能会消除这个限制.

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

/// HelloMacro derive
///
/// # Examples
///
/// ```
/// #[derive(HelloMacro)]
/// ```
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 解析语法树
    let ast = syn::parse(input).unwrap();

    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    // 获取目标名称
    let name = &ast.ident;

    // 产生代码
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                // `stringify!` 是 Rust 内置宏, 直接输入表达式, 不会求值.
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };

    gen.into()
}

/// Attribute-like 宏
///
/// # Examples
///
/// ```
/// #[route(GET, "/")]
/// fn index() {}
/// ```
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Function-like 宏
///
/// # Examples
///
/// ```
/// let sql = sql!(SELECT * FROM posts WHERE id=1);
/// ```
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    input
}
