use proc_macro2::{TokenStream, TokenTree, Group, Ident, Spacing};
use syn::{LitInt, parse::{Result, Error}};

pub enum Mode {
    Sv32,
    Sv39,
    Sv48,
}

pub struct EntryConfig {
    pte: [usize; 512],
}

impl EntryConfig {
    pub fn new() -> Self {
        EntryConfig {
            pte: [0; 512]
        }
    }
}

pub fn parse(input: TokenStream, mode: Mode) -> Result<EntryConfig> {
    let mut should_next_be_group = true;
    let mut entry_config = EntryConfig::new();
    for tree in input {
        match (tree, should_next_be_group) {
            (TokenTree::Group(group), true) => {
                let (pte_index, pte_value) = parse_group(group)?;
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
fn parse_group(group: Group) -> Result<(usize, usize)> {
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
    let mut flags = Flags::new();
    for tree in group.stream() {
        match (tree, should_be_next) {
            (TokenTree::Literal(literal), State::VaLiteral) => {
                let vaddr = parse_virt_addr(literal)?;
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
                flags = parse_flags(ident)?;
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
    (0, 0)
}

fn parse_virt_addr(literal: Literal) -> Result<usize> {
    println!("{}", literal);
    let int: LitInt = syn::parse2(literal)?;
    let vaddr: usize = int.base10_parse();
    println!("{:x}", vaddr);
    Ok(vaddr)
}

fn parse_flags(ident: Ident) -> Result<Flags> {
    let flags = match ident.to_string() {
        "r"   => Flags::READABLE,
        "rw"  => Flags::READABLE | Flags::WRITABLE,
        "x"   => Flags::EXECUTABLE,
        "rx"  => Flags::READABLE | Flags::EXECUTABLE,
        "rwx" => Flags::READABLE | Flags::WRITABLE | Flags::EXECUTABLE,
        _ => return Error::new(ident.span(), 
            "expected one of `r`, `rw`, `x`, `rx` or `rwx`"
        )
    };
    let flags = flags | Flags::VALID;
    Ok(flags)
}

bitflags! {
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
