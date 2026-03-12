//! AZC Pattern Matching
//!
//! Implements pattern matching for match expressions and destructuring.

use std::collections::HashMap;
use std::fmt;

use super::Type;

/// Pattern for matching
#[derive(Debug, Clone)]
pub enum Pattern {
    /// Wildcard pattern (_)
    Wildcard,

    /// Variable binding
    Variable(String),

    /// Literal pattern
    Literal(LiteralPattern),

    /// Tuple pattern
    Tuple(Vec<Pattern>),

    /// Array pattern
    Array {
        elements: Vec<Pattern>,
        rest: Option<Box<Pattern>>,
    },

    /// Struct pattern
    Struct {
        type_name: String,
        fields: Vec<(String, Pattern)>,
        rest: bool,
    },

    /// Enum variant pattern
    Enum {
        type_name: String,
        variant: String,
        fields: Option<Vec<Pattern>>,
    },

    /// Range pattern
    Range {
        start: LiteralPattern,
        end: LiteralPattern,
        inclusive: bool,
    },

    /// Or pattern (alternatives)
    Or(Vec<Pattern>),

    /// Guarded pattern
    Guarded {
        pattern: Box<Pattern>,
        guard: String,
    },

    /// Type annotation
    As {
        pattern: Box<Pattern>,
        type_ann: Type,
    },
}

impl Pattern {
    /// Create a wildcard pattern
    pub fn wildcard() -> Self {
        Pattern::Wildcard
    }

    /// Create a variable pattern
    pub fn var(name: impl Into<String>) -> Self {
        Pattern::Variable(name.into())
    }

    /// Create a literal pattern
    pub fn lit(lit: LiteralPattern) -> Self {
        Pattern::Literal(lit)
    }

    /// Create an integer literal pattern
    pub fn int(n: i64) -> Self {
        Pattern::Literal(LiteralPattern::Int(n))
    }

    /// Create a string literal pattern
    pub fn string(s: impl Into<String>) -> Self {
        Pattern::Literal(LiteralPattern::String(s.into()))
    }

    /// Create a tuple pattern
    pub fn tuple(elements: Vec<Pattern>) -> Self {
        Pattern::Tuple(elements)
    }

    /// Create an array pattern
    pub fn array(elements: Vec<Pattern>) -> Self {
        Pattern::Array {
            elements,
            rest: None,
        }
    }

    /// Create a struct pattern
    pub fn struct_(type_name: impl Into<String>, fields: Vec<(String, Pattern)>) -> Self {
        Pattern::Struct {
            type_name: type_name.into(),
            fields,
            rest: false,
        }
    }

    /// Create an enum pattern
    pub fn enum_variant(
        type_name: impl Into<String>,
        variant: impl Into<String>,
        fields: Option<Vec<Pattern>>,
    ) -> Self {
        Pattern::Enum {
            type_name: type_name.into(),
            variant: variant.into(),
            fields,
        }
    }

    /// Create a range pattern
    pub fn range(start: LiteralPattern, end: LiteralPattern, inclusive: bool) -> Self {
        Pattern::Range {
            start,
            end,
            inclusive,
        }
    }

    /// Check if pattern is irrefutable (always matches)
    pub fn is_irrefutable(&self) -> bool {
        match self {
            Pattern::Wildcard | Pattern::Variable(_) => true,
            Pattern::Tuple(elements) => elements.iter().all(|e| e.is_irrefutable()),
            Pattern::Array { elements, rest } => {
                elements.iter().all(|e| e.is_irrefutable()) && rest.is_none()
            }
            Pattern::Struct { fields, rest, .. } => {
                fields.iter().all(|(_, p)| p.is_irrefutable()) && !rest
            }
            _ => false,
        }
    }

