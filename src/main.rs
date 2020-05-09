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
        Journal::List => println!("{} Jornal pages:\n{:#?}", coll.len(), coll),
    };
}

fn main() {
    let input_args = Journal::from_args();

    let mut coll = Collection::new();
    exec_action(&mut coll, input_args);
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
