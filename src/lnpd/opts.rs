// LNP Node: node running lightning network protocol and generalized lightning
// channels.
// Written in 2020 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

use clap::{AppSettings, Clap};

use crate::channeld::RgbOpts;
use crate::peerd::KeyOpts;

/// Lightning node management daemon; part of LNP Node
///
/// The daemon is controlled though ZMQ ctl socket (see `ctl-socket` argument
/// description)
#[derive(Clap, Clone, PartialEq, Eq, Debug)]
#[clap(
    name = "lnpd",
    bin_name = "lnpd",
    author,
    version,
    setting = AppSettings::ColoredHelp
)]
pub struct Opts {
    /// RGB configuration
    #[clap(flatten)]
    pub rgb_opts: RgbOpts,

    /// Node key configuration
    #[clap(flatten)]
    pub key_opts: KeyOpts,

    /// These params can be read also from the configuration file, not just
    /// command-line args or environment variables
    #[clap(flatten)]
    pub shared: crate::opts::Opts,
}

impl Opts {
    pub fn process(&mut self) {
        self.shared.process();
        self.key_opts.process(&self.shared);
        self.rgb_opts.process(&self.shared);
    }
}
