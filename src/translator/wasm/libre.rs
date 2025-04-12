use gloo_utils::format::JsValueSerdeExt;
use wasm_bindgen::prelude::*;

#[derive(Clone, Default)]
#[wasm_bindgen]
pub struct Libre {
    translator: super::Translator,
    // String
    api_key: JsValue,
    // String
    url: JsValue,
    // usize
    alternatives: usize,
}

#[wasm_bindgen]
impl Libre {
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

    #[wasm_bindgen(setter, js_name = setUrl)]
    pub fn set_url(&mut self, url: &str) {
        self.url = JsValue::from(url);
    }

    #[wasm_bindgen(setter, js_name = setAlternatives)]
    pub fn set_alternatives(&mut self, alternatives: usize) {
        self.alternatives = alternatives;
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

    #[wasm_bindgen(getter, js_name = getUrl)]
    pub fn url(&self) -> JsValue {
        self.url.clone()
    }

    #[wasm_bindgen(setter, js_name = getAlternatives)]
    pub fn alternatives(&mut self) -> usize {
        self.alternatives
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

impl From<Libre> for crate::Translator {
    #[inline(always)]
    fn from(wasm_translator: Libre) -> crate::Translator {
        let Libre {
            translator: super::Translator { source, target },
            api_key,
            url,
            alternatives
        } = wasm_translator;

        let source = source.as_string().unwrap();
        let target = target.as_string().unwrap();
        let api_key = api_key.as_string().unwrap();
        let url = url.as_string().unwrap();
        let engine = crate::Engine::Libre(crate::Libre {
            api_key,
            url,
            alternatives,
        });
       
        crate::Translator::with_engine(&source, &target, engine)
    }
}
