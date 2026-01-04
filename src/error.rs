/*! Error handling and warning system for Cirru Parser */

use std::fmt;

/// Position information in the source code
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourcePos {
  /// Line number (1-indexed)
  pub line: usize,
  /// Column number (1-indexed)
  pub column: usize,
  /// Byte offset in the source
  pub offset: usize,
}

impl SourcePos {
  pub fn new(line: usize, column: usize, offset: usize) -> Self {
    Self { line, column, offset }
  }
}

impl fmt::Display for SourcePos {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "line {}, column {}", self.line, self.column)
  }
}

/// Context information for better error messages
#[derive(Debug, Clone, PartialEq)]
pub struct ErrorContext {
  /// Position where the error occurred
  pub pos: SourcePos,
  /// Code snippet around the error (if available)
  pub snippet: Option<String>,
  /// Current parsing context (e.g., "in string", "in list", "at top level")
  pub context_info: String,
}

impl ErrorContext {
  pub fn new(pos: SourcePos, snippet: Option<String>, context_info: String) -> Self {
    Self {
      pos,
      snippet,
      context_info,
    }
  }
}

/// Different kinds of parse errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CirruErrorKind {
  /// Unexpected character during parsing
  UnexpectedChar(char),
  /// Unexpected end of file
  UnexpectedEof,
  /// Unmatched parenthesis
  UnmatchedParen,
  /// Invalid indentation (odd number of spaces)
  InvalidIndentation(u8),
  /// Unexpected newline in string literal
  UnexpectedNewlineInString,
  /// Invalid escape sequence
  InvalidEscape(char),
  /// Unexpected closing parenthesis
  UnexpectedCloseParen,
  /// Wrong number of top-level expressions
  WrongExprCount { expected: usize, got: usize },
  /// Generic error with custom message
  Other(String),
}

impl fmt::Display for CirruErrorKind {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::UnexpectedChar(c) => write!(f, "Unexpected character '{c}'"),
      Self::UnexpectedEof => write!(f, "Unexpected end of file"),
      Self::UnmatchedParen => write!(f, "Unmatched parenthesis"),
      Self::InvalidIndentation(n) => write!(f, "Invalid indentation (odd number: {n})"),
      Self::UnexpectedNewlineInString => write!(f, "Unexpected newline in string literal"),
      Self::InvalidEscape(c) => write!(f, "Invalid escape sequence '\\{c}'"),
      Self::UnexpectedCloseParen => write!(f, "Unexpected closing parenthesis ')'"),
      Self::WrongExprCount { expected, got } => {
        write!(f, "Expected {expected} expression(s), but got {got}")
      }
      Self::Other(msg) => write!(f, "{msg}"),
    }
  }
}

/// Main error type for Cirru parsing
#[derive(Debug, Clone, PartialEq)]
pub struct CirruError {
  pub kind: CirruErrorKind,
  pub context: Option<ErrorContext>,
}

impl CirruError {
  pub fn new(kind: CirruErrorKind) -> Self {
    Self { kind, context: None }
  }

  pub fn with_context(kind: CirruErrorKind, context: ErrorContext) -> Self {
    Self {
      kind,
      context: Some(context),
    }
  }

  /// Create error from simple message (for backward compatibility)
  pub fn from_message(msg: impl Into<String>) -> Self {
    Self::new(CirruErrorKind::Other(msg.into()))
  }

  /// Format error with detailed context
  pub fn format_detailed(&self, source_code: Option<&str>) -> String {
    let mut output = format!("Error: {}", self.kind);

    if let Some(ctx) = &self.context {
      output.push_str(&format!("\n  at {}", ctx.pos));
      output.push_str(&format!("\n  context: {}", ctx.context_info));

      // Show snippet if available
      if let Some(snippet) = &ctx.snippet {
        output.push_str(&format!("\n  near (escaped): {snippet}"));
      } else if let Some(code) = source_code {
        // Try to extract snippet from source code
        if let Some(snippet) = extract_snippet(code, &ctx.pos) {
          output.push_str(&format!("\n\n{snippet}"));
        }
      }
    }

    output
  }
}

impl fmt::Display for CirruError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.kind)?;
    if let Some(ctx) = &self.context {
      write!(f, " at {}", ctx.pos)?;
    }
    Ok(())
  }
}

impl std::error::Error for CirruError {}

// For backward compatibility with String errors
impl From<String> for CirruError {
  fn from(msg: String) -> Self {
    Self::from_message(msg)
  }
}

impl From<&str> for CirruError {
  fn from(msg: &str) -> Self {
    Self::from_message(msg)
  }
}

impl From<CirruError> for String {
  fn from(error: CirruError) -> Self {
    error.to_string()
  }
}

/// Extract a code snippet around the given position
fn extract_snippet(code: &str, pos: &SourcePos) -> Option<String> {
  let lines: Vec<&str> = code.lines().collect();
  if pos.line == 0 || pos.line > lines.len() {
    return None;
  }

  let line_idx = pos.line - 1;
  let start = line_idx.saturating_sub(1);
  let end = (line_idx + 2).min(lines.len());

  let mut snippet = String::new();
  for (i, line) in lines[start..end].iter().enumerate() {
    let current_line = start + i + 1;
    snippet.push_str(&format!("{current_line:4} | {line}\n"));

    // Add error pointer for the error line
    if current_line == pos.line {
      let pointer_offset = pos.column.saturating_sub(1);
      snippet.push_str(&format!("     | {}^\n", " ".repeat(pointer_offset)));
    }
  }

  Some(snippet)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_error_display() {
    let err = CirruError::new(CirruErrorKind::UnexpectedChar('x'));
    assert_eq!(err.to_string(), "Unexpected character 'x'");
  }

  #[test]
  fn test_error_with_context() {
    let pos = SourcePos::new(10, 5, 100);
    let ctx = ErrorContext::new(pos, Some("foo bar".to_string()), "in list".to_string());
    let err = CirruError::with_context(CirruErrorKind::UnexpectedEof, ctx);
    let display = err.to_string();
    assert!(display.contains("line 10"));
  }

  #[test]
  fn test_snippet_extraction() {
    let code = "line1\nline2\nline3\nline4\nline5";
    let pos = SourcePos::new(3, 3, 12);
    let snippet = extract_snippet(code, &pos).unwrap();
    assert!(snippet.contains("line2"));
    assert!(snippet.contains("line3"));
    assert!(snippet.contains("line4"));
    assert!(snippet.contains("^"));
  }
}
