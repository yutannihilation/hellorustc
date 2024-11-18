/* automatically generated by rust-bindgen 0.70.1 */

pub const QNPF_REDRAW: u32 = 1;
pub const QDFLAG_DISPLAY_LIST: u32 = 1;
pub const QDFLAG_INTERACTIVE: u32 = 2;
pub const QDFLAG_RASTERIZED: u32 = 4;
pub const QPFLAG_ANTIALIAS: u32 = 256;
pub const QuartzParam_EmbeddingFlags: &[u8; 16] = b"embedding flags\0";
pub const QP_Flags_CFLoop: u32 = 1;
pub const QP_Flags_Cocoa: u32 = 2;
pub const QP_Flags_Front: u32 = 4;
pub type CGContextRef = *mut ::std::os::raw::c_void;
pub type QuartzDesc_t = *mut ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct QuartzBackend_s {
    pub size: ::std::os::raw::c_int,
    pub width: f64,
    pub height: f64,
    pub scalex: f64,
    pub scaley: f64,
    pub pointsize: f64,
    pub bg: ::std::os::raw::c_int,
    pub canvas: ::std::os::raw::c_int,
    pub flags: ::std::os::raw::c_int,
    pub userInfo: *mut ::std::os::raw::c_void,
    pub getCGContext: ::std::option::Option<
        unsafe extern "C" fn(
            dev: QuartzDesc_t,
            userInfo: *mut ::std::os::raw::c_void,
        ) -> CGContextRef,
    >,
    pub locatePoint: ::std::option::Option<
        unsafe extern "C" fn(
            dev: QuartzDesc_t,
            userInfo: *mut ::std::os::raw::c_void,
            x: *mut f64,
            y: *mut f64,
        ) -> ::std::os::raw::c_int,
    >,
    pub close: ::std::option::Option<
        unsafe extern "C" fn(dev: QuartzDesc_t, userInfo: *mut ::std::os::raw::c_void),
    >,
    pub newPage: ::std::option::Option<
        unsafe extern "C" fn(
            dev: QuartzDesc_t,
            userInfo: *mut ::std::os::raw::c_void,
            flags: ::std::os::raw::c_int,
        ),
    >,
    pub state: ::std::option::Option<
        unsafe extern "C" fn(
            dev: QuartzDesc_t,
            userInfo: *mut ::std::os::raw::c_void,
            state: ::std::os::raw::c_int,
        ),
    >,
    pub par: ::std::option::Option<
        unsafe extern "C" fn(
            dev: QuartzDesc_t,
            userInfo: *mut ::std::os::raw::c_void,
            set: ::std::os::raw::c_int,
            key: *const ::std::os::raw::c_char,
            value: *mut ::std::os::raw::c_void,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub sync: ::std::option::Option<
        unsafe extern "C" fn(dev: QuartzDesc_t, userInfo: *mut ::std::os::raw::c_void),
    >,
    pub cap: ::std::option::Option<
        unsafe extern "C" fn(
            dev: QuartzDesc_t,
            userInfo: *mut ::std::os::raw::c_void,
        ) -> *mut ::std::os::raw::c_void,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of QuartzBackend_s"][::std::mem::size_of::<QuartzBackend_s>() - 136usize];
    ["Alignment of QuartzBackend_s"][::std::mem::align_of::<QuartzBackend_s>() - 8usize];
    ["Offset of field: QuartzBackend_s::size"]
        [::std::mem::offset_of!(QuartzBackend_s, size) - 0usize];
    ["Offset of field: QuartzBackend_s::width"]
        [::std::mem::offset_of!(QuartzBackend_s, width) - 8usize];
    ["Offset of field: QuartzBackend_s::height"]
        [::std::mem::offset_of!(QuartzBackend_s, height) - 16usize];
    ["Offset of field: QuartzBackend_s::scalex"]
        [::std::mem::offset_of!(QuartzBackend_s, scalex) - 24usize];
    ["Offset of field: QuartzBackend_s::scaley"]
        [::std::mem::offset_of!(QuartzBackend_s, scaley) - 32usize];
    ["Offset of field: QuartzBackend_s::pointsize"]
        [::std::mem::offset_of!(QuartzBackend_s, pointsize) - 40usize];
    ["Offset of field: QuartzBackend_s::bg"][::std::mem::offset_of!(QuartzBackend_s, bg) - 48usize];
    ["Offset of field: QuartzBackend_s::canvas"]
        [::std::mem::offset_of!(QuartzBackend_s, canvas) - 52usize];
    ["Offset of field: QuartzBackend_s::flags"]
        [::std::mem::offset_of!(QuartzBackend_s, flags) - 56usize];
    ["Offset of field: QuartzBackend_s::userInfo"]
        [::std::mem::offset_of!(QuartzBackend_s, userInfo) - 64usize];
    ["Offset of field: QuartzBackend_s::getCGContext"]
        [::std::mem::offset_of!(QuartzBackend_s, getCGContext) - 72usize];
    ["Offset of field: QuartzBackend_s::locatePoint"]
        [::std::mem::offset_of!(QuartzBackend_s, locatePoint) - 80usize];
    ["Offset of field: QuartzBackend_s::close"]
        [::std::mem::offset_of!(QuartzBackend_s, close) - 88usize];
    ["Offset of field: QuartzBackend_s::newPage"]
        [::std::mem::offset_of!(QuartzBackend_s, newPage) - 96usize];
    ["Offset of field: QuartzBackend_s::state"]
        [::std::mem::offset_of!(QuartzBackend_s, state) - 104usize];
    ["Offset of field: QuartzBackend_s::par"]
        [::std::mem::offset_of!(QuartzBackend_s, par) - 112usize];
    ["Offset of field: QuartzBackend_s::sync"]
        [::std::mem::offset_of!(QuartzBackend_s, sync) - 120usize];
    ["Offset of field: QuartzBackend_s::cap"]
        [::std::mem::offset_of!(QuartzBackend_s, cap) - 128usize];
};
pub type QuartzBackend_t = QuartzBackend_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct QuartzParameters_s {
    pub size: ::std::os::raw::c_int,
    pub type_: *const ::std::os::raw::c_char,
    pub file: *const ::std::os::raw::c_char,
    pub title: *const ::std::os::raw::c_char,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub pointsize: f64,
    pub family: *const ::std::os::raw::c_char,
    pub flags: ::std::os::raw::c_int,
    pub connection: ::std::os::raw::c_int,
    pub bg: ::std::os::raw::c_int,
    pub canvas: ::std::os::raw::c_int,
    pub dpi: *mut f64,
    pub pard1: f64,
    pub pard2: f64,
    pub pari1: ::std::os::raw::c_int,
    pub pari2: ::std::os::raw::c_int,
    pub pars1: *const ::std::os::raw::c_char,
    pub pars2: *const ::std::os::raw::c_char,
    pub parv: *mut ::std::os::raw::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of QuartzParameters_s"][::std::mem::size_of::<QuartzParameters_s>() - 152usize];
    ["Alignment of QuartzParameters_s"][::std::mem::align_of::<QuartzParameters_s>() - 8usize];
    ["Offset of field: QuartzParameters_s::size"]
        [::std::mem::offset_of!(QuartzParameters_s, size) - 0usize];
    ["Offset of field: QuartzParameters_s::type_"]
        [::std::mem::offset_of!(QuartzParameters_s, type_) - 8usize];
    ["Offset of field: QuartzParameters_s::file"]
        [::std::mem::offset_of!(QuartzParameters_s, file) - 16usize];
    ["Offset of field: QuartzParameters_s::title"]
        [::std::mem::offset_of!(QuartzParameters_s, title) - 24usize];
    ["Offset of field: QuartzParameters_s::x"]
        [::std::mem::offset_of!(QuartzParameters_s, x) - 32usize];
    ["Offset of field: QuartzParameters_s::y"]
        [::std::mem::offset_of!(QuartzParameters_s, y) - 40usize];
    ["Offset of field: QuartzParameters_s::width"]
        [::std::mem::offset_of!(QuartzParameters_s, width) - 48usize];
    ["Offset of field: QuartzParameters_s::height"]
        [::std::mem::offset_of!(QuartzParameters_s, height) - 56usize];
    ["Offset of field: QuartzParameters_s::pointsize"]
        [::std::mem::offset_of!(QuartzParameters_s, pointsize) - 64usize];
    ["Offset of field: QuartzParameters_s::family"]
        [::std::mem::offset_of!(QuartzParameters_s, family) - 72usize];
    ["Offset of field: QuartzParameters_s::flags"]
        [::std::mem::offset_of!(QuartzParameters_s, flags) - 80usize];
    ["Offset of field: QuartzParameters_s::connection"]
        [::std::mem::offset_of!(QuartzParameters_s, connection) - 84usize];
    ["Offset of field: QuartzParameters_s::bg"]
        [::std::mem::offset_of!(QuartzParameters_s, bg) - 88usize];
    ["Offset of field: QuartzParameters_s::canvas"]
        [::std::mem::offset_of!(QuartzParameters_s, canvas) - 92usize];
    ["Offset of field: QuartzParameters_s::dpi"]
        [::std::mem::offset_of!(QuartzParameters_s, dpi) - 96usize];
    ["Offset of field: QuartzParameters_s::pard1"]
        [::std::mem::offset_of!(QuartzParameters_s, pard1) - 104usize];
    ["Offset of field: QuartzParameters_s::pard2"]
        [::std::mem::offset_of!(QuartzParameters_s, pard2) - 112usize];
    ["Offset of field: QuartzParameters_s::pari1"]
        [::std::mem::offset_of!(QuartzParameters_s, pari1) - 120usize];
    ["Offset of field: QuartzParameters_s::pari2"]
        [::std::mem::offset_of!(QuartzParameters_s, pari2) - 124usize];
    ["Offset of field: QuartzParameters_s::pars1"]
        [::std::mem::offset_of!(QuartzParameters_s, pars1) - 128usize];
    ["Offset of field: QuartzParameters_s::pars2"]
        [::std::mem::offset_of!(QuartzParameters_s, pars2) - 136usize];
    ["Offset of field: QuartzParameters_s::parv"]
        [::std::mem::offset_of!(QuartzParameters_s, parv) - 144usize];
};
pub type QuartzParameters_t = QuartzParameters_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct QuartzFunctons_s {
    pub Create: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *mut QuartzBackend_t,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub DevNumber:
        ::std::option::Option<unsafe extern "C" fn(desc: QuartzDesc_t) -> ::std::os::raw::c_int>,
    pub Kill: ::std::option::Option<unsafe extern "C" fn(desc: QuartzDesc_t)>,
    pub ResetContext: ::std::option::Option<unsafe extern "C" fn(desc: QuartzDesc_t)>,
    pub GetWidth: ::std::option::Option<unsafe extern "C" fn(desc: QuartzDesc_t) -> f64>,
    pub GetHeight: ::std::option::Option<unsafe extern "C" fn(desc: QuartzDesc_t) -> f64>,
    pub SetSize:
        ::std::option::Option<unsafe extern "C" fn(desc: QuartzDesc_t, width: f64, height: f64)>,
    pub GetScaledWidth: ::std::option::Option<unsafe extern "C" fn(desc: QuartzDesc_t) -> f64>,
    pub GetScaledHeight: ::std::option::Option<unsafe extern "C" fn(desc: QuartzDesc_t) -> f64>,
    pub SetScaledSize:
        ::std::option::Option<unsafe extern "C" fn(desc: QuartzDesc_t, width: f64, height: f64)>,
    pub GetXScale: ::std::option::Option<unsafe extern "C" fn(desc: QuartzDesc_t) -> f64>,
    pub GetYScale: ::std::option::Option<unsafe extern "C" fn(desc: QuartzDesc_t) -> f64>,
    pub SetScale:
        ::std::option::Option<unsafe extern "C" fn(desc: QuartzDesc_t, scalex: f64, scaley: f64)>,
    pub SetTextScale: ::std::option::Option<unsafe extern "C" fn(desc: QuartzDesc_t, scale: f64)>,
    pub GetTextScale: ::std::option::Option<unsafe extern "C" fn(desc: QuartzDesc_t) -> f64>,
    pub SetPointSize: ::std::option::Option<unsafe extern "C" fn(desc: QuartzDesc_t, ps: f64)>,
    pub GetPointSize: ::std::option::Option<unsafe extern "C" fn(desc: QuartzDesc_t) -> f64>,
    pub GetDirty:
        ::std::option::Option<unsafe extern "C" fn(desc: QuartzDesc_t) -> ::std::os::raw::c_int>,
    pub SetDirty: ::std::option::Option<
        unsafe extern "C" fn(desc: QuartzDesc_t, dirty: ::std::os::raw::c_int),
    >,
    pub ReplayDisplayList: ::std::option::Option<unsafe extern "C" fn(desc: QuartzDesc_t)>,
    pub GetSnapshot: ::std::option::Option<
        unsafe extern "C" fn(
            desc: QuartzDesc_t,
            last: ::std::os::raw::c_int,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub RestoreSnapshot: ::std::option::Option<
        unsafe extern "C" fn(desc: QuartzDesc_t, snapshot: *mut ::std::os::raw::c_void),
    >,
    pub GetAntialias:
        ::std::option::Option<unsafe extern "C" fn(desc: QuartzDesc_t) -> ::std::os::raw::c_int>,
    pub SetAntialias:
        ::std::option::Option<unsafe extern "C" fn(desc: QuartzDesc_t, aa: ::std::os::raw::c_int)>,
    pub GetBackground:
        ::std::option::Option<unsafe extern "C" fn(desc: QuartzDesc_t) -> ::std::os::raw::c_int>,
    pub Activate: ::std::option::Option<unsafe extern "C" fn(desc: QuartzDesc_t)>,
    pub SetParameter: ::std::option::Option<
        unsafe extern "C" fn(
            desc: QuartzDesc_t,
            key: *const ::std::os::raw::c_char,
            value: *mut ::std::os::raw::c_void,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub GetParameter: ::std::option::Option<
        unsafe extern "C" fn(
            desc: QuartzDesc_t,
            key: *const ::std::os::raw::c_char,
        ) -> *mut ::std::os::raw::c_void,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of QuartzFunctons_s"][::std::mem::size_of::<QuartzFunctons_s>() - 224usize];
    ["Alignment of QuartzFunctons_s"][::std::mem::align_of::<QuartzFunctons_s>() - 8usize];
    ["Offset of field: QuartzFunctons_s::Create"]
        [::std::mem::offset_of!(QuartzFunctons_s, Create) - 0usize];
    ["Offset of field: QuartzFunctons_s::DevNumber"]
        [::std::mem::offset_of!(QuartzFunctons_s, DevNumber) - 8usize];
    ["Offset of field: QuartzFunctons_s::Kill"]
        [::std::mem::offset_of!(QuartzFunctons_s, Kill) - 16usize];
    ["Offset of field: QuartzFunctons_s::ResetContext"]
        [::std::mem::offset_of!(QuartzFunctons_s, ResetContext) - 24usize];
    ["Offset of field: QuartzFunctons_s::GetWidth"]
        [::std::mem::offset_of!(QuartzFunctons_s, GetWidth) - 32usize];
    ["Offset of field: QuartzFunctons_s::GetHeight"]
        [::std::mem::offset_of!(QuartzFunctons_s, GetHeight) - 40usize];
    ["Offset of field: QuartzFunctons_s::SetSize"]
        [::std::mem::offset_of!(QuartzFunctons_s, SetSize) - 48usize];
    ["Offset of field: QuartzFunctons_s::GetScaledWidth"]
        [::std::mem::offset_of!(QuartzFunctons_s, GetScaledWidth) - 56usize];
    ["Offset of field: QuartzFunctons_s::GetScaledHeight"]
        [::std::mem::offset_of!(QuartzFunctons_s, GetScaledHeight) - 64usize];
    ["Offset of field: QuartzFunctons_s::SetScaledSize"]
        [::std::mem::offset_of!(QuartzFunctons_s, SetScaledSize) - 72usize];
    ["Offset of field: QuartzFunctons_s::GetXScale"]
        [::std::mem::offset_of!(QuartzFunctons_s, GetXScale) - 80usize];
    ["Offset of field: QuartzFunctons_s::GetYScale"]
        [::std::mem::offset_of!(QuartzFunctons_s, GetYScale) - 88usize];
    ["Offset of field: QuartzFunctons_s::SetScale"]
        [::std::mem::offset_of!(QuartzFunctons_s, SetScale) - 96usize];
    ["Offset of field: QuartzFunctons_s::SetTextScale"]
        [::std::mem::offset_of!(QuartzFunctons_s, SetTextScale) - 104usize];
    ["Offset of field: QuartzFunctons_s::GetTextScale"]
        [::std::mem::offset_of!(QuartzFunctons_s, GetTextScale) - 112usize];
    ["Offset of field: QuartzFunctons_s::SetPointSize"]
        [::std::mem::offset_of!(QuartzFunctons_s, SetPointSize) - 120usize];
    ["Offset of field: QuartzFunctons_s::GetPointSize"]
        [::std::mem::offset_of!(QuartzFunctons_s, GetPointSize) - 128usize];
    ["Offset of field: QuartzFunctons_s::GetDirty"]
        [::std::mem::offset_of!(QuartzFunctons_s, GetDirty) - 136usize];
    ["Offset of field: QuartzFunctons_s::SetDirty"]
        [::std::mem::offset_of!(QuartzFunctons_s, SetDirty) - 144usize];
    ["Offset of field: QuartzFunctons_s::ReplayDisplayList"]
        [::std::mem::offset_of!(QuartzFunctons_s, ReplayDisplayList) - 152usize];
    ["Offset of field: QuartzFunctons_s::GetSnapshot"]
        [::std::mem::offset_of!(QuartzFunctons_s, GetSnapshot) - 160usize];
    ["Offset of field: QuartzFunctons_s::RestoreSnapshot"]
        [::std::mem::offset_of!(QuartzFunctons_s, RestoreSnapshot) - 168usize];
    ["Offset of field: QuartzFunctons_s::GetAntialias"]
        [::std::mem::offset_of!(QuartzFunctons_s, GetAntialias) - 176usize];
    ["Offset of field: QuartzFunctons_s::SetAntialias"]
        [::std::mem::offset_of!(QuartzFunctons_s, SetAntialias) - 184usize];
    ["Offset of field: QuartzFunctons_s::GetBackground"]
        [::std::mem::offset_of!(QuartzFunctons_s, GetBackground) - 192usize];
    ["Offset of field: QuartzFunctons_s::Activate"]
        [::std::mem::offset_of!(QuartzFunctons_s, Activate) - 200usize];
    ["Offset of field: QuartzFunctons_s::SetParameter"]
        [::std::mem::offset_of!(QuartzFunctons_s, SetParameter) - 208usize];
    ["Offset of field: QuartzFunctons_s::GetParameter"]
        [::std::mem::offset_of!(QuartzFunctons_s, GetParameter) - 216usize];
};
pub type QuartzFunctions_t = QuartzFunctons_s;
pub type quartz_create_fn_t = ::std::option::Option<
    unsafe extern "C" fn(
        dd: *mut ::std::os::raw::c_void,
        fn_: *mut QuartzFunctions_t,
        par: *mut QuartzParameters_t,
    ) -> QuartzDesc_t,
>;
extern "C" {
    pub fn QuartzDevice_Create(
        dd: *mut ::std::os::raw::c_void,
        def: *mut QuartzBackend_t,
    ) -> QuartzDesc_t;
    pub fn getQuartzFunctions() -> *mut QuartzFunctions_t;
    pub static mut ptr_QuartzBackend: ::std::option::Option<
        unsafe extern "C" fn(
            dd: *mut ::std::os::raw::c_void,
            fn_: *mut QuartzFunctions_t,
            par: *mut QuartzParameters_t,
        ) -> QuartzDesc_t,
    >;
    pub fn Quartz_C(
        par: *mut QuartzParameters_t,
        q_create: quartz_create_fn_t,
        errorCode: *mut ::std::os::raw::c_int,
    ) -> QuartzDesc_t;
}
