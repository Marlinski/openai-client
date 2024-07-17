use openai_dive::v1::api::Client;
use openai_dive::v1::models::WhisperEngine;
use openai_dive::v1::resources::audio::{AudioOutputFormat, AudioTranslationParametersBuilder};
use openai_dive::v1::resources::shared::FileUpload;

#[tokio::main]
async fn main() {
    let api_key = std::env::var("OPENAI_API_KEY").expect("$OPENAI_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = AudioTranslationParametersBuilder::default()
        .file(FileUpload::File("./audio/multilingual.mp3".to_string()))
        .model(WhisperEngine::Whisper1.to_string())
        .response_format(AudioOutputFormat::Srt)
        .build()
        .unwrap();

    let result = client.audio().create_translation(parameters).await.unwrap();

    println!("{:#?}", result);
}
