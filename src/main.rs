use std::path::PathBuf;

use clap::Parser;
use sokudo_core::run::run_simulation;
use sokudo_playback::play;

#[derive(clap::Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(clap::Subcommand)]
enum Commands {
    Run {
        /// The file to read as the initial world state.
        world: PathBuf,

        /// The file to output the simulation data to.
        history: PathBuf,
    },
    Bake {
        /// The file to read as the initial world state.
        world: PathBuf,

        /// The file to output the simulation data to.
        history: PathBuf,
    },
    Play {
        /// The file to read as the initial world state.
        world: PathBuf,

        /// The file to read as the computed simulation data.
        history: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run {
            world,
            history,
        } => {
            match run_simulation(world.clone(), history.clone()) {
                Ok(_) => (),
                Err(err) => {
                    println!("{}", err);
                },
            }

            match play(world, history) {
                Ok(_) => (),
                Err(err) => {
                    println!("{}", err);
                }
            }
        },
        Commands::Bake {
            world,
            history,
        } => {
            match run_simulation(world, history) {
                Ok(_) => (),
                Err(err) => {
                    println!("{}", err);
                }
            }
        },
        Commands::Play {
            world,
            history,
        } => {
            match play(world, history) {
                Ok(_) => (),
                Err(err) => {
                    println!("{}", err);
                }
            }
        },
    }
}
