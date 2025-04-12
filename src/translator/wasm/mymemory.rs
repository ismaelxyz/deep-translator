use gloo_utils::format::JsValueSerdeExt;
use wasm_bindgen::prelude::*;

#[derive(Clone, Default)]
#[wasm_bindgen]
pub struct MyMemory {
    translator: super::Translator,
    // String
    email: JsValue,
    return_all: bool,
}

#[wasm_bindgen]
impl MyMemory {
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

    #[wasm_bindgen(setter, js_name = setReturnAll)]
    pub fn set_return_all(&mut self, return_all: bool) {
        self.return_all = return_all;
    }

    #[wasm_bindgen(getter, js_name = getSource)]
    pub fn source(&self) -> JsValue {
        self.translator.source.clone()
    }

    #[wasm_bindgen(getter, js_name = getTarget)]
    pub fn target(&self) -> JsValue {
        self.translator.target.clone()
    }

    #[wasm_bindgen(setter, js_name = setEmail)]
    pub fn set_email(&mut self, email: &str) {
        self.email = JsValue::from(email);
    }

    #[wasm_bindgen(getter, js_name = getEmail)]
    pub fn email(&self) -> JsValue {
        self.email.clone()
    }

    #[wasm_bindgen(getter, js_name = getReturnAll)]
    pub fn return_all(&self) -> bool {
        self.return_all
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

impl From<MyMemory> for crate::Translator {
    #[inline(always)]
    fn from(wasm_translator: MyMemory) -> crate::Translator {
        let MyMemory {
            translator: super::Translator { source, target },
            email,
            return_all,
        } = wasm_translator;

        let source = source.as_string().unwrap();
        let target = target.as_string().unwrap();
        let email = email.as_string().unwrap();

        let engine = crate::Engine::MyMemory { email, return_all };

        crate::Translator::with_engine(&source, &target, engine)
    }
}
