use crate::{bindings, Note};

pub struct NdbStr<'a> {
    ndb_str: bindings::ndb_str,
    note: &'a Note<'a>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NdbStrVariant<'a> {
    Id(&'a [u8; 32]),
    Str(&'a str),
}

impl bindings::ndb_str {
    pub fn str(&self) -> *const ::std::os::raw::c_char {
        unsafe { self.__bindgen_anon_1.str_ }
    }

    pub fn id(&self) -> *const ::std::os::raw::c_uchar {
        unsafe { self.__bindgen_anon_1.id }
    }
}

impl<'a> NdbStr<'a> {
    pub fn note(&self) -> &'a Note<'a> {
        self.note
    }

    pub(crate) fn new(ndb_str: bindings::ndb_str, note: &'a Note<'a>) -> Self {
        NdbStr { ndb_str, note }
    }

    pub fn len(&self) -> usize {
        if self.ndb_str.flag == (bindings::NDB_PACKED_ID as u8) {
            32
        } else {
            unsafe { libc::strlen(self.ndb_str.str()) }
        }
    }

    pub fn variant(&self) -> NdbStrVariant<'a> {
        if self.ndb_str.flag == (bindings::NDB_PACKED_ID as u8) {
            unsafe { NdbStrVariant::Id(&*(self.ndb_str.id() as *const [u8; 32])) }
        } else {
            let s = unsafe {
                let byte_slice =
                    std::slice::from_raw_parts(self.ndb_str.str() as *const u8, self.len());
                std::str::from_utf8_unchecked(byte_slice)
            };

            NdbStrVariant::Str(s)
        }
    }
}
