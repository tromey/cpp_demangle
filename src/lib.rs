//! TODO FITZGEN

#![allow(dead_code, unused_variables)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
// #![deny(warnings)]

// The `error_chain!` macro can recurse deeply.
#![recursion_limit = "1024"]

use std::fmt;

#[macro_use]
extern crate error_chain;

pub mod error;
mod index_str;

use error::{ErrorKind, Result};
use index_str::IndexStr;

#[macro_use]
mod testing;

/// TODO FITZGEN
pub type OwnedSymbol = Symbol<Vec<u8>>;

/// TODO FITZGEN
pub type BorrowedSymbol<'a> = Symbol<&'a [u8]>;

/// TODO FITZGEN
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Symbol<T> {
    raw: T,
    parsed: MangledName,
}

impl<T> Symbol<T>
    where T: AsRef<[u8]>
{
    /// TODO FITZGEN
    pub fn new(raw: T) -> Result<Symbol<T>> {
        let input = IndexStr::new(raw.as_ref());
        let _ = input;
        unimplemented!()
    }
}

/// TODO FITZGEN
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct MangledName(usize, Encoding);

impl MangledName {
    fn parse(input: IndexStr) -> Result<(MangledName, IndexStr)> {
        unimplemented!()
    }
}

/// TODO FITZGEN
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Encoding {
    /// TODO FITZGEN
    Function(Name, BareFunctionType),

    /// TODO FITZGEN
    Data(Name),

    /// TODO FITZGEN
    Special(SpecialName),
}

/// TODO FITZGEN
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Name {
    /// TODO FITZGEN
    Nested(NestedName),

    /// TODO FITZGEN
    Unscoped(UnscopedName),

    /// TODO FITZGEN
    UnscopedTemplate(UnscopedTemplateName, TemplateArgs),

    /// TODO FITZGEN
    Local(LocalName),
}

/// TODO FITZGEN
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum UnscopedName {
    /// TODO FITZGEN
    Unqualified(UnqualifiedName),

    /// TODO FITZGEN
    Std(UnqualifiedName),
}

/// TODO FITZGEN
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum UnscopedTemplateName {
    /// TODO FITZGEN
    Unscoped(UnscopedName),

    /// TODO FITZGEN
    Substitution(Substitution),
}

/// TODO FITZGEN
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum NestedName {
    /// TODO FITZGEN
    Unqualified(CvQualifiers, RefQualifier, Prefix, UnqualifiedName),

    /// TODO FITZGEN
    Template(CvQualifiers, RefQualifier, TemplatePrefix, TemplateArgs),
}

/// TODO FITZGEN
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Prefix {
    /// TODO FITZGEN
    Unqualified(UnqualifiedName, Option<PrefixTail>),
    /// TODO FITZGEN
    Template(TemplatePrefix, TemplateArgs, Option<PrefixTail>),
    /// TODO FITZGEN
    TemplateParam(TemplateParam, Option<PrefixTail>),
}

/// TODO FITZGEN
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Substitution(Option<SeqId>);

/// TODO FITZGEN
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct BareFunctionType(Type);

/// TODO FITZGEN
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct SeqId(usize);

impl SeqId {
    fn parse(input: IndexStr) -> Result<(SeqId, IndexStr)> {
        parse_number(36, false, input).map(|(num, tail)| (SeqId(num as _), tail))
    }
}

/// TODO FITZGEN
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Type;

/// TODO FITZGEN
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct SpecialName;

/// TODO FITZGEN
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct TemplateArgs;

/// TODO FITZGEN
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct LocalName;

/// TODO FITZGEN
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct UnqualifiedName;

/// The <source-name> non-terminal.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct SourceName(Identifier);

impl SourceName {
    fn parse(input: IndexStr) -> Result<(SourceName, IndexStr)> {
        let (source_name_len, input) = try!(parse_number(10, false, input));
        debug_assert!(source_name_len >= 0);
        if source_name_len == 0 {
            return Err(ErrorKind::UnexpectedText.into());
        }

        let (head, tail) = match input.try_split_at(source_name_len as _) {
            Some((head, tail)) => (head, tail),
            None => return Err(ErrorKind::UnexpectedEnd.into()),
        };

        let (identifier, empty) = try!(Identifier::parse(head));
        if !empty.is_empty() {
            return Err(ErrorKind::UnexpectedText.into());
        }

        let source_name = SourceName(identifier);
        Ok((source_name, tail))
    }
}

/// The <identifier> pseudo terminal.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Identifier {
    start: usize,
    end: usize,
}

impl Identifier {
    fn parse(input: IndexStr) -> Result<(Identifier, IndexStr)> {
        if input.len() == 0 {
            return Err(ErrorKind::UnexpectedEnd.into());
        }

        let end = input.as_ref()
            .iter()
            .map(|&c| c as char)
            .take_while(|&c| c == '_' || c.is_digit(36))
            .count();

        if end == 0 {
            return Err(ErrorKind::UnexpectedText.into());
        }

        let tail = input.range_from(end..);

        let identifier = Identifier {
            start: input.index(),
            end: tail.index(),
        };

        Ok((identifier, tail))
    }
}

