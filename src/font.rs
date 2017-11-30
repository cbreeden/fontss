use std::marker::PhantomData;
use std::io::Cursor;
use primitive::Tag;
use decode::StaticSize;
use error::FontParseError;

pub enum FontVersion {
    OpenType,
    TrueType,
}

pub struct Font<'buf> {
    buffer: &'buf [u8],
    version: FontVersion,
    num_tables: usize,
    tables: &'buf [u8],

    // maxp
    number_glyphs: u16,
}

type Codepoint = u32;
type Index = u16;

impl<'buf> Font<'buf> {
    /// Lookup the glyph index for a given codepoint.  This method does not take
    /// Unicode Variation Sequences into account.  In case no corresponding glyph
    /// is found, a `.notdef` (of index 0) is returned.
    pub fn glyph_index(&self, cp: Codepoint) -> Result<Index, FontParseError> {
        unimplemented!()
    }

    pub fn unicode_variation_sequence(
        &self,
        lhs: Codepoint,
        rhs: Codepoint,
    ) -> Result<Index, FontParseError> {
        unimplemented!()
    }
}

pub struct FontTableIter<'buf> {
    buffer: &'buf [u8],
    cursor: &'buf [u8],
}

impl<'buf> Iterator for FontTableIter<'buf> {
    type Item = FontTable<'buf>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor.is_empty() {
            return None;
        }

        unimplemented!()
    }
}

pub struct FontTable<'buf> {
    tag: Tag,
    offset: &'buf [u8],
}

impl<'buf> FontTable<'buf> {
    fn parse(buffer: &'buf [u8], offset: &'buf [u8]) -> Result<Self, FontParseError> {
        unimplemented!()
    }
}

impl<'buf> StaticSize for FontTable<'buf> {
    const SIZE: usize = 16;
}