use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, DeriveInput, ReturnType};

#[proc_macro_derive(FieldCounter)]
pub fn field_counter(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    impl_field_counter(&input)
}

fn impl_field_counter(inp: &DeriveInput) -> TokenStream {
    let name = &inp.ident;

    let count = if let syn::Data::Struct(data) = &inp.data {
        data.fields.iter().len()
    } else if let syn::Data::Enum(data) = &inp.data {
        data.variants.iter().len()
    } else {
        0
    };

    let expanded = quote! {
      impl #name {
        pub fn field_count() -> usize {
          #count
        }
      }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Random)]
pub fn random_enum_variant(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    impl_random_variant(&input)
}

fn impl_random_variant(inp: &DeriveInput) -> TokenStream {
    let name = &inp.ident;

    let (count, variants) = if let syn::Data::Enum(data) = &inp.data {
        (data.variants.iter().len(), &data.variants)
    } else {
        panic!("RandomVariant procedural macro can only be used on Enum's, please fix it");
    };

    // let mut list = vec![];

    let first_variant = &variants[0].ident;

    let mappings = variants.iter().enumerate().map(|(idx, variant)| {
        let variant_ident = &variant.ident; // Get the variant identifier
        quote! { #idx => #name::#variant_ident }
    });

    let expanded = quote! {
      use rand::distr::{Distribution, StandardUniform};
      use rand::Rng;

      impl Distribution<#name> for StandardUniform {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> #name {
          let num = rng.random_range(0..#count);

          match num {
              #(#mappings),*,
              _ => #name::#first_variant
          }
        }
    }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn log_calls(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemFn);

    let func_name = &input.sig.ident;
    let func_block = &input.block;
    let func_attrs = &input.sig.inputs;

    let expanded = match &input.sig.output {
        ReturnType::Default => quote! {
          fn #func_name(#func_attrs) {
            println!("Calling function: {}", stringify!(#func_name));
            #func_block
          }
        },
        ReturnType::Type(_d, r) => quote! {
          fn #func_name(#func_attrs) -> #r {
            println!("Calling function: {}", stringify!(#func_name));
            #func_block
          }
        },
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn time_it(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as syn::ItemFn);

    let func_name = &input.sig.ident;
    let func_stmts = &input.block.stmts;

    let mut stmts_list = vec![parse_quote! {use std::time::SystemTime;}];

    stmts_list.push(parse_quote! {let start_time = SystemTime::now();});

    for (idx, stmt) in func_stmts.iter().enumerate() {
        let str_statement = TokenStream::from(quote! {#stmt}).to_string();

        if str_statement.contains("return")
            || (idx + 1 == func_stmts.len()
                && !str_statement.trim().ends_with(";")
                && str_statement.trim() != "}")
        {
            let v = parse_quote! {
                  println!("{} took {:#?}", stringify!(#func_name), SystemTime::now().duration_since(start_time).unwrap());
            };

            stmts_list.push(v);
        }

        stmts_list.push(stmt.clone());
    }

    input.block.stmts = stmts_list;

    let expanded = quote! {
      #input
    };

    TokenStream::from(expanded)
}
