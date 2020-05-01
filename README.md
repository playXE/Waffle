# JSRS
JavaScript implementation in Rust.

This JS runtime is highly inspired by JSC and V8, some ideas have been taken from 
BEAM VM.


## Why? 
This project started as simple scripting language but it advanced to runtime with optimizations on bytecode, inline caching and other stuff and this scripting language already looked like JS so I decided to make JS implementation from it.

## Features
- Bytecode optimizations (constant folding, CFG simplifier).
- Fast bytecode interpreter.
- NaN boxed values for fast access.
- Generational copying GC using [cgc](https://github.com/playxe/cgc)

## TODO
- Parser & Lexer.
- Bytecode re-write.
- Implement JS Objects.
- All ES5 standard objects in runtime.
- Generators and async.
- Just-In-Time compiler using Cranelift or own IR.
