use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{spanned::Spanned, Data, DeriveInput, Fields};

pub fn expand_prism(input: DeriveInput) -> syn::Result<TokenStream> {
    let variants = match input.data {
        Data::Enum(e) => e.variants,
        _ => panic!("this derive macro only works on structs with named fields"),
    };

    let enum_name = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    variants
        .iter()
        .map(|v| {
            let variant_name = &v.ident;
            let name = format_ident!("{}{}", enum_name, variant_name, span = v.span());

            let inner_type;
            let inner_expr;
            let cloned_inner;
            let variant_expr;

            match &v.fields {
                Fields::Named(_) => {
                    return Err(syn::Error::new_spanned(
                        &v,
                        "variants with named fields are not supported for deriving `Prism`",
                    ));
                }
                Fields::Unnamed(f) => {
                    let fields = f.unnamed.iter();

                    // By having the comma outside instead of inside the #(),
                    // it is only added between items, not after the last one.
                    // For `Variant()` the inner type is `()`, for `Variant(A)`
                    // it is `(A)` (equal to just `A`), for `Variant(A, B)` it
                    // is the tuple `(A, B)`.
                    inner_type = quote! { (#(#fields),*) };

                    let fields = (0..f.unnamed.len()).map(|n| format_ident!("_v{}", n + 1));
                    let cloned = fields
                        .clone()
                        .map(|f| quote! { ::std::clone::Clone::clone(#f) });

                    inner_expr = quote! { (#(#fields),*) };
                    cloned_inner = quote! { (#(#cloned),*) };
                    variant_expr = inner_expr.clone();
                }
                Fields::Unit => {
                    inner_type = quote! { () };
                    inner_expr = quote! { () };
                    cloned_inner = quote! { () };
                    variant_expr = quote! {};
                }
            }

            Ok(quote! {
                #[derive(Clone)]
                struct #name;

                impl #impl_generics ::druid_widget_nursery::prism::Prism<
                    #enum_name #ty_generics,
                    #inner_type,
                > for #name #where_clause {
                    fn get(
                        &self,
                        data: &#enum_name #ty_generics,
                    ) -> ::std::option::Option<#inner_type> {
                        match data {
                            #enum_name::#variant_name #variant_expr => {
                                ::std::option::Option::Some(#cloned_inner)
                            }
                            _ => ::std::option::Option::None,
                        }
                    }

                    fn put(&self, data: &mut #enum_name #ty_generics, #inner_expr: #inner_type) {
                        *data = #enum_name::#variant_name #variant_expr;
                    }
                }
            })
        })
        .collect()
}
