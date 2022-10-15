use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(help = "Input text", required = true)]
    text: Vec<String>,

    #[arg(short = 'n', long = None, help = "Do not print newline")]
    omit_newline: bool,
}

fn main() {
    let cli = Cli::parse();

    let end = if cli.omit_newline { "" } else { "\n" };
    print!("{}{}", cli.text.join(" "), end);
}
