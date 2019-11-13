# Rust Cheatsheet

Abridged [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/).

This repo provides a set of examples for getting quickly acquainted with the Rust programming language given some experience in software development.

This software is licensed under the Apache 2.0 software license (see LICENSE file). Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.

The author of this software makes no representation or guarantee that this software (including any third-party libraries) will perform as intended or will be free of errors, bugs or faulty code. The software may fail which could completely or partially limit functionality or compromise computer systems. If you use or implement the software, you do so at your own risk. In no event will the author of this software be liable to any party for any damages whatsoever, even if it had been advised of the possibility of damage.

To get started, just jump through the files in the repo as needed.

## Quick tips

- A lambda expression in called a closure in Rust.

- `format!` is `print!` but writes to heap if you want to store the string.

- `x?` is shorthand for `try!(x)`

- Underscores can be used for numeric readability: 1_000 is the same as 1000, and 0.000_001 is the same as 0.000001

- Struct is like a dict. Enums are structs which take one of the types in the enum.

- Signed integer example: `128 as i8` = `-128` because there are 256 values, 128 negative, 127 positive, and 0. `256 as i8` = `0` because w wrapped round -128 and added another 128 to get 0. `384 as i8` = `-128`. It works like modular arithmetic.
