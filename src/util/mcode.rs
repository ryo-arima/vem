/*
 * Message code definitions for VEM
 * Naming convention: VEM_FUNCTION_NUMBER (uppercase)
 * Used for all log output and error messages
 */
#![allow(dead_code)]
#![allow(non_camel_case_types)]

/*
 * Log level for message output
 * EMERG: System is unusable
 * ALERT: Action must be taken immediately
 * CRIT: Critical conditions
 * ERROR: Error conditions
 * WARN: Warning conditions
 * NOTICE: Normal but significant condition
 * INFO: Informational messages
 * DEBUG: Debug-level messages
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
pub enum log_level_t {
    EMERG,
    ALERT,
    CRIT,
    ERROR,
    WARN,
    NOTICE,
    INFO,
    DEBUG,
}

impl log_level_t {
    pub fn as_str(&self) -> &'static str {
        match self {
            log_level_t::EMERG => "EMERG",
            log_level_t::ALERT => "ALERT",
            log_level_t::CRIT => "CRIT",
            log_level_t::ERROR => "ERROR",
            log_level_t::WARN => "WARN",
            log_level_t::NOTICE => "NOTICE",
            log_level_t::INFO => "INFO",
            log_level_t::DEBUG => "DEBUG",
        }
    }
}

/* Configuration (CNF) Layer Codes - VEM_CONFIG_*/
// VEM_CONFIG_LOAD - Config loading operations
struct vem_config_load;
impl vem_config_load {
    const VCL1: &'static str = "Config load success";
    const VCL2: &'static str = "Config load failed";
    const VCL3: &'static str = "Config parse error";
}
pub const VCL1: &str = vem_config_load::VCL1;
pub const VCL2: &str = vem_config_load::VCL2;
pub const VCL3: &str = vem_config_load::VCL3;

// VEM_CONFIG_SAVE - Config saving operations
struct vem_config_save;
impl vem_config_save {
    const VCS1: &'static str = "Config save success";
    const VCS2: &'static str = "Config save failed";
    const VCS3: &'static str = "Config serialize error";
}
pub const VCS1: &str = vem_config_save::VCS1;
pub const VCS2: &str = vem_config_save::VCS2;
pub const VCS3: &str = vem_config_save::VCS3;

// VEM_CONFIG_VALIDATE - Config validation
struct vem_config_validate;
impl vem_config_validate {
    const VCV1: &'static str = "Config validation success";
    const VCV2: &'static str = "Config validation failed";
}
pub const VCV1: &str = vem_config_validate::VCV1;
pub const VCV2: &str = vem_config_validate::VCV2;

/* Repository (REP) Layer Codes - VEM_ENV_*/
// VEM_ENV_CREATE - Environment creation
struct vem_env_create;
impl vem_env_create {
    const VEC1: &'static str = "Environment created successfully";
    const VEC2: &'static str = "Environment creation failed";
    const VEC3: &'static str = "Environment already exists";
    const VEC4: &'static str = "Invalid environment name";
}
pub const VEC1: &str = vem_env_create::VEC1;
pub const VEC2: &str = vem_env_create::VEC2;
pub const VEC3: &str = vem_env_create::VEC3;
pub const VEC4: &str = vem_env_create::VEC4;

// VEM_ENV_LIST - Environment listing
struct vem_env_list;
impl vem_env_list {
    const VEL1: &'static str = "Environment list success";
    const VEL2: &'static str = "Environment list failed";
    const VEL3: &'static str = "No environments found";
}
pub const VEL1: &str = vem_env_list::VEL1;
pub const VEL2: &str = vem_env_list::VEL2;
pub const VEL3: &str = vem_env_list::VEL3;

// VEM_ENV_GET - Environment retrieval
struct vem_env_get;
impl vem_env_get {
    const VEG1: &'static str = "Environment get success";
    const VEG2: &'static str = "Environment not found";
    const VEG3: &'static str = "Environment get failed";
}
pub const VEG1: &str = vem_env_get::VEG1;
pub const VEG2: &str = vem_env_get::VEG2;
pub const VEG3: &str = vem_env_get::VEG3;

// VEM_ENV_UPDATE - Environment update
struct vem_env_update;
impl vem_env_update {
    const VEU1: &'static str = " Environment update success";
    const VEU2: &str = "Environment update failed";
}
pub const VEU1: &str = vem_env_update::VEU1;
pub const VEU2: &str = vem_env_update::VEU2;

// VEM_ENV_DELETE - Environment deletion
struct vem_env_delete;
impl vem_env_delete {
    const VED1: &'static str = "Environment delete success";
    const VED2: &'static str = "Environment delete failed";
    const VED3: &'static str = "Cannot delete active environment";
}
pub const VED1: &str = vem_env_delete::VED1;
pub const VED2: &str = vem_env_delete::VED2;
pub const VED3: &str = vem_env_delete::VED3;

// VEM_ENV_SWITCH - Environment switching
struct vem_env_switch;
impl vem_env_switch {
    const VES1: &'static str = "Environment switch success";
    const VES2: &'static str = "Environment switch failed";
}
pub const VES1: &str = vem_env_switch::VES1;
pub const VES2: &str = vem_env_switch::VES2;

// VEM_ENV_CURRENT - Current environment operations
struct vem_env_current;
impl vem_env_current {
    const VECU1: &'static str = "Get current environment success";
    const VECU2: &'static str = "No current environment set";
    const VECU3: &'static str = "Get current environment failed";
}
pub const VECU1: &str = vem_env_current::VECU1;
pub const VECU2: &str = vem_env_current::VECU2;
pub const VECU3: &str = vem_env_current::VECU3;

