use anyhow::{Result, anyhow};
use clap::{Parser, Subcommand};
use list_outputs::list_outputs;
use output::Orientation;
use rotate_screen::rotate_screen;

mod ksd_cmd_builder;
mod list_outputs;
mod output;
mod parse_ksd_config;
mod rotate_screen;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Clockwise,
    CounterClockwise,
}

impl Direction {
    pub fn try_from_str(s: &str) -> Result<Self> {
        match s {
            "clockwise" => Ok(Direction::Clockwise),
            "counter-clockwise" => Ok(Direction::CounterClockwise),
            _ => Err(anyhow!("invalid direction string: {s}")),
        }
    }
}

#[derive(Debug, Subcommand)]
/// rotate your screen cli
enum Command {
    /// reset orientation
    Reset,
    /// set screen orientation
    Orient {
        #[arg(default_value = "normal")]
        /// orientation (normal, left, right, or inverted)
        orientation: String,
    },
    /// rotate the screen
    Rotate {
        #[arg(default_value = "clockwise")]
        /// direction (clockwise, counter-clockwise)
        direction: String,
    },
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let outputs = list_outputs()?;
    if outputs.len() != 1 {
        return Err(anyhow!(
            "don't know how to deal with more/less than 1 output"
        ));
    }
    let output = &outputs[0];

    match &cli.command {
        Command::Reset => rotate_screen(&output.name, Orientation::Normal)?,
        Command::Orient { orientation } => {
            let orientation = Orientation::try_from_str(orientation)?;

            rotate_screen(&output.name, orientation)?;
        }
        Command::Rotate { direction } => {
            let direction = Direction::try_from_str(direction)?;
            let orientation = match direction {
                Direction::Clockwise => output.orientation.rotate_clockwise(),
                Direction::CounterClockwise => output.orientation.rotate_counter_clockwise(),
            };

            rotate_screen(&output.name, orientation)?;
        }
    }

    Ok(())
}
