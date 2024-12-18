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

    let kind_to_default_arms = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        match &variant.fields {
            Fields::Unit => {
                quote! {
                    #kind_name::#variant_name => #name::#variant_name,
                }
            }
            Fields::Unnamed(_) => {
                quote! {
                    #kind_name::#variant_name => #name::#variant_name(Default::default()),
                }
            }
            Fields::Named(_) => {
                panic!("Named fields not supported by Kind");
            }
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
        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash, Encode, Decode)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub enum #kind_name {
            #[default]
            #(#kind_enum_arms)*
        }

        impl #kind_name {
            pub fn to_vec() -> alloc::vec::Vec<Self> {
                alloc::vec![
                    #(#kind_enum_vec_arms)*
                ]
            }

            pub fn to_default(&self) -> #name {
                match self {
                    #(#kind_to_default_arms)*
                }
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
                    let mut field_csvs: alloc::vec::Vec<alloc::string::String> = (&self.#field_ident as &dyn ToCsv).to_csv();
                    for entry in &mut field_csvs {
                        *entry = format!("{}.{}", #field_name, &entry);
                    }
                    values.extend(field_csvs);
                } }
            });

            let arms_enum = fields.iter().map(|field| {
                let field_ident = field.clone().ident.unwrap();
                let field_name = field_ident.to_string();

                quote! { {
                    let mut field_csvs: alloc::vec::Vec<alloc::string::String> = (&self.#field_ident as &dyn ToCsv).enumerate();
                    
                    for entry in &mut field_csvs {
                        *entry = format!("{}.{}", #field_name, &entry);
                    }
                    values.extend(field_csvs);
                } }
            });

            quote! {
                impl ToCsv for #name {
                    fn to_csv(&self) -> alloc::vec::Vec<alloc::string::String> {
                        extern crate alloc;
                        use alloc::format;

                        let mut values:alloc::vec::Vec<alloc::string::String> = alloc::vec::Vec::new();
                        #(#arms)*
                        return values;
                    }

                    fn enumerate(&self) -> alloc::vec::Vec<alloc::string::String> {
                        extern crate alloc;
                        use alloc::format;
                        let mut values:alloc::vec::Vec<alloc::string::String> = alloc::vec::Vec::new();
                        #(#arms_enum)*
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
                            let mut field_csvs: alloc::vec::Vec<alloc::string::String> = (l as &dyn ToCsv).to_csv();
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

            let arms_enum = variants.iter().map(|variant| {
                let variant_name = &variant.ident;
                let variant_name_s = variant_name.to_string();

                match &variant.fields {
                    Fields::Unit => {
                        quote! { values.push(format!(" = {}::{}", #name_s, #variant_name_s)); }
                    }
                    Fields::Unnamed(var) => {
                        let var_type = 
                        if let Some(first_field) = var.unnamed.first() {
                            first_field
                        } else {
                            panic!("Struct has no fields!");
                        };
                        quote! {              
                                {
                                    let l = #var_type::default();
                                    let mut field_csvs: alloc::vec::Vec<alloc::string::String> = (&l as &dyn ToCsv).enumerate();
                                    
                                    for entry in &mut field_csvs {
                                        *entry = format!("{}.{}", #variant_name_s, &entry);
                                    }
                                    values.extend(field_csvs);
                                }
                            }              
    
                    }
                    Fields::Named(_) => {
                        panic!("Named fields not supported by ToCsv");
                    }
                }
            });

            quote! {
                impl ToCsv for #name {
                    fn to_csv(&self) -> alloc::vec::Vec<alloc::string::String> {
                        extern crate alloc;
                        use alloc::format;
                        let mut values: alloc::vec::Vec<alloc::string::String> = alloc::vec::Vec::new();

                        match self {
                            #(#arms)*
                        }
                        return values;
                    }

                    fn enumerate(&self) -> alloc::vec::Vec<alloc::string::String> {
                        extern crate alloc;
                        use alloc::format;
                       
                        let mut values:alloc::vec::Vec<alloc::string::String> = alloc::vec::Vec::new();
                        #(#arms_enum)*
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


#[proc_macro_derive(Reflect)]
pub fn reflect_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let name_s = name.to_string();

    let expanded = match input.data {
        Data::Struct(data_struct) => {
            let fields = data_struct.fields;

            let field_arms = fields.iter().map(|field| {
                let field_ident = field.clone().ident.unwrap();
                let field_name = field_ident.to_string();

                quote! { {
                    fields.push((#field_name, &mut self.#field_ident as &mut dyn Reflect));
                } }
            });

            let arms_enum = fields.iter().map(|field| {
                let field_ident = field.clone().ident.unwrap();
                let field_name = field_ident.to_string();

                quote! { {
                    let mut field_csvs: alloc::vec::Vec<alloc::string::String> = (&self.#field_ident as &dyn ToCsv).enumerate();
                    
                    for entry in &mut field_csvs {
                        *entry = format!("{}.{}", #field_name, &entry);
                    }
                    values.extend(field_csvs);
                } }
            });

            quote! {
                impl Reflect for #name {
                    fn reflect_type(&self) -> ReflectType {
                        ReflectType::Structure
                    }
                
                    fn type_name(&self) -> &str {
                        #name_s
                    }
                
                    fn fields(&mut self) -> Vec<(&str, &mut dyn Reflect)> {
                        let mut fields = Vec::new();
                        #(#field_arms)*
                        fields
                    }
                
                    fn set_value(&mut self, value: ReflectValue) {
                    }
                
                    fn get_value(&self) -> ReflectValue {
                        ReflectValue::None
                    }
                
                    fn variants(&self) -> Vec<(&str, Box<dyn Reflect>)> {
                        Vec::new()
                    }

                    fn as_variant(&mut self, _i: usize) -> Option<&mut dyn Reflect> {
                        None
                    }
                }
            }
        }





        Data::Enum(data_enum) => {
            let variants = data_enum.variants;
            let as_arms = variants.iter().enumerate().map(|(i, variant)| {
                let variant_name = &variant.ident;
                let variant_name_s = variant_name.to_string();

                match &variant.fields {
                    Fields::Unit => {
                        quote! { 
                            if _i == #i {
                                *self = Self::#variant_name;
                                return Some(self);
                            } 
                        }
                        // quote! { Self::#variant_name => {}, }
                    }
                    Fields::Unnamed(_) => {
                        quote! {
                            if _i == #i {
                                *self = Self::#variant_name(Default::default());
                                if let Self::#variant_name(r) = self {
                                    return Some(r);
                                }
                            } 
                        }
                    }
                    Fields::Named(_) => {
                        panic!("Named fields not supported by ToCsv");
                    }
                }
            });

            let variant_arms = variants.iter().map(|variant| {
                let variant_name = &variant.ident;
                let variant_name_s = variant_name.to_string();

                match &variant.fields {
                    Fields::Unit => {
                        quote! { 
                            {
                                let value: Box<dyn Reflect> = Box::new(Self::#variant_name);
                                variants.push((#variant_name_s, value)); 
                            }
                         }
                    }
                    Fields::Unnamed(_) => {
                        quote! {              
                                {
                                    let value: Box<dyn Reflect> = Box::new(Self::#variant_name(Default::default()));
                                    variants.push((#variant_name_s, value)); 
                                }
                            }              
    
                    }
                    Fields::Named(_) => {
                        panic!("Named fields not supported by ToCsv");
                    }
                }
            });

            quote! {
                impl Reflect for #name {
                    fn reflect_type(&self) -> ReflectType {
                        ReflectType::Enumeration
                    }
                
                    fn type_name(&self) -> &str {
                        #name_s
                    }
                
                    fn fields(&mut self) -> Vec<(&str, &mut dyn Reflect)> {
                        Vec::new()
                    }
                
                    fn set_value(&mut self, value: ReflectValue) {
                    }
                
                    fn get_value(&self) -> ReflectValue {
                        ReflectValue::None
                    }
                
                    fn variants(&self) -> Vec<(&str, Box<dyn Reflect>)> {
                        let mut variants = Vec::new();
                        #(#variant_arms)*
                        variants
                    }

                    fn as_variant(&mut self, _i: usize) -> Option<&mut dyn Reflect> {
                        #(#as_arms)*
                        panic!("variant incorrect");
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