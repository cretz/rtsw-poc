use std::os::raw::{c_char, c_int};

#[repr(C)]
#[derive(Debug,Copy,Clone)]
pub struct tor_main_configuration_t {
    _unused: [u8; 0]
}

extern "C" {
    pub fn tor_main_configuration_new() -> *mut tor_main_configuration_t;

    pub fn tor_main_configuration_set_command_line(cfg: *mut tor_main_configuration_t,
                                                   argc: c_int,
                                                   argv: *mut *mut c_char) -> c_int;

    pub fn tor_main_configuration_free(cfg: *mut tor_main_configuration_t);
    
    pub fn tor_run_main(arg1: *const tor_main_configuration_t) -> c_int;
    
    pub fn tor_main(argc: c_int, argv: *mut *mut c_char) -> c_int;
}