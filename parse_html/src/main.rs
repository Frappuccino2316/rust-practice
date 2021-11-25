use structopt::StructOpt;

fn main() {
    let args: Cli = Cli::from_args();
    let sentence: &String = &args.pattern;
    for i in sentence.chars() {
        println!("{}", i);
    }
}

#[derive(StructOpt)]
struct Cli {
    pattern: String,
}
