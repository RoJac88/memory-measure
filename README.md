
# Memory Measure 📊

## Overview
Memory Measure is a tool designed to measure and analyze memory usage of a specified process. This repository provides the source code and documentation for the Memory Measure project, aimed at helping developers monitor and optimize memory consumption in their applications. 🚀

## Features
- **Memory Usage Tracking**: Monitor real-time memory consumption of a specified process. 📈
- **Detailed Reporting**: Generate detailed reports on memory allocation and usage patterns, stored in a CSV file with one line per process ID (PID) per measurement. 📄
- **Cross-Platform Support**: Compatible with multiple operating systems (Windows, Linux, macOS). 🖥️
- **Lightweight**: Minimal overhead to ensure accurate measurements without impacting performance. ⚡

## Getting Started
1. **Download the Binary**: Visit the [Releases](https://github.com/RoJac88/memory-measure/releases) page and download the appropriate compiled binary for your operating system. 📥
2. **Extract the Binary**: Unzip the downloaded file to a directory of your choice. 📂
3. **Run the Tool**: Use the command line to execute the binary as described in the Usage section. ▶️

## Usage
Run the Memory Measure tool to start monitoring memory usage of a process:
```bash
./memory-measure <process_name> [interval]
```

### Arguments
- `<process_name>`: The name of the process to monitor (e.g., `firefox`). 🖱️
- `[interval]`: The interval between measurements in seconds (e.g., `2` for 2 seconds). Defaults to 20 seconds if not specified. ⏱️

### Output
Measurements are stored in a CSV file, with one line per PID per measurement, allowing easy analysis and visualization. 📑

Example:
```bash
./memory-measure firefox 2
```

This command monitors the memory usage of the Firefox process, taking measurements every 2 seconds and saving them to a CSV file.

Example with default interval:
```bash
./memory-measure firefox
```

This uses the default 20-second interval for measurements.

## Build from Source
To build Memory Measure from source, you need to have Rust and Cargo installed. Follow these steps:

1. **Install Rust and Cargo**:
   - Download and install Rust using `rustup` by running:
     ```bash
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```
   - Follow the on-screen instructions to complete the installation.
   - Verify the installation:
     ```bash
     rustc --version
     cargo --version
     ```

2. **Clone the Repository**:
   ```bash
   git clone https://github.com/RoJac88/memory-measure.git
   cd memory-measure
   ```

3. **Build the Project**:
   ```bash
   cargo build --release
   ```
   The compiled binary will be located in the `target/release` directory.

4. **Run the Project**:
   ```bash
   cargo run --release -- <process_name> [interval]
   ```
   Example:
   ```bash
   cargo run --release -- firefox 2
   ```

## Contact
For questions or support, please open an issue on the GitHub repository or contact the maintainer at [rodrigo.jacob@gmail.com](mailto:rodrigo.jacob@gmail.com). 📧

---

# Medidor de Memória 📊

## Visão Geral
O Medidor de Memória é uma ferramenta projetada para medir e analisar o uso de memória de um processo especificado. Este repositório contém o código-fonte e a documentação do projeto Medidor de Memória, com o objetivo de ajudar desenvolvedores a monitorar e otimizar o consumo de memória em seus aplicativos. 🚀

## Funcionalidades
- **Rastreamento de Uso de Memória**: Monitore o consumo de memória em tempo real de um processo especificado. 📈
- **Relatórios Detalhados**: Gere relatórios detalhados sobre alocação e padrões de uso de memória, armazenados em um arquivo CSV com uma linha por ID de processo (PID) por medição. 📄
- **Suporte Multiplataforma**: Compatível com vários sistemas operacionais (Windows, Linux, macOS). 🖥️
- **Leve**: Baixo impacto para garantir medições precisas sem afetar o desempenho. ⚡

## Começando
1. **Baixe o Binário**: Visite a página de [Releases](https://github.com/RoJac88/memory-measure/releases) e baixe o binário compilado apropriado para o seu sistema operacional. 📥
2. **Extraia o Binário**: Descompacte o arquivo baixado em um diretório de sua escolha. 📂
3. **Execute a Ferramenta**: Use a linha de comando para executar o binário conforme descrito na seção de Uso. ▶️

## Uso
Execute a ferramenta Medidor de Memória para começar a monitorar o uso de memória de um processo:
```bash
./memory-measure <nome_do_processo> [intervalo]
```

### Argumentos
- `<nome_do_processo>`: O nome do processo a ser monitorado (por exemplo, `firefox`). 🖱️
- `[intervalo]`: O intervalo entre as medições em segundos (por exemplo, `2` para 2 segundos). O padrão é 20 segundos se não especificado. ⏱️

### Saída
As medições são armazenadas em um arquivo CSV, com uma linha por PID por medição, permitindo fácil análise e visualização. 📑

Exemplo:
```bash
./memory-measure firefox 2
```

Este comando monitora o uso de memória do processo Firefox, realizando medições a cada 2 segundos e salvando-as em um arquivo CSV.

Exemplo com intervalo padrão:
```bash
./memory-measure firefox
```

Este usa o intervalo padrão de 20 segundos para as medições.

## Compilar a Partir do Código-Fonte
Para compilar o Medidor de Memória a partir do código-fonte, você precisa ter o Rust e o Cargo instalados. Siga estes passos:

1. **Instale o Rust e o Cargo**:
   - Baixe e instale o Rust usando o `rustup` executando:
     ```bash
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```
   - Siga as instruções na tela para completar a instalação.
   - Verifique a instalação:
     ```bash
     rustc --version
     cargo --version
     ```

2. **Clone o Repositório**:
   ```bash
   git clone https://github.com/RoJac88/memory-measure.git
   cd memory-measure
   ```

3. **Compile o Projeto**:
   ```bash
   cargo build --release
   ```
   O binário compilado estará localizado no diretório `target/release`.

4. **Execute o Projeto**:
   ```bash
   cargo run --release -- <nome_do_processo> [intervalo]
   ```
   Exemplo:
   ```bash
   cargo run --release -- firefox 2
   ```

## Contato
Para perguntas ou suporte, por favor, abra uma issue no repositório do GitHub ou contate o mantenedor em [rodrigo.jacob@gmail.com](mailto:rodrigo.jacob@gmail.com). 📧
