mod server;
mod model;
mod sampling;
mod config;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let modelo = model::carregar_model()?;
    let tokenizer = model::carregar_tokenizer()?;
    server::rodar(modelo.weights,tokenizer).await;
    Ok(())
}
