use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name="cnct")]
#[command(about="a content-first posit connect CLI", long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Deploy content
    #[command(arg_required_else_help = true)]
    Deploy(Deploy),
    /// Get logs for deployed content
    #[command(arg_required_else_help = true)]
    Logs {
        #[arg(short, long)]
        tail: bool
    },
    /// Configure deployment target
    #[command(arg_required_else_help = true)]
    Remote(Remote),
    /// Download bundle from Connect
    #[command(arg_required_else_help = true)]
    Download {
        /// the guid of the bundle to download
        guid: String
    },
    /// Downloand example content from Connect
    #[command(arg_required_else_help = true)]
    Jumpstart {
        /// example content type to download
        content: String,
    },

}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct Remote {
    #[command(subcommand)]
    command: Option<RemoteCommands>
}

#[derive(Debug, Subcommand)]
enum RemoteCommands {
    List,
    
    #[command(arg_required_else_help = true)]
    Add(RemoteAdd),

    #[command(arg_required_else_help = true)]
    Remove { remote : Option<String>},

    #[command(arg_required_else_help = true)]
    Bootstrap { jwt : Option<String>, server : Option<String>},

    #[command(arg_required_else_help = true)]
    Info { remote: Option<String> },
}

#[derive(Debug, Args)]
struct RemoteAdd {
    #[arg(long, short)]
    nickname: Option<String>,

    #[arg(long, short)]
    address: Option<String>,
    
    #[arg(long, short)]
    key: Option<String>
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct Deploy {
    #[command(subcommand)]
    command: Option<DeployCommands>,
}

#[derive(Debug, Subcommand)]
enum DeployCommands {
    Flask { deploy : Option<String> },
    Bokeh { deploy : Option<String> },
    Dash { deploy : Option<String> },
    Fastapi { deploy : Option<String> },
    Streamlit { deploy : Option<String> },
    Static { deploy : Option<String> },
    Notebook { deploy : Option<String> },
    Quarto { deploy : Option<String> },
    Shiny { deploy : Option<String> },
    Manifest { deploy : Option<String> },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Jumpstart { content } => {
            println!("Downloading example {} content", content)
        }
        Commands::Deploy(deploy) => {
            let content_type = deploy.command.unwrap();
            match content_type {
                DeployCommands::Bokeh { deploy } => {
                    println!("deploying {:?} app", deploy)
                }
                DeployCommands::Flask { deploy } => {
                    println!("deploying {:?} app", deploy)
                },
                DeployCommands::Dash { deploy } => {
                    println!("deploying {:?} app", deploy)
                },
                DeployCommands::Fastapi { deploy } => {
                    println!("deploying {:?} app", deploy)
                },
                DeployCommands::Streamlit { deploy } => {
                    println!("deploying {:?} app", deploy)
                },
                DeployCommands::Static { deploy } => {
                    println!("deploying {:?} app", deploy)
                },
                DeployCommands::Notebook { deploy } => {
                    println!("deploying {:?} app", deploy)
                },
                DeployCommands::Quarto { deploy } => {
                    println!("deploying {:?} app", deploy)
                },
                DeployCommands::Shiny { deploy } => {
                    println!("deploying {:?} app", deploy)
                },
                DeployCommands::Manifest { deploy } => {
                    println!("deploying {:?} app", deploy)
                },
            }
        }
        Commands::Download { guid } => {
            println!("Downloading latest bundle from {}", guid)
        }
        Commands::Logs { tail } => {
            if tail {
                println!("Streaming logs:")
            } else {
                println!("Latest logs:")
            }
        }
        Commands::Remote(remote) => {
            let remote_action = remote.command.unwrap();
            match remote_action {
                RemoteCommands::List {  } => {
                    println!("list of configured remote servers")
                },
                RemoteCommands::Add(add) => {
                    let addr = add.address;
                    let nick = add.nickname;
                    let apikey = add.key;
                    println!("added remote {:?} with url {:?} and key {:?}", nick, addr, apikey )
                }
                RemoteCommands::Remove { remote } => {
                    println!("removing {:?} from list of remotes", remote)
                },
                RemoteCommands::Bootstrap { server,jwt} => {
                    println!("bootstrapping server {:?} with token {:?}", jwt, server)
                },
                RemoteCommands::Info { remote } => {
                    println!("list of runtimes and capabailities for {:?}", remote)
                },
            }
        }

    }
}