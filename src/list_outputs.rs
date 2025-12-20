use std::io::Cursor;

use anyhow::{Result, anyhow};

use crate::{
    k_screen_doctor_cmd_builder::CmdBuilder,
    output::{Orientation, Output},
    parse_kscreen_doctor_config,
};

pub fn list_outputs() -> Result<Vec<Output>> {
    let mut cmd = CmdBuilder::config_json();

    let out = cmd
        .output()
        .map_err(|e| anyhow!("failed to get output list: `{:?}`: {}", cmd, e))?;

    if !out.status.success() {
        return Err(anyhow!(
            "failed to get output list: `{:?}`: {}",
            cmd,
            String::from_utf8_lossy(&out.stderr)
        ));
    }

    let out = String::from_utf8(out.stdout)
        .map_err(|e| anyhow!("failed to get output list: `{:?}`: {}", cmd, e))?;
    let mut cursor = Cursor::new(&out);

    let cfg = parse_kscreen_doctor_config(&mut cursor)?;

    let mut outputs = Vec::with_capacity(cfg.outputs.len());
    for output in cfg.outputs {
        outputs.push(Output {
            name: output.name.clone(),
            orientation: Orientation::try_from_u8(output.rotation)?,
        });
    }

    Ok(outputs)
}
