# uni-glsl

A just works but completed (GLSL_ES 2.0 aka WebGL) glsl preprocessor and parser in Rust.

## Features

### Preprocessor
Supported syntax : 
```
# #define #undef #ifdef #ifndef #else #endif #if #elif defined #include
```
Ignored :
```
#line #version #extension #pragma
```
Todo :
```
Update to GLSL_ES 3.0
```

### Parser
Full syntax based on https://www.khronos.org/registry/OpenGL/specs/es/2.0/GLSL_ES_Specification_1.00.pdf

## Usage
See the integeration test in tests/integeration_test.rs