    /// Get all bindings in this pattern
    pub fn bindings(&self) -> Vec<&str> {
        match self {
            Pattern::Variable(name) => vec![name],
            Pattern::Tuple(elements) => elements.iter().flat_map(|e| e.bindings()).collect(),
            Pattern::Array { elements, rest } => {
                let mut bindings: Vec<&str> = elements.iter().flat_map(|e| e.bindings()).collect();
                if let Some(r) = rest {
                    bindings.extend(r.bindings());
                }
                bindings
            }
            Pattern::Struct { fields, .. } => {
                fields.iter().flat_map(|(_, p)| p.bindings()).collect()
            }
            Pattern::Enum {
                fields: Some(fields),
                ..
            } => fields.iter().flat_map(|f| f.bindings()).collect(),
            Pattern::Or(patterns) => patterns.first().map(|p| p.bindings()).unwrap_or_default(),
            Pattern::Guarded { pattern, .. } => pattern.bindings(),
            Pattern::As { pattern, .. } => pattern.bindings(),
            _ => Vec::new(),
        }
    }
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Pattern::Wildcard => write!(f, "_"),
            Pattern::Variable(name) => write!(f, "{}", name),
            Pattern::Literal(lit) => write!(f, "{}", lit),
            Pattern::Tuple(elements) => {
                write!(f, "(")?;
                for (i, elem) in elements.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", elem)?;
                }
                write!(f, ")")
            }
            Pattern::Array { elements, rest } => {
                write!(f, "[")?;
                for (i, elem) in elements.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", elem)?;
                }
                if let Some(r) = rest {
                    write!(f, ", ..{}", r)?;
                }
                write!(f, "]")
            }
            Pattern::Struct {
                type_name,
                fields,
                rest,
            } => {
                write!(f, "{} {{ ", type_name)?;
                for (i, (name, pattern)) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", name, pattern)?;
                }
                if *rest {
                    write!(f, ", ..")?;
                }
                write!(f, " }}")
            }
            Pattern::Enum {
                type_name,
                variant,
                fields,
            } => {
                write!(f, "{}::{}", type_name, variant)?;
                if let Some(fields) = fields {
                    write!(f, "(")?;
                    for (i, field) in fields.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", field)?;
                    }
                    write!(f, ")")?;
                }
                Ok(())
            }
            Pattern::Range {
                start,
                end,
                inclusive,
            } => {
                write!(f, "{}", start)?;
                if *inclusive {
                    write!(f, "..=")?;
                } else {
                    write!(f, "..")?;
                }
                write!(f, "{}", end)
            }
            Pattern::Or(patterns) => {
                for (i, p) in patterns.iter().enumerate() {
                    if i > 0 {
                        write!(f, " | ")?;
                    }
                    write!(f, "{}", p)?;
                }
                Ok(())
            }
            Pattern::Guarded { pattern, guard } => {
                write!(f, "{} if {}", pattern, guard)
            }
            Pattern::As { pattern, type_ann } => {
                write!(f, "{}: {}", pattern, type_ann)
            }
        }
    }
}

/// Literal pattern
#[derive(Debug, Clone, PartialEq)]
pub enum LiteralPattern {
    Int(i64),
    Float(f64),
    Bool(bool),
    Char(char),
    String(String),
}

impl fmt::Display for LiteralPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiteralPattern::Int(n) => write!(f, "{}", n),
            LiteralPattern::Float(n) => write!(f, "{}", n),
            LiteralPattern::Bool(b) => write!(f, "{}", b),
            LiteralPattern::Char(c) => write!(f, "'{}'", c),
            LiteralPattern::String(s) => write!(f, "\"{}\"", s),
        }
    }
}

/// Match arm
#[derive(Debug, Clone)]
pub struct MatchArm {
    /// Pattern to match
    pub pattern: Pattern,
    /// Optional guard condition
    pub guard: Option<String>,
    /// Body expression
    pub body: String,
}

impl MatchArm {
    pub fn new(pattern: Pattern, body: impl Into<String>) -> Self {
        MatchArm {
            pattern,
            guard: None,
            body: body.into(),
        }
    }

    pub fn with_guard(mut self, guard: impl Into<String>) -> Self {
        self.guard = Some(guard.into());
        self
    }
}

/// Pattern matcher
#[derive(Debug)]
pub struct PatternMatcher {
    /// Matched bindings
    bindings: HashMap<String, String>,
}

impl PatternMatcher {
    pub fn new() -> Self {
        PatternMatcher {
            bindings: HashMap::new(),
        }
    }

    /// Try to match a value against a pattern
    pub fn match_pattern(&mut self, pattern: &Pattern, value: &str) -> bool {
        self.bindings.clear();
        self.match_impl(pattern, value)
    }

