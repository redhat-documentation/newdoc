use clap::CommandFactory;

// We're reusing the module just for the Cli struct. Ignore the rest of the code
// and don't report it as "never used" in this build script.
#[allow(dead_code)]
#[path = "src/cmd_line.rs"]
mod cmd_line;
use cmd_line::Cli;

fn main() -> std::io::Result<()> {
    let cmd: clap::Command = Cli::command();

    println!("{:?}", cmd);

    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    std::fs::write("newdoc.1", buffer)?;

    Ok(())
}
