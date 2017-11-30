#![allow(warnings)]

extern crate failure;
extern crate byteorder;

#[macro_use]
extern crate failure_derive;

mod error;
mod decode;
mod primitive;
mod font;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
