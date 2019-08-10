mod grettings {
  mod english {
    hello () -> String { "hello".to_string(); }
    goodbye () -> String { "good bye".to_string(); }
  }
  mod french {
    hello () -> String { "bonjour".to_string(); }
    goodbye () -> String { "au revoir".to_string(); }
  }
}