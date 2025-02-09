use deeptranslator::{Engine, Translator};

#[tokio::main]
async fn main() {
    let translator = Translator::with_engine("es", "en", Engine::Google);

    let translation_result = translator.translate("Saludos a todo el mundo").await;

    println!("{:?}", translation_result);
}
