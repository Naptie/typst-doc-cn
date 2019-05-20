//! Dynamic typesetting functions.

use std::any::Any;
use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};

use crate::syntax::FuncHeader;
use crate::parsing::{ParseContext, ParseResult};
use crate::engine::{TypesetContext, TypesetResult};


/// Types that act as functions.
///
/// These types have to be able to parse tokens into themselves and store the
/// relevant information from the parsing to do their role in typesetting later.
///
/// The trait `FunctionBounds` is automatically implemented for types which can be
/// used as functions, that is they fulfill the bounds  `Debug + PartialEq + 'static`.
pub trait Function: FunctionBounds {
    /// Parse the tokens of the context with the given header and scope into self.
    fn parse(header: &FuncHeader, body: Option<&str>, ctx: &ParseContext)
        -> ParseResult<Self> where Self: Sized;

    /// Execute the function and optionally yield a return value.
    fn typeset(&self, ctx: &TypesetContext) -> TypesetResult<()>;
}

impl PartialEq for dyn Function {
    fn eq(&self, other: &dyn Function) -> bool {
        self.help_eq(other)
    }
}

/// A helper trait that describes requirements for types that can implement [`Function`].
///
/// Automatically implemented for all types which fulfill to the bounds
/// `Debug + PartialEq + 'static`. There should be no need to implement this manually.
pub trait FunctionBounds: Debug {
    /// Cast self into `Any`.
    fn help_cast_as_any(&self) -> &dyn Any;

    /// Compare self with another function.
    fn help_eq(&self, other: &dyn Function) -> bool;
}

impl<T> FunctionBounds for T where T: Debug + PartialEq + 'static {
    fn help_cast_as_any(&self) -> &dyn Any {
        self
    }

    fn help_eq(&self, other: &dyn Function) -> bool {
        if let Some(other) = other.help_cast_as_any().downcast_ref::<Self>() {
            self == other
        } else {
            false
        }
    }
}

/// A map from identifiers to functions.
pub struct Scope {
    parsers: HashMap<String, Box<ParseFunc>>,
}

/// A function which transforms a parsing context into a boxed function.
type ParseFunc = dyn Fn(&FuncHeader, Option<&str>, &ParseContext)
                       -> ParseResult<Box<dyn Function>>;

impl Scope {
    /// Create a new empty scope.
    pub fn new() -> Scope {
        Scope { parsers: HashMap::new() }
    }

    /// Create a new scope with the standard functions contained.
    pub fn with_std() -> Scope {
        Scope::new()
    }

    /// Add a function type to the scope with a given name.
    pub fn add<F: Function + 'static>(&mut self, name: &str) {
        self.parsers.insert(
            name.to_owned(),
            Box::new(|h, b, c| {
                F::parse(h, b, c).map(|func| Box::new(func) as Box<dyn Function>)
            })
        );
    }

    /// Return the parser with the given name if there is one.
    pub(crate) fn get_parser(&self, name: &str) -> Option<&ParseFunc> {
        self.parsers.get(name).map(|x| &**x)
    }
}

impl Debug for Scope {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Scope ")?;
        write!(f, "{:?}", self.parsers.keys())
    }
}
