use proc_macro2::{TokenStream, TokenTree, Group, Ident, Spacing, Literal};
use syn::{LitInt, parse::{Result, Error}};
use std::ops::Index;

#[derive(Clone, Copy)]
pub enum Mode {
    Sv32,
    Sv39,
    Sv48,
}

#[derive(Debug)]
pub struct EntryConfig {
    pte: Vec<usize>,
}

impl EntryConfig {
    pub fn new() -> Self {
        EntryConfig {
            pte: vec![0; 512]
        }
    }
}

impl Index<usize> for EntryConfig {
    type Output = usize;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.pte[idx]
    }
}

pub fn parse(input: TokenStream, mode: Mode) -> Result<EntryConfig> {
    let mut should_next_be_group = true;
    let mut entry_config = EntryConfig::new();
    for tree in input {
        match (tree, should_next_be_group) {
            (TokenTree::Group(group), true) => {
                let (pte_index, pte_value) = parse_group(group, mode)?;
                entry_config.pte[pte_index] = pte_value;
                should_next_be_group = false;
            },
            (TokenTree::Punct(punct), false) => {
                // does not check spacing
                if punct.as_char() != ';' {
                    return Err(Error::new(punct.span(), 
                        "expected `;`"
                    ))
                }
                should_next_be_group = true;
            },
            (tree @ _, true) => return Err(
                Error::new(tree.span(), 
                    "expected a group `(virt_addr => phys_addr, flags)`"
                )
            ),
            (tree @ _, false) => return Err(
                Error::new(tree.span(), 
                    "expected `;`"
                )
            ),
        }
    }
    Ok(entry_config)
}

// returns a (pte index, page table entry value)
fn parse_group(group: Group, mode: Mode) -> Result<(usize, usize)> {
    // does not check Group::delimiter
    #[derive(Copy, Clone)]
    enum State {
        VaLiteral,
        PunctEq,
        PunctGt,
        PaLiteral,
        PunctComma,
        ConfigIdent,
        None
    }
    let mut should_be_next = State::VaLiteral;
    let mut flags = None;
    let mut vaddr = None;
    let mut paddr = None;
    for tree in group.stream() {
        match (tree, should_be_next) {
            (TokenTree::Literal(literal), State::VaLiteral) => {
                vaddr = Some(parse_virt_addr(literal, mode)?);
                should_be_next = State::PunctEq;
            },
            (TokenTree::Punct(punct), State::PunctEq) => {
                if punct.as_char() != '=' || punct.spacing() != Spacing::Joint {
                    return Err(Error::new(punct.span(), 
                        "expected `=>`"
                    ))
                }
                should_be_next = State::PunctGt;
            },
            (TokenTree::Punct(punct), State::PunctGt) => {
                if punct.as_char() != '>' {
                    return Err(Error::new(punct.span(), 
                        "expected `=>`"
                    ))
                }
                should_be_next = State::PaLiteral;
            },
            (TokenTree::Literal(literal), State::PaLiteral) => {
                paddr = Some(parse_phys_addr(literal)?);
                should_be_next = State::PunctComma;
            },
            (TokenTree::Punct(punct), State::PunctComma) => {
                if punct.as_char() != ',' {
                    return Err(Error::new(punct.span(), 
                        "expected `,`"
                    ))
                }
                should_be_next = State::ConfigIdent;
            },
            (TokenTree::Ident(ident), State::ConfigIdent) => {
                flags = Some(parse_flags(ident)?);
                should_be_next = State::None;
            },
            (tree @ _, State::VaLiteral) => return Err(
                Error::new(tree.span(), 
                    "expected literal for virtual address"
                )
            ),
            (tree @ _, State::PunctEq) => return Err(
                Error::new(tree.span(), 
                    "expected `=>`"
                )
            ),
            (tree @ _, State::PunctGt) => return Err(
                Error::new(tree.span(), 
                    "expected `=>`"
                )
            ),
            (tree @ _, State::PaLiteral) => return Err(
                Error::new(tree.span(), 
                    "expected literal for physical address"
                )
            ),
            (tree @ _, State::PunctComma) => return Err(
                Error::new(tree.span(), 
                    "expected `,`"
                )
            ),
            (tree @ _, State::ConfigIdent) => return Err(
                Error::new(tree.span(), 
                    "expected one of `r`, `rw`, `x`, `rx` or `rwx`"
                )
            ),
            (tree @ _, State::None) => return Err(
                Error::new(tree.span(), 
                    "expected end of group"
                )
            ),
        }
    }
    // pte_index = vpn[2]
    if let (Some(vaddr), Some(paddr), Some(flags)) = (vaddr, paddr, flags) {
        // todo: use ppn and vpn instead of paddr and vaddr
        let pte_index = match mode {
            Mode::Sv32 => (vaddr >> 22) & 0x1FF,
            Mode::Sv39 => (vaddr >> 30) & 0x1FF,
            Mode::Sv48 => (vaddr >> 39) & 0x1FF,
        };
        let pte_value = match mode {
            Mode::Sv32 => (paddr >> 2) | flags.bits() as usize,
            Mode::Sv39 => (paddr >> 2) | flags.bits() as usize,
            Mode::Sv48 => (paddr >> 2) | flags.bits() as usize,
        };
        Ok((pte_index, pte_value))
    } else {
        Err(Error::new(group.span(), "bug!"))
    }
}

