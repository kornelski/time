//! Parsing for various types.

pub(crate) mod combinator;
mod component;
pub(crate) mod parsable;
mod parsed;

pub use parsed::Parsed;

/// An item that has been parsed. Represented as a `(remaining, value)` pair.
#[derive(Debug, Clone)]
pub(crate) struct ParsedItem<'a, T>(pub(crate) &'a str, pub(crate) T);

impl<'a, T> ParsedItem<'a, T> {
    /// Map the value to a new value, preserving the remaining input.
    pub(crate) fn map<U>(self, f: impl Fn(T) -> U) -> ParsedItem<'a, U> {
        ParsedItem(self.0, f(self.1))
    }

    /// Map the value to a new, optional value, preserving the remaining input.
    pub(crate) fn flat_map<U>(self, f: impl Fn(T) -> Option<U>) -> Option<ParsedItem<'a, U>> {
        Some(ParsedItem(self.0, f(self.1)?))
    }

    /// Consume the stored value, assigning it to the provided target. The remaining input is
    /// returned.
    pub(crate) fn assign_value_to(self, target: &mut Option<T>) -> &'a str {
        *target = Some(self.1);
        self.0
    }

    /// Consume the stored value, assigning the result of the given function to the provided target.
    /// The remaining input is returned.
    pub(crate) fn assign_value_to_with<U>(
        self,
        target: &mut Option<U>,
        f: impl Fn(T) -> U,
    ) -> &'a str {
        *target = Some(f(self.1));
        self.0
    }
}

impl<'a> ParsedItem<'a, ()> {
    /// Discard the unit value, returning the remaining input.
    pub(crate) const fn unwrap(self) -> &'a str {
        self.0
    }
}
