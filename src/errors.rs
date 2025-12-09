use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug, Clone)]
pub enum EVSEError {
    #[error("Property was not of type {t}!")]
    #[diagnostic()]
    OcppPropertyError { t: String },
}
