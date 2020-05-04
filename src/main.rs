use structopt::StructOpt;

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
    println!("{:?}", input_args);
}
