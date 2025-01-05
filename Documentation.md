# Micron Documentation 2025r0 (WIP)
This is the documentation for Micron 2025 Release 0.

## Valid Micron Program
Aside from valid syntax and semantics, a valid Micron file must use the `.mcrn` file extension. 
If not the Micron Interpreter will error out.

## The Anatomy of a Micron Program
A Micron Program, currently can be broken up into two sections:
1. The `#include <library_name>` section
   - In this section additional libraries can added for extended functionality.
2. The program's logic section
   - In this section the main logic for the program is written.

### Libraries
- `bool`
  - `invert(bool x)`
- `condition`
  - `greater(int x, int y)`, `less(int x, int y)`, `equal(int x, int y)`, `greater_equal(int x, int y)`, `less_equal(int x, int y)`, `not_equal(int x, int y)`
- `math`
  - `add(int x, int y)`, `sub(int x, int y)`, `mul(int x, int y)`, `div(int x, int y)`, `mod(int x, int y)`, `exp(int x, int y)`, `rad(int x, int y)`
- `print`
  - `print(String str)`, `println(String str)`