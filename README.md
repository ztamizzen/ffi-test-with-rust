# A test implementation to allow Rust and Node to cooperate

It's not usefull in any way, I was just curious on how to get FFI to work with Rust.

## Output should be as below:
```javascript
λ node node-ffi-runner.js
123 + 123 = 246
28 'chars in string' 'I\'m a string and I\'m lövely!'
💣 na na na na na Batman! 💣
12
Tuple { x: 10, y: 20, 'ref.buffer': <Buffer 0a 00 00 00 14 00 00 00> }
(20, 10)
Population: 77377
```