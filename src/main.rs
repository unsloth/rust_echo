use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(help = "Input text", required = true)]
    text: Vec<String>,

    #[arg(short, long = None, help = "Do not print newline")]
    new: bool,
}

fn main() {
    let cli = Cli::parse();

    let newline = if cli.new { "" } else { "\n" };
    print!("{}{}", cli.text.join(" "), newline);
}
