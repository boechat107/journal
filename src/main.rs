use crate::pages::Page;
use structopt::StructOpt;

mod pages;

#[derive(StructOpt, Debug)]
enum Journal {
    New {
        #[structopt(short)]
        text: String,
    },
    List,
}

fn main() {
    let input_args = Journal::from_args();
    println!("Arguments: {:?}", input_args);

    match input_args {
        Journal::New { text } => println!("New page: {:?}", Page::new(12, text)),
        _ => println!("Another command"),
    }
}
