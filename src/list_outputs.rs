use std::io::Cursor;

use anyhow::{Result, anyhow};

use crate::{
    ksd_cmd_builder::ksd_config_json,
    output::{Orientation, Output},
    parse_ksd_config::parse_ksd_config,
};

pub fn list_outputs() -> Result<Vec<Output>> {
    let mut cmd = ksd_config_json();

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

    let cfg = parse_ksd_config(&mut cursor)?;

    let mut outputs = Vec::with_capacity(cfg.outputs.len());
    for output in cfg.outputs {
        let current_mode = output
            .modes
            .iter()
            .find_map(|mode| {
                if mode.id == output.current_mode_id {
                    Some(mode.name.clone())
                } else {
                    None
                }
            })
            .unwrap_or("invalid_mode".to_string());

        outputs.push(Output {
            name: output.name.clone(),
            orientation: Orientation::try_from_u8(output.rotation)?,
            current_mode,
            enabled: output.enabled,
        });
    }

    Ok(outputs)
}
