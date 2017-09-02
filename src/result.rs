use std::result::Result as RResult;

use error::KairosError;

pub type Result<T> = RResult<T, KairosError>;
