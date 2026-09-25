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

use std::io::{self, Write};

pub fn llm(
    mut model: ModelWeights,
    tokenizer: &Tokenizer,
    prompt: String
) -> Result<(), Box<dyn std::error::Error>> {

    let encoding = tokenizer.encode(prompt, false)
        .map_err(|e| e.to_string())?;

    let mut tokens = encoding.get_ids().to_vec();

    let eos_token_id = tokenizer
        .get_vocab(true)
        .get("<|im_end|>")
        .copied()
        .ok_or("EOS não encontrado")?;

    let input = Tensor::new(tokens.as_slice(), &Device::Cpu)?
    .unsqueeze(0)?;

    let logits = model.forward(&input, 0)?;

    let mut next_token = logits
    .flatten_all()?
    .argmax(0)?
    .to_scalar::<u32>()?;

    let mut offset = tokens.len();

for _ in 0..100 {
    if next_token == eos_token_id {
        break;
    }

    let texto = tokenizer
        .decode(&[next_token], true)
        .map_err(|e| e.to_string())?;

    print!("{}", texto);
    std::io::stdout().flush()?;

    tokens.push(next_token);

    let input = Tensor::new(&[next_token], &Device::Cpu)?
        .unsqueeze(0)?;

    use std::time::Instant;

let inicio = Instant::now();

let logits = model.forward(&input, offset)?;

println!("forward: {:?}", inicio.elapsed());

    next_token = logits
        .flatten_all()?
        .argmax(0)?
        .to_scalar::<u32>()?;

    offset += 1;
}

    Ok(())
}