use crate::primes;

pub use primes::Cirru;

/// Resolves comma syntax in-place.
///
/// The comma syntax is a way to flatten nested expressions.
/// For example, the expression `a (b, c)` is syntactic sugar for `a b c`.
/// In the tree representation, this means `[a, [b, c]]` becomes `[a, b, c]`.
///
/// # Examples
///
/// ```cirru
/// a
///   b, c
/// ```
///
/// which is parsed as `[a, [",", b, c]]`, will be transformed into `[a, b, c]`.
pub fn resolve_comma(xs: &mut Vec<Cirru>) {
  loop {
    let mut comma_pos: Option<usize> = None;
    // Find a comma expression, which is a list starting with a comma leaf.
    // For example: `[`,`, `item1`, `item2`, ...]`
    for (i, x) in xs.iter().enumerate() {
      if let Cirru::List(ys) = x {
        if let Some(Cirru::Leaf(s)) = ys.first() {
          if &**s == "," {
            comma_pos = Some(i);
            break;
          }
        }
      }
    }

    // If a comma expression is found, its children are moved to the current level.
    if let Some(p) = comma_pos {
      let mut to_insert = match xs.remove(p) {
        Cirru::List(ys) => ys,
        _ => unreachable!(),
      };
      to_insert.remove(0);
      for (i, item) in to_insert.into_iter().enumerate() {
        xs.insert(p + i, item);
      }
    } else {
      break;
    }
  }

  let mut i = 0;
  while i < xs.len() {
    if let Cirru::Leaf(s) = &xs[i] {
      if &**s == "," {
        xs.remove(i);
      } else {
        i += 1;
      }
    } else {
      i += 1;
    }
  }

  // Recursively resolve commas in nested lists first.
  for x in &mut *xs {
    if let Cirru::List(ys) = x {
      resolve_comma(ys);
    }
  }
}

/// Resolves dollar syntax in-place.
///
/// The dollar syntax is a shorthand for creating a nested expression.
/// For example, `a $ b c` is equivalent to `a (b c)`.
///
/// # Examples
///
/// ```cirru
/// defn fib (x)
///   fib $ dec x
/// ```
///
/// This will be transformed from `[defn, fib, [x], [fib, $, dec, x]]`
/// to `[defn, fib, [x], [fib, [dec, x]]]`
pub fn resolve_dollar(xs: &mut Vec<Cirru>) {
  // Recursively resolve dollars in nested lists first.
  for x in &mut *xs {
    if let Cirru::List(ys) = x {
      resolve_dollar(ys);
    }
  }

  loop {
    let mut dollar_pos: Option<usize> = None;
    // Find the first occurrence of a dollar sign leaf from the right.
    for (i, x) in xs.iter().enumerate().rev() {
      if let Cirru::Leaf(s) = x {
        if &**s == "$" {
          dollar_pos = Some(i);
          break;
        }
      }
    }

    // If a dollar sign is found, the rest of the elements in the list
    // are wrapped into a new nested list.
    if let Some(p) = dollar_pos {
      let mut new_list = vec![Cirru::List(xs.drain(p + 1..).collect())];
      xs.truncate(p);
      xs.append(&mut new_list);
    } else {
      break;
    }
  }
}
