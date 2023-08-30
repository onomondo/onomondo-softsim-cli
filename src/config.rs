use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(next_line_help = true)]
#[command(arg_required_else_help(true))]
pub struct Args {
    /// Verbosity level
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbosity: u8,

    #[clap(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    /// Fetch profiles from API
    Fetch {
        #[arg(short, long)]
        api_key: String,
        #[arg(short, long = "count", default_value = "1")]
        num_of_profiles: u32,
        #[arg(short, long = "out", default_value = "profiles")]
        output: PathBuf,
        #[arg(short, long, default_value = "https://api.onomondo.com/sims/profile")]
        endpoint: String,
    },
    /// Decrypt and decode profiles
    Decrypt {
        /// Path to private key
        #[arg(short, long)]
        key: PathBuf,
        #[arg(short, long = "profiles")]
        set_of_profiles: Option<PathBuf>,
        /// decode single profile
        #[arg()]
        profile: Option<String>,
    },
    Next {
        /// Path to private key
        #[arg(short, long)]
        key: PathBuf,
        /// Path to encrypted profiles.
        #[arg(short = 'i', long = "in", default_value = "./profiles")]
        set_of_profiles: Option<PathBuf>,
        /// Output format.
        #[arg(
            long,
            require_equals = true,
            value_name = "FORMAT",
            num_args = 0..=1,
            default_value_t = Format::Hex,
            default_missing_value = "hex",
            value_enum
        )]
        format: Format,
    },
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Format {
    Hex,
    Json,
}

// /// Set persistent configuration. Export path, api endpoint, api key can be set.
//   Configure {
//       /// Export path for encrypted profiles
//       #[arg(short, long = "profiles_path")]
//       set_of_profiles: Option<PathBuf>,
//       /// SoftSIM API key
//       #[arg(short, long)]
//       api_key: Option<String>,
//       /// SoftSIM API endpoint
//       /// Default: https://api.onomondo.com/sims/profile
//       #[arg(short, long)]
//       endpoint: Option<String>,
//   },
