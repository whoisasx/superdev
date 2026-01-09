use quote::quote;
use proc_macro::TokenStream;
use syn::{Data, DeriveInput, Fields, Type};

#[proc_macro_derive(SerializeStruct)]
pub fn serialize_struct(input:TokenStream)->TokenStream{
  let ast:DeriveInput=syn::parse(input).unwrap();
  let name=&ast.ident;

  let serialized_fields=match &ast.data {
      Data::Struct(ds)=> match &ds.fields{
        Fields::Named(fields)=> {
          let serialized=fields.named.iter().map(|field| {
            let fname=&field.ident;
            let ftype=&field.ty;

            match ftype{
              Type::Path(tp) if tp.path.is_ident("String")=>{
                quote!{
                  let bytes = self.#fname.as_bytes();
                  let len = bytes.len() as u32;
                  result.extend_from_slice(&len.to_be_bytes());
                  result.extend_from_slice(bytes);
                }
              },
              _=>{
                quote!{
                  result.extend_from_slice(&self.#fname.to_be_bytes());
                }
              }
            }
          });
          quote!{
            #(#serialized)*
          }
        },
        _=>panic!("Expected named fields.")
      },
      _=>panic!("Expected structs.")
  };

  let expanded=quote!{
    impl Serialize for #name{
      fn serialize(&self)->Vec<u8>{
        let mut result=Vec::new();
        #serialized_fields
        return result;
      }
    }
  };
  expanded.into()
}

#[proc_macro_derive(DeserializeStruct)]
pub fn deserialize_struct(input:TokenStream)->TokenStream{
  let ast:DeriveInput=syn::parse(input).unwrap();
  let name=&ast.ident;

  let (deserialized_fields, assigned_data, total_size)= match &ast.data{
    Data::Struct(ds)=>{
      match &ds.fields{
        Fields::Named(fields)=>{
          let mut total=quote!{0usize};
          let mut deser_field=Vec::new();
          let mut assign_data=Vec::new();

          for field in &fields.named {
            let field_name = &field.ident;
            let field_type = &field.ty;

            match field_type {
              Type::Path(tp) if tp.path.is_ident("String") => {
                deser_field.push(quote!{
                  let len: u32 = {
                    let bytes: [u8; 4] = base[offset..offset+4]
                      .try_into()
                      .map_err(|_| Error)?;
                    offset += 4;
                    u32::from_be_bytes(bytes)
                  };
                  let size = len as usize;
                  let #field_name = {
                    let bytes = &base[offset..offset+size];
                    offset += size;
                    String::from_utf8(bytes.to_vec()).map_err(|_| Error)?
                  };
                })
              },
              _ => {
                deser_field.push(quote!{
                  let size = std::mem::size_of::<#field_type>();
                  let #field_name = {
                    let bytes: [u8; std::mem::size_of::<#field_type>()] =
                      base[offset..offset+size]
                        .try_into()
                        .map_err(|_| Error)?;
                    offset += size;
                    #field_type::from_be_bytes(bytes)
                  };
                })
              }
            }

            assign_data.push(quote!{#field_name});
            match field_type {
              Type::Path(tp) if tp.path.is_ident("String") => {
                total = quote!{ #total + 4usize };
              }
              _ => {
                total = quote!{ #total + std::mem::size_of::<#field_type>() };
              }
            }
          }

          (deser_field,assign_data,total)
        },
        _=>panic!("Expected named fields.")
      }
    },
    _=>panic!("Expected structs.")
  };

  let expanded=quote!{
    impl Deserialize for #name{
      fn deserialize(base:&[u8])->Result<Self,Error>{
        let mut offset=0usize;
        if base.len()<#total_size{
          return Err(Error);
        }

        #(#deserialized_fields)*
        Ok(#name{
          #(#assigned_data,)*
        })
      }
    }
  };
  expanded.into()
}