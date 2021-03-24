//! # Description
//! This Rust library can make you use the arrow operater `->` to access row pointers' fields, which is like C/C++.
//! # Usage
//! `pt![`\<link\>`]`
//!
//! get a mutable reference of the pointed field.
//!
//! `pt![`\<link\> `=` \<expression\>`];`
//!
//! assign expression to the pointed field.
//!
//! `pt![`\<link\> `=` \<link\>`];`
//!
//! assign the pointed field to the other pointed field.
//!
//! ### Explanation
//! | Objects | Explanations |
//! | :---:    | :---- |
//! | \<func\> | A function returning the `*mut` pointer of a struct.               |
//! | \<met\>  | A method which returns the `*mut` pointer of a struct.             |
//! | \<ptr\>`->`\<field\>   | Dereferences and accesses the field.                    |
//! | \<struct\>`.`\<field\> | Accesses the field directly.                           |
//! | \<link\> | (\<func\> \| \<ptr\> \| \<struct\>) ((`->` \| `.`)(\<field\> \| \<met\>))+ |
//! 
//! # Example
//! ```rust
//! let mut stack: Stack<char> = Stack::new();
//! "abcdefgh"
//!     .chars()
//!     .for_each(|c| stack.push(c));
//!
//! let top_back = pt![stack.top->back];
//! pt![back_of(top_back)->back->data = 'x'];
//! pt![stack.top->backs(4)->data = stack.back_of_top()->data];
//! ```

pub mod c_arrow;