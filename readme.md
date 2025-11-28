# **obs-clip**

[🇧🇷 Português](#português-brasil) · [🇺🇸 English](#english)

<p align="left">
  <img src="https://img.shields.io/badge/Rust-stable-orange?style=for-the-badge" />
  <img src="https://img.shields.io/badge/OBS-WebSocket%20v5+-brightgreen?style=for-the-badge" />
  <img src="https://img.shields.io/badge/Wayland-Friendly-blue?style=for-the-badge" />
  <img src="https://img.shields.io/badge/License-MIT-purple?style=for-the-badge" />
  <img src="https://img.shields.io/badge/Build-Cargo-success?style=for-the-badge" />
</p>

Um utilitário simples em **Rust** para controlar o **Replay Buffer do OBS** externamente.  
A ideia é permitir criar clipes instantaneamente usando qualquer sistema de atalhos global — especialmente útil em ambientes **Wayland**, onde os atalhos nativos do OBS frequentemente falham.

A simple **Rust** utility to control the **OBS Replay Buffer** externally.  
Designed mainly to provide reliable clip creation on **Wayland**, where OBS native hotkeys often do not work properly.

---

# 🇧🇷 Português (Brasil)

## 📦 Instalação

Existem duas formas de instalar:

### 🔹 **1. Instalar diretamente via Cargo**
Se você clonou o repositório:

```
cargo install --path .
```

O binário será instalado automaticamente em `~/.cargo/bin/`.

---

### 🔹 **2. Compilar manualmente e copiar para o PATH**

Compile em modo release:

```
cargo build --release
```

Copie para uma pasta acessível no PATH (ex.: `~/.local/bin`):

```
mkdir -p ~/.local/bin
cp target/release/obs-clip ~/.local/bin/
```

---

## 📌 O que o utilitário faz

Comportamento:

- Se o **Replay Buffer estiver desativado**, ele **o ativa**.
- Se o **Replay Buffer estiver ativo**, o utilitário **salva um clipe**.

Um comportamento simples que resolve os problemas de hotkeys no Wayland.

---

## ⚙️ Configuração

### 1. Ativar WebSocket no OBS  
No OBS:

**Ferramentas → WebSocket Server**

- Habilite o WebSocket  
- Verifique a porta  
- Configure uma senha

---

### 2. Criar o arquivo `.env`

```
OBS_PASSWORD=SUA_SENHA_AQUI
OBS_HOST=localhost
OBS_PORT=4455
```

---

## 🛠️ Tecnologias utilizadas

### Linguagem
- Rust

### Crates
- `obws` — integração com OBS via WebSocket  
- `dotenv` — leitura de variáveis do `.env`  
- `anyhow` — tratamento de erros  
- `tokio` — runtime assíncrono

---

## 🎯 Motivação

Criei este utilitário porque os atalhos nativos do OBS costumam ser pouco confiáveis no Wayland.
Um pequeno binário em Rust junto com o OBS WebSocket oferece uma solução simples e consistente, funcionando com qualquer sistema de atalhos do desktop.
Depois de adicioná-lo aos atalhos globais do KDE, meu problema foi completamente resolvido.

---

# 🇺🇸 English

## 📦 Installation

You can install it in two ways:

### 🔹 **1. Install directly using Cargo**

If you cloned the repository:

```
cargo install --path .
```

This installs the binary into `~/.cargo/bin/`.

---

### 🔹 **2. Build manually and copy into your PATH**

Build in release mode:

```
cargo build --release
```

Copy the binary to a directory in your PATH:

```
mkdir -p ~/.local/bin
cp target/release/obs-clip ~/.local/bin/
```

---

## 📌 What this utility does

Behavior:

- If the **Replay Buffer is disabled**, the tool **enables it**.
- If the **Replay Buffer is enabled**, it **saves a clip**.

A simple and reliable workaround for OBS hotkey issues on Wayland.

---

## ⚙️ Setup

### 1. Enable WebSocket in OBS  
In OBS:

**Tools → WebSocket Server**

- Enable WebSocket  
- Check the port  
- Set a password

---

### 2. Create the `.env` file

```
OBS_PASSWORD=YOUR_PASSWORD_HERE
OBS_HOST=localhost
OBS_PORT=4455
```

---

## 🛠️ Technologies used

### Language
- Rust

### Crates
- `obws` — OBS WebSocket integration  
- `dotenv` — loads environment variables  
- `anyhow` — error handling  
- `tokio` — async runtime

---

## 🎯 Motivation

I created this utility because OBS native hotkeys are often unreliable on Wayland.
A lightweight Rust binary combined with OBS WebSocket provides a clean, consistent solution that works with any desktop shortcut system.
After adding it to my KDE global shortcuts, my issue was fully resolved.

---
