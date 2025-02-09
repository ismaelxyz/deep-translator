use deeptranslator::{Engine, Translator};

#[tokio::main]
async fn main() {
    // NOTE: Note that the system adds the suffix "/translate" to the URL you use,
    // let me know if that causes problems with any mirrors
    let free_libre_url = "https://translate.flossboxin.org.in".to_string();
    let engine = Engine::Libre {
        api_key: String::new(),
        url: free_libre_url,
        alternatives: 2,
    };

    let translator = Translator::with_engine("de", "en", engine);
    let translation_result = translator.translate("laufen").await;

    println!("{:?}", translation_result);
}
