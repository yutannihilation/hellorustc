/* automatically generated by rust-bindgen 0.70.1 */

/* OS: unix */
/* Platform: raarch64-apple-darwin20 */
/* rustc 1.81.0 (eeb90cda1 2024-09-04) */
/* R version: 4.4.1 */

extern "C" {
    #[doc = " TRUE during interactive use"]
    pub static mut R_Interactive: Rboolean;
    #[doc = " do not echo R code"]
    pub static mut R_NoEcho: Rboolean;
    pub fn R_RestoreGlobalEnv();
    pub fn R_RestoreGlobalEnvFromFile(arg1: *const ::std::os::raw::c_char, arg2: Rboolean);
    pub fn R_SaveGlobalEnv();
    pub fn R_SaveGlobalEnvToFile(arg1: *const ::std::os::raw::c_char);
    pub fn R_FlushConsole();
    pub fn R_ClearerrConsole();
    pub fn R_Suicide(arg1: *const ::std::os::raw::c_char);
    pub fn R_HomeDir() -> *mut ::std::os::raw::c_char;
    #[doc = " Current image dirty"]
    pub static mut R_DirtyImage: ::std::os::raw::c_int;
    pub static mut R_GUIType: *mut ::std::os::raw::c_char;
    pub fn R_setupHistory();
    #[doc = " Name of the history file"]
    pub static mut R_HistoryFile: *mut ::std::os::raw::c_char;
    #[doc = " Size of the history file"]
    pub static mut R_HistorySize: ::std::os::raw::c_int;
    #[doc = " restore the history file?"]
    pub static mut R_RestoreHistory: ::std::os::raw::c_int;
    #[doc = " Root of the R tree"]
    pub static mut R_Home: *mut ::std::os::raw::c_char;
    pub fn Rf_jump_to_toplevel();
    pub fn Rf_mainloop();
    pub fn Rf_onintr();
    pub fn Rf_onintrNoResume();
    #[doc = " Need opaque pointer type for export"]
    pub static mut R_GlobalContext: *mut ::std::os::raw::c_void;
    pub fn process_site_Renviron();
    pub fn process_system_Renviron();
    pub fn process_user_Renviron();
    pub static mut R_Consolefile: *mut FILE;
    pub static mut R_Outputfile: *mut FILE;
    #[doc = " in ../unix/sys-unix.c"]
    pub fn R_setStartTime();
    pub fn fpu_setup(arg1: Rboolean);
    #[doc = " in ../unix/system.c"]
    pub static mut R_running_as_main_program: ::std::os::raw::c_int;
    pub static mut ptr_R_Suicide:
        ::std::option::Option<unsafe extern "C" fn(arg1: *const ::std::os::raw::c_char)>;
    pub static mut ptr_R_ShowMessage:
        ::std::option::Option<unsafe extern "C" fn(arg1: *const ::std::os::raw::c_char)>;
    pub static mut ptr_R_ReadConsole: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            arg2: *mut ::std::os::raw::c_uchar,
            arg3: ::std::os::raw::c_int,
            arg4: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >;
    pub static mut ptr_R_WriteConsole: ::std::option::Option<
        unsafe extern "C" fn(arg1: *const ::std::os::raw::c_char, arg2: ::std::os::raw::c_int),
    >;
    pub static mut ptr_R_WriteConsoleEx: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            arg2: ::std::os::raw::c_int,
            arg3: ::std::os::raw::c_int,
        ),
    >;
    pub static mut ptr_R_ResetConsole: ::std::option::Option<unsafe extern "C" fn()>;
    pub static mut ptr_R_FlushConsole: ::std::option::Option<unsafe extern "C" fn()>;
    pub static mut ptr_R_ClearerrConsole: ::std::option::Option<unsafe extern "C" fn()>;
    pub static mut ptr_R_Busy:
        ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>;
    pub static mut ptr_R_CleanUp: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: SA_TYPE,
            arg2: ::std::os::raw::c_int,
            arg3: ::std::os::raw::c_int,
        ),
    >;
    pub static mut ptr_R_ShowFiles: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: ::std::os::raw::c_int,
            arg2: *mut *const ::std::os::raw::c_char,
            arg3: *mut *const ::std::os::raw::c_char,
            arg4: *const ::std::os::raw::c_char,
            arg5: Rboolean,
            arg6: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >;
    pub static mut ptr_R_ChooseFile: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: ::std::os::raw::c_int,
            arg2: *mut ::std::os::raw::c_char,
            arg3: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >;
    pub static mut ptr_R_EditFile: ::std::option::Option<
        unsafe extern "C" fn(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int,
    >;
    pub static mut ptr_R_loadhistory:
        ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: SEXP, arg3: SEXP, arg4: SEXP)>;
    pub static mut ptr_R_savehistory:
        ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: SEXP, arg3: SEXP, arg4: SEXP)>;
    pub static mut ptr_R_addhistory:
        ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: SEXP, arg3: SEXP, arg4: SEXP)>;
    #[doc = " added in 3.0.0"]
    pub static mut ptr_R_EditFiles: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: ::std::os::raw::c_int,
            arg2: *mut *const ::std::os::raw::c_char,
            arg3: *mut *const ::std::os::raw::c_char,
            arg4: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >;
    #[doc = " naming follows earlier versions in R.app"]
    pub static mut ptr_do_selectlist: ::std::option::Option<
        unsafe extern "C" fn(arg1: SEXP, arg2: SEXP, arg3: SEXP, arg4: SEXP) -> SEXP,
    >;
    pub static mut ptr_do_dataentry: ::std::option::Option<
        unsafe extern "C" fn(arg1: SEXP, arg2: SEXP, arg3: SEXP, arg4: SEXP) -> SEXP,
    >;
    pub static mut ptr_do_dataviewer: ::std::option::Option<
        unsafe extern "C" fn(arg1: SEXP, arg2: SEXP, arg3: SEXP, arg4: SEXP) -> SEXP,
    >;
    pub static mut ptr_R_ProcessEvents: ::std::option::Option<unsafe extern "C" fn()>;
    #[doc = " These two are not used by R itself, but are used by the tcltk package"]
    pub static mut R_timeout_handler:
        ::std::option::Option<unsafe extern "C" fn() -> ::std::os::raw::c_int>;
    pub static mut R_timeout_val: ::std::os::raw::c_long;
    pub static mut R_SignalHandlers: ::std::os::raw::c_int;
}
