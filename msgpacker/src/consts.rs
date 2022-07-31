/// Packable representation for variant `None` of [`Option`]
pub const OPTION_NONE: isize = 0x00;

/// Packable representation for variant `Some` of [`Option`]
pub const OPTION_SOME: isize = 0x01;

/// Packable representation for variant `Ok` of [`Result`]
pub const RESULT_OK: isize = 0x00;

/// Packable representation for variant `Err` of [`Result`]
pub const RESULT_ERR: isize = 0x01;