fn parse_virt_addr(literal: Literal, mode: Mode) -> Result<usize> {
    let mut stream = TokenStream::new();
    let span = literal.span();
    stream.extend(vec![TokenTree::Literal(literal)]);
    let int: LitInt = syn::parse2(stream)?;
    let vaddr: usize = int.base10_parse()?;
    match mode {
        Mode::Sv32 => {
            if !is_lower_bits_zero(vaddr, 22) {
                return Err(Error::new(span, 
                    "expected Sv32 virtual address with bits 0..=21 zeroed"
                ))
            }
        },
        Mode::Sv39 => {
            if !is_lower_bits_zero(vaddr, 30) {
                return Err(Error::new(span, 
                    "expected Sv39 virtual address with bits 0..=29 zeroed"
                ))
            }
            if !is_sign_extend_ok(vaddr, 38) {
                return Err(Error::new(span, 
                    "expected Sv39 sign extended virtual address i.e. bit 39..=64 should be all equal to bit 38"
                ))
            }
        },
        Mode::Sv48 => {
            if !is_lower_bits_zero(vaddr, 39) {
                return Err(Error::new(span, 
                    "expected Sv48 virtual address with bits 0..=38 zeroed"
                ))
            }
            if !is_sign_extend_ok(vaddr, 47) {
                return Err(Error::new(span, 
                    "expected Sv48 sign extended virtual address i.e. bit 48..=64 should be all equal to bit 47"
                ))
            }
        },
    }
    Ok(vaddr)
}

fn parse_phys_addr(literal: Literal) -> Result<usize> {
    let mut stream = TokenStream::new();
    let span = literal.span();
    stream.extend(vec![TokenTree::Literal(literal)]);
    let int: LitInt = syn::parse2(stream)?;
    let paddr: usize = int.base10_parse()?;
    if !is_lower_bits_zero(paddr, 12) {
        return Err(Error::new(span, 
            "expected physical address with bits 0..=11 zeroed"
        ))
    }
    // todo: in Sv32, the physical address space is over 32 bits; it's actually 34 bits
    // should be handled properly. e.g. support oversized integer or other solutions
    Ok(paddr)
}

// is 0..cnt bit of `int` all zero?
#[inline]
fn is_lower_bits_zero(int: usize, cnt: usize) -> bool {
    int & ((1 << cnt) - 1) == 0
}

#[inline]
fn is_sign_extend_ok(int: usize, highest_bit_idx: usize) -> bool {
    let sign = int & (1 << highest_bit_idx) != 0;
    let hi_mask = !((1 << (highest_bit_idx + 1)) - 1);
    if sign {
        int & hi_mask == hi_mask
    } else {
        int & hi_mask == 0
    }
}

