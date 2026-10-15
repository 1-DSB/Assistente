# Assistente de Voz — 1º DSB

Este projeto está sendo desenvolvido pela turma **1º DSB da ETEC de Itaquera** com o objetivo de aprender conceitos básicos de sistemas embarcados, buscando recriar, em pequena escala, um assistente de voz.

> **Status: Em desenvolvimento. O projeto ainda não está funcional.**

## Pré-requisitos

Antes de começar, instale as seguintes ferramentas:

- [Python](https://www.python.org/downloads/)
- [Rust](https://www.rust-lang.org/tools/install)
- [Git](https://git-scm.com/downloads/)

## 1. Clonar o repositório

Abra o terminal (CMD no Windows ou terminal no Linux) e execute:

```bash
git clone https://github.com/1-DSB/Assistente.git
cd Assistente
```

## 2. Compilar o componente Rust

Entre na pasta `llm` e compile o projeto:

```bash
cd llm
cargo build
```

Aguarde a conclusão da compilação.

## 3. Baixar o modelo de linguagem

Atualmente, estamos utilizando provisoriamente o modelo **Qwen3-0.6B-Q8_0.gguf**.

Baixe o arquivo pelo link abaixo:

[Qwen3-0.6B-Q8_0.gguf — Hugging Face](https://huggingface.co/unsloth/Qwen3-0.6B-GGUF/blob/main/Qwen3-0.6B-Q8_0.gguf)

Após o download, coloque o arquivo na seguinte pasta do projeto:

```text
Assistente/
└── llm/
    └── models/
        └── Qwen3-0.6B-Q8_0.gguf
```

Se a pasta `models` não existir, crie-a manualmente.

## 4. Configurar o ambiente Python

Volte para a pasta raiz do projeto e entre na pasta `python`:

```bash
cd ../python
```

Crie um ambiente virtual:

```bash
python -m venv .venv
```

No Linux, caso o comando falhe por falta do módulo `venv`, instale-o conforme sua distribuição. No Ubuntu, por exemplo:

```bash
sudo apt install python3-venv
```

## 5. Executar o assistente

Execute o arquivo principal usando o interpretador Python do ambiente virtual.

**Windows (CMD):**

```cmd
.\.venv\Scripts\python.exe main.py
```

**Linux:**

```bash
./.venv/bin/python main.py
```

O programa principal está preparado para instalar as dependências Python necessárias durante a inicialização.

### Caso as dependências não sejam instaladas automaticamente

Com o ambiente virtual criado, execute:

**Windows:**

```cmd
.\.venv\Scripts\python.exe -m pip install -r requirements.txt
```

**Linux:**

```bash
./.venv/bin/python -m pip install -r requirements.txt
```

Depois, execute novamente o arquivo `main.py`.

## Tecnologias utilizadas

- **Python:** lógica do assistente e componentes relacionados à interação por voz.
- **Rust:** implementação do componente de processamento da linguagem.
- **Qwen3:** modelo de linguagem utilizado experimentalmente.
- **Git:** controle de versão do projeto.

## Objetivo educacional

O projeto tem como finalidade explorar programação, processamento de linguagem natural e conceitos introdutórios de sistemas embarcados por meio do desenvolvimento de um assistente de voz em pequena escala.