    fn match_impl(&mut self, pattern: &Pattern, value: &str) -> bool {
        match pattern {
            Pattern::Wildcard => true,

            Pattern::Variable(name) => {
                self.bindings.insert(name.clone(), value.to_string());
                true
            }

            Pattern::Literal(lit) => {
                let expected = match lit {
                    LiteralPattern::Int(n) => n.to_string(),
                    LiteralPattern::Float(n) => n.to_string(),
                    LiteralPattern::Bool(b) => b.to_string(),
                    LiteralPattern::Char(c) => format!("'{}'", c),
                    LiteralPattern::String(s) => format!("\"{}\"", s),
                };
                value == expected
            }

            Pattern::Tuple(elements) => {
                // Parse tuple value
                let values = self.parse_tuple(value);
                if values.len() != elements.len() {
                    return false;
                }

                for (elem, val) in elements.iter().zip(values.iter()) {
                    if !self.match_impl(elem, val) {
                        return false;
                    }
                }
                true
            }

            Pattern::Array { elements, rest } => {
                // Parse array value
                let values = self.parse_array(value);

                if rest.is_none() && values.len() != elements.len() {
                    return false;
                }

                if elements.len() > values.len() {
                    return false;
                }

                for (elem, val) in elements.iter().zip(values.iter()) {
                    if !self.match_impl(elem, val) {
                        return false;
                    }
                }

                if let Some(rest_pattern) = rest {
                    // Match remaining elements
                    let remaining = &values[elements.len()..];
                    let remaining_str = format!("[{}]", remaining.join(", "));
                    return self.match_impl(rest_pattern, &remaining_str);
                }

                true
            }

            Pattern::Or(patterns) => patterns.iter().any(|p| self.match_impl(p, value)),

            _ => {
                // Complex patterns require runtime support
                false
            }
        }
    }

    fn parse_tuple(&self, value: &str) -> Vec<String> {
        let value = value.trim();
        if !value.starts_with('(') || !value.ends_with(')') {
            return vec![];
        }

        let inner = &value[1..value.len() - 1];
        inner.split(',').map(|s| s.trim().to_string()).collect()
    }

    fn parse_array(&self, value: &str) -> Vec<String> {
        let value = value.trim();
        if !value.starts_with('[') || !value.ends_with(']') {
            return vec![];
        }

        let inner = &value[1..value.len() - 1];
        inner.split(',').map(|s| s.trim().to_string()).collect()
    }

    /// Get binding value
    pub fn get(&self, name: &str) -> Option<&String> {
        self.bindings.get(name)
    }

    /// Get all bindings
    pub fn get_bindings(&self) -> &HashMap<String, String> {
        &self.bindings
    }

    /// Clear bindings
    pub fn clear(&mut self) {
        self.bindings.clear();
    }
}

