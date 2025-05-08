use thiserror::Error;

#[derive(Clone, Debug, Error)]
#[error("Blog post title cannot be empty")]
pub struct PostTitleEmptyError;

#[derive(Clone, Debug, Error)]
#[error("Blog post body cannot be empty")]
pub struct PostBodyEmptyError;
