// Kaleidoscope: RGB command-line wallet utility
// Written in 2019-2020 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//     Alekos Filini <alekos.filini@gmail.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.


use std::path::PathBuf;
use clap::Clap;

use crate::constants::*;
use crate::commands::*;


#[derive(Clap, Clone, Debug, Display)]
#[display_from(Debug)]
#[clap(
    name = "kaleidoscope",
    version = "0.2.0",
    author = "Dr Maxim Orlovsky <orlovsky@pandoracore.com>, Alekos Filini <alekos.filini@gmail.com>",
    about =  "Kaleidoscope: RGB command-line wallet utility"
)]
pub struct Opts {
    /// Sets verbosity level; can be used multiple times to increase verbosity
    #[clap(global=true, short, long,
      min_values=0, max_values=4, parse(from_occurrences))]
    pub verbose: u8,

    /// Data directory for keeping information about keyrings, assets etc
    #[clap(global=true, long, default_value="~/.kaleidoscope", env="KALEIDOSCOPE_DATA_DIR")]
    pub data_dir: PathBuf,

    /// IPC connection string for bp daemon API
    #[clap(global=true, long, default_value=BPD_API_ADDR, env="KALEIDOSCOPE_BPD_API")]
    pub bpd_api: String,

    /// IPC connection string for bp daemon push notifications on transaction
    /// updates
    #[clap(global=true, long, default_value=BPD_PUSH_ADDR, env="KALEIDOSCOPE_BPD_SUBSCR")]
    pub bpd_subscr: String,

    /// Network to use
    #[clap(global=true, short, long, default_value="Testnet", env="KALEIDOSCOPE_NETWORK")]
    pub network: lnpbp::bitcoin::Network,

    #[clap(subcommand)]
    pub command: Command
}


// We need config structure since not all of the parameters can be specified
// via environment and command-line arguments. Thus we need a config file and
// default set of configuration
#[derive(Clone, PartialEq, Eq, Debug, Display)]
#[display_from(Debug)]
pub struct Config {
    pub verbose: u8,
    pub network: lnpbp::bitcoin::Network,
    pub bpd_api: String,
    pub bpd_subscr: String,
}

impl From<Opts> for Config {
    fn from(opts: Opts) -> Self {
        Self {
            verbose: opts.verbose,
            network: opts.network,
            bpd_api: opts.bpd_api,
            bpd_subscr: opts.bpd_subscr,

            ..Config::default()
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            verbose: 0,
            network: lnpbp::bitcoin::Network::Testnet,
            bpd_api: BPD_API_ADDR.to_string(),
            bpd_subscr: BPD_PUSH_ADDR.to_string()
        }
    }
}