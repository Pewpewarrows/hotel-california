#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![warn(private_doc_tests)]

//! TODO

#[macro_use]
extern crate error_chain;

// TODO: mermaid state diagrams
// stateDiagram
//      [*] --> Idle

mod errors {
    error_chain!{}
}
use errors::*;

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
