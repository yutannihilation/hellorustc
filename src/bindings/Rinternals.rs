/* automatically generated by rust-bindgen 0.70.1 */

pub const R_LEN_T_MAX: u32 = 2147483647;
pub const R_XLEN_T_MAX: u64 = 4503599627370496;
pub const R_SHORT_LEN_MAX: u32 = 2147483647;
pub const R_PRIdXLEN_T: &[u8; 3] = b"td\0";
pub const TYPE_BITS: u32 = 5;
pub const MAX_NUM_SEXPTYPE: u32 = 32;
pub const NAMEDMAX: u32 = 7;
pub const R_XDR_DOUBLE_SIZE: u32 = 8;
pub const R_XDR_INTEGER_SIZE: u32 = 4;
pub const R_CODESET_MAX: u32 = 63;
pub const IDENT_NUM_AS_BITS: u32 = 1;
pub const IDENT_NA_AS_BITS: u32 = 2;
pub const IDENT_ATTR_BY_ORDER: u32 = 4;
pub const IDENT_USE_BYTECODE: u32 = 8;
pub const IDENT_USE_CLOENV: u32 = 16;
pub const IDENT_USE_SRCREF: u32 = 32;
pub const IDENT_EXTPTR_AS_REF: u32 = 64;
pub const HT_TYPE_IDENTICAL: u32 = 0;
pub const HT_TYPE_ADDRESS: u32 = 1;
pub type Rbyte = ::std::os::raw::c_uchar;
pub type R_len_t = ::std::os::raw::c_int;
pub type R_xlen_t = isize;
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SEXPTYPE {
    NILSXP = 0,
    SYMSXP = 1,
    LISTSXP = 2,
    CLOSXP = 3,
    ENVSXP = 4,
    PROMSXP = 5,
    LANGSXP = 6,
    SPECIALSXP = 7,
    BUILTINSXP = 8,
    CHARSXP = 9,
    LGLSXP = 10,
    INTSXP = 13,
    REALSXP = 14,
    CPLXSXP = 15,
    STRSXP = 16,
    DOTSXP = 17,
    ANYSXP = 18,
    VECSXP = 19,
    EXPRSXP = 20,
    BCODESXP = 21,
    EXTPTRSXP = 22,
    WEAKREFSXP = 23,
    RAWSXP = 24,
    OBJSXP = 25,
    NEWSXP = 30,
    FREESXP = 31,
    FUNSXP = 99,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SEXPREC {
    _unused: [u8; 0],
}
pub type SEXP = *mut SEXPREC;
pub type PROTECT_INDEX = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct R_allocator {
    _unused: [u8; 0],
}
pub type R_allocator_t = R_allocator;
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum nchar_type {
    Bytes = 0,
    Chars = 1,
    Width = 2,
}
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cetype_t {
    CE_NATIVE = 0,
    CE_UTF8 = 1,
    CE_LATIN1 = 2,
    CE_BYTES = 3,
    CE_SYMBOL = 5,
    CE_ANY = 99,
}
pub type R_CFinalizer_t = ::std::option::Option<unsafe extern "C" fn(arg1: SEXP)>;
pub type R_pstream_data_t = *mut ::std::os::raw::c_void;
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum R_pstream_format_t {
    R_pstream_any_format = 0,
    R_pstream_ascii_format = 1,
    R_pstream_binary_format = 2,
    R_pstream_xdr_format = 3,
    R_pstream_asciihex_format = 4,
}
pub type R_outpstream_t = *mut R_outpstream_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct R_outpstream_st {
    pub data: R_pstream_data_t,
    pub type_: R_pstream_format_t,
    pub version: ::std::os::raw::c_int,
    pub OutChar: ::std::option::Option<
        unsafe extern "C" fn(arg1: R_outpstream_t, arg2: ::std::os::raw::c_int),
    >,
    pub OutBytes: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: R_outpstream_t,
            arg2: *mut ::std::os::raw::c_void,
            arg3: ::std::os::raw::c_int,
        ),
    >,
    pub OutPersistHookFunc:
        ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: SEXP) -> SEXP>,
    pub OutPersistHookData: SEXP,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of R_outpstream_st"][::std::mem::size_of::<R_outpstream_st>() - 48usize];
    ["Alignment of R_outpstream_st"][::std::mem::align_of::<R_outpstream_st>() - 8usize];
    ["Offset of field: R_outpstream_st::data"]
        [::std::mem::offset_of!(R_outpstream_st, data) - 0usize];
    ["Offset of field: R_outpstream_st::type_"]
        [::std::mem::offset_of!(R_outpstream_st, type_) - 8usize];
    ["Offset of field: R_outpstream_st::version"]
        [::std::mem::offset_of!(R_outpstream_st, version) - 12usize];
    ["Offset of field: R_outpstream_st::OutChar"]
        [::std::mem::offset_of!(R_outpstream_st, OutChar) - 16usize];
    ["Offset of field: R_outpstream_st::OutBytes"]
        [::std::mem::offset_of!(R_outpstream_st, OutBytes) - 24usize];
    ["Offset of field: R_outpstream_st::OutPersistHookFunc"]
        [::std::mem::offset_of!(R_outpstream_st, OutPersistHookFunc) - 32usize];
    ["Offset of field: R_outpstream_st::OutPersistHookData"]
        [::std::mem::offset_of!(R_outpstream_st, OutPersistHookData) - 40usize];
};
pub type R_inpstream_t = *mut R_inpstream_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct R_inpstream_st {
    pub data: R_pstream_data_t,
    pub type_: R_pstream_format_t,
    pub InChar:
        ::std::option::Option<unsafe extern "C" fn(arg1: R_inpstream_t) -> ::std::os::raw::c_int>,
    pub InBytes: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: R_inpstream_t,
            arg2: *mut ::std::os::raw::c_void,
            arg3: ::std::os::raw::c_int,
        ),
    >,
    pub InPersistHookFunc:
        ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: SEXP) -> SEXP>,
    pub InPersistHookData: SEXP,
    pub native_encoding: [::std::os::raw::c_char; 64usize],
    pub nat2nat_obj: *mut ::std::os::raw::c_void,
    pub nat2utf8_obj: *mut ::std::os::raw::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of R_inpstream_st"][::std::mem::size_of::<R_inpstream_st>() - 128usize];
    ["Alignment of R_inpstream_st"][::std::mem::align_of::<R_inpstream_st>() - 8usize];
    ["Offset of field: R_inpstream_st::data"]
        [::std::mem::offset_of!(R_inpstream_st, data) - 0usize];
    ["Offset of field: R_inpstream_st::type_"]
        [::std::mem::offset_of!(R_inpstream_st, type_) - 8usize];
    ["Offset of field: R_inpstream_st::InChar"]
        [::std::mem::offset_of!(R_inpstream_st, InChar) - 16usize];
    ["Offset of field: R_inpstream_st::InBytes"]
        [::std::mem::offset_of!(R_inpstream_st, InBytes) - 24usize];
    ["Offset of field: R_inpstream_st::InPersistHookFunc"]
        [::std::mem::offset_of!(R_inpstream_st, InPersistHookFunc) - 32usize];
    ["Offset of field: R_inpstream_st::InPersistHookData"]
        [::std::mem::offset_of!(R_inpstream_st, InPersistHookData) - 40usize];
    ["Offset of field: R_inpstream_st::native_encoding"]
        [::std::mem::offset_of!(R_inpstream_st, native_encoding) - 48usize];
    ["Offset of field: R_inpstream_st::nat2nat_obj"]
        [::std::mem::offset_of!(R_inpstream_st, nat2nat_obj) - 112usize];
    ["Offset of field: R_inpstream_st::nat2utf8_obj"]
        [::std::mem::offset_of!(R_inpstream_st, nat2utf8_obj) - 120usize];
};
pub const SORTED_DECR_NA_1ST: _bindgen_ty_1 = _bindgen_ty_1::SORTED_DECR_NA_1ST;
pub const SORTED_DECR: _bindgen_ty_1 = _bindgen_ty_1::SORTED_DECR;
pub const UNKNOWN_SORTEDNESS: _bindgen_ty_1 = _bindgen_ty_1::UNKNOWN_SORTEDNESS;
pub const SORTED_INCR: _bindgen_ty_1 = _bindgen_ty_1::SORTED_INCR;
pub const SORTED_INCR_NA_1ST: _bindgen_ty_1 = _bindgen_ty_1::SORTED_INCR_NA_1ST;
pub const KNOWN_UNSORTED: _bindgen_ty_1 = _bindgen_ty_1::KNOWN_UNSORTED;
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_1 {
    SORTED_DECR_NA_1ST = -2,
    SORTED_DECR = -1,
    UNKNOWN_SORTEDNESS = -2147483648,
    SORTED_INCR = 1,
    SORTED_INCR_NA_1ST = 2,
    KNOWN_UNSORTED = 0,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct R_hashtab_type {
    pub cell: SEXP,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of R_hashtab_type"][::std::mem::size_of::<R_hashtab_type>() - 8usize];
    ["Alignment of R_hashtab_type"][::std::mem::align_of::<R_hashtab_type>() - 8usize];
    ["Offset of field: R_hashtab_type::cell"]
        [::std::mem::offset_of!(R_hashtab_type, cell) - 0usize];
};
extern "C" {
    pub fn R_CHAR(x: SEXP) -> *const ::std::os::raw::c_char;
    pub fn Rf_isNull(s: SEXP) -> Rboolean;
    pub fn Rf_isSymbol(s: SEXP) -> Rboolean;
    pub fn Rf_isLogical(s: SEXP) -> Rboolean;
    pub fn Rf_isReal(s: SEXP) -> Rboolean;
    pub fn Rf_isComplex(s: SEXP) -> Rboolean;
    pub fn Rf_isExpression(s: SEXP) -> Rboolean;
    pub fn Rf_isEnvironment(s: SEXP) -> Rboolean;
    pub fn Rf_isString(s: SEXP) -> Rboolean;
    pub fn Rf_isObject(s: SEXP) -> Rboolean;
    pub fn ATTRIB(x: SEXP) -> SEXP;
    pub fn OBJECT(x: SEXP) -> ::std::os::raw::c_int;
    pub fn MARK(x: SEXP) -> ::std::os::raw::c_int;
    pub fn TYPEOF(x: SEXP) -> ::std::os::raw::c_int;
    pub fn NAMED(x: SEXP) -> ::std::os::raw::c_int;
    pub fn REFCNT(x: SEXP) -> ::std::os::raw::c_int;
    pub fn SET_ATTRIB(x: SEXP, v: SEXP);
    pub fn DUPLICATE_ATTRIB(to: SEXP, from: SEXP);
    pub fn SHALLOW_DUPLICATE_ATTRIB(to: SEXP, from: SEXP);
    pub fn MARK_NOT_MUTABLE(x: SEXP);
    pub fn IS_S4_OBJECT(x: SEXP) -> ::std::os::raw::c_int;
    pub fn LENGTH(x: SEXP) -> ::std::os::raw::c_int;
    pub fn XLENGTH(x: SEXP) -> R_xlen_t;
    pub fn TRUELENGTH(x: SEXP) -> R_xlen_t;
    pub fn IS_LONG_VEC(x: SEXP) -> ::std::os::raw::c_int;
    pub fn LEVELS(x: SEXP) -> ::std::os::raw::c_int;
    pub fn LOGICAL(x: SEXP) -> *mut ::std::os::raw::c_int;
    pub fn INTEGER(x: SEXP) -> *mut ::std::os::raw::c_int;
    pub fn RAW(x: SEXP) -> *mut Rbyte;
    pub fn REAL(x: SEXP) -> *mut f64;
    pub fn COMPLEX(x: SEXP) -> *mut Rcomplex;
    pub fn LOGICAL_RO(x: SEXP) -> *const ::std::os::raw::c_int;
    pub fn INTEGER_RO(x: SEXP) -> *const ::std::os::raw::c_int;
    pub fn RAW_RO(x: SEXP) -> *const Rbyte;
    pub fn REAL_RO(x: SEXP) -> *const f64;
    pub fn COMPLEX_RO(x: SEXP) -> *const Rcomplex;
    pub fn VECTOR_ELT(x: SEXP, i: R_xlen_t) -> SEXP;
    pub fn SET_STRING_ELT(x: SEXP, i: R_xlen_t, v: SEXP);
    pub fn SET_VECTOR_ELT(x: SEXP, i: R_xlen_t, v: SEXP) -> SEXP;
    pub fn STRING_PTR(x: SEXP) -> *mut SEXP;
    pub fn STRING_PTR_RO(x: SEXP) -> *const SEXP;
    pub fn VECTOR_PTR(x: SEXP) -> *mut SEXP;
    pub fn INTEGER_GET_REGION(
        sx: SEXP,
        i: R_xlen_t,
        n: R_xlen_t,
        buf: *mut ::std::os::raw::c_int,
    ) -> R_xlen_t;
    pub fn REAL_GET_REGION(sx: SEXP, i: R_xlen_t, n: R_xlen_t, buf: *mut f64) -> R_xlen_t;
    pub fn LOGICAL_GET_REGION(
        sx: SEXP,
        i: R_xlen_t,
        n: R_xlen_t,
        buf: *mut ::std::os::raw::c_int,
    ) -> R_xlen_t;
    pub fn COMPLEX_GET_REGION(sx: SEXP, i: R_xlen_t, n: R_xlen_t, buf: *mut Rcomplex) -> R_xlen_t;
    pub fn RAW_GET_REGION(sx: SEXP, i: R_xlen_t, n: R_xlen_t, buf: *mut Rbyte) -> R_xlen_t;
    pub fn INTEGER_IS_SORTED(x: SEXP) -> ::std::os::raw::c_int;
    pub fn INTEGER_NO_NA(x: SEXP) -> ::std::os::raw::c_int;
    pub fn REAL_IS_SORTED(x: SEXP) -> ::std::os::raw::c_int;
    pub fn REAL_NO_NA(x: SEXP) -> ::std::os::raw::c_int;
    pub fn LOGICAL_IS_SORTED(x: SEXP) -> ::std::os::raw::c_int;
    pub fn LOGICAL_NO_NA(x: SEXP) -> ::std::os::raw::c_int;
    pub fn STRING_IS_SORTED(x: SEXP) -> ::std::os::raw::c_int;
    pub fn STRING_NO_NA(x: SEXP) -> ::std::os::raw::c_int;
    pub fn TAG(e: SEXP) -> SEXP;
    pub fn CDR(e: SEXP) -> SEXP;
    pub fn CAAR(e: SEXP) -> SEXP;
    pub fn CDAR(e: SEXP) -> SEXP;
    pub fn CADR(e: SEXP) -> SEXP;
    pub fn CDDR(e: SEXP) -> SEXP;
    pub fn CDDDR(e: SEXP) -> SEXP;
    pub fn CADDR(e: SEXP) -> SEXP;
    pub fn CADDDR(e: SEXP) -> SEXP;
    pub fn CAD4R(e: SEXP) -> SEXP;
    pub fn CAD5R(e: SEXP) -> SEXP;
    pub fn MISSING(x: SEXP) -> ::std::os::raw::c_int;
    pub fn SET_TAG(x: SEXP, y: SEXP);
    pub fn SETCAR(x: SEXP, y: SEXP) -> SEXP;
    pub fn SETCDR(x: SEXP, y: SEXP) -> SEXP;
    pub fn SETCADR(x: SEXP, y: SEXP) -> SEXP;
    pub fn SETCADDR(x: SEXP, y: SEXP) -> SEXP;
    pub fn SETCADDDR(x: SEXP, y: SEXP) -> SEXP;
    pub fn SETCAD4R(e: SEXP, y: SEXP) -> SEXP;
    pub fn FORMALS(x: SEXP) -> SEXP;
    pub fn BODY(x: SEXP) -> SEXP;
    pub fn CLOENV(x: SEXP) -> SEXP;
    pub fn RDEBUG(x: SEXP) -> ::std::os::raw::c_int;
    pub fn RSTEP(x: SEXP) -> ::std::os::raw::c_int;
    pub fn RTRACE(x: SEXP) -> ::std::os::raw::c_int;
    pub fn SET_RDEBUG(x: SEXP, v: ::std::os::raw::c_int);
    pub fn SET_RSTEP(x: SEXP, v: ::std::os::raw::c_int);
    pub fn SET_RTRACE(x: SEXP, v: ::std::os::raw::c_int);
    pub fn SET_FORMALS(x: SEXP, v: SEXP);
    pub fn SET_BODY(x: SEXP, v: SEXP);
    pub fn SET_CLOENV(x: SEXP, v: SEXP);
    pub fn PRINTNAME(x: SEXP) -> SEXP;
    pub fn SYMVALUE(x: SEXP) -> SEXP;
    pub fn INTERNAL(x: SEXP) -> SEXP;
    pub fn DDVAL(x: SEXP) -> ::std::os::raw::c_int;
    pub fn FRAME(x: SEXP) -> SEXP;
    pub fn ENCLOS(x: SEXP) -> SEXP;
    pub fn HASHTAB(x: SEXP) -> SEXP;
    pub fn ENVFLAGS(x: SEXP) -> ::std::os::raw::c_int;
    pub fn PRCODE(x: SEXP) -> SEXP;
    pub fn PRENV(x: SEXP) -> SEXP;
    pub fn PRVALUE(x: SEXP) -> SEXP;
    pub fn PRSEEN(x: SEXP) -> ::std::os::raw::c_int;
    pub fn EXTPTR_PROT(arg1: SEXP) -> SEXP;
    pub fn EXTPTR_TAG(arg1: SEXP) -> SEXP;
    pub fn EXTPTR_PTR(arg1: SEXP) -> *mut ::std::os::raw::c_void;
    pub static mut R_GlobalEnv: SEXP;
    pub static mut R_EmptyEnv: SEXP;
    pub static mut R_BaseEnv: SEXP;
    pub static mut R_BaseNamespace: SEXP;
    pub static mut R_NamespaceRegistry: SEXP;
    pub static mut R_Srcref: SEXP;
    pub static mut R_NilValue: SEXP;
    pub static mut R_UnboundValue: SEXP;
    pub static mut R_MissingArg: SEXP;
    pub static mut R_InBCInterpreter: SEXP;
    pub static mut R_CurrentExpression: SEXP;
    pub static mut R_RestartToken: SEXP;
    pub static mut R_AsCharacterSymbol: SEXP;
    pub static mut R_AtsignSymbol: SEXP;
    pub static mut R_baseSymbol: SEXP;
    pub static mut R_BaseSymbol: SEXP;
    pub static mut R_BraceSymbol: SEXP;
    pub static mut R_Bracket2Symbol: SEXP;
    pub static mut R_BracketSymbol: SEXP;
    pub static mut R_ClassSymbol: SEXP;
    pub static mut R_DeviceSymbol: SEXP;
    pub static mut R_DimNamesSymbol: SEXP;
    pub static mut R_DimSymbol: SEXP;
    pub static mut R_DollarSymbol: SEXP;
    pub static mut R_DotsSymbol: SEXP;
    pub static mut R_DoubleColonSymbol: SEXP;
    pub static mut R_DropSymbol: SEXP;
    pub static mut R_EvalSymbol: SEXP;
    pub static mut R_FunctionSymbol: SEXP;
    pub static mut R_LastvalueSymbol: SEXP;
    pub static mut R_LevelsSymbol: SEXP;
    pub static mut R_ModeSymbol: SEXP;
    pub static mut R_NaRmSymbol: SEXP;
    pub static mut R_NameSymbol: SEXP;
    pub static mut R_NamesSymbol: SEXP;
    pub static mut R_NamespaceEnvSymbol: SEXP;
    pub static mut R_PackageSymbol: SEXP;
    pub static mut R_PreviousSymbol: SEXP;
    pub static mut R_QuoteSymbol: SEXP;
    pub static mut R_RowNamesSymbol: SEXP;
    pub static mut R_SeedsSymbol: SEXP;
    pub static mut R_SortListSymbol: SEXP;
    pub static mut R_SourceSymbol: SEXP;
    pub static mut R_SpecSymbol: SEXP;
    pub static mut R_TripleColonSymbol: SEXP;
    pub static mut R_TspSymbol: SEXP;
    pub static mut R_dot_defined: SEXP;
    pub static mut R_dot_Method: SEXP;
    pub static mut R_dot_packageName: SEXP;
    pub static mut R_dot_target: SEXP;
    pub static mut R_dot_Generic: SEXP;
    pub static mut R_NaString: SEXP;
    pub static mut R_BlankString: SEXP;
    pub static mut R_BlankScalarString: SEXP;
    pub fn R_GetCurrentSrcref(arg1: ::std::os::raw::c_int) -> SEXP;
    pub fn R_GetSrcFilename(arg1: SEXP) -> SEXP;
    pub fn Rf_asChar(arg1: SEXP) -> SEXP;
    pub fn Rf_coerceVector(arg1: SEXP, arg2: SEXPTYPE) -> SEXP;
    pub fn Rf_PairToVectorList(x: SEXP) -> SEXP;
    pub fn Rf_VectorToPairList(x: SEXP) -> SEXP;
    pub fn Rf_asCharacterFactor(x: SEXP) -> SEXP;
    pub fn Rf_asLogical(x: SEXP) -> ::std::os::raw::c_int;
    pub fn Rf_asInteger(x: SEXP) -> ::std::os::raw::c_int;
    pub fn Rf_asReal(x: SEXP) -> f64;
    pub fn Rf_asComplex(x: SEXP) -> Rcomplex;
    pub fn Rf_acopy_string(arg1: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
    pub fn Rf_alloc3DArray(
        arg1: SEXPTYPE,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
    ) -> SEXP;
    pub fn Rf_allocArray(arg1: SEXPTYPE, arg2: SEXP) -> SEXP;
    pub fn Rf_allocMatrix(
        arg1: SEXPTYPE,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    ) -> SEXP;
    pub fn Rf_allocLang(arg1: ::std::os::raw::c_int) -> SEXP;
    pub fn Rf_allocList(arg1: ::std::os::raw::c_int) -> SEXP;
    pub fn Rf_allocS4Object() -> SEXP;
    pub fn Rf_allocSExp(arg1: SEXPTYPE) -> SEXP;
    pub fn Rf_allocVector3(arg1: SEXPTYPE, arg2: R_xlen_t, arg3: *mut R_allocator_t) -> SEXP;
    pub fn Rf_any_duplicated(x: SEXP, from_last: Rboolean) -> R_xlen_t;
    pub fn Rf_any_duplicated3(x: SEXP, incomp: SEXP, from_last: Rboolean) -> R_xlen_t;
    pub fn Rf_classgets(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_cons(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_copyMatrix(arg1: SEXP, arg2: SEXP, arg3: Rboolean);
    pub fn Rf_copyListMatrix(arg1: SEXP, arg2: SEXP, arg3: Rboolean);
    pub fn Rf_copyMostAttrib(arg1: SEXP, arg2: SEXP);
    pub fn Rf_copyVector(arg1: SEXP, arg2: SEXP);
    pub fn Rf_defineVar(arg1: SEXP, arg2: SEXP, arg3: SEXP);
    pub fn Rf_dimgets(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_dimnamesgets(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_duplicate(arg1: SEXP) -> SEXP;
    pub fn Rf_shallow_duplicate(arg1: SEXP) -> SEXP;
    pub fn R_duplicate_attr(arg1: SEXP) -> SEXP;
    pub fn R_shallow_duplicate_attr(arg1: SEXP) -> SEXP;
    pub fn Rf_lazy_duplicate(arg1: SEXP) -> SEXP;
    pub fn Rf_duplicated(arg1: SEXP, arg2: Rboolean) -> SEXP;
    pub fn Rf_eval(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_findFun(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_findVar(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_findVarInFrame(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_findVarInFrame3(arg1: SEXP, arg2: SEXP, arg3: Rboolean) -> SEXP;
    pub fn R_existsVarInFrame(arg1: SEXP, arg2: SEXP) -> Rboolean;
    pub fn R_removeVarFromFrame(arg1: SEXP, arg2: SEXP);
    pub fn Rf_getAttrib(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_GetArrayDimnames(arg1: SEXP) -> SEXP;
    pub fn Rf_GetColNames(arg1: SEXP) -> SEXP;
    pub fn Rf_GetMatrixDimnames(
        arg1: SEXP,
        arg2: *mut SEXP,
        arg3: *mut SEXP,
        arg4: *mut *const ::std::os::raw::c_char,
        arg5: *mut *const ::std::os::raw::c_char,
    );
    pub fn Rf_GetOption(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_GetOption1(arg1: SEXP) -> SEXP;
    pub fn Rf_GetOptionDigits() -> ::std::os::raw::c_int;
    pub fn Rf_GetOptionWidth() -> ::std::os::raw::c_int;
    pub fn Rf_GetRowNames(arg1: SEXP) -> SEXP;
    pub fn Rf_gsetVar(arg1: SEXP, arg2: SEXP, arg3: SEXP);
    pub fn Rf_install(arg1: *const ::std::os::raw::c_char) -> SEXP;
    pub fn Rf_installChar(arg1: SEXP) -> SEXP;
    pub fn Rf_installNoTrChar(arg1: SEXP) -> SEXP;
    pub fn Rf_installTrChar(arg1: SEXP) -> SEXP;
    pub fn Rf_isOrdered(arg1: SEXP) -> Rboolean;
    pub fn Rf_isUnordered(arg1: SEXP) -> Rboolean;
    pub fn Rf_isUnsorted(arg1: SEXP, arg2: Rboolean) -> Rboolean;
    pub fn R_isTRUE(arg1: SEXP) -> Rboolean;
    pub fn Rf_lengthgets(arg1: SEXP, arg2: R_len_t) -> SEXP;
    pub fn Rf_xlengthgets(arg1: SEXP, arg2: R_xlen_t) -> SEXP;
    pub fn R_lsInternal(arg1: SEXP, arg2: Rboolean) -> SEXP;
    pub fn R_lsInternal3(arg1: SEXP, arg2: Rboolean, arg3: Rboolean) -> SEXP;
    pub fn Rf_match(arg1: SEXP, arg2: SEXP, arg3: ::std::os::raw::c_int) -> SEXP;
    pub fn Rf_namesgets(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_mkChar(arg1: *const ::std::os::raw::c_char) -> SEXP;
    pub fn Rf_mkCharLen(arg1: *const ::std::os::raw::c_char, arg2: ::std::os::raw::c_int) -> SEXP;
    pub fn Rf_NonNullStringMatch(arg1: SEXP, arg2: SEXP) -> Rboolean;
    pub fn Rf_ncols(arg1: SEXP) -> ::std::os::raw::c_int;
    pub fn Rf_nrows(arg1: SEXP) -> ::std::os::raw::c_int;
    pub fn Rf_nthcdr(arg1: SEXP, arg2: ::std::os::raw::c_int) -> SEXP;
    pub fn R_nchar(
        string: SEXP,
        type_: nchar_type,
        allowNA: Rboolean,
        keepNA: Rboolean,
        msg_name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn R_ParseEvalString(arg1: *const ::std::os::raw::c_char, arg2: SEXP) -> SEXP;
    pub fn R_ParseString(arg1: *const ::std::os::raw::c_char) -> SEXP;
    pub fn Rf_PrintValue(arg1: SEXP);
    pub fn Rf_setAttrib(arg1: SEXP, arg2: SEXP, arg3: SEXP) -> SEXP;
    pub fn Rf_setVar(arg1: SEXP, arg2: SEXP, arg3: SEXP);
    pub fn Rf_str2type(arg1: *const ::std::os::raw::c_char) -> SEXPTYPE;
    pub fn Rf_StringBlank(arg1: SEXP) -> Rboolean;
    pub fn Rf_substitute(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_topenv(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_translateChar(arg1: SEXP) -> *const ::std::os::raw::c_char;
    pub fn Rf_translateCharUTF8(arg1: SEXP) -> *const ::std::os::raw::c_char;
    pub fn Rf_type2char(arg1: SEXPTYPE) -> *const ::std::os::raw::c_char;
    pub fn R_typeToChar(arg1: SEXP) -> *const ::std::os::raw::c_char;
    pub fn Rf_type2rstr(arg1: SEXPTYPE) -> SEXP;
    pub fn Rf_type2str(arg1: SEXPTYPE) -> SEXP;
    pub fn Rf_type2str_nowarn(arg1: SEXPTYPE) -> SEXP;
    pub fn Rf_unprotect_ptr(arg1: SEXP);
    pub fn R_tryEval(arg1: SEXP, arg2: SEXP, arg3: *mut ::std::os::raw::c_int) -> SEXP;
    pub fn R_tryEvalSilent(arg1: SEXP, arg2: SEXP, arg3: *mut ::std::os::raw::c_int) -> SEXP;
    pub fn R_GetCurrentEnv() -> SEXP;
    pub fn Rf_isS4(arg1: SEXP) -> Rboolean;
    pub fn Rf_asS4(arg1: SEXP, arg2: Rboolean, arg3: ::std::os::raw::c_int) -> SEXP;
    pub fn Rf_S3Class(arg1: SEXP) -> SEXP;
    pub fn Rf_isBasicClass(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn Rf_getCharCE(arg1: SEXP) -> cetype_t;
    pub fn Rf_mkCharCE(arg1: *const ::std::os::raw::c_char, arg2: cetype_t) -> SEXP;
    pub fn Rf_mkCharLenCE(
        arg1: *const ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
        arg3: cetype_t,
    ) -> SEXP;
    pub fn Rf_reEnc(
        x: *const ::std::os::raw::c_char,
        ce_in: cetype_t,
        ce_out: cetype_t,
        subst: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
    pub fn Rf_reEnc3(
        x: *const ::std::os::raw::c_char,
        fromcode: *const ::std::os::raw::c_char,
        tocode: *const ::std::os::raw::c_char,
        subst: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
    pub fn R_forceAndCall(e: SEXP, n: ::std::os::raw::c_int, rho: SEXP) -> SEXP;
    pub fn R_MakeExternalPtr(p: *mut ::std::os::raw::c_void, tag: SEXP, prot: SEXP) -> SEXP;
    pub fn R_ExternalPtrAddr(s: SEXP) -> *mut ::std::os::raw::c_void;
    pub fn R_ExternalPtrTag(s: SEXP) -> SEXP;
    pub fn R_ExternalPtrProtected(s: SEXP) -> SEXP;
    pub fn R_ClearExternalPtr(s: SEXP);
    pub fn R_SetExternalPtrAddr(s: SEXP, p: *mut ::std::os::raw::c_void);
    pub fn R_SetExternalPtrTag(s: SEXP, tag: SEXP);
    pub fn R_SetExternalPtrProtected(s: SEXP, p: SEXP);
    pub fn R_MakeExternalPtrFn(p: DL_FUNC, tag: SEXP, prot: SEXP) -> SEXP;
    pub fn R_ExternalPtrAddrFn(s: SEXP) -> DL_FUNC;
    pub fn R_RegisterFinalizer(s: SEXP, fun: SEXP);
    pub fn R_RegisterCFinalizer(s: SEXP, fun: R_CFinalizer_t);
    pub fn R_RegisterFinalizerEx(s: SEXP, fun: SEXP, onexit: Rboolean);
    pub fn R_RegisterCFinalizerEx(s: SEXP, fun: R_CFinalizer_t, onexit: Rboolean);
    pub fn R_RunPendingFinalizers();
    pub fn R_MakeWeakRef(key: SEXP, val: SEXP, fin: SEXP, onexit: Rboolean) -> SEXP;
    pub fn R_MakeWeakRefC(key: SEXP, val: SEXP, fin: R_CFinalizer_t, onexit: Rboolean) -> SEXP;
    pub fn R_WeakRefKey(w: SEXP) -> SEXP;
    pub fn R_WeakRefValue(w: SEXP) -> SEXP;
    pub fn R_RunWeakRefFinalizer(w: SEXP);
    pub fn R_PromiseExpr(arg1: SEXP) -> SEXP;
    pub fn R_ClosureExpr(arg1: SEXP) -> SEXP;
    pub fn R_BytecodeExpr(e: SEXP) -> SEXP;
    pub fn R_ToplevelExec(
        fun: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
        data: *mut ::std::os::raw::c_void,
    ) -> Rboolean;
    pub fn R_ExecWithCleanup(
        fun: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> SEXP>,
        data: *mut ::std::os::raw::c_void,
        cleanfun: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
        cleandata: *mut ::std::os::raw::c_void,
    ) -> SEXP;
    pub fn R_tryCatch(
        arg1: ::std::option::Option<
            unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> SEXP,
        >,
        arg2: *mut ::std::os::raw::c_void,
        arg3: SEXP,
        arg4: ::std::option::Option<
            unsafe extern "C" fn(arg1: SEXP, arg2: *mut ::std::os::raw::c_void) -> SEXP,
        >,
        arg5: *mut ::std::os::raw::c_void,
        arg6: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
        arg7: *mut ::std::os::raw::c_void,
    ) -> SEXP;
    pub fn R_tryCatchError(
        arg1: ::std::option::Option<
            unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> SEXP,
        >,
        arg2: *mut ::std::os::raw::c_void,
        arg3: ::std::option::Option<
            unsafe extern "C" fn(arg1: SEXP, arg2: *mut ::std::os::raw::c_void) -> SEXP,
        >,
        arg4: *mut ::std::os::raw::c_void,
    ) -> SEXP;
    pub fn R_withCallingErrorHandler(
        arg1: ::std::option::Option<
            unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> SEXP,
        >,
        arg2: *mut ::std::os::raw::c_void,
        arg3: ::std::option::Option<
            unsafe extern "C" fn(arg1: SEXP, arg2: *mut ::std::os::raw::c_void) -> SEXP,
        >,
        arg4: *mut ::std::os::raw::c_void,
    ) -> SEXP;
    pub fn R_MakeUnwindCont() -> SEXP;
    pub fn R_ContinueUnwind(cont: SEXP);
    pub fn R_UnwindProtect(
        fun: ::std::option::Option<unsafe extern "C" fn(data: *mut ::std::os::raw::c_void) -> SEXP>,
        data: *mut ::std::os::raw::c_void,
        cleanfun: ::std::option::Option<
            unsafe extern "C" fn(data: *mut ::std::os::raw::c_void, jump: Rboolean),
        >,
        cleandata: *mut ::std::os::raw::c_void,
        cont: SEXP,
    ) -> SEXP;
    pub fn R_NewEnv(arg1: SEXP, arg2: ::std::os::raw::c_int, arg3: ::std::os::raw::c_int) -> SEXP;
    pub fn R_IsPackageEnv(rho: SEXP) -> Rboolean;
    pub fn R_PackageEnvName(rho: SEXP) -> SEXP;
    pub fn R_FindPackageEnv(info: SEXP) -> SEXP;
    pub fn R_IsNamespaceEnv(rho: SEXP) -> Rboolean;
    pub fn R_NamespaceEnvSpec(rho: SEXP) -> SEXP;
    pub fn R_FindNamespace(info: SEXP) -> SEXP;
    pub fn R_LockEnvironment(env: SEXP, bindings: Rboolean);
    pub fn R_EnvironmentIsLocked(env: SEXP) -> Rboolean;
    pub fn R_LockBinding(sym: SEXP, env: SEXP);
    pub fn R_unLockBinding(sym: SEXP, env: SEXP);
    pub fn R_MakeActiveBinding(sym: SEXP, fun: SEXP, env: SEXP);
    pub fn R_BindingIsLocked(sym: SEXP, env: SEXP) -> Rboolean;
    pub fn R_BindingIsActive(sym: SEXP, env: SEXP) -> Rboolean;
    pub fn R_ActiveBindingFunction(sym: SEXP, env: SEXP) -> SEXP;
    pub fn R_HasFancyBindings(rho: SEXP) -> Rboolean;
    pub fn Rf_errorcall(arg1: SEXP, arg2: *const ::std::os::raw::c_char, ...);
    pub fn Rf_warningcall(arg1: SEXP, arg2: *const ::std::os::raw::c_char, ...);
    pub fn Rf_warningcall_immediate(arg1: SEXP, arg2: *const ::std::os::raw::c_char, ...);
    pub fn R_XDREncodeDouble(d: f64, buf: *mut ::std::os::raw::c_void);
    pub fn R_XDRDecodeDouble(buf: *mut ::std::os::raw::c_void) -> f64;
    pub fn R_XDREncodeInteger(i: ::std::os::raw::c_int, buf: *mut ::std::os::raw::c_void);
    pub fn R_XDRDecodeInteger(buf: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
    pub fn R_InitInPStream(
        stream: R_inpstream_t,
        data: R_pstream_data_t,
        type_: R_pstream_format_t,
        inchar: ::std::option::Option<
            unsafe extern "C" fn(arg1: R_inpstream_t) -> ::std::os::raw::c_int,
        >,
        inbytes: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: R_inpstream_t,
                arg2: *mut ::std::os::raw::c_void,
                arg3: ::std::os::raw::c_int,
            ),
        >,
        phook: ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: SEXP) -> SEXP>,
        pdata: SEXP,
    );
    pub fn R_InitOutPStream(
        stream: R_outpstream_t,
        data: R_pstream_data_t,
        type_: R_pstream_format_t,
        version: ::std::os::raw::c_int,
        outchar: ::std::option::Option<
            unsafe extern "C" fn(arg1: R_outpstream_t, arg2: ::std::os::raw::c_int),
        >,
        outbytes: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: R_outpstream_t,
                arg2: *mut ::std::os::raw::c_void,
                arg3: ::std::os::raw::c_int,
            ),
        >,
        phook: ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: SEXP) -> SEXP>,
        pdata: SEXP,
    );
    pub fn R_InitFileInPStream(
        stream: R_inpstream_t,
        fp: *mut FILE,
        type_: R_pstream_format_t,
        phook: ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: SEXP) -> SEXP>,
        pdata: SEXP,
    );
    pub fn R_InitFileOutPStream(
        stream: R_outpstream_t,
        fp: *mut FILE,
        type_: R_pstream_format_t,
        version: ::std::os::raw::c_int,
        phook: ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: SEXP) -> SEXP>,
        pdata: SEXP,
    );
    pub fn R_Serialize(s: SEXP, ops: R_outpstream_t);
    pub fn R_Unserialize(ips: R_inpstream_t) -> SEXP;
    pub fn R_SerializeInfo(ips: R_inpstream_t) -> SEXP;
    pub fn R_do_slot(obj: SEXP, name: SEXP) -> SEXP;
    pub fn R_do_slot_assign(obj: SEXP, name: SEXP, value: SEXP) -> SEXP;
    pub fn R_has_slot(obj: SEXP, name: SEXP) -> ::std::os::raw::c_int;
    pub fn R_S4_extends(klass: SEXP, useTable: SEXP) -> SEXP;
    pub fn R_do_MAKE_CLASS(what: *const ::std::os::raw::c_char) -> SEXP;
    pub fn R_getClassDef(what: *const ::std::os::raw::c_char) -> SEXP;
    pub fn R_getClassDef_R(what: SEXP) -> SEXP;
    pub fn R_has_methods_attached() -> Rboolean;
    pub fn R_isVirtualClass(class_def: SEXP, env: SEXP) -> Rboolean;
    pub fn R_extends(class1: SEXP, class2: SEXP, env: SEXP) -> Rboolean;
    pub fn R_do_new_object(class_def: SEXP) -> SEXP;
    pub fn R_check_class_and_super(
        x: SEXP,
        valid: *mut *const ::std::os::raw::c_char,
        rho: SEXP,
    ) -> ::std::os::raw::c_int;
    pub fn R_check_class_etc(
        x: SEXP,
        valid: *mut *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn R_PreserveObject(arg1: SEXP);
    pub fn R_ReleaseObject(arg1: SEXP);
    pub fn R_NewPreciousMSet(arg1: ::std::os::raw::c_int) -> SEXP;
    pub fn R_PreserveInMSet(x: SEXP, mset: SEXP);
    pub fn R_ReleaseFromMSet(x: SEXP, mset: SEXP);
    pub fn R_ReleaseMSet(mset: SEXP, keepSize: ::std::os::raw::c_int);
    pub fn R_dot_Last();
    pub fn R_RunExitFinalizers();
    pub fn R_system(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn R_compute_identical(arg1: SEXP, arg2: SEXP, arg3: ::std::os::raw::c_int) -> Rboolean;
    pub fn R_body_no_src(x: SEXP) -> SEXP;
    pub fn R_orderVector(
        indx: *mut ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        arglist: SEXP,
        nalast: Rboolean,
        decreasing: Rboolean,
    );
    pub fn R_orderVector1(
        indx: *mut ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        x: SEXP,
        nalast: Rboolean,
        decreasing: Rboolean,
    );
    pub fn Rf_allocVector(arg1: SEXPTYPE, arg2: R_xlen_t) -> SEXP;
    pub fn Rf_conformable(arg1: SEXP, arg2: SEXP) -> Rboolean;
    pub fn Rf_elt(arg1: SEXP, arg2: ::std::os::raw::c_int) -> SEXP;
    pub fn Rf_inherits(arg1: SEXP, arg2: *const ::std::os::raw::c_char) -> Rboolean;
    pub fn Rf_isArray(arg1: SEXP) -> Rboolean;
    pub fn Rf_isFactor(arg1: SEXP) -> Rboolean;
    pub fn Rf_isFrame(arg1: SEXP) -> Rboolean;
    pub fn Rf_isFunction(arg1: SEXP) -> Rboolean;
    pub fn Rf_isInteger(arg1: SEXP) -> Rboolean;
    pub fn Rf_isLanguage(arg1: SEXP) -> Rboolean;
    pub fn Rf_isList(arg1: SEXP) -> Rboolean;
    pub fn Rf_isMatrix(arg1: SEXP) -> Rboolean;
    pub fn Rf_isNewList(arg1: SEXP) -> Rboolean;
    pub fn Rf_isNumber(arg1: SEXP) -> Rboolean;
    pub fn Rf_isNumeric(arg1: SEXP) -> Rboolean;
    pub fn Rf_isPairList(arg1: SEXP) -> Rboolean;
    pub fn Rf_isPrimitive(arg1: SEXP) -> Rboolean;
    pub fn Rf_isTs(arg1: SEXP) -> Rboolean;
    pub fn Rf_isUserBinop(arg1: SEXP) -> Rboolean;
    pub fn Rf_isValidString(arg1: SEXP) -> Rboolean;
    pub fn Rf_isValidStringF(arg1: SEXP) -> Rboolean;
    pub fn Rf_isVector(arg1: SEXP) -> Rboolean;
    pub fn Rf_isVectorAtomic(arg1: SEXP) -> Rboolean;
    pub fn Rf_isVectorList(arg1: SEXP) -> Rboolean;
    pub fn Rf_isVectorizable(arg1: SEXP) -> Rboolean;
    pub fn Rf_lang1(arg1: SEXP) -> SEXP;
    pub fn Rf_lang2(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_lang3(arg1: SEXP, arg2: SEXP, arg3: SEXP) -> SEXP;
    pub fn Rf_lang4(arg1: SEXP, arg2: SEXP, arg3: SEXP, arg4: SEXP) -> SEXP;
    pub fn Rf_lang5(arg1: SEXP, arg2: SEXP, arg3: SEXP, arg4: SEXP, arg5: SEXP) -> SEXP;
    pub fn Rf_lang6(arg1: SEXP, arg2: SEXP, arg3: SEXP, arg4: SEXP, arg5: SEXP, arg6: SEXP)
        -> SEXP;
    pub fn Rf_lastElt(arg1: SEXP) -> SEXP;
    pub fn Rf_lcons(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_length(arg1: SEXP) -> R_len_t;
    pub fn Rf_list1(arg1: SEXP) -> SEXP;
    pub fn Rf_list2(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_list3(arg1: SEXP, arg2: SEXP, arg3: SEXP) -> SEXP;
    pub fn Rf_list4(arg1: SEXP, arg2: SEXP, arg3: SEXP, arg4: SEXP) -> SEXP;
    pub fn Rf_list5(arg1: SEXP, arg2: SEXP, arg3: SEXP, arg4: SEXP, arg5: SEXP) -> SEXP;
    pub fn Rf_list6(arg1: SEXP, arg2: SEXP, arg3: SEXP, arg4: SEXP, arg5: SEXP, arg6: SEXP)
        -> SEXP;
    pub fn Rf_listAppend(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_mkNamed(arg1: SEXPTYPE, arg2: *mut *const ::std::os::raw::c_char) -> SEXP;
    pub fn Rf_mkString(arg1: *const ::std::os::raw::c_char) -> SEXP;
    pub fn Rf_nlevels(arg1: SEXP) -> ::std::os::raw::c_int;
    pub fn Rf_stringPositionTr(
        arg1: SEXP,
        arg2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn Rf_ScalarComplex(arg1: Rcomplex) -> SEXP;
    pub fn Rf_ScalarInteger(arg1: ::std::os::raw::c_int) -> SEXP;
    pub fn Rf_ScalarLogical(arg1: ::std::os::raw::c_int) -> SEXP;
    pub fn Rf_ScalarRaw(arg1: Rbyte) -> SEXP;
    pub fn Rf_ScalarReal(arg1: f64) -> SEXP;
    pub fn Rf_ScalarString(arg1: SEXP) -> SEXP;
    pub fn Rf_xlength(arg1: SEXP) -> R_xlen_t;
    pub fn XTRUELENGTH(x: SEXP) -> R_xlen_t;
    pub fn LENGTH_EX(
        x: SEXP,
        file: *const ::std::os::raw::c_char,
        line: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn XLENGTH_EX(x: SEXP) -> R_xlen_t;
    pub fn Rf_protect(arg1: SEXP) -> SEXP;
    pub fn Rf_unprotect(arg1: ::std::os::raw::c_int);
    pub fn R_ProtectWithIndex(arg1: SEXP, arg2: *mut PROTECT_INDEX);
    pub fn R_Reprotect(arg1: SEXP, arg2: PROTECT_INDEX);
    pub fn CAR(e: SEXP) -> SEXP;
    pub fn DATAPTR(x: SEXP) -> *mut ::std::os::raw::c_void;
    pub fn DATAPTR_RO(x: SEXP) -> *const ::std::os::raw::c_void;
    pub fn DATAPTR_OR_NULL(x: SEXP) -> *const ::std::os::raw::c_void;
    pub fn LOGICAL_OR_NULL(x: SEXP) -> *const ::std::os::raw::c_int;
    pub fn INTEGER_OR_NULL(x: SEXP) -> *const ::std::os::raw::c_int;
    pub fn REAL_OR_NULL(x: SEXP) -> *const f64;
    pub fn COMPLEX_OR_NULL(x: SEXP) -> *const Rcomplex;
    pub fn RAW_OR_NULL(x: SEXP) -> *const Rbyte;
    pub fn INTEGER_ELT(x: SEXP, i: R_xlen_t) -> ::std::os::raw::c_int;
    pub fn REAL_ELT(x: SEXP, i: R_xlen_t) -> f64;
    pub fn LOGICAL_ELT(x: SEXP, i: R_xlen_t) -> ::std::os::raw::c_int;
    pub fn COMPLEX_ELT(x: SEXP, i: R_xlen_t) -> Rcomplex;
    pub fn RAW_ELT(x: SEXP, i: R_xlen_t) -> Rbyte;
    pub fn STRING_ELT(x: SEXP, i: R_xlen_t) -> SEXP;
    pub fn SET_LOGICAL_ELT(x: SEXP, i: R_xlen_t, v: ::std::os::raw::c_int);
    pub fn SET_INTEGER_ELT(x: SEXP, i: R_xlen_t, v: ::std::os::raw::c_int);
    pub fn SET_REAL_ELT(x: SEXP, i: R_xlen_t, v: f64);
    pub fn SET_COMPLEX_ELT(x: SEXP, i: R_xlen_t, v: Rcomplex);
    pub fn SET_RAW_ELT(x: SEXP, i: R_xlen_t, v: Rbyte);
    pub fn ALTREP_CLASS(x: SEXP) -> SEXP;
    pub fn R_altrep_data1(x: SEXP) -> SEXP;
    pub fn R_altrep_data2(x: SEXP) -> SEXP;
    pub fn R_set_altrep_data1(x: SEXP, v: SEXP);
    pub fn R_set_altrep_data2(x: SEXP, v: SEXP);
    pub fn LOGICAL0(x: SEXP) -> *mut ::std::os::raw::c_int;
    pub fn INTEGER0(x: SEXP) -> *mut ::std::os::raw::c_int;
    pub fn REAL0(x: SEXP) -> *mut f64;
    pub fn COMPLEX0(x: SEXP) -> *mut Rcomplex;
    pub fn RAW0(x: SEXP) -> *mut Rbyte;
    pub fn ALTREP(x: SEXP) -> ::std::os::raw::c_int;
    pub fn R_asHashtable(h: SEXP) -> R_hashtab_type;
    pub fn R_HashtabSEXP(h: R_hashtab_type) -> SEXP;
    pub fn R_isHashtable(h: SEXP) -> ::std::os::raw::c_int;
    pub fn R_mkhashtab(type_: ::std::os::raw::c_int, arg1: ::std::os::raw::c_int)
        -> R_hashtab_type;
    pub fn R_gethash(h: R_hashtab_type, key: SEXP, nomatch: SEXP) -> SEXP;
    pub fn R_sethash(h: R_hashtab_type, key: SEXP, value: SEXP) -> SEXP;
    pub fn R_remhash(h: R_hashtab_type, key: SEXP) -> ::std::os::raw::c_int;
    pub fn R_numhash(h: R_hashtab_type) -> ::std::os::raw::c_int;
    pub fn R_typhash(h: R_hashtab_type) -> ::std::os::raw::c_int;
    pub fn R_maphash(h: R_hashtab_type, FUN: SEXP) -> SEXP;
    pub fn R_maphashC(
        h: R_hashtab_type,
        FUN: ::std::option::Option<
            unsafe extern "C" fn(arg1: SEXP, arg2: SEXP, arg3: *mut ::std::os::raw::c_void),
        >,
        data: *mut ::std::os::raw::c_void,
    );
    pub fn R_clrhash(h: R_hashtab_type);
    pub fn SET_TYPEOF(x: SEXP, v: ::std::os::raw::c_int);
    pub fn SET_OBJECT(x: SEXP, v: ::std::os::raw::c_int);
    pub fn SET_S4_OBJECT(x: SEXP);
    pub fn UNSET_S4_OBJECT(x: SEXP);
    pub fn R_curErrorBuf() -> *const ::std::os::raw::c_char;
    pub fn IS_SCALAR(x: SEXP, type_: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn Rf_psmatch(
        arg1: *const ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
        arg3: Rboolean,
    ) -> Rboolean;
    pub fn SETLENGTH(x: SEXP, v: R_xlen_t);
    pub fn SET_TRUELENGTH(x: SEXP, v: R_xlen_t);
    pub fn SETLEVELS(x: SEXP, v: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn SET_ENVFLAGS(x: SEXP, v: ::std::os::raw::c_int);
    pub fn SET_FRAME(x: SEXP, v: SEXP);
    pub fn SET_ENCLOS(x: SEXP, v: SEXP);
    pub fn SET_HASHTAB(x: SEXP, v: SEXP);
    pub fn SET_PRENV(x: SEXP, v: SEXP);
    pub fn SET_PRVALUE(x: SEXP, v: SEXP);
    pub fn SET_PRCODE(x: SEXP, v: SEXP);
    pub fn STDVEC_DATAPTR(x: SEXP) -> *mut ::std::os::raw::c_void;
    pub fn IS_GROWABLE(x: SEXP) -> ::std::os::raw::c_int;
    pub fn SET_GROWABLE_BIT(x: SEXP);
    pub fn SET_NAMED(x: SEXP, v: ::std::os::raw::c_int);
    pub fn R_tryWrap(arg1: SEXP) -> SEXP;
}
