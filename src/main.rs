use crate::pages::Collection;
use bincode;
use std::env;
use std::fs::File;
use structopt::StructOpt;

mod pages;
mod serialization;

const DEFAULT_STORAGE_PATH: &str = "./journal.bin";

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
    // Indicates if the command-line operation mutates data.
    let is_mutation = match input_args {
        Journal::List => false,
        _ => true,
    };
    // Defining the path of the file used for persistence.
    let fpath = env::var("JOURNAL_STORAGE_PATH").unwrap_or(String::from(DEFAULT_STORAGE_PATH));
    // Reading an existent Collection from file or creating a new one.
    let mut coll = {
        match File::open(&fpath) {
            Ok(file) => bincode::deserialize_from(file).unwrap(),
            Err(_) => Collection::new(),
        }
    };
    // Execute the action given by the command-line.
    exec_action(&mut coll, input_args);
    // If the action changes or adds data, we save a new file (or overwrite one).
    if is_mutation {
        let mut file = File::create(fpath).unwrap();
        bincode::serialize_into(&mut file, &coll).unwrap();
    }
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
