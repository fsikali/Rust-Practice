/*
--- Shortcuts for Panic on Error:unwrap and expect 
- Using match works well enough, but it can be a bit verbose and doesn't always communicate
  intent well.
- The Result<T, E> type has many helper methods defined on it to do various, more specific tasks.
- The unwrap method is a shortcut method implemented just like the match expression
- If the Result value is the Ok variant, unwrap will return the value inside the Ok.
- If the Result is the Err variant, unwrap will call the panic! macro for us.
- Example in main.rs

- If we run this code without a hello.txt file, we'll see an error message from the panic!
  call that the unwrap method makes:

*/
