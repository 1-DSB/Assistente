use std::fs::File;
use candle_core::quantized::gguf_file;
use memmap2::Mmap;
use std::time::{Duration, Instant};
use candle_transformers::models::quantized_qwen3::ModelWeights;
use candle_transformers::*;
use candle_core::{Device, Tensor};
use tokenizers::Tokenizer;

pub struct Model {
    pub mmap: Mmap,
    pub weights: ModelWeights,
    pub tokens: Vec<candle_core::quantized::gguf_file::Value>,
}

pub fn carregar_tokenizer() -> Result<Tokenizer,Box<dyn std::error::Error>> {
    let tokenizer = Tokenizer::from_file("models/tokenizer.json")
        .map_err(|e| e.to_string())?;

    Ok(tokenizer)
}

pub fn carregar_model() -> Result<Model, Box<dyn std::error::Error>> {
    let contador = Instant::now();
    let arquivo = File::open("models/Qwen3-0.6B-Q8_0.gguf")?;
    let mmap = unsafe { Mmap::map(&arquivo)? };
    let mut cursor = std::io::Cursor::new(&mmap[..]);
    let content = gguf_file::Content::read(&mut cursor)?;
    println!("tempo metadados: {:?}", contador.elapsed());
    let contador2 = Instant::now();
    let device = Device::Cpu;
    let tokens = match content.metadata
        .get("tokenizer.ggml.tokens").unwrap(){
        gguf_file::Value::Array(tokens) => tokens.clone(), _ => return Err("não é lista".into()),
    };

    for (id, token) in tokens.iter().enumerate().take(500) {
        let vocab: Vec<String> = tokens
            .iter()
            .filter_map(|token| {
                match token {
                    gguf_file::Value::String(s) => Some(s.clone()),
                    _ => None,
                }
            })
            .collect();
    }
    let weights = ModelWeights::from_gguf(content, &mut cursor, &device)?;
    println!("tempo pesos: {:?}", contador2.elapsed());

    Ok(Model { mmap, weights,tokens})
}

pub fn llm(mut model: ModelWeights,tokenizer: &Tokenizer,prompt: String) -> Result<(), Box<dyn std::error::Error>> {
    let encoding = tokenizer.encode(prompt, false)
        .map_err(|e| e.to_string())?;

    let ids = encoding.get_ids();

    let input = Tensor::new(ids, &Device::Cpu)?
        .unsqueeze(0)?;

    let logits = model.forward(&input,0)?;

    println!("Logits: {:?}", logits);

    Ok(())
}