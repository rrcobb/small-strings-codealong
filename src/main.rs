pub mod sample;
pub mod report;
pub mod alloc;

#[global_allocator]
pub static ALLOCATOR: alloc::Tracing = alloc::Tracing::new();

use argh::FromArgs;

#[derive(FromArgs)]
/// Small string demo
struct Args {
    #[argh(subcommand)]
    subcommand: Subcommand,
}

#[derive(FromArgs)]
#[argh(subcommand)]
enum Subcommand {
    Sample(sample::Sample),
    Report(report::Report),
}

impl Subcommand {
    fn run(self) {
        match self {
            Subcommand::Sample(x) => x.run(),
            Subcommand::Report(x) => x.run(),
        }
    }
}

fn main() {
    argh::from_env::<Args>().subcommand.run();
}
