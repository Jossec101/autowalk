use autowalk::data::Args;
use autowalk::run;
use clap::StructOpt;
use log::{debug, info};
use simple_logger::SimpleLogger;

fn main() {
    let args = Args::parse();

    if args.debug {
        SimpleLogger::new()
            .with_level(log::LevelFilter::Debug)
            .with_local_timestamps()
            .init()
            .unwrap();
    } else {
        SimpleLogger::new()
            .with_level(log::LevelFilter::Info)
            .with_local_timestamps()
            .init()
            .unwrap();
    }
    info!("Program start");

    debug!("----------------");
    debug!("{:?}", args);

    run(&args);
}
