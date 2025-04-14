#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TokenType {
    /// `{`
    OpeningBrace,
    /// `}`
    ClosingBrace,
    /// `key`: value
    Key,
    /// key`:` value
    Assigner,
    /// key: `value`
    Value,
    /// key: value`,`
    Separator,
}
