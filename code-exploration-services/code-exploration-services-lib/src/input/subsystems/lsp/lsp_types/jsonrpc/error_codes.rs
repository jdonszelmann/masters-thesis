//! Predefined error codes.

// Defined by JSON RPC
pub const PARSE_ERROR: i32 = -32700;
pub const INVALID_REQUEST: i32 = -32600;
pub const METHOD_NOT_FOUND: i32 = -32601;
pub const INVALID_PAR_A_PAMS: i32 = -32602;
pub const INTERNAL_ERROR: i32 = -32603;
pub const SERVER_ERROR_START: i32 = -32099;
pub const SERVER_ERROR_END: i32 = -32000;
pub const SERVER_NOT_INITIALIZED: i32 = -32002;
pub const UNKNOWN_ERROR_CODE: i32 = -32001;

// Defined by the protocol.
pub const REQUEST_CANCELLED: i32 = -32800;
