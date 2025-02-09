use deeptranslator::{Engine, Translator};

#[tokio::main]
async fn main() {
    let engine = Engine::MyMemory {
        email: "emailexample@email.com".to_string(),
        return_all: false,
    };

    let translator = Translator::with_engine("ar", "en", engine);
    let translation_result = translator.translate("آخُذ اَلْباص.").await;

    println!("{:?}", translation_result);
}
