// TODO: this is an example of an orphan rule violation.
//  We're implementing a foreign trait (`PartialEq`, from `std`) on
//  a foreign type (`u32`, from `std`).
//  Look at the compiler error to get familiar with what it looks like.
//  Then delete the code below and move on to the next exercise.

/**
   Compiling orphan v0.1.0 (D:\Google Drive\project\100-exercises-to-learn-rust\exercises\04_traits\02_orphan_rule)
error[E0117]: only traits defined in the current crate can be implemented for primitive types
 --> exercises\04_traits\02_orphan_rule\src\lib.rs:7:1
  |
  | ^^^^^---------^^^^^---
  | |    |             |
  | |    `u32` is not defined in the current crate
  |
  = note: define and implement a trait or new type instead

For more information about this error, try `rustc --explain E0117`.
*/