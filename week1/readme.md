# Traits & Macros

## Traits

-   Traits in rust are the type implementation of a characterstics of some variable. any struct implementing the trait must define all the declared fields in traits. it is similar to interfaces in java & javascript and abstract classes in c++.
-   a simple code example:
    ```rust
      trait Shape{
        type: String,
        fn get_shape():String
      }
    ```

## Macros

-   Macros in rust are code that write code while compiling called **meta programming**. Macros gets expanded at compile time and its not a runtime thing in rust.
-   There are two types of macros:

    1. declarative macros
    2. procedural macros
        - custom derive: they are defined on struct and enum.
        - attribute like macro: acts as an attribute.
        - function like macro: similar to declarative macro but capable of passing multiple variables.

-   procedural macros are more like a function. it recieves some code as input, operates on that code and then produces some code as output rather than just matching against pattern and replacing the code with other code as declarative macros do.
-   custom derive only works on enum and struct while attribute like macros can work upon other entities as well.

Tokenstream(raw tokens) --> syn::parse --> Structured Abstracted Syntax tree(AST) -->[your logic] --> quote! --> Tokenstream(generated code).

-   the `syn` crate parses the rust code from a string into a data structure that we can perform some operations on. the `quote` crate turns syn data structure back into rust code. these crates make it much simpler to parse any rust code that we want work on.

```markdown
Source code (.rs)
↓
Lexing (characters → tokens)
↓
Parsing (tokens → AST)
↓
MACRO EXPANSION ← ⭐ macros live here
↓
Name resolution & visibility checks
↓
Type checking
↓
Borrow checking
↓
MIR generation
↓
LLVM IR
↓
Machine code
```

-   `quote` exist because humans do not want construct token manually.
    ```
    rust looking code -> Tokenstreams
    ```
-   This is not string interpolation. it is token interpolation.
-   tokens are loseless representation of rust syntax.
-   `syn` exist because humans do not want to manually parse tokens.
    ```
      Tokenstream -> Structured AST (syn types)
    ```
-   this AST is not compiler level , its library level AST.
