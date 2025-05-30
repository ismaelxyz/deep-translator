use gloo_utils::format::JsValueSerdeExt;
use wasm_bindgen::prelude::*;

#[derive(Clone, Default)]
#[wasm_bindgen]
pub struct Google {
    translator: super::Translator,
}

#[wasm_bindgen]
impl Google {
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

impl From<Google> for crate::Translator {
    #[inline(always)]
    fn from(wasm_translator: Google) -> crate::Translator {
        let super::Translator { source, target } = wasm_translator.translator;

        let source = source.as_string().unwrap();
        let target = target.as_string().unwrap();
        let engine = crate::Engine::Google;

        crate::Translator::with_engine(&source, &target, engine)
    }
}
