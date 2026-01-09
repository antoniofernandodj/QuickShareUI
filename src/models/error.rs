use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ApiError {
    FormDataCreation,
    FormDataAppend,
    RequestBuild(String),
    NetworkError(String),
    JsonParse(String),
    ServerError(u16),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::FormDataCreation => write!(f, "Erro ao criar FormData"),
            ApiError::FormDataAppend => write!(f, "Erro ao adicionar arquivo ao formulário"),
            ApiError::RequestBuild(e) => write!(f, "Erro ao montar requisição: {}", e),
            ApiError::NetworkError(e) => write!(f, "Erro de rede: {}", e),
            ApiError::JsonParse(e) => write!(f, "Erro ao processar resposta: {}", e),
            ApiError::ServerError(code) => write!(f, "Erro no servidor (código {})", code),
        }
    }
}

impl std::error::Error for ApiError {}
