use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::Fields;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(Kind)]
pub fn kind_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let kind_name = format_ident!("{name}Kind");

    let variants = if let Data::Enum(data_enum) = input.data {
        data_enum.variants
    } else {
        return syn::Error::new_spanned(name, "Kind derive can only be used on enums")
            .to_compile_error()
            .into();
    };

    let kind_enum_arms = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        quote! {
            #variant_name,
        }
    });
    let kind_enum_vec_arms = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        quote! {
            #kind_name::#variant_name,
        }
    });

    let kind_arms = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        match &variant.fields {
            Fields::Unit => {
                quote! {
                    Self::#variant_name => #kind_name::#variant_name,
                }
            }
            Fields::Unnamed(_) => {
                quote! {
                    Self::#variant_name(_) => #kind_name::#variant_name,
                }
            }
            Fields::Named(_) => {
                panic!("Named fields not supported by Kind");
            }
        }
    });

    let expanded = quote! {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Encode, Decode)]
        #[cfg_attr(feature = "to_csv", derive(ToCsv))]
        pub enum #kind_name {
            #(#kind_enum_arms)*
        }

        impl #kind_name {
            pub fn to_vec() -> alloc::vec::Vec<Self> {
                alloc::vec![
                    #(#kind_enum_vec_arms)*
                ]
            }
        }

        impl #name {
            pub fn kind(&self) -> #kind_name {
                match self {
                    #(#kind_arms)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(ToCsv)]
pub fn to_csv_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let name_s = name.to_string();

    let expanded = match input.data {
        Data::Struct(data_struct) => {
            let fields = data_struct.fields;

            let arms = fields.iter().map(|field| {
                let field_ident = field.clone().ident.unwrap();
                let field_name = field_ident.to_string();

                quote! { {
                    let mut field_csvs: alloc::vec::Vec<String> = (&self.#field_ident as &dyn ToCsv).to_csv();
                    for entry in &mut field_csvs {
                        *entry = format!("{}.{}", #field_name, &entry);
                    }
                    values.extend(field_csvs);
                } }
            });

            quote! {
                impl ToCsv for #name {
                    fn to_csv(&self) -> alloc::vec::Vec<String> {
                        extern crate alloc;
                        use alloc::format;

                        let mut values:alloc::vec::Vec<String> = alloc::vec::Vec::new();
                        #(#arms)*
                        for value in &mut values {
                            *value = format!("{}", value);
                        }
                        return values;
                    }
                }
            }
        }
        Data::Enum(data_enum) => {
            let variants = data_enum.variants;
            let arms = variants.iter().map(|variant| {
                let variant_name = &variant.ident;
                let variant_name_s = variant_name.to_string();

                match &variant.fields {
                    Fields::Unit => {
                        quote! { Self::#variant_name => values.push(format!(" = {}::{}", #name_s, #variant_name_s)), }
                        // quote! { Self::#variant_name => {}, }
                    }
                    Fields::Unnamed(_) => {
                        quote! {Self::#variant_name(l) => {
                            let mut field_csvs: alloc::vec::Vec<String> = (l as &dyn ToCsv).to_csv();
                            for entry in &mut field_csvs {
                                *entry = format!("{}.{}", #variant_name_s, &entry);
                            }
                            values.extend(field_csvs);
                        },}
                    }
                    Fields::Named(_) => {
                        panic!("Named fields not supported by ToCsv");
                    }
                }
            });

            quote! {
                impl ToCsv for #name {
                    fn to_csv(&self) -> alloc::vec::Vec<String> {
                        extern crate alloc;
                        use alloc::format;
                        let mut values: alloc::vec::Vec<String> = alloc::vec::Vec::new();

                        match self {
                            #(#arms)*
                        }
                        return values;
                    }
                }
            }
        }
        Data::Union(_data_union) => {
            return syn::Error::new_spanned(name, "ToCsv not implemented for unions")
                .to_compile_error()
                .into();
        }
    };

    TokenStream::from(expanded)
}
