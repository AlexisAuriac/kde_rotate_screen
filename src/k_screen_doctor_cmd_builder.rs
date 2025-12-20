use std::process::Command;

pub struct CmdBuilder {}

impl CmdBuilder {
    pub fn config_json() -> Command {
        let mut cmd = Command::new("kscreen-doctor");
        cmd.args(["--json"]);
        cmd
    }
}
