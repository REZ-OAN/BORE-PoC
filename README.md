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

## From 4th Commit
    - Added shared.rs
        - In this file, they have declared some data structure for client Messages and server Messages, Also defines a function which handles read/write streams between two end points (Client and Server) Bidirectionally. This is for implementing proxies or tunneling systems, to relay data between two end points.
    - 