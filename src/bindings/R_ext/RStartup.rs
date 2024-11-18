/* automatically generated by rust-bindgen 0.70.1 */

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}
pub const RSTART_VERSION: u32 = 1;
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SA_TYPE {
    SA_NORESTORE = 0,
    SA_RESTORE = 1,
    SA_DEFAULT = 2,
    SA_NOSAVE = 3,
    SA_SAVE = 4,
    SA_SAVEASK = 5,
    SA_SUICIDE = 6,
}
#[repr(C)]
pub struct structRstart {
    pub R_Quiet: Rboolean,
    pub R_NoEcho: Rboolean,
    pub R_Interactive: Rboolean,
    pub R_Verbose: Rboolean,
    pub LoadSiteFile: Rboolean,
    pub LoadInitFile: Rboolean,
    pub DebugInitFile: Rboolean,
    pub RestoreAction: SA_TYPE,
    pub SaveAction: SA_TYPE,
    pub vsize: usize,
    pub nsize: usize,
    pub max_vsize: usize,
    pub max_nsize: usize,
    pub ppsize: usize,
    pub _bitfield_align_1: [u16; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    pub nconnections: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of structRstart"][::std::mem::size_of::<structRstart>() - 88usize];
    ["Alignment of structRstart"][::std::mem::align_of::<structRstart>() - 8usize];
    ["Offset of field: structRstart::R_Quiet"]
        [::std::mem::offset_of!(structRstart, R_Quiet) - 0usize];
    ["Offset of field: structRstart::R_NoEcho"]
        [::std::mem::offset_of!(structRstart, R_NoEcho) - 4usize];
    ["Offset of field: structRstart::R_Interactive"]
        [::std::mem::offset_of!(structRstart, R_Interactive) - 8usize];
    ["Offset of field: structRstart::R_Verbose"]
        [::std::mem::offset_of!(structRstart, R_Verbose) - 12usize];
    ["Offset of field: structRstart::LoadSiteFile"]
        [::std::mem::offset_of!(structRstart, LoadSiteFile) - 16usize];
    ["Offset of field: structRstart::LoadInitFile"]
        [::std::mem::offset_of!(structRstart, LoadInitFile) - 20usize];
    ["Offset of field: structRstart::DebugInitFile"]
        [::std::mem::offset_of!(structRstart, DebugInitFile) - 24usize];
    ["Offset of field: structRstart::RestoreAction"]
        [::std::mem::offset_of!(structRstart, RestoreAction) - 28usize];
    ["Offset of field: structRstart::SaveAction"]
        [::std::mem::offset_of!(structRstart, SaveAction) - 32usize];
    ["Offset of field: structRstart::vsize"][::std::mem::offset_of!(structRstart, vsize) - 40usize];
    ["Offset of field: structRstart::nsize"][::std::mem::offset_of!(structRstart, nsize) - 48usize];
    ["Offset of field: structRstart::max_vsize"]
        [::std::mem::offset_of!(structRstart, max_vsize) - 56usize];
    ["Offset of field: structRstart::max_nsize"]
        [::std::mem::offset_of!(structRstart, max_nsize) - 64usize];
    ["Offset of field: structRstart::ppsize"]
        [::std::mem::offset_of!(structRstart, ppsize) - 72usize];
    ["Offset of field: structRstart::nconnections"]
        [::std::mem::offset_of!(structRstart, nconnections) - 84usize];
};
impl structRstart {
    #[inline]
    pub fn NoRenviron(&self) -> Rboolean {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 16u8) as u32) }
    }
    #[inline]
    pub fn set_NoRenviron(&mut self, val: Rboolean) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 16u8, val as u64)
        }
    }
    #[inline]
    pub fn RstartVersion(&self) -> ::std::os::raw::c_int {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(16usize, 16u8) as u32) }
    }
    #[inline]
    pub fn set_RstartVersion(&mut self, val: ::std::os::raw::c_int) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(16usize, 16u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        NoRenviron: Rboolean,
        RstartVersion: ::std::os::raw::c_int,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 16u8, {
            let NoRenviron: u32 = unsafe { ::std::mem::transmute(NoRenviron) };
            NoRenviron as u64
        });
        __bindgen_bitfield_unit.set(16usize, 16u8, {
            let RstartVersion: u32 = unsafe { ::std::mem::transmute(RstartVersion) };
            RstartVersion as u64
        });
        __bindgen_bitfield_unit
    }
}
pub type Rstart = *mut structRstart;
extern "C" {
    pub fn R_DefParams(arg1: Rstart);
    pub fn R_DefParamsEx(arg1: Rstart, arg2: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn R_SetParams(arg1: Rstart);
    pub fn R_DefCallbacks(arg1: Rstart, arg2: ::std::os::raw::c_int);
    pub fn R_SetWin32(arg1: Rstart);
    pub fn R_SizeFromEnv(arg1: Rstart);
    pub fn R_common_command_line(
        arg1: *mut ::std::os::raw::c_int,
        arg2: *mut *mut ::std::os::raw::c_char,
        arg3: Rstart,
    );
    pub fn R_set_command_line_arguments(
        argc: ::std::os::raw::c_int,
        argv: *mut *mut ::std::os::raw::c_char,
    );
    pub fn setup_Rmainloop();
}
