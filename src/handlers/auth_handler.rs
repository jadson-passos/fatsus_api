use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::auth::{credenciais_validas, gerar_token, ApiKeyAuth};

#[derive(Deserialize, ToSchema)]
#[schema(example = json!({
    "usuario": "string",
    "senha": "string"
}))]
pub struct LoginPayload {
    pub usuario: String,
    pub senha: String,
}

#[derive(Serialize, ToSchema)]
pub struct LoginResponse {
    pub token: String,
}

#[utoipa::path(
    post,
    path = "/api/login",
    params(
        ("X-API-Key" = String, Header, description = "Chave da aplicação cliente", example = "fatsus-delphi-local-api-key-min-32-chars")
    ),
    request_body = LoginPayload,
    responses(
        (status = 200, description = "Login efetuado com sucesso e token JWT gerado", body = LoginResponse),
        (status = 400, description = "Usuário ou senha em formato inválido"),
        (status = 401, description = "API Key ausente ou credenciais inválidas"),
        (status = 403, description = "API Key inválida"),
        (status = 500, description = "Erro interno no servidor")
    ),
    security(("api_key" = [])),
    tag = "Autenticação"
)]
pub async fn login(
    _api_key: ApiKeyAuth,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let usuario = payload.usuario.trim();
    let senha = payload.senha.trim();
    if usuario.is_empty() || usuario.len() > 80 || senha.is_empty() || senha.len() > 128 {
        return Err(StatusCode::BAD_REQUEST);
    }

    if !credenciais_validas(usuario, senha) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    match gerar_token(usuario) {
        Ok(token) => Ok(Json(LoginResponse { token })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
