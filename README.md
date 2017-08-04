# gugelhupf

An ECMA-Script VM.

## Design

```
Create Gugelhupf-Context
        ||
        `´
  Add source code   --->   Parse AST
        ||                    ||
        `´                    `´
     Run loop      <---  Inject parsed AST
```
