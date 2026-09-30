use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};

use crate::{
    auth::AuthenticatedUser,
    db::DbService,
    models::Clinica,
    services::{
        paginacao::{ListQuery, PaginaClinicas},
        ClinicaService,
    },
};

#[utoipa::path(
    get,
    path = "/api/clinicas",
    params(
        ListQuery,
        ("X-API-Key" = String, Header, description = "Chave da aplicação cliente", example = "fatsus-delphi-local-api-key-min-32-chars"),
        ("X-Access-Token" = String, Header, description = "Cole só o token do login (sem a palavra Bearer)", example = "")
    ),
    responses(
        (status = 200, description = "Página de clínicas cadastradas", body = PaginaClinicas),
        (status = 401, description = "Token JWT não fornecido ou inválido"),
        (status = 403, description = "API Key inválida")
    ),
    security(("jwt_token" = [], "api_key" = [])),
    tag = "Clínicas"
)]
pub async fn listar(
    _auth: AuthenticatedUser,
    State(db): State<DbService>,
    Query(query): Query<ListQuery>,
) -> Result<Json<PaginaClinicas>, StatusCode> {
    ClinicaService::listar(&db, query)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

#[utoipa::path(
    get,
    path = "/api/clinicas/{documento}",
    params(
        ("documento" = String, Path, description = "CNES ou CNPJ da clínica"),
        ("X-API-Key" = String, Header, description = "Chave da aplicação cliente", example = "fatsus-delphi-local-api-key-min-32-chars"),
        ("X-Access-Token" = String, Header, description = "Cole só o token do login (sem a palavra Bearer)", example = "")
    ),
    responses(
        (status = 200, description = "Clínica encontrada", body = Clinica),
        (status = 400, description = "Documento inválido"),
        (status = 404, description = "Clínica não encontrada")
    ),
    security(("jwt_token" = [], "api_key" = [])),
    tag = "Clínicas"
)]
pub async fn buscar_por_cnes_ou_cnpj(
    _auth: AuthenticatedUser,
    State(db): State<DbService>,
    Path(documento): Path<String>,
) -> Result<Json<Clinica>, StatusCode> {
    if !documento.chars().any(|c| c.is_ascii_digit()) {
        return Err(StatusCode::BAD_REQUEST);
    }

    match ClinicaService::buscar_por_cnes_ou_cnpj(&db, &documento).await {
        Ok(Some(clinica)) => Ok(Json(clinica)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[utoipa::path(
    post,
    path = "/api/clinicas",
    params(
        ("X-API-Key" = String, Header, description = "Chave da aplicação cliente", example = "fatsus-delphi-local-api-key-min-32-chars"),
        ("X-Access-Token" = String, Header, description = "Cole só o token do login (sem a palavra Bearer)", example = "")
    ),
    request_body = Clinica,
    responses(
        (status = 201, description = "Clínica cadastrada com sucesso"),
        (status = 500, description = "Erro interno")
    ),
    security(("jwt_token" = [], "api_key" = [])),
    tag = "Clínicas"
)]
pub async fn criar(
    _auth: AuthenticatedUser,
    State(db): State<DbService>,
    Json(payload): Json<Clinica>,
) -> Result<(StatusCode, Json<Value>), StatusCode> {
    match ClinicaService::criar(&db, payload).await {
        Ok(id) => Ok((StatusCode::CREATED, Json(json!({ "id": id })))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[utoipa::path(
    put,
    path = "/api/clinicas/{documento}",
    params(
        ("documento" = String, Path, description = "CNES ou CNPJ da clínica a atualizar"),
        ("X-API-Key" = String, Header, description = "Chave da aplicação cliente", example = "fatsus-delphi-local-api-key-min-32-chars"),
        ("X-Access-Token" = String, Header, description = "Cole só o token do login (sem a palavra Bearer)", example = "")
    ),
    request_body = Clinica,
    responses(
        (status = 200, description = "Clínica atualizada com sucesso"),
        (status = 400, description = "Documento inválido"),
        (status = 404, description = "Clínica não encontrada")
    ),
    security(("jwt_token" = [], "api_key" = [])),
    tag = "Clínicas"
)]
pub async fn atualizar(
    _auth: AuthenticatedUser,
    State(db): State<DbService>,
    Path(documento): Path<String>,
    Json(payload): Json<Clinica>,
) -> Result<StatusCode, StatusCode> {
    if !documento.chars().any(|c| c.is_ascii_digit()) {
        return Err(StatusCode::BAD_REQUEST);
    }

    match ClinicaService::atualizar_por_cnes_ou_cnpj(&db, &documento, payload).await {
        Ok(true) => Ok(StatusCode::OK),
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[utoipa::path(
    delete,
    path = "/api/clinicas/{documento}",
    params(
        ("documento" = String, Path, description = "CNES ou CNPJ da clínica a remover"),
        ("X-API-Key" = String, Header, description = "Chave da aplicação cliente", example = "fatsus-delphi-local-api-key-min-32-chars"),
        ("X-Access-Token" = String, Header, description = "Cole só o token do login (sem a palavra Bearer)", example = "")
    ),
    responses(
        (status = 204, description = "Clínica removida com sucesso"),
        (status = 400, description = "Documento inválido"),
        (status = 404, description = "Clínica não encontrada")
    ),
    security(("jwt_token" = [], "api_key" = [])),
    tag = "Clínicas"
)]
pub async fn apagar(
    _auth: AuthenticatedUser,
    State(db): State<DbService>,
    Path(documento): Path<String>,
) -> Result<StatusCode, StatusCode> {
    if !documento.chars().any(|c| c.is_ascii_digit()) {
        return Err(StatusCode::BAD_REQUEST);
    }

    match ClinicaService::deletar_por_cnes_ou_cnpj(&db, &documento).await {
        Ok(true) => Ok(StatusCode::NO_CONTENT),
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
