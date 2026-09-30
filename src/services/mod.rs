pub mod clinica_service;
pub mod documento;
pub mod exame_service;
pub mod medico_service;
pub mod paciente_service;
pub mod paginacao;

pub use clinica_service::ClinicaService;
pub use exame_service::ExameService;
pub use medico_service::MedicoService;
pub use paciente_service::PacienteService;