
pub const SNAPMAIL_DEFAULT_INTEGRITY_ZOME_NAME: &'static str = "snapmail_model";
pub const SNAPMAIL_DEFAULT_COORDINATOR_ZOME_NAME: &'static str = "snapmail";
pub const SNAPMAIL_DEFAULT_ROLE_NAME: &'static str = "rSnapmail";

pub const DIRECT_SEND_TIMEOUT_MS: usize = 1000;
pub const DIRECT_SEND_CHUNK_TIMEOUT_MS: usize = 10000;

// const CHUNK_MAX_SIZE: usize = 1 * 1024 * 1024;
pub const CHUNK_MAX_SIZE: usize = 200 * 1024;
pub const FILE_MAX_SIZE: usize = 10 * 1024 * 1024;
//pub const FILE_MAX_SIZE: usize = 4_000_000;



/// PSEUDO CONDITIONAL COMPILATION FOR DEBUGGING / TESTING
pub const CAN_DM: bool = true;
