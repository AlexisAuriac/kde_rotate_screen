use std::process::Command;

use crate::output::Orientation;

pub fn ksd_config_json() -> Command {
    let mut cmd = Command::new("kscreen-doctor");
    cmd.args(["--json"]);
    cmd
}

// output can either be id or name
pub fn ksd_set_orientation(output: &str, orientation: Orientation) -> Command {
    let mut cmd = Command::new("kscreen-doctor");
    cmd.args([format!(
        "output.{}.rotation.{}",
        output,
        orientation.to_str(),
    )]);
    cmd
}