impl Default for PatternMatcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wildcard_pattern() {
        let mut matcher = PatternMatcher::new();
        assert!(matcher.match_pattern(&Pattern::wildcard(), "anything"));
    }

    #[test]
    fn test_variable_pattern() {
        let mut matcher = PatternMatcher::new();
        assert!(matcher.match_pattern(&Pattern::var("x"), "42"));
        assert_eq!(matcher.get("x"), Some(&"42".to_string()));
    }

    #[test]
    fn test_literal_pattern() {
        let mut matcher = PatternMatcher::new();
        assert!(matcher.match_pattern(&Pattern::int(42), "42"));
        assert!(!matcher.match_pattern(&Pattern::int(42), "43"));
    }

    #[test]
    fn test_tuple_pattern() {
        let mut matcher = PatternMatcher::new();
        let pattern = Pattern::tuple(vec![Pattern::var("a"), Pattern::var("b")]);

        assert!(matcher.match_pattern(&pattern, "(1, 2)"));
        assert_eq!(matcher.get("a"), Some(&"1".to_string()));
        assert_eq!(matcher.get("b"), Some(&"2".to_string()));
    }

    #[test]
    fn test_or_pattern() {
        let mut matcher = PatternMatcher::new();
        let pattern = Pattern::Or(vec![Pattern::int(1), Pattern::int(2)]);

        assert!(matcher.match_pattern(&pattern, "1"));
        assert!(matcher.match_pattern(&pattern, "2"));
        assert!(!matcher.match_pattern(&pattern, "3"));
    }

    // ===== Comprehensive v0.3.0 Tests =====

    #[test]
    fn test_array_pattern() {
        let mut matcher = PatternMatcher::new();
        let pattern = Pattern::array(vec![Pattern::var("first"), Pattern::var("second")]);

        assert!(matcher.match_pattern(&pattern, "[1, 2]"));
        assert_eq!(matcher.get("first"), Some(&"1".to_string()));
        assert_eq!(matcher.get("second"), Some(&"2".to_string()));
    }

    #[test]
    #[ignore = "Struct patterns not yet implemented in pattern matcher"]
    fn test_struct_pattern() {
        let mut matcher = PatternMatcher::new();
        let pattern = Pattern::struct_(
            "Point",
            vec![
                ("x".to_string(), Pattern::var("px")),
                ("y".to_string(), Pattern::var("py")),
            ],
        );

        assert!(matcher.match_pattern(&pattern, "Point { x: 1, y: 2 }"));
    }

    #[test]
    #[ignore = "Enum patterns not yet implemented in pattern matcher"]
    fn test_enum_pattern_simple() {
        let mut matcher = PatternMatcher::new();
        let pattern = Pattern::enum_variant(
            "Option",
            "Some",
            Some(vec![Pattern::var("value")]),
        );

        assert!(matcher.match_pattern(&pattern, "Some(42)"));
        assert_eq!(matcher.get("value"), Some(&"42".to_string()));
    }

    #[test]
    #[ignore = "Range patterns not yet implemented in pattern matcher"]
    fn test_range_pattern() {
        let mut matcher = PatternMatcher::new();
        let pattern = Pattern::range(
            LiteralPattern::Int(1),
            LiteralPattern::Int(10),
            true,
        );

        assert!(matcher.match_pattern(&pattern, "1"));
        assert!(matcher.match_pattern(&pattern, "5"));
        assert!(matcher.match_pattern(&pattern, "10"));
        assert!(!matcher.match_pattern(&pattern, "0"));
        assert!(!matcher.match_pattern(&pattern, "11"));
    }

    #[test]
    #[ignore = "Nested tuple patterns not yet fully supported in pattern matcher"]
    fn test_nested_tuple_pattern() {
        let mut matcher = PatternMatcher::new();
        let pattern = Pattern::tuple(vec![
            Pattern::tuple(vec![Pattern::var("a"), Pattern::var("b")]),
            Pattern::var("c"),
        ]);

        assert!(matcher.match_pattern(&pattern, "((1, 2), 3)"));
        assert_eq!(matcher.get("a"), Some(&"1".to_string()));
        assert_eq!(matcher.get("b"), Some(&"2".to_string()));
        assert_eq!(matcher.get("c"), Some(&"3".to_string()));
    }

    #[test]
    fn test_wildcard_in_tuple() {
        let mut matcher = PatternMatcher::new();
        let pattern = Pattern::tuple(vec![
            Pattern::var("x"),
            Pattern::wildcard(),
            Pattern::var("z"),
        ]);

        assert!(matcher.match_pattern(&pattern, "(1, 2, 3)"));
        assert_eq!(matcher.get("x"), Some(&"1".to_string()));
        assert_eq!(matcher.get("z"), Some(&"3".to_string()));
    }

    #[test]
    fn test_pattern_clear() {
        let mut matcher = PatternMatcher::new();
        matcher.match_pattern(&Pattern::var("x"), "42");
        assert!(matcher.get("x").is_some());
        
        matcher.clear();
        assert!(matcher.get("x").is_none());
    }

    #[test]
    fn test_pattern_default() {
        let matcher = PatternMatcher::default();
        assert!(matcher.bindings.is_empty());
    }

    #[test]
    fn test_literal_pattern_float() {
        let mut matcher = PatternMatcher::new();
        let pattern = Pattern::Literal(LiteralPattern::Float(3.14));
        
        assert!(matcher.match_pattern(&pattern, "3.14"));
        assert!(!matcher.match_pattern(&pattern, "2.71"));
    }

    #[test]
    fn test_literal_pattern_string() {
        let mut matcher = PatternMatcher::new();
        let pattern = Pattern::string("hello");
        
        // String patterns expect the value with quotes
        assert!(matcher.match_pattern(&pattern, "\"hello\""));
        assert!(!matcher.match_pattern(&pattern, "\"world\""));
    }

    #[test]
    fn test_literal_pattern_bool() {
        let mut matcher = PatternMatcher::new();
        let pattern = Pattern::Literal(LiteralPattern::Bool(true));
        
        assert!(matcher.match_pattern(&pattern, "true"));
        assert!(!matcher.match_pattern(&pattern, "false"));
    }
}
