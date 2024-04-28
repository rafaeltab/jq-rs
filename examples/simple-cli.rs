extern crate jq_rs;
use std::env;

fn main() {
    let mut args = env::args().skip(1);

    let program = args.next().expect("jq program");
    let input = args.next().expect("data input");
    match jq_rs::run_advanced(
        &program,
        &input,
        jq_rs::JqOptions::default()
            .with_colorization(jq_rs::JqColorization::Colorize)
            .with_indentation(jq_rs::JqIndentation::Spaces(2)),
    ) {
        Ok(s) => print!("{}", s), // The output will include a trailing newline
        Err(e) => eprintln!("{}", e),
    }
}
