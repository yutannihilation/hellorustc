/* automatically generated by rust-bindgen 0.70.1 */

/* OS: unix */
/* Platform: aarch64-apple-darwin20 */
/* rustc 1.81.0 (eeb90cda1 2024-09-04) */
/* R version: 4.4.1 */

#[repr(u32)]
#[non_exhaustive]
#[doc = " PARSE_NULL will not be returned by R_ParseVector"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ParseStatus {
    PARSE_NULL = 0,
    PARSE_OK = 1,
    PARSE_INCOMPLETE = 2,
    PARSE_ERROR = 3,
    PARSE_EOF = 4,
}
extern "C" {
    pub fn R_ParseVector(
        arg1: SEXP,
        arg2: ::std::os::raw::c_int,
        arg3: *mut ParseStatus,
        arg4: SEXP,
    ) -> SEXP;
}
