# 📁 Quickshare

Quickshare é um sistema de compartilhamento temporário de arquivos, construído com **Rust**, **Axum** e **Yew**.  
Arquivos enviados expiram automaticamente em 24 horas.  

O projeto possui:

- Frontend em **Yew** (WebAssembly)  
- Backend em **Axum** com armazenamento **MongoDB GridFS**  
- Upload e download de arquivos via API  
- Lista de arquivos com download e cópia de link  
- Indicador de carregamento (spinner) durante upload/download  

---

## ⚡ Funcionalidades

- Upload de arquivos pelo navegador  
- Download de arquivos preservando o nome original  
- Expiração automática de arquivos (TTL: 24 horas)  
- Lista de arquivos enviados  
- Copiar link de download para compartilhamento rápido  

---

## 🛠 Tecnologias

- **Rust**  
- **Yew** – frontend WebAssembly  
- **Axum** – backend HTTP  
- **MongoDB GridFS** – armazenamento de arquivos  
- **gloo-net** – requests HTTP no frontend  
- **Tower HTTP CORS** – suporte a CORS

---

## 🚀 Instalação e execução

### Pré-requisitos

- Rust >= 1.70  
- MongoDB rodando localmente ou remoto
- Trunk, para servir a aplicação

### Backend

1. Configure o MongoDB (`db` e `fs`)  
2. Compile e rode o backend:

```bash
trunk serve
