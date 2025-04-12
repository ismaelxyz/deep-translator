use gloo_utils::format::JsValueSerdeExt;
use wasm_bindgen::prelude::*;

#[derive(Clone, Default)]
#[wasm_bindgen]
pub struct Microsoft {
    translator: super::Translator,
    // String
    api_key: JsValue,
    // String
    region: JsValue,
}

#[wasm_bindgen]
impl Microsoft {
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

    #[wasm_bindgen(setter, js_name = setRegion)]
    pub fn set_region(&mut self, region: &str) {
        self.region = JsValue::from(region);
    }

    #[wasm_bindgen(getter, js_name = getRegion)]
    pub fn region(&self) -> JsValue {
        self.region.clone()
    }

    #[wasm_bindgen]
    pub async fn translate(self, text: &str) -> Result<JsValue, JsValue> {
        let translator = crate::Translator::from(self);

        translator
            .translate(text)
            .await
            .map_err(|err| JsValue::from(err.to_string()))
            .map(|ok| <JsValue as JsValueSerdeExt>::from_serde(&ok).unwrap())
    }
}


impl From<Microsoft> for crate::Translator {
    #[inline(always)]
    fn from(wasm_translator: Microsoft) -> crate::Translator {
        let Microsoft {
            translator: super::Translator { source, target },
            api_key,
            region,
        } = wasm_translator;

        let source = source.as_string().unwrap();
        let target = target.as_string().unwrap();
        let api_key = api_key.as_string().unwrap();
        let region = region.as_string().unwrap();

        let engine = crate::Engine::Microsoft { api_key, region };

        crate::Translator::with_engine(&source, &target, engine)
    }
}
