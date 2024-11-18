/* automatically generated by rust-bindgen 0.70.1 */

#[derive(PartialEq, Copy, Clone, Hash, Debug, Default)]
#[repr(C)]
pub struct __BindgenComplex<T> {
    pub re: T,
    pub im: T,
}
pub const Rboolean_FALSE: Rboolean = 0;
pub const Rboolean_TRUE: Rboolean = 1;
pub type Rboolean = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Copy, Clone)]
pub union Rcomplex {
    pub __bindgen_anon_1: Rcomplex__bindgen_ty_1,
    pub private_data_c: __BindgenComplex<f64>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Rcomplex__bindgen_ty_1 {
    pub r: f64,
    pub i: f64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Rcomplex__bindgen_ty_1"][::std::mem::size_of::<Rcomplex__bindgen_ty_1>() - 16usize];
    ["Alignment of Rcomplex__bindgen_ty_1"]
        [::std::mem::align_of::<Rcomplex__bindgen_ty_1>() - 8usize];
    ["Offset of field: Rcomplex__bindgen_ty_1::r"]
        [::std::mem::offset_of!(Rcomplex__bindgen_ty_1, r) - 0usize];
    ["Offset of field: Rcomplex__bindgen_ty_1::i"]
        [::std::mem::offset_of!(Rcomplex__bindgen_ty_1, i) - 8usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Rcomplex"][::std::mem::size_of::<Rcomplex>() - 16usize];
    ["Alignment of Rcomplex"][::std::mem::align_of::<Rcomplex>() - 8usize];
    ["Offset of field: Rcomplex::private_data_c"]
        [::std::mem::offset_of!(Rcomplex, private_data_c) - 0usize];
};
pub type wchar_t = ::std::os::raw::c_int;
pub type max_align_t = f64;
extern "C" {
    pub fn R_isort(arg1: *mut ::std::os::raw::c_int, arg2: ::std::os::raw::c_int);
    pub fn R_rsort(arg1: *mut f64, arg2: ::std::os::raw::c_int);
    pub fn R_csort(arg1: *mut Rcomplex, arg2: ::std::os::raw::c_int);
    pub fn rsort_with_index(
        arg1: *mut f64,
        arg2: *mut ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    );
    pub fn Rf_revsort(
        arg1: *mut f64,
        arg2: *mut ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    );
    pub fn Rf_iPsort(
        arg1: *mut ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    );
    pub fn Rf_rPsort(arg1: *mut f64, arg2: ::std::os::raw::c_int, arg3: ::std::os::raw::c_int);
    pub fn Rf_cPsort(arg1: *mut Rcomplex, arg2: ::std::os::raw::c_int, arg3: ::std::os::raw::c_int);
    pub fn R_qsort(v: *mut f64, i: usize, j: usize);
    pub fn R_qsort_I(
        v: *mut f64,
        II: *mut ::std::os::raw::c_int,
        i: ::std::os::raw::c_int,
        j: ::std::os::raw::c_int,
    );
    pub fn R_qsort_int(iv: *mut ::std::os::raw::c_int, i: usize, j: usize);
    pub fn R_qsort_int_I(
        iv: *mut ::std::os::raw::c_int,
        II: *mut ::std::os::raw::c_int,
        i: ::std::os::raw::c_int,
        j: ::std::os::raw::c_int,
    );
    pub fn R_ExpandFileName(arg1: *const ::std::os::raw::c_char) -> *const ::std::os::raw::c_char;
    pub fn Rf_setIVector(
        arg1: *mut ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    );
    pub fn Rf_setRVector(arg1: *mut f64, arg2: ::std::os::raw::c_int, arg3: f64);
    pub fn Rf_StringFalse(arg1: *const ::std::os::raw::c_char) -> Rboolean;
    pub fn Rf_StringTrue(arg1: *const ::std::os::raw::c_char) -> Rboolean;
    pub fn Rf_isBlankString(arg1: *const ::std::os::raw::c_char) -> Rboolean;
    pub fn R_atof(str_: *const ::std::os::raw::c_char) -> f64;
    pub fn R_strtod(c: *const ::std::os::raw::c_char, end: *mut *mut ::std::os::raw::c_char)
        -> f64;
    pub fn R_tmpnam(
        prefix: *const ::std::os::raw::c_char,
        tempdir: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn R_tmpnam2(
        prefix: *const ::std::os::raw::c_char,
        tempdir: *const ::std::os::raw::c_char,
        fileext: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn R_free_tmpnam(name: *mut ::std::os::raw::c_char);
    pub fn R_CheckUserInterrupt();
    pub fn R_CheckStack();
    pub fn R_CheckStack2(arg1: usize);
    pub fn findInterval(
        xt: *mut f64,
        n: ::std::os::raw::c_int,
        x: f64,
        rightmost_closed: Rboolean,
        all_inside: Rboolean,
        ilo: ::std::os::raw::c_int,
        mflag: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn findInterval2(
        xt: *mut f64,
        n: ::std::os::raw::c_int,
        x: f64,
        rightmost_closed: Rboolean,
        all_inside: Rboolean,
        left_open: Rboolean,
        ilo: ::std::os::raw::c_int,
        mflag: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn find_interv_vec(
        xt: *mut f64,
        n: *mut ::std::os::raw::c_int,
        x: *mut f64,
        nx: *mut ::std::os::raw::c_int,
        rightmost_closed: *mut ::std::os::raw::c_int,
        all_inside: *mut ::std::os::raw::c_int,
        indx: *mut ::std::os::raw::c_int,
    );
    pub fn R_max_col(
        matrix: *mut f64,
        nr: *mut ::std::os::raw::c_int,
        nc: *mut ::std::os::raw::c_int,
        maxes: *mut ::std::os::raw::c_int,
        ties_meth: *mut ::std::os::raw::c_int,
    );
}
