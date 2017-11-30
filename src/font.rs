use std::marker::PhantomData;
use std::io::Cursor;
use primitive::{Tag, Offset32};
use primitive::Primitive;
use decode::StaticSize;
use error::FontParseError;

pub enum FontVersion {
    OpenType,
    TrueType,
}

impl FontVersion {
    pub fn parse(buf: &[u8]) -> Result<(FontVersion, &[u8]), FontParseError> {
        let (ver, buf) = u32::parse(buf)?;

        let ver = match ver {
            0x4F54544F /* OTTO */ => FontVersion::OpenType,
            0x0010000 => FontVersion::TrueType,
            _ => return Err(FontParseError::UnexpectedEof),
        };

        Ok((ver, buf))
    }
}

pub struct Font<'buf> {
    buffer: &'buf [u8],
    version: FontVersion,
    //num_tables: usize,
    tables: &'buf [u8],

    // maxp
    //number_glyphs: u16,
}

type Codepoint = u32;
type Index = u16;

impl<'buf> Font<'buf> {
    pub fn parse(buffer: &'buf [u8], window: &'buf [u8]) -> Result<Font<'buf>, FontParseError> {
        if window.len() < 12 {
            return Err(FontParseError::UnexpectedEof);
        }

        let (version, window) = FontVersion::parse(window)?;
        let (num_tables, window) = u16::parse(window)?;

        // skip the next 3 entries, since we will use a linear search
        let window = &window[6..];
        let size = num_tables as usize * 16;
        if window.len() < size {
            return Err(FontParseError::UnexpectedEof);
        }

        Ok(Font {
            buffer: buffer,
            version: version,
            tables: &window[..size],
        })
    }

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

    pub fn table_iter(&self) -> FontTableIter {
        FontTableIter {
            buffer: self.buffer,
            cursor: self.tables,
        }
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

        // The only possible way for reading a table
        // to fail, is an unexpected EOF.  We check
        // for this while constructing the iterator.
        let table = FontTable::parse(self.buffer, self.cursor)
            .expect("you were wrong about fonttable iter, dumbass");

        self.cursor = &self.cursor[16..];
        Some(table)
    }
}

pub struct FontTable<'buf> {
    tag: Tag,
    offset: &'buf [u8],
}

impl<'buf> FontTable<'buf> {
    fn parse(buffer: &'buf [u8], window: &'buf [u8]) -> Result<Self, FontParseError> {
        if window.len() < 16 {
            return Err(FontParseError::UnexpectedEof);
        }

        let (tag, buf) = Tag::parse(window)?;
        let (offset, _) = Offset32::parse(&buf[4..])?;
        let offset = offset.as_usize();

        // The offset is relative the the beginning of the font file.
        if offset > buffer.len() {
            return Err(FontParseError::UnexpectedEof);
        }

        let offset = &buffer[offset..];

        Ok(FontTable {
            tag: tag,
            offset: offset
        })
    }
}

impl<'buf> StaticSize for FontTable<'buf> {
    const SIZE: usize = 16;
}

#[test]
fn read_font() {
    use std::fs::File;
    use std::io::BufReader;
    use std::io::prelude::*;

    let file = File::open("test/Roboto-Regular.ttf").unwrap();
    let mut buf = BufReader::new(file);
    let mut contents = Vec::new();
    buf.read_to_end(&mut contents).unwrap();

    let font = Font::parse(&contents, &contents).expect("fialed to read font");

    for table in font.table_iter() {
        println!("Tag: {:?}, Offset: {}", table.tag, table.offset.as_ptr() as usize);
    }
    panic!()
}