#![allow(warnings)]

mod server;
mod model;
mod sampling;
mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let modelo = model::carregar_model()?;
    let tokenizer = model::carregar_tokenizer()?;
    println!("Rodando server - Console Output");
    server::rodar(modelo.weights,tokenizer).await;
    Ok(())
}
