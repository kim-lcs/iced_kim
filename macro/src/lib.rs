use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::DeriveInput;

/// iced_kim table row derive [TableRow]
#[proc_macro_derive(TableRow)]
pub fn table_row_macro_derive(input: TokenStream) -> TokenStream {
    // 基于 input 构建 AST 语法树
    let ast: DeriveInput = syn::parse(input).unwrap();

    // 构建特征实现代码
    impl_table_row_macro(&ast)
}

/// implement iced table row
fn impl_table_row_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let data = &ast.data;
    let fields = match data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(fields),
            ..
        }) => fields.named.iter(),
        _ => panic!("TableRow can only be derived for structs"),
    };

    let mut match_arms = Vec::new();
    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_str = field_name.to_string();
        match_arms.push(quote! {
            #field_str => (&self.#field_name).to_string(),
        });
    }

    let gen = quote! {
        impl iced_kim::TableRow for #name {
            /// get the value of this field by filed name
            fn get_value(&self, filed_name: &str) -> String {
                match filed_name {
                    #(#match_arms)*
                    _ => String::new(),
                }
            }
        }
    };
    gen.into()
}

/// iced_kim Message derive [Message]
#[proc_macro_derive(Message)]
pub fn message_macro_derive(input: TokenStream) -> TokenStream {
    // 基于 input 构建 AST 语法树
    let ast: DeriveInput = syn::parse(input).unwrap();

    // 构建特征实现代码
    impl_message_macro(&ast)
}

/// implement message
fn impl_message_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let data = &ast.data;
    match data {
        syn::Data::Enum(..) => {}
        _ => panic!("Message only be derived for enum types"),
    };

    let gen = quote! {
        impl iced_kim::IWindowMessage for #name {
        }
    };
    gen.into()
}
