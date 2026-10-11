// SPDX-License-Identifier: Apache-2.0
//! Print the crate's normalisation tables per code point for `parity/unicode_tables.py`: a header
//! line with the library's Unicode version, then one line per code point that has a combining class
//! or decomposes, `CODEPOINT<TAB>COMBINING_CLASS<TAB>DECOMPOSITION` (the full canonical
//! decomposition in hex, empty when the code point decomposes to itself). A code point with no line
//! has class 0 and decomposes to itself.

use unicode_normalization::char::{canonical_combining_class, decompose_canonical};

fn main() {
    let (a, b, c) = unicode_normalization::UNICODE_VERSION;
    println!("unicode-normalization tables, Unicode {a}.{b}.{c}");
    for cp in 0u32..=0x0010_FFFF {
        if let Some(ch) = char::from_u32(cp) {
            let mut parts = Vec::new();
            decompose_canonical(ch, |d| parts.push(d));
            let decomposition = if parts == [ch] {
                String::new()
            } else {
                parts
                    .iter()
                    .map(|d| format!("{:X}", u32::from(*d)))
                    .collect::<Vec<_>>()
                    .join(" ")
            };
            let class = canonical_combining_class(ch);
            if class != 0 || !decomposition.is_empty() {
                println!("{cp:X}\t{class}\t{decomposition}");
            }
        }
    }
}
