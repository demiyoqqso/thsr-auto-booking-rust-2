use clap::Parser;

/// A CLI tool for booking Taiwan High Speed Rail tickets.
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Personal ID. Keep this in Railway Variables as THSR_PERSONAL_ID.
    #[arg(long, short = 'i', value_name = "ID", env = "THSR_PERSONAL_ID")]
    pub personal_id: Option<String>,

    #[arg(long)]
    pub list_station: bool,

    #[arg(long)]
    pub list_time_table: bool,
}
