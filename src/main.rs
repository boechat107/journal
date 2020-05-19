use crate::pages::Collection;
use bincode;
use std::fs::File;
use structopt::StructOpt;

mod pages;
mod serialization;

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
        Journal::List => println!("{} Jornal pages:\n{:#?}", coll.len(), coll),
    };
}

fn main() {
    let input_args = Journal::from_args();

    let mut coll = {
        match File::open("./journal.bin") {
            Ok(file) => bincode::deserialize_from(file).unwrap(),
            Err(_) => Collection::new(),
        }
    };
    exec_action(&mut coll, input_args);

    // TODO: There is no problem to overwrite the file, but it is
    // better to do it only for data mutations.
    let mut file = File::create("./journal.bin").unwrap();
    bincode::serialize_into(&mut file, &coll).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exec_action() {
        let mut coll = Collection::new();

        // This action changes the Journal.
        exec_action(
            &mut coll,
            Journal::New {
                text: String::from("my first page"),
            },
        );
        assert_eq!(coll.len(), 1);

        // This action doesn't change anything.
        exec_action(&mut coll, Journal::List);
        assert_eq!(coll.len(), 1);
    }
}
