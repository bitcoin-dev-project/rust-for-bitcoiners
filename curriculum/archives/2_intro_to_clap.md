# Introduction to the Clap crate

*Clap is a popular library used for creating a polished, feature-rich command line interface. This includes common argument behavior, help generation, suggested fixes for users, colored outputs, shell completion, etc.*

## Example: Replicating a command from Bitcoin-CLI
Let's replicate a command from the bitcoin-cli that comes packaged with a Bitcoin core node. If we run the following command:
```shell
$ bitcoin-cli help getblockhash
```

We'll get the following response:
```console
getblockhash height

Returns hash of block in best-block-chain at height provided.

Arguments:
1. height    (numeric, required) The height index

Result:
"hex"    (string) The block hash

Examples:
> bitcoin-cli getblockhash 1000
> curl --user myusername --data-binary '{"jsonrpc": "1.0", "id": "curltest", "method": "getblockhash", "params": [1000]}' -H 'content-type: text/plain;' http://127.0.0.1:8332/
```

Let's re-create this using the Clap crate. There are two ways to do this. We can use the derive attributes that the crate offers or we can use the builder method. 

### Setup

Let's create a new project and install the Clap crate with the optional derive feature:
```shell
$ cargo new clap_example
$ cd clap_example
$ cargo add clap --features derive
```

### Derive Example

We can create a command and accept the `getblockhash` subcommand like so using various derive attributes:

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Bitcoin CLI")]
#[command(version = "1.0")]
#[command(about = "Bitcoin Core RPC Client", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    /// Returns hash of block in best-block-chain at height provided.
    Getblockhash { 
        #[arg(
            required = true,
            help = "(numeric, required) The height index",
        )]
        height: u64 
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Getblockhash { height }) => {
            println!("returns blockhash for height: {height:?}")
        }
        None => {
            eprintln!("Error: too few parameters")
        }
    }
}
```

### Builder Example

```rust
use clap::{arg, Command};

fn main() {
    let matches = Command::new("Bitcoin CLI")
        .version("1.0")
        .about("Bitcoin Core RPC Client")
        .subcommand_required(true)
        .subcommand(
            Command::new("getblockhash")
                .about("Returns hash of block in best-block-chain at height provided.")
                .arg(
                    arg!([height]).required(true)
                )
        )
        .get_matches();

    match matches.subcommand() {
        Some(("getblockhash", sub_matches)) => {
            println!(
                "returns blockhash for height: {}",
                sub_matches.get_one::<String>("height").unwrap()
            )
        },
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`")
    }
}
```

### Questions
1. Can you explain how derive attributes work? 
2. Why would you want to use the derive vs builder API?
3. What are the differences between the Clap output and the output from Bitcoin-CLI? What are the limitations of Clap? 
4. How might you modify the help output using the `clap-help` crate to more closely mirror the Bitcoin-CLI help output?

### Extra Practice
1. Create a method that calls the Bitcoin Core RPC server and returns the blockhash for a given height and return it from this CLI
2. Add another subcommand `getblock` that returns a JSON object with information about the block
