use anyhow::{Result, anyhow};

use crate::{ksd_cmd_builder::ksd_set_orientation, output::Orientation};

pub fn rotate_screen(name: &str, orientation: Orientation) -> Result<()> {
    let mut cmd = ksd_set_orientation(name, orientation);

    let cmd_out = cmd
        .output()
        .map_err(|e| anyhow!("failed to rotate screen: `{:?}`: {}", cmd, e))?;

    // todo: kscreen-doctor doesn't exit with an error even if there is one
    if !cmd_out.status.success() {
        return Err(anyhow!(
            "failed to rotate screen: `{:?}`: {}",
            cmd,
            String::from_utf8_lossy(&cmd_out.stderr)
        ));
    }

    Ok(())
}
