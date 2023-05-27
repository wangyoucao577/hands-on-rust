# Notes of The Rust Programming Language     
My notes when reading [The Rust Programming Language](https://doc.rust-lang.org/book/) book.     

## 4. Understanding Ownership

### Ownership Rules   
- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.   

### Ownership pattern   
- The ownership of a variable follows the same pattern every time: **assigning a value to another variable moves it**.      
  - i.e., move semantics by default            
- When a variable that includes data on the heap goes out of scope, the value will be cleaned up by `drop` unless ownership of the data has been moved to another variable.     
  - i.e., free the memory if goes out of scope AND has ownership       

### The Rules of References
- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.       

### Borrowing    
- We call **the action of creating a reference** borrowing. As in real life, if a person owns something, you can borrow it from them. When you’re done, you have to give it back. You don’t own it.     

### One big restriction of Mutable references
- if you have a mutable reference to a value, you can have **no other** references to that value. 
- The restriction **preventing multiple mutable references to the same data at the same time allows for mutation but in a very controlled fashion**.      
- The benefit of having this restriction is that Rust can **prevent data races at compile time**.
  - A data race is similar to a race condition and happens when these three behaviors occur:
    - Two or more pointers access the same data at the same time.
    - At least one of the pointers is being used to write to the data.
    - There’s no mechanism being used to synchronize access to the data.     
  - Data races cause undefined behavior and can be difficult to diagnose and fix when you’re trying to track them down at runtime;     

## 7. Managing Growing Projects with Packages, Crates and Modules

### Packages and Crates     
- A crate is the smallest amount of code that the Rust compiler considers at a time.      
- A crate can come in one of two forms: a binary crate or a library crate.     
- A package can contain **as many binary crates** as you like, but **at most only one library crate**.
- Cargo follows a convention that `src/main.rs` is the crate root of a binary crate with the same name as the package.   
- Likewise, Cargo knows that if the package directory contains `src/lib.rs`, the package contains a library crate with the same name as the package, and `src/lib.rs` is its crate root.    
- A package can have **multiple binary crates** by placing files in the `src/bin` directory: each file will be a separate binary crate.      

### [Modules Cheat Sheet](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html#modules-cheat-sheet)    
- **Start from the crate root:** When compiling a crate, the compiler first looks in the crate root file (usually `src/lib.rs` for a library crate or `src/main.rs` for a binary crate) for code to compile.
- **Declaring modules:** In the crate root file, you can declare new modules; say, you declare a “garden” module with `mod garden`;. The compiler will look for the module’s code in these places:
  - Inline, within curly brackets that replace the semicolon following `mod garden`
  - In the file `src/garden.rs`
  - In the file `src/garden/mod.rs`(old style, not recommended)
- **Declaring submodules:** In any file other than the crate root, you can declare submodules. For example, you might declare `mod vegetables`; in `src/garden.rs`. The compiler will look for the submodule’s code within the directory named for the parent module in these places:
  - Inline, directly following `mod vegetables`, within curly brackets instead of the semicolon
  - In the file `src/garden/vegetables.rs`
  - In the file `src/garden/vegetables/mod.rs`(old style, not recommended)
- **Paths to code in modules:** Once a module is part of your crate, you can refer to code in that module from anywhere else in that same crate, as long as the privacy rules allow, using the path to the code. For example, an `Asparagus` type in the garden vegetables module would be found at `crate::garden::vegetables::Asparagus`.
- **Private vs public:** Code within a module is private from its parent modules by default. To make a module public, declare it with `pub mod` instead of `mod`. To make items within a public module public as well, use `pub` before their declarations.
- **The `use` keyword:** Within a scope, the `use` keyword creates shortcuts to items to reduce repetition of long paths. In any scope that can refer to `crate::garden::vegetables::Asparagus`, you can create a shortcut with `use crate::garden::vegetables::Asparagus`; and from then on you only need to write `Asparagus` to make use of that type in the scope.    

## 10. Generic Types, Traits, and Lifetimes

### Traits: Defining Shared Behavior

- Note: **Traits** are **similar to** a feature often called **interfaces** in other languages, although **with some differences**.

### Validating References with Lifetimes     

- The main aim of **lifetimes** is to prevent **dangling references**. 
- There is no way we can specify **lifetime** parameters that would change the **dangling reference**, and Rust won’t let us create a **dangling reference**. 
- **Lifetime** annotations **don’t change** how long any of the references live. Rather, they describe the relationships of the **lifetimes** of multiple references to each other without affecting the **lifetimes**. 
- One **lifetime** annotation by itself doesn’t have much meaning, because the annotations are meant to tell Rust how generic lifetime parameters of multiple references relate to each other.
- **lifetime elision rules**
  - These aren’t rules for programmers to follow; they’re a set of particular cases that the compiler will consider, and if your code fits these cases, you don’t need to write the lifetimes explicitly.
  - The compiler uses **three rules** to figure out the lifetimes of the references when there aren’t explicit annotations.
    - The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference. In other words, a function with one parameter gets one lifetime parameter: `fn foo<'a>(x: &'a i32)`; a function with two parameters gets two separate lifetime parameters: `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`; and so on.
    - The second rule is that, if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: `fn foo<'a>(x: &'a i32) -> &'a i32`.
    - The third rule is that, if there are multiple input lifetime parameters, but one of them is `&self` or `&mut self` because this is a method, the lifetime of self is assigned to all output lifetime parameters. 
- One special lifetime we need to discuss is `'static`, which denotes that the affected reference can live for the entire duration of the program.
  - But before specifying `'static` as the lifetime for a reference, think about whether the reference you have actually lives the entire lifetime of your program or not, and whether you want it to.
  - Most of the time, an error message suggesting the `'static` lifetime results from attempting to create a **dangling reference** or a mismatch of the available lifetimes. In such cases, the solution is fixing those problems, not specifying the `'static` lifetime.

## 11. Writing Automated Tests

### Two Main Categories of Tests   
- The Rust community thinks about tests in terms of two main categories: **unit tests** and **integration tests**.
  - **unit tests**: small and more focused, testing one module in isolation at a time, and can test private interfaces      
  - **integration tests**: entirely external to your library and use your code in the same way any other external code would, using only the public interface and potentially exercising multiple modules per test.




## Useful Tools
- [Appendix D - Useful Development Tools](https://doc.rust-lang.org/book/appendix-04-useful-development-tools.html)    
- [rust-analyzer](https://rust-analyzer.github.io/)
- [Rust in Visual Studio Code](https://code.visualstudio.com/docs/languages/rust)      


## References
- [The Rust Programming Language](https://doc.rust-lang.org/book/)    
