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


## References
- [The Rust Programming Language](https://doc.rust-lang.org/book/)    
