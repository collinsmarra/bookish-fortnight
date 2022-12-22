pub enum HTTPErrors{
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    InternalServerError,
    ServiceUnavailable,
    Unknown,
}

impl Default for HTTPErrors{
    fn default() -> Self{
        HTTPErrors::Unknown
    }
}

pub fn error_handler(status: u16) -> HTTPErrors{
    match status{
        400 => HTTPErrors::BadRequest,
        401 => HTTPErrors::Unauthorized,
        403 => HTTPErrors::Forbidden,
        404 => HTTPErrors::NotFound,
        500 => HTTPErrors::InternalServerError,
        503 => HTTPErrors::ServiceUnavailable,
        _ => HTTPErrors::Unknown,
    }
}