/// TODO FITZGEN
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct CvQualifiers;

/// TODO FITZGEN
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct RefQualifier;

/// TODO FITZGEN
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct TemplatePrefix;

/// TODO FITZGEN
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct PrefixTail;

/// TODO FITZGEN
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct TemplateParam;

fn parse_number(base: u32, allow_signed: bool, mut input: IndexStr) -> Result<(isize, IndexStr)> {
    if input.is_empty() {
        return Err(ErrorKind::UnexpectedEnd.into());
    }

    let num_is_negative = if allow_signed && input.as_ref()[0] == b'n' {
        input = input.range_from(1..);
        true
    } else {
        false
    };

    let num_numeric = input.as_ref()
        .iter()
        .map(|&c| c as char)
        .take_while(|c| c.is_digit(base) && (c.is_numeric() || c.is_uppercase()))
        .count();
    if num_numeric == 0 {
        return Err(ErrorKind::UnexpectedText.into());
    }

    let (head, tail) = input.split_at(num_numeric);
    let head = head.as_ref();

    if num_numeric > 1 && head[0] == b'0' {
        // "<number>s appearing in mangled names never have leading zeroes,
        // except for the value zero, represented as '0'."
        //
        // There is similar behavior for <seq-id>.
        return Err(ErrorKind::UnexpectedText.into());
    }

    let head = unsafe {
        // Safe because we know we only have valid numeric chars in this
        // slice.
        ::std::str::from_utf8_unchecked(head)
    };

    let mut number = isize::from_str_radix(head, base)
        .expect("We should only have numeric characters");
    if num_is_negative {
        number = -number;
    }

    Ok((number, tail))
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
enum Number {}

impl Number {
    fn parse(input: IndexStr) -> Result<(isize, IndexStr)> {
        parse_number(10, true, input)
    }
}

/// Define a "vocabulary" nonterminal, something like `OperatorName` or
/// `CtorDtorName` that's basically a big list of constant strings.
/// This declares:
///
/// - the enum itself
/// - a `parse` method
/// - a `std::fmt::Display` impl
///
/// See the definition of `CTorDtorName` for an example of its use.
macro_rules! define_vocabulary {
    ( $typename:ident { $($variant:ident ( $mangled:pat, $printable:expr )),* } ) => {

        #[derive(Clone, Debug, Hash, PartialEq, Eq)]
        enum $typename {
            $(
                #[doc=$printable]
                $variant
            ),*
        }

        impl $typename {
            fn parse(input: IndexStr) -> Result<($typename, IndexStr)> {
                let (head, tail) = match input.try_split_at(2) {
                    Some((head, tail)) => (head, tail),
                    None => {
                        return Err(ErrorKind::UnexpectedEnd.into());
                    }
                };
                let name = match head.as_ref() {
                    $(
                        $mangled => $typename::$variant,
                    )*
                    _ => {
                        return Err(ErrorKind::UnexpectedText.into());
                    }
                };
                Ok((name, tail))
            }
        }

        impl fmt::Display for $typename {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str(match *self {
                    $(
                        $typename::$variant => $printable
                    ),*
                })
            }
        }
    }
}

define_vocabulary! {
    OperatorName {
        // enum variant(mangled form, printable description)
        New              (b"nw",  "`new`"),
        NewArray         (b"na",  "`new[]`"),
        Delete           (b"dl",  "`delete`"),
        DeleteArray      (b"da",  "`delete[]`"),
        UnaryPlus        (b"ps",  "`+` (unary)"),
        Neg              (b"ng",  "`-` (unary)"),
        AddressOf        (b"ad",  "`&` (unary)"),
        Deref            (b"de",  "`*` (unary)"),
        BitNot           (b"co",  "`~`"),
        Add              (b"pl",  "`+`"),
        Sub              (b"mi",  "`-`"),
        Mul              (b"ml",  "`*`"),
        Div              (b"dv",  "`/`"),
        Rem              (b"rm",  "`%`"),
        BitAnd           (b"an",  "`&`"),
        BitOr            (b"or",  "`|`"),
        BitXor           (b"eo",  "`^`"),
        Assign           (b"aS",  "`=`"),
        AddAssign        (b"pL",  "`+=`"),
        SubAssign        (b"mI",  "`-=`"),
        MulAssign        (b"mL",  "`*=`"),
        DivAssign        (b"dV",  "`/=`"),
        RemAssign        (b"rM",  "`%=`"),
        BitAndAssign     (b"aN",  "`&=`"),
        BitOrAssign      (b"oR",  "`|=`"),
        BitXorAssign     (b"eO",  "`^=`"),
        Shl              (b"ls",  "`<<`"),
        Shr              (b"rs",  "`>>`"),
        ShlAssign        (b"lS",  "`<<=`"),
        ShrAssign        (b"rS",  "`>>=`"),
        Eq               (b"eq",  "`==`"),
        Ne               (b"ne",  "`!=`"),
        Less             (b"lt",  "`<`"),
        Greater          (b"gt",  "`>`"),
        LessEq           (b"le",  "`<=`"),
        GreaterEq        (b"ge",  "`>=`"),
        Not              (b"nt",  "`!`"),
        LogicalAnd       (b"aa",  "`&&`"),
        LogicalOr        (b"oo",  "`||`"),
        PostInc          (b"pp",  "`++` (postfix in <expression> context)"),
        PostDec          (b"mm",  "`--` (postfix in <expression> context)"),
        Comma            (b"cm",  "`,`"),
        DerefMemberPtr   (b"pm",  "`->*`"),
        DerefMember      (b"pt",  "`->`"),
        Call             (b"cl",  "`()`"),
        Index            (b"ix",  "`[]`"),
        Question         (b"qu",  "`?:`")
    }
}

