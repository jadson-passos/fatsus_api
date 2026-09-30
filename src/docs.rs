use utoipa::openapi::security::{ApiKey, ApiKeyValue, HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};

use crate::handlers::{
    auth_handler, clinica_handler, exame_handler, medico_handler, paciente_handler,
};
use crate::models::{Clinica, Exame, Medico, Paciente};
use crate::services::paginacao::{PaginaClinicas, PaginaExames, PaginaMedicos, PaginaPacientes};

#[derive(OpenApi)]
#[openapi(
    paths(
        auth_handler::login,
        medico_handler::listar,
        medico_handler::buscar_por_cns_ou_cpf,
        medico_handler::criar,
        medico_handler::atualizar,
        medico_handler::apagar,
        paciente_handler::listar,
        paciente_handler::buscar_por_cns_ou_cpf,
        paciente_handler::criar,
        paciente_handler::atualizar,
        paciente_handler::apagar,
        exame_handler::listar,
        exame_handler::buscar_por_cod_interno,
        exame_handler::criar,
        exame_handler::atualizar,
        exame_handler::apagar,
        clinica_handler::listar,
        clinica_handler::buscar_por_cnes_ou_cnpj,
        clinica_handler::criar,
        clinica_handler::atualizar,
        clinica_handler::apagar,
    ),
    components(
        schemas(
            auth_handler::LoginPayload,
            auth_handler::LoginResponse,
            Medico,
            Paciente,
            Exame,
            Clinica,
            PaginaMedicos,
            PaginaPacientes,
            PaginaExames,
            PaginaClinicas
        )
    ),
    tags(
        (name = "Autenticação", description = "Endpoints de login e autenticação"),
        (name = "Médicos", description = "Serviço de gestão de médicos"),
        (name = "Pacientes", description = "Serviço de gestão de pacientes"),
        (name = "Exames", description = "Serviço de gestão de exames"),
        (name = "Clínicas", description = "Serviço de gestão de clínicas")
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "jwt_token",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("X-API-Key"))),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utoipa::OpenApi;

    #[test]
    fn swagger_contem_rotas_schemas_e_paginacao() {
        let spec = ApiDoc::openapi();
        let json = serde_json::to_value(&spec).expect("openapi serializa");
        let paths = json["paths"].as_object().expect("paths");

        for path in [
            "/api/login",
            "/api/medicos",
            "/api/medicos/{documento}",
            "/api/pacientes",
            "/api/pacientes/{documento}",
            "/api/exames",
            "/api/exames/{cod_interno}",
            "/api/clinicas",
            "/api/clinicas/{documento}",
        ] {
            assert!(paths.contains_key(path), "Swagger sem a rota {path}");
        }

        let login_params = json["paths"]["/api/login"]["post"]["parameters"]
            .as_array()
            .expect("POST /api/login deve declarar parâmetros");
        let login_param_names: Vec<&str> = login_params
            .iter()
            .map(|p| p["name"].as_str().unwrap())
            .collect();
        assert!(
            login_param_names.contains(&"X-API-Key"),
            "POST /api/login deve expor o header X-API-Key no Swagger"
        );

        let get_medicos = &json["paths"]["/api/medicos"]["get"];
        assert!(get_medicos.get("parameters").is_some());
        let param_names: Vec<&str> = get_medicos["parameters"]
            .as_array()
            .unwrap()
            .iter()
            .map(|p| p["name"].as_str().unwrap())
            .collect();
        assert!(param_names.contains(&"limit"), "GET /api/medicos sem query limit");
        assert!(param_names.contains(&"cursor"), "GET /api/medicos sem query cursor");
        assert!(
            param_names.contains(&"X-API-Key") && param_names.contains(&"X-Access-Token"),
            "GET /api/medicos deve expor X-API-Key e X-Access-Token no Swagger"
        );

        for list_path in ["/api/pacientes", "/api/exames", "/api/clinicas"] {
            let names: Vec<&str> = json["paths"][list_path]["get"]["parameters"]
                .as_array()
                .unwrap()
                .iter()
                .map(|p| p["name"].as_str().unwrap())
                .collect();
            assert!(
                names.contains(&"X-API-Key") && names.contains(&"X-Access-Token"),
                "{list_path} deve expor X-API-Key e X-Access-Token no Swagger"
            );
        }

        let schemas = json["components"]["schemas"].as_object().expect("schemas");
        for schema in [
            "LoginPayload",
            "LoginResponse",
            "Medico",
            "Paciente",
            "Exame",
            "Clinica",
            "PaginaMedicos",
            "PaginaPacientes",
            "PaginaExames",
            "PaginaClinicas",
        ] {
            assert!(schemas.contains_key(schema), "Swagger sem schema {schema}");
        }

        let pagina = &schemas["PaginaMedicos"]["properties"];
        assert!(pagina.get("data").is_some());
        assert!(pagina.get("nextCursor").is_some(), "PaginaMedicos deve usar camelCase nextCursor");
        assert!(pagina.get("hasMore").is_some());
        assert!(pagina.get("limit").is_some());

        let login = &schemas["LoginPayload"];
        let required = login["required"]
            .as_array()
            .expect("LoginPayload.required");
        let required_names: Vec<&str> = required.iter().map(|v| v.as_str().unwrap()).collect();
        assert_eq!(required_names, vec!["usuario", "senha"]);

        let example = login.get("example").expect("LoginPayload.example");
        let example_keys: Vec<&str> = example.as_object().unwrap().keys().map(String::as_str).collect();
        assert_eq!(example_keys, vec!["usuario", "senha"]);

        let security = json["components"]["securitySchemes"].as_object().expect("securitySchemes");
        assert!(security.contains_key("jwt_token"));
        assert!(security.contains_key("api_key"));
    }
}