/* Metadata Operations - VEM_META_*/
// VEM_META_SAVE - Metadata saving
struct vem_meta_save;
impl vem_meta_save {
    const VMS1: &'static str = "Metadata save success";
    const VMS2: &'static str = "Metadata save failed";
    const VMS3: &'static str = "Metadata serialize error";
}
pub const VMS1: &str = vem_meta_save::VMS1;
pub const VMS2: &str = vem_meta_save::VMS2;
pub const VMS3: &str = vem_meta_save::VMS3;

// VEM_META_LOAD - Metadata loading
struct vem_meta_load;
impl vem_meta_load {
    const VML1: &'static str = "Metadata load success";
    const VML2: &'static str = "Metadata load failed";
    const VML3: &'static str = "Metadata parse error";
}
pub const VML1: &str = vem_meta_load::VML1;
pub const VML2: &str = vem_meta_load::VML2;
pub const VML3: &str = vem_meta_load::VML3;

/* Use-case (USC) Layer Codes - VEM_USC_*/
// VEM_USC_INIT - Use-case initialization
struct vem_usc_init;
impl vem_usc_init {
    const VUI1: &'static str = "Use-case initialization success";
    const VUI2: &'static str = "Use-case initialization failed";
}
pub const VUI1: &str = vem_usc_init::VUI1;
pub const VUI2: &str = vem_usc_init::VUI2;

/* Controller (CTL) Layer Codes - VEM_CLI_*/
// VEM_CLI_EXEC - CLI command execution
struct vem_cli_exec;
impl vem_cli_exec {
    const VCE1: &'static str = "CLI command executed successfully";
    const VCE2: &'static str = "CLI command execution failed";
}
pub const VCE1: &str = vem_cli_exec::VCE1;
pub const VCE2: &str = vem_cli_exec::VCE2;

// VEM_CLI_PARSE - CLI parsing
struct vem_cli_parse;
impl vem_cli_parse {
    const VCP1: &'static str = "CLI parse error";
}
pub const VCP1: &str = vem_cli_parse::VCP1;

// VEM_CLI_CONFIRM - User confirmations
struct vem_cli_confirm;
impl vem_cli_confirm {
    const VCC1: &'static str = "User confirmation prompt";
}
pub const VCC_CONFIRM: &str = vem_cli_confirm::VCC1;

// VEM_CLI_CANCEL - Operation cancellation
struct vem_cli_cancel;
impl vem_cli_cancel {
    const VCC1: &'static str = "Operation cancelled by user";
}
pub const VCC_CANCEL: &str = vem_cli_cancel::VCC1;

/* File System Operations - VEM_FS_*/
// VEM_FS_OP - General file system operations
struct vem_fs_op;
impl vem_fs_op {
    const VFO1: &'static str = "File system operation success";
    const VFO2: &'static str = "File system operation failed";
}
pub const VFO1: &str = vem_fs_op::VFO1;
pub const VFO2: &str = vem_fs_op::VFO2;

// VEM_FS_MKDIR - Directory creation
struct vem_fs_mkdir;
impl vem_fs_mkdir {
    const VFM1: &'static str = "Directory creation success";
    const VFM2: &'static str = "Directory creation failed";
}
pub const VFM1: &str = vem_fs_mkdir::VFM1;
pub const VFM2: &str = vem_fs_mkdir::VFM2;

// VEM_FS_WRITE - File writing
struct vem_fs_write;
impl vem_fs_write {
    const VFW1: &'static str = "File write success";
    const VFW2: &'static str = "File write failed";
}
pub const VFW1: &str = vem_fs_write::VFW1;
pub const VFW2: &str = vem_fs_write::VFW2;

// VEM_FS_READ - File reading
struct vem_fs_read;
impl vem_fs_read {
    const VFR1: &'static str = "File read success";
    const VFR2: &'static str = "File read failed";
}
pub const VFR1: &str = vem_fs_read::VFR1;
pub const VFR2: &str = vem_fs_read::VFR2;

// VEM_FS_SYMLINK - Symlink operations
struct vem_fs_symlink;
impl vem_fs_symlink {
    const VFS1: &'static str = "Symlink create success";
    const VFS2: &'static str = "Symlink create failed";
}
pub const VFS1: &str = vem_fs_symlink::VFS1;
pub const VFS2: &str = vem_fs_symlink::VFS2;

/* General System Codes - VEM_SYS_*/
// VEM_SYS_START - Application startup
struct vem_sys_start;
impl vem_sys_start {
    const VSS1: &'static str = "Application started";
}
pub const VSS1: &str = vem_sys_start::VSS1;

// VEM_SYS_EXIT - Application exit
struct vem_sys_exit;
impl vem_sys_exit {
    const VSE1: &'static str = "Application terminated successfully";
    const VSE2: &'static str = "Application terminated with error";
}
pub const VSE1: &str = vem_sys_exit::VSE1;
pub const VSE2: &str = vem_sys_exit::VSE2;

// VEM_SYS_ERROR - System errors
struct vem_sys_error;
impl vem_sys_error {
    const VSE1: &'static str = "Unexpected error occurred";
}
pub const VSE_ERROR: &str = vem_sys_error::VSE1;

/* Helper Functions*/
// Format a message with code prefix and log level
pub fn format_message(level: log_level_t, code: &str, optional_message: &str) -> String {
    format!("[{}][{}] {}", level.as_str(), code, optional_message)
}
