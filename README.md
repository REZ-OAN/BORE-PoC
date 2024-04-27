# bore cli using rust - poc

## From 3rd Commit
 - Separated the `args` from `main.rs` to `args.rs` 
 
 Ags Structure :
```
| subcommands 
    | local
        | - local_port
        | - to
        | - port (optional)
    | server
        | - min_port
```
 - Added the dependencies from the 3rd commit