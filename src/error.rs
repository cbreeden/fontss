use failure::Error;

#[derive(Debug, Fail)]
pub enum FontParseError {
    #[fail(display = "unexpected end of file")]
    UnexpectedEof,
}
