mod auth;
mod db;
mod docs;
mod handlers;
mod models;
mod services;

use axum::{
    routing::{get, post},
    Router,
};
use db::DbService;
use handlers::{
    auth_handler, clinica_handler, exame_handler, medico_handler, paciente_handler,
};
use std::net::SocketAddr;
use std::time::Duration;
use tower_http::{
    compression::CompressionLayer, limit::RequestBodyLimitLayer, timeout::TimeoutLayer,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    auth::init();

    let db_service = DbService::init().await;

    let app = Router::new()
        // Interface Gráfica do Swagger UI e arquivo JSON da documentação
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", docs::ApiDoc::openapi()))
        // Rotas da API
        .route("/api/login", post(auth_handler::login))
        .route(
            "/api/medicos",
            get(medico_handler::listar).post(medico_handler::criar),
        )
        .route(
            "/api/medicos/:documento",
            get(medico_handler::buscar_por_cns_ou_cpf)
                .put(medico_handler::atualizar)
                .delete(medico_handler::apagar),
        )
        .route(
            "/api/pacientes",
            get(paciente_handler::listar).post(paciente_handler::criar),
        )
        .route(
            "/api/pacientes/:documento",
            get(paciente_handler::buscar_por_cns_ou_cpf)
                .put(paciente_handler::atualizar)
                .delete(paciente_handler::apagar),
        )
        .route(
            "/api/exames",
            get(exame_handler::listar).post(exame_handler::criar),
        )
        .route(
            "/api/exames/:cod_interno",
            get(exame_handler::buscar_por_cod_interno)
                .put(exame_handler::atualizar)
                .delete(exame_handler::apagar),
        )
        .route(
            "/api/clinicas",
            get(clinica_handler::listar).post(clinica_handler::criar),
        )
        .route(
            "/api/clinicas/:documento",
            get(clinica_handler::buscar_por_cnes_ou_cnpj)
                .put(clinica_handler::atualizar)
                .delete(clinica_handler::apagar),
        )
        .layer(CompressionLayer::new())
        .layer(RequestBodyLimitLayer::new(256 * 1024))
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .with_state(db_service);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("API fatsus_api executando em http://{}", addr);
    println!("Documentação Swagger UI disponível em http://{}/swagger-ui/", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}