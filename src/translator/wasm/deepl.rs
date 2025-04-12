use gloo_utils::format::JsValueSerdeExt;
use wasm_bindgen::prelude::*;

#[derive(Clone, Default)]
#[wasm_bindgen]
pub struct Deepl {
    translator: super::Translator,
    // String
    api_key: JsValue,
    version: super::Version,
    use_free_api: bool,
}

#[wasm_bindgen]
impl Deepl {
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

    #[wasm_bindgen(setter, js_name = setApiKey)]
    pub fn set_api_key(&mut self, api_key: &str) {
        self.api_key = JsValue::from(api_key);
    }

    #[wasm_bindgen(setter, js_name = setVersion)]
    pub fn set_version(&mut self, version: super::Version) {
        self.version = version;
    }

    #[wasm_bindgen(setter, js_name = setUseFreeApi)]
    pub fn set_use_free_api(&mut self, use_free_api: bool) {
        self.use_free_api = use_free_api;
    }

    #[wasm_bindgen(getter, js_name = getSource)]
    pub fn source(&self) -> JsValue {
        self.translator.source.clone()
    }

    #[wasm_bindgen(getter, js_name = getTarget)]
    pub fn target(&self) -> JsValue {
        self.translator.target.clone()
    }

    #[wasm_bindgen(getter, js_name = getApiKey)]
    pub fn api_key(&self) -> JsValue {
        self.api_key.clone()
    }

    #[wasm_bindgen(getter, js_name = getVersion)]
    pub fn version(&self) -> super::Version {
        self.version
    }

    #[wasm_bindgen(getter, js_name = getUseFreeApi)]
    pub fn use_free_api(&self) -> bool {
        self.use_free_api
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

impl From<Deepl> for crate::Translator {
    #[inline(always)]
    fn from(wasm_translator: Deepl) -> crate::Translator {
        let Deepl {
            translator: super::Translator { source, target },
            api_key,
            version,
            use_free_api,
        } = wasm_translator;

        let source = source.as_string().unwrap();
        let target = target.as_string().unwrap();
        let api_key = api_key.as_string().unwrap();
        let version = crate::Version::from(version);

        let engine = crate::Engine::Deepl {
            api_key,
            version,
            use_free_api,
        };

        crate::Translator::with_engine(&source, &target, engine)
    }
}
