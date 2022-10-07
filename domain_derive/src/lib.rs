use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Type};

#[proc_macro_derive(Entity)]
pub fn entity(input: TokenStream) -> TokenStream {
  // Parse the input tokens into a syntax tree
  let input = parse_macro_input!(input as DeriveInput);
  let name = input.ident;

  println!("Deriving Entity for {}", name);

  match input.data {
    Data::Struct(str) => {
      str.fields.iter().for_each(|f| {
        let ident = f.ident.as_ref().unwrap();
        println!("Field: {}", ident);
        match f.ty {
          Type::Array(_) => {}
          Type::BareFn(_) => {}
          Type::Group(_) => {}
          Type::ImplTrait(_) => {}
          Type::Infer(_) => {}
          Type::Macro(_) => {}
          Type::Never(_) => {}
          Type::Paren(_) => {}
          Type::Path(_) => {}
          Type::Ptr(_) => {}
          Type::Reference(_) => {}
          Type::Slice(_) => {}
          Type::TraitObject(_) => {}
          Type::Tuple(_) => {}
          Type::Verbatim(_) => {}
          _ => {}
        };
      });
    },
    Data::Enum(_) => {}
    Data::Union(_) => {}
  };

  // Build the output, possibly using quasi-quotation
  let expanded = quote! {
        // ...
    };

  // Hand the output tokens back to the compiler
  TokenStream::from(expanded)
}