fn parse_flags(ident: Ident) -> Result<Flags> {
    let flags = match ident.to_string().as_str() {
        "r"   => Flags::READABLE,
        "rw"  => Flags::READABLE | Flags::WRITABLE,
        "x"   => Flags::EXECUTABLE,
        "rx"  => Flags::READABLE | Flags::EXECUTABLE,
        "rwx" => Flags::READABLE | Flags::WRITABLE | Flags::EXECUTABLE,
        _ => return Err(Error::new(ident.span(), 
            "expected one of `r`, `rw`, `x`, `rx` or `rwx`"
        ))
    };
    let flags = flags | Flags::VALID;
    Ok(flags)
}

bitflags::bitflags! {
    #[derive(Default)]
    pub struct Flags: u8 {
        const VALID =       1 << 0;
        const READABLE =    1 << 1;
        const WRITABLE =    1 << 2;
        const EXECUTABLE =  1 << 3;
        const USER =        1 << 4;
        const GLOBAL =      1 << 5;
        const ACCESSED =    1 << 6;
        const DIRTY =       1 << 7;
    }
}

/*
TokenStream [
    Group {
        delimiter: Parenthesis,
        stream: TokenStream [
            Literal {
                kind: Integer,
                symbol: "0xffffffff_80000000",
                suffix: None,
                span: #0 bytes(366..385),
            },
            Punct {
                ch: '=',
                spacing: Joint,
                span: #0 bytes(386..388),
            },
            Punct {
                ch: '>',
                spacing: Alone,
                span: #0 bytes(386..388),
            },
            Literal {
                kind: Integer,
                symbol: "0x00000000_80000000",
                suffix: None,
                span: #0 bytes(389..408),
            },
            Punct {
                ch: ',',
                spacing: Alone,
                span: #0 bytes(408..409),
            },
            Ident {
                ident: "rwx",
                span: #0 bytes(410..413),
            },
        ],
        span: #0 bytes(365..414),
    },
    Punct {
        ch: ';',
        spacing: Alone,
        span: #0 bytes(414..415),
    },
    Group {
        delimiter: Parenthesis,
        stream: TokenStream [
            Literal {
                kind: Integer,
                symbol: "0xffffffff_00000000",
                suffix: None,
                span: #0 bytes(421..440),
            },
            Punct {
                ch: '=',
                spacing: Joint,
                span: #0 bytes(441..443),
            },
            Punct {
                ch: '>',
                spacing: Alone,
                span: #0 bytes(441..443),
            },
            Literal {
                kind: Integer,
                symbol: "0x00000000_00000000",
                suffix: None,
                span: #0 bytes(444..463),
            },
            Punct {
                ch: ',',
                spacing: Alone,
                span: #0 bytes(463..464),
            },
            Ident {
                ident: "rwx",
                span: #0 bytes(465..468),
            },
        ],
        span: #0 bytes(420..469),
    },
    Punct {
        ch: ';',
        spacing: Alone,
        span: #0 bytes(469..470),
    },
    Group {
        delimiter: Parenthesis,
        stream: TokenStream [
            Literal {
                kind: Integer,
                symbol: "0x00000000_80000000",
                suffix: None,
                span: #0 bytes(476..495),
            },
            Punct {
                ch: '=',
                spacing: Joint,
                span: #0 bytes(496..498),
            },
            Punct {
                ch: '>',
                spacing: Alone,
                span: #0 bytes(496..498),
            },
            Literal {
                kind: Integer,
                symbol: "0x00000000_80000000",
                suffix: None,
                span: #0 bytes(499..518),
            },
            Punct {
                ch: ',',
                spacing: Alone,
                span: #0 bytes(518..519),
            },
            Ident {
                ident: "rwx",
                span: #0 bytes(520..523),
            },
        ],
        span: #0 bytes(475..524),
    },
    Punct {
        ch: ';',
        spacing: Alone,
        span: #0 bytes(524..525),
    },
]

*/
