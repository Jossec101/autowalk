use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, env = "DEBUG")]
    pub debug: bool,

    /// Key to enable the autowalk check list in https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes, and convert hex to int base10 e.g. (0x12 = 18 = ALT key)
    #[clap(short, long, default_value = "18", env = "AUTOWALK_TRIGGER_KEY")]
    pub autowalk_trigger_key: u8,

    /// Interval in milliseconds to trigger the autowalk between two key presses
    #[clap(short, long, default_value = "200", env = "TRIGGER_INTERVAL_MILLIS")]
    pub trigger_interval_millis: i64,
}
