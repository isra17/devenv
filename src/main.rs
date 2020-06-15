use config::Config;
use std::path::PathBuf;
use structopt::StructOpt;

mod config;

#[derive(Debug, StructOpt)]
enum Command {
    /// Initialize your development environment, installing and setuping any
    /// dependencies.
    Init {},
    Run {},
    Test {},
    Lint {},
}

#[derive(Debug, StructOpt)]
/// Manage your development environment.
///
/// Use devenv to init, run and tests your projects components. Extend
/// possibilities with your own scripts and plugins.
struct Args {
    #[structopt(subcommand)]
    cmd: Command,
    config_path: Option<PathBuf>,
}

#[paw::main]
fn main(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    println!("{:?}", args);
    let config = Config::load(args.config_path)?;
    println!("{:?}", config);
    Ok(())
}

# dev run firework
# dev run staging:firework
          ^       ^
          |       |
       Provider   |
                Target

# dev run local:firework -- -p 8080
# dev run firework -e "PYRO_DATABASE__URL={staging.postgresql.url}"
# dev run firework -e "PYRO_DATABASE__URL={staging.postgresql.url}"
