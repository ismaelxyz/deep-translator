//! Support for web platform

mod google;
mod libre;
mod deepl;
mod linguee;
mod microsoft;
mod mymemory;
mod qcri;
mod papago;

use wasm_bindgen::prelude::*;
pub use google::*;
pub use deepl::*;
pub use libre::*;
pub use linguee::*;
pub use microsoft::*;
pub use mymemory::*;
pub use qcri::*;
pub use papago::*;


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



#[derive(Clone, Default)]
#[wasm_bindgen]
pub struct Pons {
    translator: Translator
}

#[wasm_bindgen]
impl Pons {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    #[wasm_bindgen(setter, js_name = setSource)]
    pub fn set_source(&mut self, source: &str) {
        self.translator.source = JsValue::from(source);
    }

    #[wasm_bindgen(setter, js_name = setTarget)]
    pub fn set_target(&mut self, target: &str) {
        self.translator.target = JsValue::from(target);
    }

    #[wasm_bindgen(getter, js_name = getSource)]
    pub fn source(&self) -> JsValue {
        self.translator.source.clone()
    }

    #[wasm_bindgen(getter, js_name = getTarget)]
    pub fn target(&self) -> JsValue {
        self.translator.target.clone()
    }
}



#[derive(Clone, Default)]
#[wasm_bindgen]
pub struct Yandex {
    translator: Translator,
    // String
    api_key: JsValue,
}

#[wasm_bindgen]
impl Yandex {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    #[wasm_bindgen(setter, js_name = setSource)]
    pub fn set_source(&mut self, source: &str) {
        self.translator.source = JsValue::from(source);
    }

    #[wasm_bindgen(setter, js_name = setTarget)]
    pub fn set_target(&mut self, target: &str) {
        self.translator.target = JsValue::from(target);
    }

    #[wasm_bindgen(getter, js_name = getSource)]
    pub fn source(&self) -> JsValue {
        self.translator.source.clone()
    }

    #[wasm_bindgen(getter, js_name = getTarget)]
    pub fn target(&self) -> JsValue {
        self.translator.target.clone()
    }

    #[wasm_bindgen(setter, js_name = setApiKey)]
    pub fn set_api_key(&mut self, api_key: &str) {
        self.api_key = JsValue::from(api_key);
    }

    #[wasm_bindgen(getter, js_name = getApiKey)]
    pub fn api_key(&self) -> JsValue {
        self.api_key.clone()
    }
    
    // #[wasm_bindgen]
    // pub async fn translate(self, text: &str) -> Result<JsValue, JsValue> {
    //     let translator = crate::Translator::from(self);

    //     translator
    //         .translate(text)
    //         .await
    //         .map_err(|err| JsValue::from(err.to_string()))
    //         .map(|ok| <JsValue as JsValueSerdeExt>::from_serde(&ok).unwrap())
    // }
}
