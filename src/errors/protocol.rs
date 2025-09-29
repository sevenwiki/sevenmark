pub mod general {
    pub const BAD_REQUEST: &str = "general:bad_request";
    pub const VALIDATION_ERROR: &str = "general:validation_error";
}

pub mod system {
    pub const SYS_INTERNAL_ERROR: &str = "system:internal_error";
    pub const SYS_NOT_FOUND: &str = "system:not_found";
    pub const SYS_DATABASE_ERROR: &str = "system:database_error";
}
