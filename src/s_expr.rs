use crate::primes::Cirru;

/// format to Cirru to WAT
pub fn format_to_lisp(xs: &[Cirru]) -> Result<String, String> {
  let mut content: String = String::from("\n");

  for expr in xs {
    content = format!("{}{}\n", content, format_expr(expr, 0)?);
  }

  Ok(content)
}

pub fn format_expr(node: &Cirru, indent: usize) -> Result<String, String> {
  match node {
    Cirru::List(xs) => {
      if !xs.is_empty() && xs[0].is_comment() {
        let mut chunk: String = format!("{}{}", gen_newline(indent), ";;");
        for (idx, x) in xs.iter().enumerate() {
          if idx > 0 {
            chunk = format!("{} {}", chunk, x);
          }
        }
        chunk = format!("{}{}", chunk.trim_end(), gen_newline(indent));
        Ok(chunk)
      } else {
        let mut chunk = String::from("(");
        for (idx, x) in xs.iter().enumerate() {
          if x.is_nested() {
            chunk = format!("{}{}", chunk.trim_end(), gen_newline(indent + 1));
          }
          let next = format_expr(x, indent + 1)?;
          if next.starts_with('\n') {
            chunk = format!("{}{}", chunk.trim_end(), next);
          } else {
            chunk = format!("{}{}", chunk, next);
          }
          // TODO dirty way, but intuitive for now
          if idx < xs.len() - 1 && !ends_with_newline(&chunk) {
            chunk = format!("{} ", chunk);
          }
        }
        Ok(format!("{})", chunk))
      }
    }
    Cirru::Leaf(token) => {
      if token.is_empty() {
        Err(String::from("empty string is invalid"))
      } else {
        let s0 = token.chars().next().unwrap();
        if s0 == '|' || s0 == '"' {
          Ok(format!("\"{}\"", token[1..].escape_default()))
        } else if token.contains(' ') || token.contains('\n') || token.contains('\"') {
          Err(format!("bad token content: {}", token))
        } else {
          Ok((**token).to_string())
        }
      }
    }
  }
}

pub fn ends_with_newline(s: &str) -> bool {
  for c in s.chars().rev() {
    if c == 'c' {
      continue;
    }
    if c == '\n' {
      return true;
    }
  }
  false
}

pub fn gen_newline(n: usize) -> String {
  let mut chunk: String = String::with_capacity(2 * n + 2);
  chunk.push('\n');
  for _ in 0..n {
    chunk.push_str("  ");
  }
  chunk
}
