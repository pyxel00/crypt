use clap::{Parser, Subcommand};
use crypt::Crypt;


#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command:Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    Encode {
        #[arg(long)]
        base64:bool,
        #[arg(long)]
        base16:bool,
        input:String
    },
    Decode {
        #[arg(long)]
        base64:bool,
        #[arg(long)]
        base16:bool,
        input:String
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Encode { base64, base16 , input}) => {
            if *base16 {
                let encoding = Crypt::encode_16(input);
                println!("{encoding}")
            } else if *base64 {
                let encoding = Crypt::encode_64(input);
                println!("{encoding}")
            }
        }
        Some(Commands::Decode { base64, base16, input }) => {
            if *base16 {
                let decoding = Crypt::decode_16(input);
                println!("{decoding}")
            } else if *base64 {
                let decoding = Crypt::decode_64(input);
                println!("{decoding}")
            }
        }
        None => {}
    }
}
