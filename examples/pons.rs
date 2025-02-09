use deeptranslator::{Engine, Translator};

#[tokio::main]
async fn main() {
    let translator = Translator::with_engine("es", "de", Engine::Pons);
    let translation_result = translator.translate("El dia es bello para todos").await;

    println!("{:?}", translation_result);
}
