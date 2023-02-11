# connect CLI experiments

a "content-first" posit Connect CLI

## install

```bash
cargo install --git https://github.com/edavidaja/cnct.git
```

## what?

`rsconnect-python` features a number of top-level administrative commands:

```
❯ rsconnect --help
Commands:
  add             Define a nickname for a Posit Connect, Posit Cloud, or
                  shinyapps.io server and credential.
  bootstrap       Create an initial admin user to bootstrap a Connect
                  instance.
  content         Interact with Posit Connect's content API.
  deploy          Deploy content to Posit Connect, Posit Cloud, or...
  details         Show details about a Posit Connect server.
  info            Show saved information about the specified deployment.
  list            List the known Posit Connect servers.
  remove          Remove the information about a Posit Connect server.
  version         Show the version of the rsconnect-python package.
  write-manifest  Create a manifest.json file for later deployment.
  ```

Since the command line interface is how we expect many publishers to interact with Connect, we should shift to a _content-first_ CLI:

```
❯ ./cnct --help  

Usage: cnct <COMMAND>

Commands:
  deploy     Deploy content
  logs       Get logs for deployed content
  remote     Configure deployment target
  download   Download bundle from Connect
  jumpstart  Download example content from Connect
  help       Print this message or the help of the given subcommand(s)
```

administrative commands mostly concern interacting with a remote connect server, and so are moved to the `remote` subcommand:

```
❯ ./cnct remote
Configure deployment target

Usage: cnct remote
       cnct remote <COMMAND>

Commands:
  list       
  add        
  remove     
  bootstrap  
  info       
  help       Print this message or the help of the given subcommand(s)
```

top-level commands should return information about the most recent deployment when appropriate:

```
cnct/target/debug on  main [!] 
❯ ./cnct logs
Get logs for deployed content

Usage: cnct logs [OPTIONS]

Options:
  -t, --tail  
  -h, --help  Print help information
```

