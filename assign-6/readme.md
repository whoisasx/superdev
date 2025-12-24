```shell
compiler tokens → structured AST → transformed AST → compiler tokens
```

```text
DeriveInput {
  ident: "User",
  data: Data::Struct(
    DataStruct {
      fields: Fields::Named(
        [ Field { ident: "id", ty: u32 }, Field { ident: "name", ty: String } ]
      )
    }
  ),
  generics,
  attrs
}
```
