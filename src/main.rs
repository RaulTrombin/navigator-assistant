mod cli;
mod data_logger;

fn main() {
    let (directory, filename) = cli::parse_args();
    cli::start_logging(&directory, &filename);
}
