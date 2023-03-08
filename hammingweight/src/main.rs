use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Selina Liu",
    about = "Calculate Hamming Weight of a integer"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Selina Liu")]
    Hamming {
        #[clap(short, long)]
        input: u32,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Hamming { input }) => {
            let result = hammingweight::hamming_weight(input);
            println!(
                "Number of 1s in the binary representation of {}  is: {}",
                input, result
            );
        }
        None => println!("No subcommand was used"),
    }
}
