use anyhow::{Result, anyhow};
use list_outputs::list_outputs;
use output::Orientation;
use rotate_screen::rotate_screen;

mod ksd_cmd_builder;
mod list_outputs;
mod output;
mod parse_ksd_config;
mod rotate_screen;

fn main() -> Result<()> {
    let outputs = list_outputs()?;
    if outputs.len() != 1 {
        return Err(anyhow!(
            "don't know how to deal with more/less than 1 output"
        ));
    }
    let output = &outputs[0];

    // rotate_screen(&output.name, Orientation::Right)?;
    rotate_screen(&output.name, Orientation::Normal)?;

    todo!("parse args (clockwise/counter clockwise, orientation, reset)");

    Ok(())
}
