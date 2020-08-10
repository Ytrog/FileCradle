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

fn main() {
    // get env variables
    let args = Cli::from_args();
    // validate vars
    // determine what to do
    // implement adding file to hash list
    // implement checking files to hash list
}
