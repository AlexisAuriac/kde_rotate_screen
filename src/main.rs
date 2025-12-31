use std::str::FromStr;

use anyhow::{Result, anyhow};
use clap::{Parser, Subcommand};
use list_outputs::list_outputs;
use output::{Orientation, Output};
use print_outputs::{print_output, print_outputs};
use rotate_screen::rotate_screen;

mod ksd_cmd_builder;
mod list_outputs;
mod output;
mod parse_ksd_config;
mod print_outputs;
mod rotate_screen;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Clockwise,
    CounterClockwise,
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
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
    /// list outputs
    Outputs,
    /// reset orientation
    Reset {
        #[arg(short = 'o', long, default_value = None)]
        /// orientation (normal, left, right, or inverted)
        output: Option<String>,
    },
    /// set screen orientation
    Orient {
        #[arg(default_value = "normal")]
        /// orientation (normal, left, right, or inverted)
        orientation: Orientation,
        #[arg(short = 'o', long, default_value = None)]
        /// orientation (normal, left, right, or inverted)
        output: Option<String>,
    },
    /// rotate the screen
    Rotate {
        #[arg(default_value = "clockwise")]
        /// direction (clockwise, counter-clockwise)
        direction: Direction,
        #[arg(short = 'o', long, default_value = None)]
        /// orientation (normal, left, right, or inverted)
        output: Option<String>,
    },
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

fn select_output<'a>(outputs: &'a [Output], name: &Option<String>) -> Result<&'a Output> {
    if outputs.is_empty() {
        return Err(anyhow!("no available outputs"));
    }

    let Some(name) = name else {
        return Ok(&outputs[0]);
    };

    for output in outputs {
        if &output.name == name {
            return Ok(output);
        }
    }

    Err(anyhow!("no outputs named {name}"))
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let outputs = list_outputs()?;
    if outputs.is_empty() {
        return Err(anyhow!("no available outputs"));
    }

    match &cli.command {
        Command::Outputs => {
            let outputs = list_outputs()?;
            print_outputs(&outputs);
        }
        Command::Reset {
            output: output_name,
        } => {
            let output = select_output(&outputs, output_name)?;
            print_output(output);
            rotate_screen(&output.name, Orientation::Normal)?
        }
        Command::Orient {
            orientation,
            output: output_name,
        } => {
            let output = select_output(&outputs, output_name)?;

            print_output(output);
            rotate_screen(&output.name, *orientation)?;
        }
        Command::Rotate {
            direction,
            output: output_name,
        } => {
            let output = select_output(&outputs, output_name)?;

            let orientation = match direction {
                Direction::Clockwise => output.orientation.rotate_clockwise(),
                Direction::CounterClockwise => output.orientation.rotate_counter_clockwise(),
            };

            print_output(output);
            rotate_screen(&output.name, orientation)?;
        }
    }

    Ok(())
}
