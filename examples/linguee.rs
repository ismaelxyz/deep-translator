use deeptranslator::{Engine, Translator};

#[tokio::main]
async fn main() {
    let engine = Engine::Linguee { return_all: false };

    let translator = Translator::with_engine("german", "english", engine);
    let translation_result = translator.translate("laufen").await;

    println!("{:?}", translation_result);
}
