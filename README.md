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
    - Added server.rs
        - In this file, they have defined server structure, starts a server that listening for new connections, in the connection handling function they have used tokio::spawn so that each new task can be handled asynchronously, two share server state between to processes they used Arc data structure. And modified the runtime of the main function to Tokio Runtime using the macro #[tokio::main]
    - Added lib.rs
        -  This organizes our code into modules
    - Modfied the main.rs code for the arg_parse as I have separated the arg_parse definition with clap into another file.