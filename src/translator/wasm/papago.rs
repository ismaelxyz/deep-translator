use gloo_utils::format::JsValueSerdeExt;
use wasm_bindgen::prelude::*;

#[derive(Clone, Default)]
#[wasm_bindgen]
pub struct Papago {
    translator: super::Translator,
    // String
    client_id: JsValue,
    // String
    secret_key: JsValue,
}

#[wasm_bindgen]
impl Papago {
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

    #[wasm_bindgen(setter, js_name = setClientId)]
    pub fn set_client_id(&mut self, client_id: &str) {
        self.client_id = JsValue::from(client_id);
    }

    #[wasm_bindgen(getter, js_name = getClientId)]
    pub fn client_id(&self) -> JsValue {
        self.client_id.clone()
    }

    #[wasm_bindgen(setter, js_name = setSecretKey)]
    pub fn set_secret_key(&mut self, secret_key: &str) {
        self.secret_key = JsValue::from(secret_key);
    }

    #[wasm_bindgen(getter, js_name = getSecretKey)]
    pub fn secret_key(&self) -> JsValue {
        self.secret_key.clone()
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

impl From<Papago> for crate::Translator {
    #[inline(always)]
    fn from(wasm_translator: Papago) -> crate::Translator {
        let Papago {
            translator: super::Translator { source, target },
            client_id,
            secret_key,
        } = wasm_translator;

        let source = source.as_string().unwrap();
        let target = target.as_string().unwrap();
        let client_id = client_id.as_string().unwrap();
        let secret_key = secret_key.as_string().unwrap();

        let engine = crate::Engine::Papago {
            client_id,
            secret_key,
        };

        crate::Translator::with_engine(&source, &target, engine)
    }
}
