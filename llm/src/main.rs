use crate::model::carregar_tokenizer;

mod server;
mod model;
mod sampling;
mod config;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let modelo = model::carregar_model()?;
    let tokenizer = model::carregar_tokenizer()?;

    model::llm(modelo.weights, &tokenizer, "ola é um teste".to_string())?;

    server::rodar().await;

    Ok(())
}
