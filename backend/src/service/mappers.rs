use crate::domain::{repository::RepositoryError, service::ServiceError};

impl From<RepositoryError> for ServiceError {
    fn from(value: RepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}
