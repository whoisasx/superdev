use proc_macro::TokenStream;
use quote::quote;
use syn::{Data,DeriveInput,Fields};

#[proc_macro_derive(SerializeFixedStruct)]
pub fn serialize_fixed_struct(input:TokenStream)->TokenStream{
  let ast:DeriveInput=syn::parse(input).unwrap();
  let name=&ast.ident;

  let fields= match &ast.data{
    Data::Struct(ds)=> match &ds.fields{
      Fields::Named(f)=> {
        let field_serializations=f.named.iter().map(|field|{
          let f_name=&field.ident;
          return quote!{
            result.extend_from_slice(&self.#f_name.to_be_bytes());
          };
        });
        quote!{
          #(#field_serializations)*
        }
      },
      _=>panic!("Expected named fields.")
    },
    _=>panic!("Expected struct")
  };

  let expanded=quote!{
    impl Serialize for #name{
      fn serialize(&self)->Vec<u8>{
        let mut result=Vec::new();
        #fields
        result
      }
    }
  };
  return expanded.into();
}

#[proc_macro_derive(DeserializeFixedStruct)]
pub fn deserialize_fixed_struct(input:TokenStream)->TokenStream{
  let ast:DeriveInput=syn::parse(input).unwrap();
  let name=&ast.ident;

  let (fields,field_assign,total_size)=match &ast.data{
    Data::Struct(ds)=>{
      match &ds.fields{
        Fields::Named(f)=>{
          let mut total=quote!{0usize};
          let mut field_deser=Vec::new();
          let mut field_assign=Vec::new();

          for field in &f.named{
            let f_name=&field.ident;
            let f_type=&field.ty;

            field_deser.push(quote!{
              let size=std::mem::size_of::<#f_type>();
              let #f_name={
                let bytes:[u8;std::mem::size_of::<#f_type>()]=base[offset..offset+size]
                .try_into()
                .map_err(|_| Error)?;
                offset+=size;
                #f_type::from_be_bytes(bytes)
              };
            });

            field_assign.push(quote!{#f_name});
            total=quote!{#total+std::mem::size_of::<#f_type>()};
          }

          (field_deser,field_assign,total)
        },
        _=>panic!("Expected name fields.")
      }
    },
    _=>panic!("Struct expected.")
  };

  let expanded=quote!{
    impl Deserialize for #name{
      fn deserialize(base:&[u8])->Result<Self,Error>{
        let mut offset=0usize;
        if base.len()<#total_size{
          return Err(Error);
        }
        #(#fields)*
        Ok(#name{
          #(#field_assign,)*
        })
      }
    }
  };
  expanded.into()
}