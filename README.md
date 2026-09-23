# Assistente
Esse é um projeto realizado pelo 1°DSB da ETEC de Itaquera com o objetivo de aprender conceitos básicos de sistemas embarcados tentando recriar em pequena escala um assistente de voz.

AINDA NÃO ESTA FUNCIONAL

instale: python, rust e git

depois disso voce clona no cmd "git clone https://github.com/1-DSB/Assistente.git" depois muda o diretorio "cd assistente"

depois "cd llm" e "cargo build"

NO PROJETO ESTAMOS USANDO PROVISORIAMENTE O MODELO Qwen3-0.6B-Q8_0.gguf QUE DA PARA ACHAR ELE AQUI https://huggingface.co/unsloth/Qwen3-0.6B-GGUF/blob/main/Qwen3-0.6B-Q8_0.gguf VOCE BAIXA E COLOCA NA PASTA llm/models

apos isso "cd .." e "cd voz" aqui voce roda "pip install -r requirements.txt" 

para usar o projeto:

cd llm

cargo run

- em outra aba

cd voz

python main.py
