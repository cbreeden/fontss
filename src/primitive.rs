use byteorder::{ReadBytesExt, BE};
use std::fmt;

pub trait Primitive {
    const SIZE: usize;
    fn parse(buffer: &[u8]) -> Result<Self>;
}

// Unsigned 24-bit integer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct U24(u32);

// Signed 16.16 fixed-point number
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Fixed(i32);

// Signed 16-bit integer describing quantity in font design units.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FWord(i16);

// Unsigned 16-bit integer describing quantity in font design units.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct UFWord(u16);

// Signed 2.14 fixed-point number
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct F2Dot14(i16);

// Date represented in number of seconds since 12:00 midnight, January 1, 1904.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LongDateTime(u64);

// Short offset to a table, same as uint16, NULL offset = 0x0000
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Offset16(u16);

// Long offset to a table, same as uint32, NULL offset = 0x00000000
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Offset32(u32);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tag(pub [u8; 4]);

impl Tag {
    fn as_u32(&self) -> u32 {
        (self.0[0] as u32) << 24 | (self.0[1] as u32) << 16 | (self.0[2] as u32) << 8 |
            (self.0[3] as u32)
    }
}

impl fmt::Debug for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::str;
        // Print the ASCII name if the contents are valid ASCII.
        // Otherwise, print hexidecimal digits.
        if self.0.iter().all(|&c| c >= 32 && c <= 128) {
            let s = str::from_utf8(&self.0[..]).unwrap();
            f.debug_tuple("Tag").field(&s).finish()
        } else {
            write!(f, "Tag(0x{:08X})", self.as_u32())
        }
    }
}

macro_rules! const_tag {
    ($($ident:ident => $name:expr),*) => (
        $(
            pub const $ident: Tag = Tag($name);
        )*
    )
}

const_tag!(
    // Font Header
    TAG_TTC  => *b"ttcf",

    // Required Tables
    TAG_CMAP => *b"cmap",
    TAG_HEAD => *b"head",
    TAG_HHEA => *b"hhea",
    TAG_HMTX => *b"hmtx",
    TAG_MAXP => *b"maxp",
    TAG_NAME => *b"name",
    TAG_OS2  => *b"OS/2",
    TAG_POST => *b"post",

    // TrueType tables
    TAG_CVT  => *b"cvt ",
    TAG_FPGM => *b"fpgm",
    TAG_GLYF => *b"glyf",
    TAG_LOCA => *b"loca",
    TAG_PREP => *b"prep",
    TAG_GASP => *b"gasp",

    // CFF tables
    TAG_CFF  => *b"CFF ",
    TAG_CFF2 => *b"CFF2",
    TAG_VORG => *b"VORG",

    // SVG outline tables
    TAG_SVG  => *b"SVG ",

    // Bitmap glyphs tables
    TAG_EBDT => *b"EBDT",
    TAG_EBLC => *b"EBLC",
    TAG_EBSC => *b"EBSC",
    TAG_CBDT => *b"cbdt",
    TAG_CBLC => *b"cblc",
    TAG_SBIX => *b"sbix",

    // Advanced Typographic Tables
    TAG_BASE => *b"BASE",
    TAG_GDEF => *b"GDEF",
    TAG_GPOS => *b"GPOS",
    TAG_GSUB => *b"GSUB",
    TAG_JSTF => *b"JSTF",
    TAG_MATH => *b"MATH",

    // OpenType Variations Tables
    TAG_AVAR => *b"avar",
    TAG_CVAR => *b"cvar",
    TAG_FVAR => *b"fvar",
    TAG_GVAR => *b"gvar",
    TAG_HVAR => *b"HVAR",
    TAG_MVAR => *b"MVAR",
    TAG_STAT => *b"STAT",
    TAG_VVAR => *b"VVAR",

    // Color Fonts Tables
    TAG_COLR => *b"COLR",
    TAG_CPAL => *b"CPAL",

    // Other OpenType Tables
    TAG_DSIG => *b"DSIG",
    TAG_HDMX => *b"hdmx",
    TAG_KERN => *b"kern",
    TAG_LTSH => *b"LTSH",
    TAG_MERG => *b"MERG",
    TAG_META => *b"meta",
    TAG_PCLT => *b"PCLT",
    TAG_VDMX => *b"VDMX",
    TAG_VHEA => *b"vhea",
    TAG_VMTX => *b"vmtx"
);