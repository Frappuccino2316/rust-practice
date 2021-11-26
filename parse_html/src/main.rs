use structopt::StructOpt;

fn main() {
    let args: Cli = Cli::from_args();
    let sentence: &String = &args.pattern;
    
    let mut start_tag: String = String::from("");
    let mut in_tag: bool = false;
    let mut end_tag: String = String::from("");

    for i in sentence.chars() {
        if i == '<' {
            start_tag.push(i);
            in_tag = !in_tag;
            break
        }
        if in_tag {
            start_tag.push(i);
            break
        }
        if i == '>' {
            start_tag.push(i);
            in_tag = !in_tag;
            break
        }
        println!("{}", start_tag);
        println!("{}", end_tag);
        println!("{}", i);
    }
}

#[derive(StructOpt)]
struct Cli {
    pattern: String,
}
