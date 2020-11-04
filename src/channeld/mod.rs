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

#[cfg(feature = "shell")]
mod opts;
//pub mod persistance;
mod runtime;
#[allow(dead_code)]
pub(self) mod storage;

#[cfg(feature = "shell")]
pub use opts::Opts;
pub use runtime::run;

// TODO: Replace with more generic persistance API

/*
pub struct ChannelTx {
    pub funding: OutPoint,
    pub commitment: Psbt,
    pub offered: HashMap<u16, Psbt>,
    pub received: HashMap<u16, Psbt>,
}
*/

// TODO: Move to LNP/BP Core Library

use lnpbp::bp::chain::AssetId;
use lnpbp::lnp::{PaymentHash, PaymentPreimage};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct HtlcKnown {
    pub preimage: PaymentPreimage,
    pub id: u64,
    pub cltv_expiry: u32,
    pub asset_id: Option<AssetId>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct HtlcSecret {
    pub preimage: PaymentHash,
    pub id: u64,
    pub cltv_expiry: u32,
    pub asset_id: Option<AssetId>,
}
