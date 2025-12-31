use crate::output::Output;

pub fn print_outputs(outputs: &[Output]) {
    for output in outputs {
        println!(
            "{}\t{}\t{}\t{}",
            output.name,
            if output.enabled {
                "enabled"
            } else {
                "disabled"
            },
            output.current_mode,
            output.orientation.to_str(),
        );
    }
}