define_vocabulary! {
    CtorDtorName {
        CompleteConstructor             (b"C1", "complete object constructor"),
        BaseConstructor                 (b"C2", "base object constructor"),
        CompleteAllocatingConstructor   (b"C3", "complete object allocating constructor"),
        DeletingDestructor              (b"D0", "deleting destructor"),
        CompleteDestructor              (b"D1", "complete object destructor"),
        BaseDestructor                  (b"D2", "base object destructor")
    }
}

#[cfg(test)]
mod tests {
    use super::{CtorDtorName, Identifier, Number, OperatorName, SeqId, SourceName};
    use error::ErrorKind;

    #[test]
    fn parse_identifier() {
        assert_parse!(Identifier: b"1abc" =>
                      Ok(Identifier { start: 0, end: 4 }, b""));
        assert_parse!(Identifier: b"_Az1..." =>
                      Ok(Identifier { start: 0, end: 4 }, b"..."));
        assert_parse!(Identifier: b"..." => Err(ErrorKind::UnexpectedText));
        assert_parse!(Identifier: b"" => Err(ErrorKind::UnexpectedEnd));
    }

    #[test]
    fn parse_source_name() {
        assert_parse!(SourceName: b"1abc" =>
                      Ok(SourceName(Identifier { start: 1, end: 2 }), b"bc"));
        assert_parse!(SourceName: b"10abcdefghijklm" =>
                      Ok(SourceName(Identifier { start: 2, end: 12 }), b"klm"));
        assert_parse!(SourceName: b"0abc" => Err(ErrorKind::UnexpectedText));
        assert_parse!(SourceName: b"n1abc" => Err(ErrorKind::UnexpectedText));
        assert_parse!(SourceName: b"10abcdef" => Err(ErrorKind::UnexpectedEnd));
        assert_parse!(SourceName: b"" => Err(ErrorKind::UnexpectedEnd));
    }

    #[test]
    fn parse_number() {
        assert_parse!(Number: b"n2n3" => Ok(-2, b"n3"));
        assert_parse!(Number: b"12345abcdef" => Ok(12345, b"abcdef"));
        assert_parse!(Number: b"0abcdef" => Ok(0, b"abcdef"));
        assert_parse!(Number: b"42" => Ok(42, b""));
        assert_parse!(Number: b"001" => Err(ErrorKind::UnexpectedText));
        assert_parse!(Number: b"wutang" => Err(ErrorKind::UnexpectedText));
        assert_parse!(Number: b"" => Err(ErrorKind::UnexpectedEnd));
    }

    #[test]
    fn parse_seq_id() {
        assert_parse!(SeqId: b"1_" => Ok(SeqId(1), b"_"));
        assert_parse!(SeqId: b"42" => Ok(SeqId(146), b""));
        assert_parse!(SeqId: b"ABCabc" => Ok(SeqId(13368), b"abc"));
        assert_parse!(SeqId: b"abc" => Err(ErrorKind::UnexpectedText));
        assert_parse!(SeqId: b"001" => Err(ErrorKind::UnexpectedText));
        assert_parse!(SeqId: b"wutang" => Err(ErrorKind::UnexpectedText));
        assert_parse!(SeqId: b"" => Err(ErrorKind::UnexpectedEnd));
    }

    #[test]
    fn parse_ctor_dtor_name() {
        assert_parse!(CtorDtorName: b"D0" => Ok(CtorDtorName::DeletingDestructor, b""));
        assert_parse!(CtorDtorName: b"C101" => Ok(CtorDtorName::CompleteConstructor, b"01"));
        assert_parse!(CtorDtorName: b"gayagaya" => Err(ErrorKind::UnexpectedText));
        assert_parse!(CtorDtorName: b"C" => Err(ErrorKind::UnexpectedEnd));
        assert_parse!(CtorDtorName: b"" => Err(ErrorKind::UnexpectedEnd));
    }

    #[test]
    fn parse_operator_name() {
        assert_parse!(OperatorName: b"qu" => Ok(OperatorName::Question, b""));
        assert_parse!(OperatorName: b"quokka" => Ok(OperatorName::Question, b"okka"));
        assert_parse!(OperatorName: b"bu-buuu" => Err(ErrorKind::UnexpectedText));
        assert_parse!(OperatorName: b"b" => Err(ErrorKind::UnexpectedEnd));
        assert_parse!(OperatorName: b"" => Err(ErrorKind::UnexpectedEnd));
    }
}
