use clap::Parser;
use std::process::{Command, Stdio};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = ".")]
    path: String,

    #[arg(short = 'x', long)]
    pattern: String,
}

fn main() {
    let args = Args::parse();
    let fd_output = Command::new("fdfind")
        .args([
            "--exclude",
            "*.json",
            "--exclude",
            "*.bin",
            "--exclude",
            "venv*",
            "-t",
            "f",
            ".",
            &args.path,
        ])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let xargs = Command::new("xargs")
        .args(["grep", "-H", "-E", &args.pattern])
        .stdin(Stdio::from(fd_output.stdout.unwrap()))
        .stdout(Stdio::piped()).spawn().unwrap().wait_with_output().unwrap();
    let output = String::from_utf8(xargs.stdout).unwrap();
    println!("{}", &output);
}
