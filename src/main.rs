use anyhow::Result;
use list_outputs::list_outputs;
use parse_kscreen_doctor_config::parse_kscreen_doctor_config;

mod k_screen_doctor_cmd_builder;
mod list_outputs;
mod output;
mod parse_kscreen_doctor_config;

fn main() -> Result<()> {
    let outputs = list_outputs()?;
    println!("{:?}", outputs);

    todo!("parse args (clockwise/counter clockwise, orientation, reset)");
    todo!("rotate screen");
}
