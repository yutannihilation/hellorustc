/* automatically generated by rust-bindgen 0.70.1 */

pub const AlgType_NREG: AlgType = 1;
pub const AlgType_OPT: AlgType = 2;
pub type AlgType = u32;
pub const VPos_F: VPos = 9;
pub const VPos_F0: VPos = 12;
pub const VPos_FDIF: VPos = 10;
pub const VPos_G: VPos = 27;
pub const VPos_HC: VPos = 70;
pub type VPos = u32;
pub const IVPos_AI: IVPos = 90;
pub const IVPos_AM: IVPos = 94;
pub const IVPos_ALGSAV: IVPos = 50;
pub const IVPos_COVMAT: IVPos = 25;
pub const IVPos_COVPRT: IVPos = 13;
pub const IVPos_COVREQ: IVPos = 14;
pub const IVPos_DRADPR: IVPos = 100;
pub const IVPos_DTYPE: IVPos = 15;
pub const IVPos_IERR: IVPos = 74;
pub const IVPos_INITH: IVPos = 24;
pub const IVPos_INITS: IVPos = 24;
pub const IVPos_IPIVOT: IVPos = 75;
pub const IVPos_IVNEED: IVPos = 2;
pub const IVPos_LASTIV: IVPos = 42;
pub const IVPos_LASTV: IVPos = 44;
pub const IVPos_LMAT: IVPos = 41;
pub const IVPos_MXFCAL: IVPos = 16;
pub const IVPos_MXITER: IVPos = 17;
pub const IVPos_NEXTV: IVPos = 46;
pub const IVPos_NFCALL: IVPos = 5;
pub const IVPos_NFCOV: IVPos = 51;
pub const IVPos_NFGCAL: IVPos = 6;
pub const IVPos_NGCOV: IVPos = 52;
pub const IVPos_NITER: IVPos = 30;
pub const IVPos_NVDFLT: IVPos = 49;
pub const IVPos_NVSAVE: IVPos = 8;
pub const IVPos_OUTLEV: IVPos = 18;
pub const IVPos_PARPRT: IVPos = 19;
pub const IVPos_PARSAV: IVPos = 48;
pub const IVPos_PERM: IVPos = 57;
pub const IVPos_PRUNIT: IVPos = 20;
pub const IVPos_QRTYP: IVPos = 79;
pub const IVPos_RDREQ: IVPos = 56;
pub const IVPos_RMAT: IVPos = 77;
pub const IVPos_SOLPRT: IVPos = 21;
pub const IVPos_STATPR: IVPos = 22;
pub const IVPos_TOOBIG: IVPos = 1;
pub const IVPos_VNEED: IVPos = 3;
pub const IVPos_VSAVE: IVPos = 59;
pub const IVPos_X0PRT: IVPos = 23;
pub type IVPos = u32;
extern "C" {
    pub fn S_Rf_divset(
        alg: ::std::os::raw::c_int,
        iv: *mut ::std::os::raw::c_int,
        liv: ::std::os::raw::c_int,
        lv: ::std::os::raw::c_int,
        v: *mut f64,
    );
    pub fn S_nlsb_iterate(
        b: *mut f64,
        d: *mut f64,
        dr: *mut f64,
        iv: *mut ::std::os::raw::c_int,
        liv: ::std::os::raw::c_int,
        lv: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        nd: ::std::os::raw::c_int,
        p: ::std::os::raw::c_int,
        r: *mut f64,
        rd: *mut f64,
        v: *mut f64,
        x: *mut f64,
    );
    pub fn S_nlminb_iterate(
        b: *mut f64,
        d: *mut f64,
        fx: f64,
        g: *mut f64,
        h: *mut f64,
        iv: *mut ::std::os::raw::c_int,
        liv: ::std::os::raw::c_int,
        lv: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        v: *mut f64,
        x: *mut f64,
    );
    pub fn S_rcont2(
        nrow: ::std::os::raw::c_int,
        ncol: ::std::os::raw::c_int,
        nrowt: *const ::std::os::raw::c_int,
        ncolt: *const ::std::os::raw::c_int,
        ntotal: ::std::os::raw::c_int,
        fact: *const f64,
        jwork: *mut ::std::os::raw::c_int,
        matrix: *mut ::std::os::raw::c_int,
    );
}
