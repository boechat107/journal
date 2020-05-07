use crate::pages::Collection;
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

fn exec_action(coll: &mut Collection, input: Journal) -> () {
    match input {
        Journal::New { text } => {
            println!("New page id: {}", coll.add(text));
            exec_action(coll, Journal::List)
        }
        Journal::List => println!("Jornal pages:\n{:?}", coll),
    };
}

fn main() {
    let input_args = Journal::from_args();
    println!("Arguments: {:?}", input_args);

    let mut coll = Collection::new();
    exec_action(&mut coll, input_args);
}
