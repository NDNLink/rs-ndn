#![cfg_attr(all(feature = "cargo-clippy", feature = "pedantic"), warn(clippy_pedantic))]
extern crate bytes;

pub mod error;
pub mod tlv;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}