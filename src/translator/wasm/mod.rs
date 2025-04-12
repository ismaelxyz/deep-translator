//! Support for web platform

mod google;
mod libre;
mod deepl;
mod linguee;
mod microsoft;
mod mymemory;
mod qcri;
mod papago;
mod pons;
mod yandex;

use wasm_bindgen::prelude::*;
pub use google::*;
pub use deepl::*;
pub use libre::*;
pub use linguee::*;
pub use microsoft::*;
pub use mymemory::*;
pub use qcri::*;
pub use papago::*;
pub use pons::*;
pub use yandex::*;


#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
)]
#[wasm_bindgen]
pub enum Version {
    V1,
    #[default]
    V2,
}

impl From<Version> for crate::Version {
    fn from(wasm_version: Version) -> super::Version {
        match wasm_version {
            Version::V1 => super::Version::V1,
            Version::V2 => super::Version::V2,
        }
    }
}


#[derive(Clone, Default)]
#[wasm_bindgen]
pub struct Translator {
    // String
    source: JsValue,
    // String
    target: JsValue,
}

