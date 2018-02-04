use std::os::raw::c_char;
use std::ffi::CString;

pub mod tor_sys;

fn main() {
    unsafe {
        // Create the conf
        let conf = tor_sys::tor_main_configuration_new();
        // Create a set of args as though we called this from CLI
        let mut args: Vec<*mut c_char> = vec![CString::new("tor.exe").unwrap().into_raw(),
                                              CString::new("--version").unwrap().into_raw()];
        let res = tor_sys::tor_main_configuration_set_command_line(conf, 2, args.as_mut_ptr());
        if res != 0 {
            panic!("Result from conf set: {}", res);
        }
        // Run main and clean up args
        let res = tor_sys::tor_run_main(conf);
        tor_sys::tor_main_configuration_free(conf);
        if res != 0 {
            panic!("Result from main: {}", res);
        }
    }
}
