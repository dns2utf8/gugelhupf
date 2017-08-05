# gugelhupf

An ECMA-Script VM.

## Design Runtime

```
Create Gugelhupf-Context
        ||
        `´
  Add source code   --->   Parse AST
        ||                    ||
        `´                    `´
     Run loop      <---  Inject parsed AST
```

## Design interface with rust

The interpreter searches the JS scopes for a symbol.
If it can not find it it will look through the "native" symbols.
Native symbols are structs stored inside the Gugelhupf-Context.

Extending native symbols is as easy as adding a struct to the Gugelhupf-Context.
