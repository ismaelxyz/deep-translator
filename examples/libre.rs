use deeptranslator::{Engine, Translator};

#[tokio::main]
async fn main() {
    let translator = Translator::with_engine("de", "en", Engine::Libre { 
        api_key: String::new(), url: "https://translate.flossboxin.org.in/translate".to_string()
    });

    let translation_result = translator.translate("laufen").await;

    println!("{:?}", translation_result);
}
