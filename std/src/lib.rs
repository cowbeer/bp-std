// Modern, minimalistic & standard-compliant cold wallet library.
//
// SPDX-License-Identifier: Apache-2.0
//
// Written in 2020-2023 by
//     Dr Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// Copyright (C) 2020-2023 LNP/BP Standards Association. All rights reserved.
// Copyright (C) 2020-2023 Dr Maxim Orlovsky. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[macro_use]
extern crate amplify;
#[cfg(feature = "serde")]
#[macro_use]
extern crate serde_crate as serde;
pub extern crate bitcoin_hashes as hashes;

mod base58;
mod index;
mod path;
mod key;
mod xpub;
mod descriptors;
mod address;
mod derive;
mod chain;
mod wallet;

pub use address::{
    Address, AddressNetwork, AddressParseError, AddressPayload, AddressType, PubkeyHash,
    ScriptHash, WPubkeyHash, WScriptHash,
};
pub use bc::{secp256k1, *};
pub use chain::{AddrInfo, BlockInfo, MiningInfo, TxInInfo, TxInfo, TxOutInfo, TxStatus, UtxoInfo};
pub use derive::{Derive, DeriveCompr, DeriveSet, DeriveSpk, DeriveXOnly};
pub use descriptors::{DescriptorStd, TrKey};
pub use index::{
    DerivationIndex, HardenedIndex, Idx, IndexError, IndexParseError, NormalIndex,
    HARDENED_INDEX_BOUNDARY,
};
pub use key::{ComprPubkey, LegacyPubkey, TaprootPubkey, UncomprPubkey};
pub use path::{DerivationParseError, DerivationPath};
pub use wallet::{Wallet, WalletCache, WalletData, WalletDescr};
pub use xpub::{Xpub, XpubDescriptor, XpubFp, XpubId, XpubMeta, XpubOrigin};