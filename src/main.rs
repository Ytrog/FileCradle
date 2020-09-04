use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(subcommand)]
    action: Action,
}

#[derive(StructOpt)]
enum Action {
    Add {
        #[structopt(parse(from_os_str))]
        path: std::path::PathBuf,
    },
    Check {
        #[structopt(parse(from_os_str))]
        path: std::path::PathBuf,
    },
    List {},
}

fn dispatch(action : Action) {
    match action {
	Action::Add{path} => println!("Add {:?}", path),
	_ => println!("Other"),
    }
}

fn main() {
    // get env variables
    let args = Cli::from_args();
    // validate vars
    // determine what to do
    dispatch(args.action);
    // implement adding file to hash list
    // implement checking files to hash list
}
