use super::action::Action;
use structopt::StructOpt;

/// A command line connect NJFU library written in Rust
#[derive(Debug, StructOpt)]
#[structopt(
    name = "NJFU-library-cli",
    long_about = "A command line connect NJFU library written in Rust",
    after_help = r##"EXAMPLES:
    njfulib login -u <username> -p <password>
    njfulib query -n <your name>
    njfulib statue
    njfulib reserve [-s <site>...] [-f <floor>...] --start <start time> --end <end time> -r 50
    njfulib reserve -s <space>... --start <start time> --end <end time>
    njfulib cancel -i <id>
    njfulib in -s <site>
    njfulib in -a
    njfulib out -i <id>"##
)]
pub struct Opt {
    #[structopt(subcommand)]
    pub action: Action,
}
