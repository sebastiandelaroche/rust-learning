fn how_many(x: i32) -> &'static str {
  match x {
    0 => "no",
    1 | 2 => "one or two",
    12 => "a dozen",
    z @ 9...11 => "lots of",
    _ if (x % 2 == 0) => "some",
    _ => "a few",
  }
}

pub fn pattern_matching() {
  for x in 0..13 {
    println!("{}: I have {} oranges", x, how_many(x));
  }
}
