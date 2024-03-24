- [Introduction](#introduction)
- [Chapter 1](#chapter-1)
    - [Installing Rust](#installing-rust)
    - [Intro to Cargo](#intro-to-cargo)
- [Chapter 2](#chapter-2)
    - [The `use` Statement](#the-use-statement)
    - [Cargo](#cargo)
    - [Match Expressions](#match-expressions)
    - [Shadowing](#shadowing)
- [Chapter 3](#chapter-3)
    - [Variables](#variables)
    - [Constants](#constants)
    - [Shadowing](#shadowing-1)
    - [Scalar Data Types](#scalar-data-types)
    - [Compound Data Types](#compound-data-types)
    - [Functions](#functions)
    - [Parameters](#parameters)
    - [Statements and Expressions](#statements-and-expressions)
    - [Functions with Return Values](#functions-with-return-values)
    - [Comments](#comments)
    - [Control Flow](#control-flow)
    - [Loops](#loops)
- [Chapter 4](#chapter-4)
    - [Ownership](#ownership)
    - [References](#references)
    - [Mutable References](#mutable-references)
    - [Dangling References](#dangling-references)
    - [The Rules of References](#the-rules-of-references)
      - [The Slice Type](#the-slice-type)
      - [Other Slices](#other-slices)
- [Chapter 5](#chapter-5)
    - [Structs](#structs)
    - [Tuple Structs](#tuple-structs)
    - [Unit-Like Structs without Any Fields](#unit-like-structs-without-any-fields)
    - [Methods](#methods)
    - [Methods with more parameters](#methods-with-more-parameters)
    - [Associated Functions](#associated-functions)
- [Chapter 6](#chapter-6)
    - [Enums](#enums)
    - [Option Enum](#option-enum)
    - [The match Control Flow Construct](#the-match-control-flow-construct)
    - [Patterns that bind to values](#patterns-that-bind-to-values)
    - [Matching with `Option<T>`](#matching-with-optiont)
    - [Catch-All Patterns and the \_ Placeholder](#catch-all-patterns-and-the-_-placeholder)
    - [Concise Control Flow with if let](#concise-control-flow-with-if-let)
- [Chapter 7](#chapter-7)
    - [Packages and Crates](#packages-and-crates)
    - [Defining Modules to Control Scope and Privacy](#defining-modules-to-control-scope-and-privacy)
    - [Paths for Referring to an item in a Module Tree](#paths-for-referring-to-an-item-in-a-module-tree)
    - [Starting Relative Paths with super](#starting-relative-paths-with-super)
    - [Making Structs and Enums Public](#making-structs-and-enums-public)
    - [Bringing Paths into Scope with the use Keyword](#bringing-paths-into-scope-with-the-use-keyword)
    - [Re-exporting Names with pub use](#re-exporting-names-with-pub-use)
    - [Using External Packages](#using-external-packages)
    - [Using Nested Paths to Clean Up Large use Lists](#using-nested-paths-to-clean-up-large-use-lists)
    - [The Glob Operator](#the-glob-operator)
    - [Separating Modules into Different Files](#separating-modules-into-different-files)
- [Chapter 8](#chapter-8)
  - [Vectors](#vectors)
    - [Storing Lists of Values with Vectors](#storing-lists-of-values-with-vectors)
    - [Updating a vector](#updating-a-vector)
    - [Reading Elements of Vectors](#reading-elements-of-vectors)
    - [Iterating Over the Values in a Vector](#iterating-over-the-values-in-a-vector)
    - [Using an Enum to Store Multiple Values](#using-an-enum-to-store-multiple-values)
    - [Dropping a Vector Drops Its Elements](#dropping-a-vector-drops-its-elements)
  - [Strings](#strings)
    - [Storing UTF-8 Encoded Text with Strings](#storing-utf-8-encoded-text-with-strings)
    - [What is a String?](#what-is-a-string)
    - [Creating a new String](#creating-a-new-string)
    - [Updating a String](#updating-a-string)
    - [Concat with + or format! Macro](#concat-with--or-format-macro)
    - [Indexing into Strings](#indexing-into-strings)
    - [Bytes and Scalar Values and Grapheme Clusters](#bytes-and-scalar-values-and-grapheme-clusters)
    - [Slicing Strings](#slicing-strings)
    - [Methods for Iterating Over Strings](#methods-for-iterating-over-strings)
  - [Storing Keys with Associated Values in Hash Maps](#storing-keys-with-associated-values-in-hash-maps)
    - [Creating a New Hash Map](#creating-a-new-hash-map)
    - [Accessing Values in a Hash Map](#accessing-values-in-a-hash-map)
    - [HashMaps and Ownership](#hashmaps-and-ownership)
    - [Updating a Hash Map](#updating-a-hash-map)
    - [Overwriting a value](#overwriting-a-value)
    - [Adding a Key and Value Only if a Key Isn't Present](#adding-a-key-and-value-only-if-a-key-isnt-present)
    - [Updating a Value Based on the Old Value](#updating-a-value-based-on-the-old-value)
- [Chapter 9](#chapter-9)
    - [Unrecoverable Errors with panic!](#unrecoverable-errors-with-panic)
    - [Recoverable Errors with Result](#recoverable-errors-with-result)
    - [Matching on Different Errors](#matching-on-different-errors)
    - [Alternatives to Using match with Result\<T, E\>](#alternatives-to-using-match-with-resultt-e)
    - [Shortcuts for Panic on Error: unwrap and expect](#shortcuts-for-panic-on-error-unwrap-and-expect)
    - [Propagating Errors](#propagating-errors)
    - [A Shortcut for Propagating Errors: The ? Operator](#a-shortcut-for-propagating-errors-the--operator)
    - [Where the ? Operator Can Be Used](#where-the--operator-can-be-used)
    - [To panic! or Not to panic!](#to-panic-or-not-to-panic)
    - [Guidelines for Error Handling](#guidelines-for-error-handling)
    - [Creating Custom Types for Validation](#creating-custom-types-for-validation)
- [Chapter 10](#chapter-10)
    - [Removing Duplication by Extracting a Function](#removing-duplication-by-extracting-a-function)
    - [Generic Data Types](#generic-data-types)
      - [In Function Definitions](#in-function-definitions)
      - [In Struct Definitions](#in-struct-definitions)
      - [In Enum Definitions](#in-enum-definitions)
      - [In Method Definitions](#in-method-definitions)
    - [Performance of Code Using Generics](#performance-of-code-using-generics)
    - [Traits: Defining Shared Behavior](#traits-defining-shared-behavior)
      - [Defining a Trait](#defining-a-trait)
      - [Implementing a Trait on a Type](#implementing-a-trait-on-a-type)
      - [Default Implementations](#default-implementations)
      - [Traits as Parameters](#traits-as-parameters)
      - [Trait Bound Syntax](#trait-bound-syntax)
      - [Specifying Multiple Trait Bounds with the + Syntax](#specifying-multiple-trait-bounds-with-the--syntax)
      - [Clearer Trait Bounds with where Clauses](#clearer-trait-bounds-with-where-clauses)
      - [Returning Types That Implement Traits](#returning-types-that-implement-traits)
      - [Using Trait Bounds to Conditionally Implement Methods](#using-trait-bounds-to-conditionally-implement-methods)
    - [Validating References with Lifetimes](#validating-references-with-lifetimes)
      - [Preventing Dangling References with Lifetimes](#preventing-dangling-references-with-lifetimes)
      - [The Borrow Checker](#the-borrow-checker)
      - [Generic Lifetimes in Functions](#generic-lifetimes-in-functions)
      - [Lifetime Annotation Syntax](#lifetime-annotation-syntax)
      - [Lifetime Annotations in Function Signatures](#lifetime-annotations-in-function-signatures)
      - [Thinking in Terms of Lifetimes](#thinking-in-terms-of-lifetimes)
      - [Lifetime Annotations in Struct Definitions](#lifetime-annotations-in-struct-definitions)
      - [Lifetime Elision](#lifetime-elision)
      - [Lifetime Annotations in Method Definitions](#lifetime-annotations-in-method-definitions)
      - [The Static Lifetime](#the-static-lifetime)
      - [Generic Type Parameters, Trait Bounds, and Lifetimes Together](#generic-type-parameters-trait-bounds-and-lifetimes-together)
      - [Summary](#summary)
- [Chapter 11](#chapter-11)
    - [How to Write Tests](#how-to-write-tests)
    - [The Anatomy of a Test Function](#the-anatomy-of-a-test-function)
    - [Checking Results with the `assert!` Macro](#checking-results-with-the-assert-macro)
    - [Testing Equality with the `assert_eq!` and `assert_ne!` Macros](#testing-equality-with-the-assert_eq-and-assert_ne-macros)
    - [Adding Custom Failure Messages](#adding-custom-failure-messages)
    - [Checking for Panics with `#[should_panic]`](#checking-for-panics-with-should_panic)
    - [Using `Result<T, E>` in Tests](#using-resultt-e-in-tests)
    - [Controlling How Tests Are Run](#controlling-how-tests-are-run)
    - [Running Tests in Parallel or Consecutively](#running-tests-in-parallel-or-consecutively)
    - [Showing Function Output](#showing-function-output)
    - [Running a Subset of Tests by Name](#running-a-subset-of-tests-by-name)
    - [Ignoring Some Tests Unless Specifically Requested](#ignoring-some-tests-unless-specifically-requested)
  - [Test Organization](#test-organization)
    - [Unit Tests](#unit-tests)
    - [The Tests Module and #\[cfg(test)\]](#the-tests-module-and-cfgtest)
    - [Testing Private Functions](#testing-private-functions)
    - [Integration Tests](#integration-tests)
    - [The tests Directory](#the-tests-directory)
    - [Submodules in Integration Tests](#submodules-in-integration-tests)
    - [Integration Tests for Binary Crates](#integration-tests-for-binary-crates)
- [Chapter 12](#chapter-12)
    - [Reading the Argument Values](#reading-the-argument-values)
    - [Separation of Concerns for Binary Projects](#separation-of-concerns-for-binary-projects)
    - [Developing the Library’s Functionality with Test-Driven Development](#developing-the-librarys-functionality-with-test-driven-development)
    - [Fixing the Error Handling](#fixing-the-error-handling)
      - [Returning a Result Instead of Calling panic!](#returning-a-result-instead-of-calling-panic)
      - [Calling Config::build and Handling Errors](#calling-configbuild-and-handling-errors)
    - [Extracting Logic from main](#extracting-logic-from-main)
      - [Handling Errors Returned from run in main](#handling-errors-returned-from-run-in-main)
    - [Splitting Code into a Library Crate](#splitting-code-into-a-library-crate)
    - [Developing the Library’s Functionality with Test- Driven Development](#developing-the-librarys-functionality-with-test--driven-development)
      - [Writing a Failing Test](#writing-a-failing-test)
      - [Working with Environment Variables](#working-with-environment-variables)
      - [Implementing the search\_case\_insensitive Function](#implementing-the-search_case_insensitive-function)
    - [Writing Error Messages to Standard Error Instead of Standard Output](#writing-error-messages-to-standard-error-instead-of-standard-output)
    - [Checking Where Errors Are Written](#checking-where-errors-are-written)
    - [Printing Errors to Standard Error](#printing-errors-to-standard-error)
- [Chapter 13](#chapter-13)
  - [Closures](#closures)
    - [Closures: Anonymous Functions That Capture Their Environment](#closures-anonymous-functions-that-capture-their-environment)
    - [Capturing the Environment with Closures](#capturing-the-environment-with-closures)
    - [Closure Type Inference and Annotation](#closure-type-inference-and-annotation)
    - [Capturing References or Moving Ownership](#capturing-references-or-moving-ownership)
    - [Moving Captured Values Out of Closures and the Fn Traits](#moving-captured-values-out-of-closures-and-the-fn-traits)
    - [FnOnce](#fnonce)
    - [FnMut](#fnmut)
    - [Processing a Series of Items with Iterators](#processing-a-series-of-items-with-iterators)
    - [The Iterator Trait and the next Method](#the-iterator-trait-and-the-next-method)
    - [Methods That Consume the Iterator](#methods-that-consume-the-iterator)
    - [Methods That Produce Other Iterators](#methods-that-produce-other-iterators)
    - [Using Closures That Capture Their Environment](#using-closures-that-capture-their-environment)
    - [Improving minigrep](#improving-minigrep)
    - [Removing a clone Using an Iterator](#removing-a-clone-using-an-iterator)
    - [Using Iterator Trait Methods Instead of Indexing](#using-iterator-trait-methods-instead-of-indexing)
    - [Making Code Clearer with Iterator Adapters](#making-code-clearer-with-iterator-adapters)
    - [Choosing Between Loops and Iterators](#choosing-between-loops-and-iterators)
    - [Comparing Performance: Loops vs. Iterators](#comparing-performance-loops-vs-iterators)

# Introduction

* Rust is designed to write fast AND safe code whose compiler works *with* you to help you.
* The Rust compiler catches errors that might occur with low-level development where you're managing your own memory.
* Every two or three years, the Rust team produces a new Rust edition. Each edition brings together the features that have landed into a clear package with fully updated documentation and tooling. Default edition is 2015 if not defined in the cargo.toml file.
* `cargo` is the included dependency manager and build tool that makes adding, compiling, and managing dependencies painless and consistent across the Rust ecosystem.
* `rust-analyzer` is Rust's language server and provides features like IntelliSense.
* `rustc` is the Rust compiler.
* `rustdoc` is the documentation tool.
* `rustfmt` is the formatting tool.
  * `rustfmt` allows finer-grained control and `cargo-fmt` understands conventions of a project that uses Cargo. 
* `rustup` is a command line tool for managing Rust versions and associated tools.
* Clippy is Rust's static analyzer.
* `cargo fix` automatically upgrades your code to a new edition.

# Chapter 1

### Installing Rust
* The version of curl that is installed by homebrew does not support the `-tls` option, so removing it from the default rust install command line fixed the error: "curl: (4) A requested feature, protocol or option was not found built-in in this libcurl due to a build-time decision." 

### Intro to Cargo
* TOML stands for Tom's Obvious, Minimal Language.
* Packages of code are referred to as crates.
  * There's binaary crates and library crates.
* We can create a project using `cargo new`.
* We can build a project using `cargo build`.
  * TO build it for release, we use `cargo build --release`.
* We can build and run a project in one step using `cargo run`.
* We can build a project without producing a binary to check for errors using `cargo check`.
* Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.

# Chapter 2

### The `use` Statement
* By default, Rust has a set of items defined in the standard library that it brings into the scope of every program. This set is called the prelude.
  * If a type you want to use isn’t in the prelude, you have to bring that type into scope explicitly with a `use` statement.

### Cargo
* When we include an external dependency, Cargo fetches the latest versions of everything that dependency needs from the registry, which is a copy of data from Crates.io.
* After updating the registry, Cargo checks the `[dependencies]` section and downloads any crates listed that aren’t already downloaded.
* Cargo.lock is usually checked into source control to ensure reproducible builds.
  * When you build a project for the first time, Cargo figures out all the versions of the dependencies that fit the criteria and then writes them to the Cargo.lock file. When you build your project in the future, Cargo will see that the Cargo.lock file exists and will use the versions specified there rather than doing all the work of figuring out versions again. This lets you have a reproducible build automatically. In other words, your project will remain at 0.8.5 until you explicitly upgrade, thanks to the Cargo.lock file. Because the Cargo.lock file is important for reproducible builds, it’s often checked into source control with the rest of the code in your project.


### Match Expressions
* A `match` expression is made up of arms. An arm consists of a pattern to match against, and the code that should be run if the value given to `match` fits that arm’s pattern. Rust takes the value given to `match` and looks through each arm’s pattern in turn. Patterns and the `match` construct are powerful Rust features: they let you express a variety of situations your code might encounter and they make sure you handle them all.


### Shadowing
* Rust allows us to shadow/reuse variable names rather than forcing us to create two unique variables.

# Chapter 3

### Variables
* Variables are immutable by default and the `mut` keyword can be used to make them mutable.

### Constants
* `const` keyword can be used to declare a constant. Its value cannot change and we cannot use `mut` keyword with `const`s.
  * `const`s aren't just immutable by default, they are always immutable.
* They are declared using the `const` keyword, not `let`.
* We must always annotate the type for constants.
* Constants can only be set to a constant expression, not the result of a value that could only be computed at run time.

### Shadowing
* The second variable overshadows the first, taking any uses of the variable name to itself until either it itself is shadowed or the scope ends.
* Shadowing != making a variable `mut`.
* When we shadow a variable, we can change the type of the value but reuse the same name.
  * But if we try to use the same name with `mut` and have a different type in the variable that is shadowing, we'll get a mismatched types compiler error.

### Scalar Data Types
* Rust is statically typed language, which means we must know the types of all variables at compile time.
  * ```Rust
    // we need to mention the type annotation `u32` otherwise the compiler won't know which type to parse "42" into
    let guess: u32 = "42".parse().expect("Not a number!");
    ```
* Integers
  * Unsigned integers; `u8`, `u16`, ..., `u128`, `usize`
  * Signed integers: `i8`, `i16`, ..., `i128`, `isize`
  * We can have Integer literals:
    * Decimal 98_000
    * Hex 0xff
    * Octal 0o77
    * Binary 0b1111_0000
    * Byte (u8 only) b'A'
  * `i32` is the default
  * We might use `isize` or `usize` when indexing some sort of collection.
  * Integer overflow: If our program results in an integer overflow in debug mode, our program will panic, but in release mode, Rust will perform two's complement wrapping so our program won't panic, but we might also not get the value we were expecting.
* Floating-point numbers
  * `f32` and `f64` (default)
  * All floating-point numbers are signed.
* Booleans
  * 1 byte in size; `true` or `false`
* Characters
  * Specified by single quotes; 4 bytes in size
  * They represent scalar values, which means it can represent a lot more than ASCII: accented letters, Chinese, emojis, etc.

### Compound Data Types
* Tuples
  * Tuples have a fixed size that cannot change once defined.
  * Each value in the tuple can be of a different type.
  * We can destructure a tuple or access a particular value at an index in the tuple.
  * The tuple without any values is called a "unit".
* Array
  * All values in the array must be of the same type.
  * Data is allocated on the stack.
  * An array is of fixed size too.
    * Vector type can grow and shrink.
  * We can initialize each element of an array to the same value as well.
  * If you access an out-of-bounds index from an array at runtime, Rust panics and gives an error instead of letting you access the memory you shouldn't be accessing.

### Functions
* Rust convention is to use snake_case for function and variable names.
* Rust doesn't care where you define your functions, as long as you define them somewhere in the scope of the caller.

### Parameters
* Function's signature has the following structure (parameter: type) as opposed to (type parameter) in some languages
* We must declare the type of each parameter.

### Statements and Expressions
* Statements are instructions that perform some action and do not return a value.
* Expressions evaluate to a resultant value.
  * Expressions do not include ending semicolons
  * ```Rust
    let y = {
      let x = 3;
      x + 1
    }
    ```

### Functions with Return Values
* We need to specify the return type after the `->` in the function signature.
* We can just have expressions in the function body without even a `return` or a `;` and that expression would be evaluated and the result be returned.

### Comments
* `//` idiomatic comment style
* Rust also supports documentation comments.

### Control Flow
* `if` Expressions
  * Standard `if`, `else`, and `else if`
  * We can use `if` in a `let` statement
  * Blocks of code associated with the conditions in `if` expressions are called arms,
    but each arm's value must be of the same type.
  * Rust will not evaluate a non-boolean to a boolean like so:
    ```Rust
    if number % 2 { };
    // So, we must do the following
    if number % 2 == 0 { };
    ```

### Loops
* `loop`
  * We can return values from `loop` by adding that expression after the `break` expression; see ch3/ch3/src/main.rs
  * For loops within loops, `break` and `continue` apply to the innermost loop by default.
  * We can have a loop label to specify a certain `break` or `continue` applies to which loop instead of the innermost one.
  * Loop labels begin with a `'`.
  
* `while`
  * conditional loop
  * can be implemented using `loop`, `if`, `else`, and `break` but `while` is much more common and better.
  
* `for`
  * conditional loop
  * loop through a collection like an array. We can do this using `while` as we check for the index being <= our array's length, but `for` is cleaner, better, and safer.

# Chapter 4

### Ownership
* Memory in Rust is managed through a system of ownership with a set of rules that the compiler enforces.
* Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so we don't run out of space are all problems that ownership addresses.
* Rules:
  * Each value in Rust has an owner
  * There can only be one owner at a time
  * When the owner goes out of scope, the value will be dropped
* `String` type can be mutated but String literals cannot and that is because of the difference in how these two types deal with memory (stack vs heap).
  * When the size of a string is known as compile time, like literals, they are hardcoded in the final executable, so their size cannot change.
  * When the size of a String is unknown at compile time and can change during the program's execution, they are allocated on the heap during runtime.
* Some languages have a garbage collector and some require the programmer to manage the memory (`allocate` and `free`). In Rust, the memory is automatically returned once the variable that owns it goes out of scope, using a function called `drop`.
* When we do something like the following:
    ```Rust
    let s1 = String::from("Hello");
    let s2 = s1;
    ```
  The data from the stack is `move`d into s1 instead of making a shallow copy. This means that when s1 and s2 go out of scope, Rust doesn't try to `drop` the same memory twice. This also means that after `let s2 = s1;`, we cannot use `s1` anymore.
* Rust never creates deep copies of data automatically, instead we use `clone()` to make a deep copy, which is more expensive than `move`.
* For data types with known size at compile type (e.g. integers), we don't have to call clone and nothing is moved into the new variable but a copy of the value is made. This copy is inexpensive since we already know the size of the variable at compile time and the values are stored on the stack.
  * `Copy` trait can be placed on types that are stored on the stack that makes the variable's values being copied instead of `move`d.
  * We cannot add `Copy` trait to types that implement the `Drop` trait.
* Passing a variable to a function will move or copy, just as assignment does.
* Returning values can also transfer ownership.
* The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it. When a variable that includes data on the heap goes out of scope, the value will be cleaned up by `drop` unless ownership of the data has been moved to another variable.

### References
* Taking ownership and then returning ownership with every function is tedious.
* References let us let a function use a value but not take ownership.
* A reference is like a pointer in that it's an address we can follow to access the data stored at that address; the data is owned by some other variable.
* Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.
*  A reference's scope starts where it is introduced and continues through the last time that reference is used.
* Because a function that takes in a reference does not have ownership to what was passed into it, the value pointed to by the reference is not dropped when the reference is last used.
* Just as variables are immutable by default, so are references.

### Mutable References
* Using `mut`, we can make references mutable.
* Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value AT THE SAME TIME. We cannot borrow a reference as mutable more than once or have a mutable and a non-mutable reference at the same time.
* Having this restriction prevents data races at compile time.
* Data races occur when:
  * Two or more pointers access the same data at the same time.
  * At least one of the pointers is being used to write to the data.
  * There’s no mechanism being used to synchronize access to the data.
* We can create a new scope using {} to allow multiple mutable references, just not simultaneous ones.
* We also cannot have a mutable reference while we have an immutable one to the same value -- Users of an immutable reference do not expect the value to suddenly change out from under them!
* Multiple immutable references are allowed because no one who is just reading the data has the ability to change the value and affect anyone else's reading of data.
* If scopes of references don't overlap, we can borrow a value with a mutable reference after the immutable reference's scope ends.
* **Important: At any given time, you can have either one mutable reference or any number of immutable references.**
  
### Dangling References
* In languages with pointers, it is easy to mistakenly create a dangling pointer by freeing some memory while preserving a pointer to that memory, but not in Rust!

### The Rules of References
#### The Slice Type
* Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.
* A slice is a kind of a reference, so it does not have ownership.
* `iter()` is a method that returns each element in a collection and `enumerate()` wraps the result of `iter()` and returns each element as part of a tuple. The first element is the index and the second is a reference to the element.
* `first_space_index()` in main.rs is a good solution but since the return value `usize` is separate from the input `string`, there's no guarantee that it will still be valid in the future. Look at the caller of `first_space_index()`. In other words, `usize` isn't tied to the state of the `string` that could lead to bugs.
* Rust has a solution to this problem: string slices, which is a reference to part of a string: `&string[starting_index..ending_index]`, where `ending_index` is one more than the index you want the slice to end.
* Using the slice version `_string_slice()` will throw a compile error helping us catch the problem at compile time since we now return `&str` instead of a `usize`.
* String slices' type is `&str`.
* `[0..2]` and `[..2]` are equal. You can drop the 0 if you want to start at 0 with this range syntax. `[3..]` means 3 onwards to the last index and `[..]` means the slice of the entire string.
* **Note:** if we try to create a slice in the middle of multibyte character, the program will exit with an error.
* We can improve the signature from `fn string_slice(string: &String) -> &str` to `fn string_slice(string: &str) -> &str`.
  * If we have a string slice, we can pass that directly instead of having to convert to a `String` with `to_string()`.
  * If we have a String, we can pass a slice of that String or a reference to the String, since Strings are `str`s.
  * This flexibility takes advantage of "deref coercions" -- covered later.
  * Defining a function to take a string slice instead of a reference to a String makes our API more general.
* String literals are string slices already.

#### Other Slices
*  There's a more general type of slice as well.
*  We can slice an array of `i32`
   *  ```Rust
      let a = [1,2,3];
      let slice = &a[0..2]; // slice will be [1,2] and of type &[i32]
      ```

# Chapter 5

### Structs
* Structs allow us to structure custom, related values into a meaningful group.
* Like in tuples, the values in structs can have different data types.
* Unlike in tuples, we can name these groups of values so we don't have to rely on the order of the values.
* We create an `instance` of a `struct` using `{}` and the ending curly brace has  a `;`.
* To access a specific value in the struct, we use the dot notation.
* If the instance is mutable, we can use dot notation to change the value in that instance.
* The entire instance should be mutable; Rust doesn't allow certain fields to be mutable.
* As with any expression, we can construct a new instance of the struct as the last expression in the function body to implicitly return a new instance.
  * It makes sense to name the function parameters with the same name as the struct fields, but if there's many fields in the Struct, this can be tedious.
  * There's a solution: "Field Init Shorthand"
    * If the parameters are exactly the same as the struct field names, we can use the field init shorthand. See `build_user_2`.
* Sometimes, we want to create a new instance of a struct that includes most of the values from another instance but changes some. We can do this with the "Struct Update Syntax"
    * ``` Rust
      let user2 = User {
        email: String::from("new@test.com"),
        ..user1 // this must come last
      };
      ```
  * But now, data from `user1` has moved into `user2` so we can no longer use `user1`. Specifically, the `String` in `user1` was moved into `user2`.
  * If we had used new values for both `email` and `username` and thus only used values for `active` and `sign_in_count` from `user1` to construct `user2`, then `user1` would still be valid because that data implements the `Copy` trait and "Stack-Only Data: Copy" occurs for them.
* The `println!` macro uses `Display` formatting as the default when we use `{}`. This is output intended for direct end user consumption.
  * The primitive types implement `Display` by default.
  * Using `{:?}` says we want to use an output format called `Debug`, which we can add using `#[derive(Debug)]` on the Struct. Rust includes functionality for us to print out debugging information but we need to opt-in to use it.
    * `{:#?}` will print the output a little more prettier.
  * Or we can use the `dbg!` macro too
    * It takes ownership of an expression (as opposed to `println!` that takes a reference), prints the file and line number of where that `dbg!` macro call occurs along with the resultant value of that expression and returns ownership of the value.
    * It prints to the `stderr`.
    * We can do something like the following since `dbg!` returns ownership, `radius` would have the same value as it would without `dbg!`.
      * ```Rust 
        let circle = Circle(radius: dbg!{1 * scale});
        ```

### Tuple Structs
* Tuple structs have the added meaning the struct name provides, but they don't have names with each field in the struct; rather each field just has types.
* Tuple structs are useful when you want to give the whole tuple a name and make the tuple a different type from other tuples.
* Each tuple struct is its own type even when the types of the fields in structs might be the same.
* We can use the index after the `.` to access each field.

### Unit-Like Structs without Any Fields
* Unit-like structs are structs that don't have nay fields.
* They behave similarly to `()`: the unit type.
* They are useful when you need to implement a trait on some type but don't have any data that you want to store in the type itself.
* We don't need `{}` when defining or instantiating unit-like structs.


### Methods
* Our `area_struct()` function in rectangles/src/main.rs is specific to Rectangles.
* Methods are similar to functions: we declare them with `fn` and a name, they can have parameters and a return value, and they run some code when the method is called from somewhere.
* Unlike functions, methods are defined within the context of a struct (or an enum or a trait object), and their first parameter is always `self`, which represents the instance of the struct the method is being called on.
* To define a method within the context of a struct etc, we use the `impl` block. Each struct is allowed to have more than 1 `impl` blocks.
* Methods can take ownership of `self`, borrow `self` immutably, or borrow `self` mutably, just ask they can any other parameter.
* `&self` is short for `self: Self` and within an `impl` block, the type `Self` is an alias for the type that the `impl` block is for. 
  * Methods must have a parameter named `self` of type `Self` for their first parameter, so Rust lets us abbreviate this with only the first name `self` in the first parameter spot.
  * `&self` in the method parameter means we don't want to take ownership; we want to just read the data.
  * `self` would mean we would take ownership.
  * `&mut self` would mean we'd take ownership to modify.
* We can choose to give the method the same name as one of our fields. These are usually used as getters. Like `rect1.width()`.
* `->`
  * In C and C++, we use `.` if we're calling a method on the object and `->` if we're calling a method on the pointer to an job and need to dereference it first. Like:
    ```C++
    // these 2 are the same in C++, but 1 is cleaner
    object->something();
    (*object).something();
    ```
  * Rust has something called "automatic referencing and dereferencing". When we call a method with `object.something()`, Rust automatically adds `&`, `&mut`, or `*` so `object` matches the signature of the method. Like:
    ```Rust
    // these 2 are the same in Rust, but 1 is cleaner
    p1.distance(&p2);
    (&p1).distance(&p2);
    ```
  * Given the receiver and the name of the  method, Rust can figure out whether the method is reading `&self`, mutating (`&mut self`), or consuming (`self`).

### Methods with more parameters
* See rectangles/src/main.rs

### Associated Functions
* All functions within an `impl` block are called associated functions since they're associated with the type names after `impl`.
* We can define associated functions that don't have `self` as their first parameter (and thus not methods) because they don't need an instance of the type to work with.
* Associated functions that aren't methods are often used for constructors that will use a create instance of the struct.

# Chapter 6

### Enums
* Enums allow you to define a type by enumerating its possible variants. It is a way of saying that a value is a one of a possible set of values.
* Variants of the enum are namespaced under its identifier, and we use `::` to separate the two. All values in an enum are of the same type.
* Rather than an enum inside a struct, we can put data directly into each enum variant. Because we attach data to each variant of the enum directly, there is no need for an extra struct.
  * This way, the name of each enum variant that we define also becomes a function that constructs an instance of that enum.
  * This has another advantage over using the enum in the struct: each variant can have different types and amounts associated data. E.g. in our `IpAddr` example, `V4` can have 4 numeric components between 0 and 255, while `V6` can still be constructed with a String.
  * You can put any kind of data inside an enum variant: strings, numeric types, structs, or even another enum. Look at `enum Message` in ch6/src/main.rs.
* We're also able to define methods on enums using `impl` as we are with structs.

### Option Enum
* The `Option<T>` type encodes the very common scenario in which a value could be something or could be nothing. It is another enum defined by the standard library.
* Rust doesn't have the null feature because if you try to use a null value as a non-null value, it would lead to errors.
  * But Rust does have the concept of a value being present or absent.
* `Option<T>` is included in the prelude and its variants `Some` and `None` that can be used **without** `Option::`.
* When we define an `Option` with `None` as its initial value, we need to give the variable an explicit type where the parameter `T` is specified so that the compiler knows what the type of variable should be.
* If we try to use `Option<T>`, even if it has a value `Some`, the compiler won't let us until we handle the case where that value could be `None`. We can do this using `match`.
* We have to convert an `Option<T>` to `T` before we can use it.

### The match Control Flow Construct
* `match` allows us to compare a value against a series of patterns and execute code based on which pattern matches.
* Patterns can be literal strings, variable names, wildcards, etc.
* `match` arms have 2 parts: a pattern and some code separated by `=>`. The arms' patterns must cover **all** possibilities.

### Patterns that bind to values
* Another useful feature of match is that they can bind to the parts of the values that match the pattern.
  * ```Rust
    enum Coin {
        Penny,
        Nickel,
        Dime,
        // binded to a UsState type
        Quarter(UsState),
    }
    ```

### Matching with `Option<T>`
* Look at ch6/src/main.rs

### Catch-All Patterns and the _ Placeholder  
* Using enums, we can also take special actions for a few particular values, but for all other values take one default action.
  * We have to put the catch-all arm at the end since the arms of `match` are evaluated in order. If we put the catch-all arm in the beginning, none of the other arms will ever be executed.
  * We can use `_` if we don't want to use the value in the catch-all pattern or we can assign any name to the value and use it as we want.
  * `_` matches any value and does not bind to that value telling Rust we aren't going to use the value.
  * We can use the unit value `()` to indicate we don't want to do anything.

### Concise Control Flow with if let
* `if let` can be useful in situations where `match` is a bit wordy as it lets us handle values that match one pattern while ignoring the rest.
* `if let` prevents us from satisfying the `match` expression by adding `_ => ()`.
* `if let` means less boilerplate code, but we lose `match`'s exhaustive checking.
* We can think of `if let` that runs code when the value matches one pattern and ignores all other values.

# Chapter 7

* As our project grows, you should organize it by splitting it into multiple modules and then multiple files.
* A package can contain multiple binary crates and optionally one library crate.
* As a package grows, you can extract parts into separate crates that become external dependencies.
* For very large projects comprising a set of interrelated packages that evolve together, Cargo provides workspaces.
* Rust has a number of features that allow you to manage your code's organization, including which details are exposed, which details are private, and what names are in each scope. These features are sometimes collectively referred to as the module system & include packages, crates, modules and use, and paths.

### Packages and Crates
![Workspace, Package, Crates, Modules](https://static.packt-cdn.com/products/9781800560963/graphics/image/Figure_1.1_B16405.jpg)

* Packages are a Cargo feature that lets you build, test, and share crates.
  * A package is a bundle of one or more crates that provides a set of functionality.
  * A package contains a Cargo.toml file that describes how to build those crates.
  * Cargo is actually a package that contains the binary crate for the command line tool you've been using to build those crates.
  * The Cargo package also contains a library crate that the binary crate depends on.
  * Other projects can depend on the Cargo library crate to use the same logic the Cargo command line tool uses.
  * A package can contain as many binary crates as you like, but at most one library crate.
  * A package must contain at least one crate whether it is a library crate or a binary crate.
  * Running `cargo new x` gives us a package named x.
  * A package can have multiple binary crates by placing files in src/bin directory: each file will be a separate binary crate.
* A Crate is a tree of modules that produce a library or executable.
  * A crate is the smallest amount of code that the Rust compiler considers at a time.
  * Even if we run `rustc` instead of `cargo` and pass in a single file, it is treated as a crate.
  * Crates can contain modules and the modules may be defined in other files that get compiled with the crate.
  * A crate can come as a binary crate or a library crate.
    * Binary crates are programs you can compile to an executable that you can run.
      * Each must have a `main` function.
    * Library crates don't have a `main` function and they don't compile to an executable, but define functionality intended to be shared with multiple projects. E.g. `rand` crate.
  * The crate root is a source file that the Rust compiler starts from and makes up the root of the module of your crate.
  * Cargo follows a convention that src/main.rs is the crate root of a binary crate and src/lib.rs is the crate root for a library crate. The package name is used as the name for the executable and library crate that are built.
  * Cargo passes the crate root to `rustc` to build the binary or library.

* We can create a library crate using `crate new <name> --lib`
  * The contents of lib.rs form a module named crate at  the root of the crate's module structure known as the module tree like:
  ![Tree](assets/images/Tree.png)
  * modules `hosting` and `serving` are siblings since they're defined within `front_of_house`.
  * module `hosting` is the child of module `front_of_house` and it in turn is the parent of `hosting`.

### Defining Modules to Control Scope and Privacy
* Modules (defined with `mod`) let us organize code within a crate for readability and easy reuse.
* Modules also allow us to control the privacy of items because code within a module is private by default.
  * Private items are internal implementation details not available for outside use.
  * We can make modules and the items in them public using `pub mod`, which exposes them to allow external code to use and depend on them.
* In Rust, all items (functions, methods, structs, enums, modules, and constants) are private to parent modules by default. If you want to make an item like a function or struct private, you put it in a module.
* Items in a parent module can’t use the private items inside child modules, but items in child modules can use the items in their ancestor modules.
  * This is because child modules wrap and hide their implementation details, but the child modules can see the context in which they’re defined.
* Making a module public with `pub` doesn't make its contents public.
* Since `eat_at_restaurant()` and `front_of_house` in `customer.rs` are defined in the same module (they're siblings), we don't have to add `pub` to `front_of_house` for `eat_at_restaurant()` to see it.

### Paths for Referring to an item in a Module Tree
* To show Rust where to find an item in a module tree, we use a path.
* A path can be of two forms, but both are followed by one or more identifiers followed by `::`.
  * An absolute path is the full path starting from a crate root
    * For code from an external crate, it begins with the crate name
    * For code from the current crate, it begins with `crate`
  * A relative path starts from the current module and uses `self`, `super`, or an identifier in the current module.


### Starting Relative Paths with super
* We can construct relative paths that begin in the parent module, rather than the current module or crate root by using `super` at the path's start.

### Making Structs and Enums Public
* We can also use `pub` to designate structs and enums as public, but there are a few extra details to the usage of `pub` with structs and enums. 
  * If we use `pub` before a struct definition, we make the struct public, but the struct’s fields will still be private. We can make each field public or not on a case-by-case basis.
    * Since `back_of_house::Breakfast` has a private field, the struct needs to provide a public associated function that constructs an instance of `Breakfast` (`summer` in our case), otherwise we wouldn't be able to construct an instance of `Breakfast` in `eat_at_restaurant` because we couldn't set the value of the private `seasonal_fruit` field in `eat_at_restaurant`.
  * In contrast, if we make an `enum` public, all of its variants are then public.

### Bringing Paths into Scope with the use Keyword
* `use` lets us bring a module into scope once and then use another, shorter name everywhere else in the scope.
* `use` only creates a shortcut for the particular scope in which the `use` occurs.
* Bringing the function’s parent module into scope with `use` means we have to specify the parent module when calling the function. Specifying the parent module when calling the function makes it clear that the function isn’t locally defined while still minimizing repetition of the full path.
* On the other hand, when bringing in structs, enums, and other items with `use`, it’s convention to specify the full path.
* Because Rust doesn't allow bringing 2 items with the same name into scope with `use`, we bring two types with the same name into the same scope using their parent modules or we can use `as` to specify a new local name for the type(s).

### Re-exporting Names with pub use
* When we bring a name into scope with `use`, the name is private to the scope.
* To enable the code that calls our code to refer to that name as if it had been defined in that code's scope, we can combine `pub` and `use`. This is called re-exporting since we're bringing an item into scope but also making that item available for others to bring into their scope.
* In module `customer`, if we didn't use `pub` in `pub use crate::front_of_house::hosting;`, external code would have to call `add_to_waitlist()` by using the path `restaurant::front_of_house::hosting::add_to_waitlist()`, but now it can just use `restaurant::hosting::add_to_waitlist()`.
* With `pub use`, we can write our code with one structure but expose a different structure (like here, external code doesn't have to think about `front_of_house`).


### Using External Packages
* We can add the package name and version in our Cargo.toml file which tells Cargo to download the package and any dependencies and make it available to our project.
* Then, we bring the definitions in the package into scope we use `use`.

### Using Nested Paths to Clean Up Large use Lists
* `use std::{cmp:Ordering, io}` instead of `use std::cmp::Ordering` and `use cmp::io` on separate lines.

### The Glob Operator
* If we want to bring all public items defined in a path into scope, we can specify that path followed by the `*` operator like `use std::collection::*;`, but this can make it harder to tell what names are in scope.

### Separating Modules into Different Files
* If we put `hosting.rs` in the src directory, the compiler would expect the hosting.rs code to be in a `hosting` module declared in the crate root, and not declared as a child of the `front_of_house` module.
* Since `hosting` and `serving` are childs of `front_of_house`, we put them in a directory `src/front_of_house`.
* The compiler's rules for which files to check for which modules' code mean the directories and files more closely match the module tree.

# Chapter 8
* The data that collections point to is stored on the heap.


## Vectors

### Storing Lists of Values with Vectors
* `Vec<T>` allows you to store more than one value in a single data structure that puts all the values next to each other in memory.
* They store values of the same type.
* `let v: Vec<i32> = Vec::new()` creates a new, empty vector of type `i32` mentioned by the required type annotation to tell the compiler what data type will be stored in this vector.
* If we create the vector with initial values, we don't need the type annotation like `let v = vec![1,2,3];`.

### Updating a vector
* After making the vector `mut`, we can use `push()` to insert values in it.

### Reading Elements of Vectors
* We can use `v[index]` or `v.get(index)` to get the value at an index.
  * Out of bounds access with `v[index]` causes the program to panic.
  * Since `v.get(index)` gives us an Option, out of bounds access will give us a `None` without panicking.
  
### Iterating Over the Values in a Vector
* The rule that we can't have both mutable and immutable references to a vector at the same time applies here too.
  * **Note:** `.push()` takes a mutable borrow of the vector so we cannot use `push()` while having a immutable reference to an element in the vector like `let first = &v[0]`.
* `for i in &v {}` can be used to iterable over the elements in the vector.
  * `for i in &mut v {}` to iterate over mutable references to each element.
    * To change the value that the mutable reference refers to, we have to dereference using `*` first.
* Because of the reference to the vector that the `for` loop holds prevents us from removing or inserting items in the `for` loop body.

### Using an Enum to Store Multiple Values
* Variants of an enum are defined as the same type so when we need one type to represent elements of different types, we can define and use an enum.
* Using an enum plus a `match` expression means Rust will ensure at compile time that every possible case is handled.
* If you don't know the exhaustive set of types a program will get at runtime to store in a vector, the enum technique won't work, but we can use a Trait object.

### Dropping a Vector Drops Its Elements
* Like any other `struct`, a vector is freed when it goes out of scope.
* When the vector gets dropped, all of its contents are also dropped, meaning the integers it holds will be cleaned up.

## Strings

### Storing UTF-8 Encoded Text with Strings
* Strings are implemented as a collection of bytes, plus some methods to interpret those bytes as text.

### What is a String?
* The `String` type, which is provided by Rust’s standard library rather than coded into the core language as `str`, is a growable, mutable, owned, UTF-8 encoded string type.

### Creating a new String
* Many of the same operations available with `Vec<T>` are available with `String` as well because `String` is actually implemented as a wrapper around a vector of bytes with some extra guarantees, restrictions, and capabilities.
* We can use `String::from("string literal")` or `"string_literal".to_string()`.

### Updating a String
* `"hello".push_str("append this");` appends a String to the end of a String.
* `"hello".push('!');` appends only one character to the end.

### Concat with + or format! Macro
* We can use `+` to concat Strings. The `+` operator uses the `add()` method whose signature looks like `fn add(self, s: &str) -> String`.
* We can only add a `&str` to a `String`; we can’t add two `String` values together. But the type of `&s2` in main.rs is `&String`, not `&str`, as specified in the second parameter to `add`. So why does it compile?
  * The compiler can coerce the `&String` argument into a `&str`. When we call the `add` method, Rust uses a deref coercion, which turns `&s2 `into `&s2[..]`.
  * `&` with `str` in `add` shows `add` doesn't take ownership of `s2`. But since there's no `&` with `self`, ownership is transferred to `add` and `s1` can no longer be used.
* For combining Strings in complicated ways, we can use the `format!` macro.
  * `format!` works like `println!` but instead of printing to the console, it returns the String with the contents.
  * `format!` doesn't take ownership either.

### Indexing into Strings
* Accessing a character in a String by index will result in an error in Rust because `String` doesn't implement the `Index<{integer}>` trait.
* This is because Rust Strings are encoded in UTF8 and each character might not be 1 byte and `string[0]` might result in an unexpected value.
  
### Bytes and Scalar Values and Grapheme Clusters
* Another point about UTF-8 is that there's actually 3 ways to look at strings in Rust: as bytes, scalar values, and grapheme clusters/letters.
* ![screenshot](assets/images/image.png)

### Slicing Strings
* Indexing into a string is often a bad idea since we dk what the return type would be: a byte value, a character, a grapheme cluster, or a string slice.
* So, Rust allows us to use `[]` with a range to create a string slice containing particular bytes. To get the first 4 bytes:
    ```Rust
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // s will be Зд
    ```

### Methods for Iterating Over Strings
* The best way to operate on pieces of strings is to be explicit about whether you want characters or bytes.
* For individual Unicode scalar values, use the `.chars()` method.
* Or, we can use `.bytes()` to return each raw byte. Remember Unicode can take more than 1 byte for a character.

## Storing Keys with Associated Values in Hash Maps
* `HashMap<K, V>` stores a mapping of keys of type `K` to values of type `V` using a hashing function.
* We need to bring it in scope first: `use std::collections:HashMap;`.
* By default, HashMap uses SipHas for its hashing function. It provides resistance to DoS attacks involving hash tables.
* We can switch the hashing algorithm by providing our own hasher, which is a type that implements `BuildHasher`.

### Creating a New Hash Map
* To create an empty hash map: `let scores = HashMap::new()`;

### Accessing Values in a Hash Map
* We can get the value out by providing the key to the `get()` method.
* If there's no value for a key, `None` will be returned.
* Our program in main calls `copied()` to get `Option<i32>` instead of `Option<&i32>`, then `unwrap_or()` with a default value in case the key doesn't exist in the map.
* We can iterate over the map as well:
  ```Rust
  for (key, value) in &scores {
    // do whatever
  }
  ```

### HashMaps and Ownership
* When using `insert` for example:
  * For types that implement the Copy trait like `i32`, the values are copied in to the hash map.
  * For types that do not implement Copy trait like `String`, the values will be moved into the hashmap and the hashmap will be the owner of those values.
* If we inserted references to values into the hashmap, the values won't be moved into the hashmap. The values that the references point to must be valid for at least as long as the hashmap is valid.

### Updating a Hash Map
* Each unique key can only have one value associated with it at a time, but not vice versa.
* When you want to change the data in a hash map, you have to decide how to handle the case when a key already has a value assigned.
  * You could replace the old value with the new value, completely disregarding the old value.
  * You could keep the old value and ignore the new value, only adding the new value if the key doesn’t already have a value. 
  * Or you could combine the old value and the new value.

### Overwriting a value
* If we insert a key and a value into a hash map and then insert that same key with a different value, the value associated with that key will be replaced.

### Adding a Key and Value Only if a Key Isn't Present
* `entry` takes the key you want to check. It returns an enum `Entry` that represents a value that might or might not exist.
* `scores.entry(String::from("Black")).or_insert(9);`
* `or_insert()` returns a mutable reference to the value for the corresponding `Entry` key if the key exists. If it doesn't exist, it inserts the parameter as the new value for this key and returns a mutable reference to the new value.

### Updating a Value Based on the Old Value
* See main.rs.

# Chapter 9
* In many cases Rust requires you to acknowledge the possibility of an error and take some action before your code will compile.
* Rust groups errors into two major categories: recoverable and unrecoverable errors.
  * For recoverable errors, we most likely want to report the problem to the user and retry the operation.
  * Unrecoverable operations are always symptoms of bugs, like accessing a location beyond the end of an array, so we immediately stop the program.
* Rust doesn't have exceptions.
  * Instead, it has the type `Result<T, E>` for recoverable errors and the `panic!` macro that stops execution when the program encounters an unrecoverable error.

### Unrecoverable Errors with panic!
* Rust has the `panic!` macro for when bad things happen in your code, and there’s nothing you can do about it.
* There are two ways to cause a panic:
  * by taking an action that causes our code to panic (like accessing an array past the end)
  * by explicitly calling the `panic!` macro. 
  * By default, these panics will print a failure message, unwind, clean up the stack, and quit.
* Since unwinding (walking up the stack) and cleaning up each frame when a panic occurs is too much work, Rust lets us choose an alternate: aborting, which ends the program without cleaning up.
  * The program's memory then needs to be cleaned by the operation system.
  * To make the resultant binary as small as possible, you can switch from unwinding to aborting upon a panic by adding `panic = 'abort'` to the appropriate `[profile]` sections in your Cargo.toml file. E.g., to abort on panic in release mode, add this:   
  `[profile.release]`  
  `panic = 'abort'`
* A backtrace is a list of all the functions that have been called to get to this point.
  * We read backtraces from the top and read until you see files you wrote & that's where the problem is.
  * We set environment variable `RUST_BACKTRACE` to 1 to see the backtrace like `RUST_BACKTRACE=1 cargo run`.
  * We need debug symbols to be enabled & they are by default when using `cargo run` without the `--release` flag.

### Recoverable Errors with Result
* An example of a recoverable error: if you try to open a file and the file doesn’t exist, you might want to create the file instead of terminating the process.
* Recall `Result` enum with variant `Ok(T)` and `Err(E)`
  * When opening a file, the generic parameter `T` is filled by the implementation of `File::open()` with the type of the success value, `std::fs::File`, which is a file handle.
  * The type of `E` used in the error value is `std::io::Error`.

### Matching on Different Errors
* Code from the previous section will panic no matter why we couldn't open the file, but we might want to take different actions for different failure reasons:
  * If `File::open` failed because the file doesn’t exist, we want to create the file and return the handle to the new file. 
  * If `File::open` failed for any other reason (e.g. we didn’t have permission to open the file) we still want the code to `panic!`.
  * The standard library provides `std::io::ErrorKind` with variants for different errors from an `io` operation.

### Alternatives to Using match with Result<T, E>
* We can use other ways like closures to save us from lot of indented code.
* Closures covered later.

### Shortcuts for Panic on Error: unwrap and expect
* Using `match` works well, but it can be a bit verbose.
* The `Result<T, E>` type has many helper methods defined on it to do various, more specific tasks.
* The `unwrap()` method is a shortcut method.
  * If the `Result` value is the `Ok` variant, `unwrap()` will return the value inside the `Ok`.
  * If the `Result` is the `Err` variant, `unwrap()` will call the `panic!` macro for us.
* The `expect()` method lets us also choose the `panic!` error message.
  * `expect()` is used in the same way as `unwrap()`: to return the file handle or call the `panic!` macro.
  * The error message used by `expect()` in its call to `panic!` will be the parameter that we pass to `expect()`, rather than the default `panic!` message that `unwrap()` uses.

### Propagating Errors
* Instead of handling the error within the function itself, you can return the error to the calling code so that it can decide what to do.

### A Shortcut for Propagating Errors: The ? Operator
* Propagating errors is so common in Rust that Rust provides the question mark operator `?` to make this easier.
* The `?` placed after a `Result` value is defined to work in almost the same way as the `match` expressions.
  * If the value of the `Result` is an O`k, the value inside the `Ok` will get returned from this expression, and the program will continue. 
  * If the value is an `Err`, the `Err` will be returned from the whole function as if we had used the return keyword so the error value gets propagated to the calling code.
* There is a difference between what the `match` expression does and what the `?` operator does:
  * Error values that have the `?` operator called on them go through the `from` function, defined in the `From` trait in the standard library, which is used to convert values from one type into another. 
  * When the `?` operator calls the `from` function, the error type received is converted into the error type defined in the return type of the current function.
  * This is useful when a function returns one error type to represent all the ways a function might fail, even if parts might fail for many different reasons.
  * E.g. we could change the `read_username_from_file_short()` function to return a custom error type named `OurError`.
    * If we also define `impl From<io::Error>` for `OurError` to construct an instance of `OurError` from an `io::Error`, then the `?` operator calls in the body of `read_username_from_file_short()` will call `from` and convert the error types without needing to add any more code to the function.
* We can even shorten code further by chaining method calls immediately after the `?`.
* Reading a file into a string is a fairly common operation, so the standard library provides the convenient `fs::read_to_string` function that opens the file, creates a new `String`, reads the contents of the file, puts the contents into that `String`, and returns it.

### Where the ? Operator Can Be Used
* The `?` operator can only be used in functions whose return type is compatible with the value the `?` is used on.
  * If we use `?` in a function whose return type isn't compatible, we will get a compile time error.
  * `?` can only be used in a function that returns `Result`, `Option`, or another type that implements `FromResidual`.
  * To fix the error, you have two choices:
    * Change the return type of your function to be compatible with the value you’re using the `?` on as long as you have no restrictions preventing that. 
    * Use a `match` or one of the `Result<T, E>` methods to handle the `Result<T, E>` in whatever way is appropriate.
* As with using `?` on `Result`, you can only use `?` on `Option` in a function that returns an `Option`.
  * The behavior of `?` when called on an `Option<T>` is similar to when called on a `Result<T, E>`:
    * if the value is `None`, the `None` will be returned early from the function at that point.
    * If the value is `Some`, the value inside the `Some` is the resultant value of the expression, and the function continues.
* You can use `?` on a `Result` in a function that returns `Result`, and you can it on an `Option` in a function that returns `Option`, but you can’t mix and match.
  * `?` won’t automatically convert a `Result` to an `Option` or vice versa.
  * In those cases, you can use methods like the `ok` method on `Result` or the `ok_or` method on `Option` to do the conversion explicitly.
* `main` can also return a `Result<(), E>`
  * The `Box<dyn Error>` type is a trait object.
  * `Box<dyn Error>` can be read to mean “any kind of error.”
  * Using `?` on a `Result` value in a main function with the error type `Box<dyn Error>` is allowed because it allows any `Err` value to be returned early.
  * Even though the body of this main function will only ever return errors of type `std::io::Error`, by specifying `Box<dyn Error>`, this signature will continue to be correct even if more code that returns other errors is added to the body of main.
  * The `main` function may return any types that implement the `std::process::Termination` trait, which contains a function `report` that returns an `ExitCode`.

### To panic! or Not to panic!
* You could call `panic!` for any error situation, whether there’s a possible way to recover or not, but then you’re making the decision that a situation is unrecoverable on behalf of the calling code. 
* When you choose to return a `Result` value, you give the calling code options.
  * The calling code could choose to attempt to recover in a way that’s appropriate for its situation
  * Or it could decide that an `Err` value in this case is unrecoverable, so it can call `panic!` and turn your recoverable error into an unrecoverable one.
* Therefore, returning `Result` is a good default choice when you’re defining a function that might fail.
* In situations such as examples, prototype code, and tests, it’s more appropriate to write code that panics instead of returning a `Result`.
* If you can ensure by manually inspecting the code that you’ll never have an `Err` variant, it’s perfectly acceptable to call `unwrap()`:
  ```Rust
  // since the ip address is hardcoded to a correct one, there will never be an Err,
  // but the compiler will force us to handle `Result<T, Err>` from `parse()`.
  // So in this situation, it is ok to call `unwrap()` or `expect()` (preferred).
  let home: IpAddr = "127.0.0.1"
      .parse()
      .expect("Hardcoded IP address should be valid");
  ```

### Guidelines for Error Handling
* It’s advisable to have your code panic when it’s possible that your code could end up in a bad state:
  * When some assumption, guarantee, contract, or invariant has been broken, such as when invalid values, contradictory values, or missing values are passed to your code—plus one or more of the following:
    * The bad state is something that is unexpected, as opposed to something that will likely happen occasionally, like a user entering data in the wrong format.
    * Your code after this point needs to rely on not being in this bad state, rather than checking for the problem at every step.
    * There’s not a good way to encode this information in the types you use.
* If someone calls your code and passes in values that don’t make sense, it’s best to return an error if you can so the user of the library can decide what they want to do in that case. 
* However, in cases where continuing could be insecure or harmful, the best choice might be to call `panic!` and alert the person using your library.
* `panic!` is also appropriate if you’re calling external code that is out of your control and it returns an invalid state that you have no way of fixing.
* However, when failure is expected, it’s more appropriate to return a `Result`, like
  * a parser being given malformed data
  * an HTTP request returning a status that indicates you have hit a rate limit

### Creating Custom Types for Validation
* Having lots of error checks in all of your functions would be verbose and annoying.
* So, we can use Rust’s type system: the type checking done by the compiler.
* If your function has a particular type as a parameter, you can proceed with your code’s logic knowing that the compiler has already ensured you have a valid value.
* E.g. using an unsigned integer type such as `u32`, which ensures the parameter is never negative.
* We can make a new type and put the validations in a function to create an instance of the type rather than repeating the validations everywhere. That way, it’s safe for functions to use the new type in their signatures and confidently use the values they receive.
* Look at `Guess`' implementation in src/main.rs.

# Chapter 10
* Generic functions differ in the types of their parameters so we can run the same code on multiple concrete values.
* Traits are used to define behavior in a generic way. You can combine traits with generic types to constrain a generic type to accept only those types that have a particular behavior, as opposed to just any type.
* Lifetimes allow us to give the compiler enough information about borrowed values so that it can ensure references will be valid in more situations than it could without our help.

### Removing Duplication by Extracting a Function
* See ch10/src/main.rs.

### Generic Data Types
* We use generics to create definitions for items like function signatures or structs, which we can then use with many different concrete data types.

#### In Function Definitions
* When defining a function that uses generics, we place the generics in the signature of the function where we would usually specify the data types of the parameters and return value.
* To define a generic function, we place type name declarations inside angle brackets, `<>`, between the name of the function and the parameter list.
  * ` fn find_largest<T>(list: &[T]) -> &T`
  * We read this as: the function `find_largest` is generic over some type `T`. This function has one parameter named list, which is a slice of values of type `T`. The `find_largest` function will return a reference to a value of the same type `T`.
* We need to make use of `std::cmp::PartialOrd`, which is a trait.
  * Because we want to compare values of type `T` in the body, we can only use types whose values can be ordered.
  * To enable comparisons, the standard library has the `std::cmp::PartialOrd` trait that you can implement on types.
  * By restricting the types valid for `T` to only those that implement `PartialOrd`, our example will compile, because the standard library implements `PartialOrd` on both `i32` and `char`.

#### In Struct Definitions
* We can also define structs to use a generic type parameter in one or more fields using the `<>` syntax.
  * We declare the name of the type parameter inside `<>` just after the name of the struct.
  * Then we use the generic type in the struct definition where we would otherwise specify concrete data types.
  * To define a `Point` struct where `x` and `y` are both generics but could have different types, we can use multiple generic type parameters.

#### In Enum Definitions
* As we did with structs, we can define enums to hold generic data types in their variants, for example:
  ```Rust
  enum Option<T> {
    Some(T),
    None
  }

  enum Result<T, E> {
    Ok(T),
    Err(E),
  }
  ```

#### In Method Definitions
* We can implement methods on structs and enums and use generic types in their definitions too.
* `impl<T> Point<T> {`
  * We have to declare `T` just after `impl` so we can use `T` to specify that we’re implementing methods on the type` Point<T>`.
  * By declaring `T` as a generic type after `impl`, Rust can identify that the type in the angle brackets in `Point` is a generic type rather than a concrete type.
  * Using the same name for this generic parameter than the generic parameter declared in the struct definition.
  * Methods written within an `impl` that declares the generic type will be defined on any instance of the type, no matter what concrete type ends up substituting for the generic type.
* We can also specify constraints on generic types when defining methods on the type. 
  * For example, implement methods only on `Point<f32>` instances rather than on `Point<T>` instances with any generic type in its own `impl` block than in `impl<T>`.
* Generic type parameters in a struct definition aren’t always the same as those you use in that same struct’s method signatures. 
  * Like we can use generic types `X1` and `Y1` for the `Point` struct and `X2` `Y2` for the `mix_points` method signature to make the example clearer. 
  * The method creates a new `Point` instance with the `x` value from the `self Point` (of type `X1`) and the `y` value from the passed-in `Point` (of type `Y2`).
  * See `mix_points` in ch10/src/main.rs

### Performance of Code Using Generics
* Using generic types won’t make your program run any slower than it would with concrete types.
  * Rust accomplishes this by performing *monomorphsization* of the code using generics at compile time.
  * Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.
  * E.g. The generic `Option<T>` is replaced with the specific definitions created by the compiler. Because Rust compiles generic code into code that specifies the type in each instance, we pay no runtime cost for using generics.

### Traits: Defining Shared Behavior
* A *trait* defines the functionality a particular type has and can share with other types. 
* We can use traits to define shared behavior in an abstract way. We can use *trait bounds* to specify that a generic type can be any type that has certain behavior.
  * Kinda similar to *interfaces* in other languages but with some differences.

#### Defining a Trait
* A type’s behavior consists of the methods we can call on that type.
* Different types share the same behavior if we can call the same methods on all of those types.
  * Like if we could call `summarize()` on an object of `NewsArticle` and of `Tweet`.
* Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.
* ```Rust
  pub trait Summary {
    fn summarize(&self) -> String;
  }
  ```
* Because we made the trait public using `pub`, crates depending on this crate can make use of this trait too.
* Each type implementing this trait must provide its own custom behavior for the body of the method. The compiler will enforce that any type that has the `Summary` trait will have the method `summarize` defined with this signature exactly.
* A trait can have multiple methods in its body: the method signatures are listed one per line, and each line ends in a semicolon.

#### Implementing a Trait on a Type
* This is how
  * ```Rust
    pub struct NewsArticle {
      pub headline: String,
      pub published: bool,
      pub author: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {}", self.headline, self.author)
        }
    }
    ```
* Users of the crate can call the trait methods on instances of `NewsArticle` and `Tweet` in the same way we call regular methods. 
* The only difference is that the user must bring the trait into scope as well as the types.
  * ```Rust
    use aggregator::{Summary, Tweet};
    ```
* Other crates that depend on the `aggregator` crate can also bring the `Summary` trait into scope to implement `Summary` on their own types.
* A restriction with traits is that we can implement a trait on a type only if either the trait or the type, or both, are local to our crate.
  * E.g. we can implement standard library traits like `Display` on a custom type like `Tweet` as part of our `aggregator` crate functionality because the type `Tweet` is local to our `aggregator` crate. 
  * We can also implement `Summary` on `Vec<T>` in our `aggregator` crate because the trait `Summary` is local to our `aggregator` crate.
* But we can’t implement external traits on external types. 
  * E.g., we can’t implement the `Display` trait on `Vec<T>` within our `aggregator` crate because `Display` and `Vec<T>` are both defined in the standard library and aren’t local to our `aggregator` crate.
* This restriction is part of a property called *coherence*, and more specifically the *orphan rule*, so named because the parent type is not present. 
* This rule ensures that other people’s code can’t break your code and vice versa. 
* Without the rule, two crates could implement the same trait for the same type, and Rust wouldn’t know which implementation to use.

#### Default Implementations
* Sometimes it’s useful to have default behavior for some or all of the methods in a trait instead of requiring implementations for all methods on every type.
* Then, as we implement the trait on a particular type, we can keep or override each method’s default behavior.
* To provide a default implementation, we specify a default implementation for the trait instead of only defining the method signature.
* To use a default implementation, we specify an empty `impl` block with `impl` of the trait for a struct.
  * If your struct wants to override the default implementation, you just do it normally since the syntax for overriding a default implementation is the same as the syntax for implementing a trait method that doesn’t have a default implementation.
* Default implementations can call other methods in the same trait, even if those other methods don’t have a default implementation.
  * In this way, a trait can provide a lot of useful functionality and only require implementors to specify a small part of it.
* **Note: it isn’t possible to call the default implementation from an
overriding implementation of that same method.**

#### Traits as Parameters
* With traits as parameters, we can define functions that accept many different types.
  * ```Rust
    pub fn notify(item: &impl Summary) {
      println!("Breaking news! {}", item.summarize());
    }
    ```
* In the body of `notify`, we can call any methods on item that come from the `Summary` trait, such as `summarize()`.
* We can pass `notify()` any instance of `Tweet` or `NewsArticle`.

#### Trait Bound Syntax
* The `impl Trait` syntax works for straightforward cases but is actually syntax sugar for a longer form known as a *trait bound*; it looks like this:
  * ```Rust
    pub fn notify<T: Summary>(item: &T) {
      println!("Breaking news! {}", item.summarize());
    }
    ```
* This above example is the same as the example in "Traits as Parameters".
* With Traits as Parameters, having multiple parameters would look like:
  * ```Rust
    pub fn notify(item1: &impl Summary, item2: &impl Summary)
    ```
* But we can shorten it with Trait Bound Syntax as:
  * ```Rust
    pub fn notify<T: Summary>(item1: &T, item2: &T)
    ``` 
* The generic type `T` specified as the type of the `item1` and `item2` parameters constrains the function such that the concrete type of the value passed as an argument for `item1` and `item2` must be the same.
  
#### Specifying Multiple Trait Bounds with the + Syntax
* We can also specify more than one trait bound. 
* Say we wanted `notify` to use display formatting as well as `summarize()` on `item`: we specify in the `notify` definition that `item` must implement both `Display` and `Summary` using the `+`:
  * ```Rust
    // Traits as Parameter
    pub fn notify(item: &impl(Summary + Display))

    // Trait Bound Syntax
    pub fn notify<T: Summarize + Display>(item: T)
    ```
* With the two trait bounds specified, the body of `notify` can call `summarize` and use `{}` to format item.

#### Clearer Trait Bounds with where Clauses
* Using too many trait bounds has its downsides. 
* Each generic has its own trait bounds, so functions with multiple generic type parameters can contain lots of trait bound information between the function’s name and its parameter list, making the function signature hard to read. 
* We can use the `where` clause like so:
  * ```Rust
    // instead of doing this
    fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {

    // we can do this
    fn some_function(T, U)(t: &T, u: &U) -> i32 {
      where
        T: Display + Clone,
        U: Clone + Debug,
    }
    ```

#### Returning Types That Implement Traits
* We can also use the `impl Trait` syntax in the return position to return a value of some type that implements a trait:
  * ```Rust
    fn returns_summarizable() -> impl Summary {
      Tweet {
        // ...
      }
    }
    ```
* You can only use `impl Trait` if you’re returning a single type. 
* E.g, this code that returns either a `NewsArticle` or a `Tweet` with the return type specified as `impl Summary` **wouldn’t work**:
  * ```Rust
    fn returns_summarizable(switch: bool) -> impl Summary {
      if switch {
        Tweet {
          // ...
        }
      } else {
        NewsArticle {
          // ...
        }
      }
    }
    ```
* Returning either a `NewsArticle` or a `Tweet` **isn’t allowed** due to restrictions around how the `impl Trait` syntax is implemented in the compiler.

#### Using Trait Bounds to Conditionally Implement Methods
* By using a trait bound with an `impl` block that uses generic type parameters, we can implement methods conditionally for types that implement the specified traits.
* E.g. the type `Pair<T>` always implements the `new` function to return a new instance of `Pair<T>`. But in the next `impl` block, `Pair<T>` only implements the `cmp_display` method if its inner type `T` implements the `PartialOrd` trait that enables comparison and the `Display` trait that enables printing.
  * ```Rust
    use std::fmt::Display;

    struct Pair<T> {
      x: T,
      y: T,
    }

    impl<T> Pair<T> {
      fn new(x: T, y: T) -> Self {
        Self { x, y }
      }
    }

    impl<T: Display + PartialOrd> Pair<T> {
      fn cmp_display(&self) {
        if self.x >= self.y {
          println!("The largest member is x = {}", self.x);
        } else {
          println!("The largest member is y = {}", self.y);
        }
      }
    }
    ```
* We can also conditionally implement a trait for any type that implements another trait.
* Implementations of a trait on any type that satisfies the trait bounds are called *blanket implementations* and are used a lot in the Rust standard library.
  * E.g. the standard library implements the `ToString` trait on any type that implements the `Display` trait.
  * The `impl` block in the standard library looks similar to this code:
    * ```Rust
      impl<T: Display> ToString for T {
        // ...
      }
      ```
  * Because the standard library has this blanket implementation, we can call the `to_string` method defined by the `ToString` trait on any type that implements the `Display` trait.
    * E.g. integers into their corresponding String values like this because integers implement `Display`: `let three = 3.to_string();`
  
### Validating References with Lifetimes
* Rather than ensuring that a type has the behavior we want (that Traits do for us), lifetimes ensure that references are valid as long as we need them to be.
* Every reference in Rust has a *lifetime*, which is the scope for which that reference is valid.
* Most of the time, lifetimes are implicit and inferred, just like most of the time, types are inferred. 
  * We must annotate types only when multiple types are possible.
  * In a similar way, we must annotate lifetimes when the lifetimes of references could be related in a few different ways.
  * Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid.

#### Preventing Dangling References with Lifetimes
* The main aim of lifetimes is to prevent *dangling references*, which cause a program to reference data other than the data it’s intended to reference.
* If Rust allowed us using references to memory that has been deallocated, anything we try to do with the reference variable wouldn’t work correctly. 
* So how does Rust determine that this code is invalid? It uses a *borrow checker*.

#### The Borrow Checker
* The Rust compiler has a borrow checker that compares scopes to determine whether all borrows are valid.
  * ```Rust
    fn main() {
      let r;              // ---------+-- 'r
                          //          |
      {                   //          |
        let x = 5;        // -+-- 'x  |
        r = &x;           //  |       |
      }                   // -+       |
                          //          | 
      println!("r: {r}"); //          |
    }                     // ---------+
    ```
* At compile time, Rust compares the size of the two lifetimes and sees that `r` has a lifetime of `'r` but that it refers to memory with a lifetime of `'x`.
* The program is rejected because `'x` is shorter than `'r`: the subject of the reference doesn’t live as long as the reference.
* If `x` had a lifetime equal or greater than `'r`, that would be allowed.

#### Generic Lifetimes in Functions
* Looking at `longest()` in src/main.rs
  * `fn longest(string1: &str, string2: &str) -> &str {` doesn't compile because according to the compiler error, the return type needs a generic lifetime parameter on it because Rust can’t tell whether the reference being returned refers to `string1` or `string2`.
  * Actually, we don’t know either, because the `if` block in the body of this function returns a reference to `string1` and the `else` block returns a reference to `string2`!
  * When we’re defining this function, we don’t know the concrete values that will be passed into this function, so we don’t know whether the `if` case or the `else` case will execute.
  * We also don’t know the concrete lifetimes of the references that will be passed in, so we can’t look at the scopes as we did in the previous examples to determine whether the reference we return will always be valid.
  * The borrow checker can’t determine this either, because it doesn’t know how the lifetimes of `string1` and `string2` relate to the lifetime of the return value.
  * To fix this error, we’ll add generic lifetime parameters that define the relationship between the references so the borrow checker can perform its analysis.

#### Lifetime Annotation Syntax
* Lifetime annotations don’t change how long any of the references live. 
* Rather, they describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.
* Just as functions can accept any type when the signature specifies a generic type parameter, functions can accept references with any lifetime by specifying a generic lifetime parameter.
* Syntax: the names of lifetime parameters must start with an apostrophe (`'`) and are usually all lowercase and very short, like generic types. 
  * Most people use the name `'a` for the first lifetime annotation.
  * We place lifetime parameter annotations after the `&` of a reference, using a space to separate the annotation from the reference’s type.
  * ```Rust
    &i32        // a reference
    &'a i32     // a reference with an explicit lifetime
    &'a mut i32 // a mutable reference with an explicit lifetime
    ```
* One lifetime annotation by itself doesn’t have much meaning because the annotations are meant to tell Rust how generic lifetime parameters of multiple references relate to each other.

#### Lifetime Annotations in Function Signatures
* To use lifetime annotations in function signatures, we need to declare the generic *lifetime* parameters inside angle brackets between the function name and the parameter list, just as we did with generic *type* parameters.
* `fn longest<'a>(string1: &'a str, string2: &'a str) -> &'a str {`
  * The function signature tells Rust that for some lifetime `'a`, the function takes two parameters, both of which are string slices that live at least as long as lifetime `'a`. 
  * The function signature also tells Rust that the string slice returned from the function will live at least as long as lifetime `'a`. 
  * **In practice, it means that the lifetime of the reference returned by the `longest()` function is the same as the smaller of the lifetimes of the values referred to by the function arguments. These relationships are what we want Rust to use when analyzing this code.**
  * We’re specifying that the borrow checker should reject any values that don’t adhere to these constraints. Note that the `longest()` function doesn’t need to know exactly how long `string1` and `string2` will live, only that some scope can be substituted for `'a` that will satisfy this signature.

#### Thinking in Terms of Lifetimes
* The way in which you need to specify lifetime parameters depends on what your function is doing. 
* E.g. if we changed the implementation of the `longest()` function to always return the first parameter rather than the longest string slice, we wouldn’t need to specify a lifetime on the `y` parameter.
  * The following code will compile: `fn longest<'a>(x: &'a str, y: &str) -> &'a str { x }`
  * We’ve specified a lifetime parameter `'a` for the parameter `x` and the return type, but not for the parameter `y`, because the lifetime of `y` does not have any relationship with the lifetime of `x` or the return value as we always return `x`.
* When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters. 
  * If the reference returned does not refer to one of the parameters, it must refer to a value created within this function. 
  * However, this would be a dangling reference because the value will go out of scope at the end of the function. Like this:
    * ```Rust
      fn longest<'a>(x: &str, y: &str) -> &'a str {
        let result = String::from("really long string");
        result.as_str()
      }
      ```
  * The problem is that `result` goes out of scope and gets cleaned up at the end of the `longest()` function resulting in a *dangling reference* not allowed by Rust.

#### Lifetime Annotations in Struct Definitions
* So far, the structs we’ve defined all hold owned types. 
* We can define structs to hold references, but in that case we would need to add a lifetime annotation on every reference in the struct’s definition.
  * ```Rust
    // this means an instance of `ImportantExcerpt` cannot outlive the reference it holds in its `part` field
    struct ImportantExcerpt<'a> {
    ```

#### Lifetime Elision
* Every reference has a lifetime and you need to specify lifetime parameters for functions or structs that use references. 
* The patterns programmed into Rust’s analysis of references are called the *lifetime elision rules*. 
* These aren’t rules for programmers to follow; they’re a set of particular cases that the compiler will consider, and if your code fits these cases, you don’t need to write the lifetimes explicitly.
* Like:
  * ```Rust
    fn first_word(s: &str) -> &str
    // instead of the following in pre-1.0 Rust versions
    fn first_word<'a>(s: &'a str) -> &'a str
    ```
* The elision rules don’t provide full inference. If Rust deterministically applies the rules but there is still ambiguity as to what lifetimes the references have, the compiler won’t guess what the lifetime of the remaining references should be. Instead, the compiler will give you an error that you can resolve by adding the lifetime annotations.
* Lifetimes on function or method parameters are called *input lifetimes*, and lifetimes on return values are called *output lifetimes*.
* The compiler uses three rules to figure out the lifetimes of the references when there aren’t explicit annotations.
  * The first rule applies to input lifetimes, and the second and third rules apply to output lifetimes.
  *  If the compiler gets to the end of the three rules and there are still references for which it can’t figure out lifetimes, the compiler will stop with an error.
  *  These rules apply to `fn` definitions as well as `impl` blocks.
  *  Rules:
     1. The compiler assigns a lifetime parameter to each parameter that’s a reference. In other words, a function with one parameter gets one lifetime parameter: `fn foo<'a>(x: &'a i32);` a function with two parameters gets two separate lifetime parameters: `fn foo<'a, 'b>(x: &'a i32, y: &'b i32);` and so on.
     2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: `fn foo<'a>(x: &'a i32) -> &'a i32`.
     3. The third rule is that, if there are multiple input lifetime parameters, but one of them is `&self` (or `&mut self` because this is a method), the lifetime of `self` is assigned to all output lifetime parameters. This rule makes methods much nicer to read and write because fewer symbols are necessary.
* Examples:
  * Compiler can apply all 3 rules to `fn first_word(s: &str) -> &str {`
  * But it cannot apply rules 2 and 3 to `fn longest(x: &str, y: &str) -> &str {` so gives an error for the programmer to specify the lifetimes.

#### Lifetime Annotations in Method Definitions
* Lifetime names for struct fields always need to be declared after the `impl` keyword and then used after the struct’s name because those lifetimes are part of the struct’s type.
* In method signatures inside the `impl` block, references might be tied to the lifetime of references in the struct’s fields, or they might be independent.
* The lifetime elision rules often make it so that lifetime annotations aren’t necessary in method signatures.
    ```Rust
    // The lifetime parameter declaration after `impl` and its use after the type 
    // name are required, but we’re not required to annotate the lifetime of the 
    // reference to `self` because of the first elision rule.
    impl<'a> ImportantExcerpt<'a> {
      fn level(&self) -> i32 {
        3
      }
    }

    // Example where the third lifetime elision rule applies:
    impl<'a> ImportantExcerpt<'a> {
      fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
      }
    }

    // In example 2, there are two input lifetimes, so Rust applies the first 
    // lifetime elision rule and gives both `&self` and announcement their own 
    // lifetimes. Then, because one of the parameters is `&self`, the return type 
    // gets the lifetime of `&self`, and all lifetimes have been accounted for.
    ```
#### The Static Lifetime
* `'static` lifetimes denote that the affected reference can live for the entire duration of the program. 
* All string literals have the 'static lifetime, which we can annotate like:
  * ```Rust
    let s: &'static str = "I have a static lifetime";
    ```
* The text of this string is stored directly in the program’s binary, which is always available.
* Before specifying `'static` as the lifetime for a reference, think about whether the reference you have actually lives the entire lifetime of your program or not, and whether you want it to.
  
#### Generic Type Parameters, Trait Bounds, and Lifetimes Together
* Syntax of specifying generic type parameters, trait bounds, and lifetimes all in one function:
  ```Rust
    use std::fmt::Display;

    fn longest_with_an_accouncement<a', T>(
      x: &'a str,
      y: &'a str,
      annoucement: T,
    ) -> &'a str 
    where
      T: Display,
    {
      println!("Annoucement! I am {annoucement}");
      if x.len() > y.len() {
        x
      }
      else {
        y
      }
    }
    ```

#### Summary
* Generic type parameters let you apply the code to different types. 
* Traits and trait bounds ensure that even though the types are generic, they’ll have the behavior the code needs. 
* lifetime annotations ensure that this flexible code won’t have any dangling references.
* And all of this analysis happens at compile time, which doesn’t affect runtime performance!

# Chapter 11

### How to Write Tests
* The bodies of test functions typically perform these three actions:
  * Set up any needed data or state.
  * Run the code you want to test.
  * Assert that the results are what you expect.

### The Anatomy of a Test Function
* A test in Rust is a function that’s annotated with the `test` attribute.
* Attributes are metadata about pieces of Rust code; like the `derive` attribute.
* To change a function into a test function, add `#[test]` on the line before `fn`.
* When you run your tests with the `cargo test` command, Rust builds a test runner binary that runs the annotated functions and reports on whether each test function passes or fails.
* Whenever we make a new library project with Cargo, a test module with a test function in it is automatically generated for us.
* We can have test and non-test functions in a module.
* The overall test summary shows a few things:
    * ```Rust
      test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
      ```
  * `ignored`: tests marked ignored don't run in a particular instance
  * `measured`: statistic is for benchmark tests that measure performance.
  * `filtered out`: shows info about tests that run only whose name matches a string.
* The next part of the test output starting at `Doc-tests`
  * Rust can compile any code examples that appear in our API documentation. This feature helps keep your docs and your code in sync!

* Tests fail when something in the test function panics.
* Each test is run in a new thread, and when the main thread sees that a test thread has died, the test is marked as failed
  
### Checking Results with the `assert!` Macro
* The `assert!` macro, provided by the standard library, ensures that some condition in a test evaluates to `true`.
* The `tests` module is a regular module that follows the usual visibility rules covered in “Paths for Referring to an Item in the Module Tree”
  * Because the `tests` module is an inner module, we need to bring the code under test in the outer module into the scope of the inner module.
  * `use super::*;` glob is used here, so anything we define in the outer module is available to this tests module.

### Testing Equality with the `assert_eq!` and `assert_ne!` Macros
* `assert_eq!` and `assert_ne!` allow us to verify functionality is to test for equality between the result of the code under test and the value you expect the code to return.
  * The order in which we specify the value we expect and the value the code produces doesn’t matter.
* When the assertions fail, these macros print their arguments using debug formatting, which means the values being compared must implement the `PartialEq` and `Debug` traits. 
  * All primitive types and most of the standard library types implement these traits. 
  * For structs and enums that you define yourself, you’ll need to implement `PartialEq` to assert equality of those types.
  * You’ll also need to implement `Debug` to print the values when the assertion fails.
  * This can be done with `#[derive(PartialEq, Debug)]` annotation to your struct or enum definition.

### Adding Custom Failure Messages
* You can also add a custom message to be printed with the failure message as optional arguments to the `assert!`, `assert_eq!`, and `assert_ne!` macros.
* Any arguments specified after the required arguments are passed along to the `format!` macro, so you can pass a format string that contains `{}` placeholders and values to go in those placeholders.

### Checking for Panics with `#[should_panic]`
* It’s important to check that our code handles error conditions as we expect.
* Tests that use `should_panic` can be imprecise. 
* A `should_panic` test would pass even if the test panics for a different reason from the one we were expecting. 
* To make `should_panic` tests more precise, we can add an optional expected parameter to the `should_panic` attribute, like `#[should_panic(expected = "between 1 and 100")]`.
* The test harness will make sure that the failure message contains the provided text.

### Using `Result<T, E>` in Tests
* We can also write tests that use `Result<T, E>`.
* Writing tests so they return a `Result<T, E>` enables you to use the question mark operator in the body of tests, which can be a convenient way to write tests that should fail if any operation within them returns an `Err` variant.
* You can’t use the `#[should_panic]` annotation on tests that use `Result<T, E>`. 
* To assert that an operation returns an `Err` variant, don’t use the question mark operator on the `Result<T, E>` value. Instead, use `assert!(value.is_err())`.

### Controlling How Tests Are Run
* Just as `cargo run` compiles your code and then runs the resultant binary, `cargo test` compiles your code in test mode and runs the resultant test binary. 
* The default behavior of the binary produced by `cargo test` is to run all the tests in parallel and capture output generated during test runs, making it easier to read the output related to the test results. 
* You can specify command line options to change this default behavior.
* Some command line options go to `cargo test`, and some go to the resultant test binary.
  * To separate these two types of arguments, you list the arguments that go to `cargo test` followed by the separator `--` and then the ones that go to the test binary. 
  * Running `cargo test --help` displays the options you can use with `cargo test`.
  * Running `cargo test -- --help` displays the options you can use after the separator.

### Running Tests in Parallel or Consecutively
* By default tests run in parallel using threads.
* Because the tests are running at the same time, you must make sure your tests don’t depend on each other or on any shared state, including a shared environment, such as the current working directory or environment variables.
* If you don’t want to run the tests in parallel or if you want more fine-grained control over the number of threads used, you can send the `--test-threads` flag and the number of threads you want to use to the test binary:
  * `cargo test -- --test-threads=1`
  * It will take longer than running them in parallel, but the tests won’t interfere with each other if they share state.

### Showing Function Output
* By default, if a test passes, Rust’s test library captures anything printed to standard output. 
  * For example, if we call `println!` in a test and the test passes, we won’t see the `println!` output in the terminal; we’ll see only the line that indicates the test passed. 
  * If a test fails, we’ll see whatever was printed to standard output with the rest of the failure message.
* If we want to see printed values for passing tests as well, we can tell Rust to also show the output of successful tests:
  * `cargo test -- --show-output`

### Running a Subset of Tests by Name
* You can choose which tests to run by passing `cargo test` the name or names of the test(s) you want to run as an argument.
* Filtering to run a single test: `cargo test it_adds_two_100`.
* Filtering to run multiple tests:
  * We can specify part of a test name, and any test whose name matches that value will be run: `cargo test adds` to run all tests with `adds` in the name.
  * We can also run all tests in a module by `cargo test <module_name>`.

### Ignoring Some Tests Unless Specifically Requested
* Sometimes a few specific tests can be very time-consuming to execute, so you might want to exclude them during most runs of `cargo test`. 
* Rather than listing as arguments all tests you do want to run, you can instead annotate the time-consuming tests using the `#[ignore]` attribute to exclude them.
  * This will ignore that test each time.
  * To run only the ignored tests we can do `cargo test -- --ignored`.
  * To run all tests (ignored or not), we can do `cargo test -- --include-ignored`.

## Test Organization
### Unit Tests
* Unit tests are small and more focused, testing one module in isolation at a time, and can test private interfaces.
* You put unit tests in the `src` directory in each file with the code that they’re testing. 
* The convention is to create a module named `tests` in each file to contain the test functions and to annotate the module with `cfg(test)`.

### The Tests Module and #[cfg(test)]
* The `#[cfg(test)]` annotation on the tests module tells Rust to compile and run the test code only when you run `cargo test`, not when you run `cargo build`.
* Because unit tests go in the same files as the code, you’ll use `#[cfg(test)]` to specify that they shouldn’t be included in the compiled result.
* The attribute `cfg` stands for configuration and tells Rust that the following item should only be included given a certain configuration option.
* In the case of `#[cfg(test)]`, the configuration option is test, which is provided by Rust for compiling and running tests. By using the `cfg` attribute, Cargo compiles our test code only if we actively run the tests with `cargo test`.
* This includes any helper functions that might be within this module, in addition to the functions annotated with `#[test]`.

### Testing Private Functions
* There’s debate within the testing community about whether or not private functions should be tested directly, and other languages make it difficult or impossible to test private functions.
* Rust’s privacy rules do allow you to test private functions.

### Integration Tests
* Integration tests are entirely external to your library and use your code in the same way any other external code would, which means they can only call functions that are part of your library’s public API.
* Because integration tests go in a different directory, they don’t need the `#[cfg(test)]` annotation.
* To create integration tests, you first need a *tests* directory.

### The tests Directory
* We create a *tests* directory at the top level of our project directory, next to *src*.
* Cargo knows to look for integration test files in this directory.
* We can then make as many test files as we want, and Cargo will compile each of the files as an individual crate.
* Each file in the tests directory is a separate crate, so we need to bring our library into each test crate’s scope. For that reason we add `use adder;` at the top of the code, which we didn’t need in the unit tests.
* We don’t need to annotate any code in tests/integration_test.rs with` #[cfg(test)]`.
  * Cargo treats the tests directory specially and compiles files in this directory only when we run `cargo test`.
* Note that if any test in a section fails, the following sections will not be run.
* We can still run a particular integration test function by specifying the test function’s name as an argument to `cargo test`. 
* To run all the tests in a particular integration test file, use the `--test` argument of `cargo test` followed by the name of the file: `cargo test --test integration_test`.

### Submodules in Integration Tests
* As you add more integration tests, you might want to make more files in the tests directory to help organize them.
* For example, you can group the test functions by the functionality they’re testing. 
* Each file in the tests directory is compiled as its own separate crate, which is useful for creating separate scopes to more closely imitate the way end users will be using your crate.
* However, this means files in the *tests* directory don’t share the same behavior as files in *src* do.
  * The different behavior of *tests* directory files is most noticeable when you have a set of helper functions to use in multiple integration test files.
  * For example, if we create "tests/common.rs" and place a function named `setup` in it, we can add some code to `setup` that we want to call from multiple test functions in multiple test files.
  * When we run the tests again, we’ll see a new section in the test output for the "common.rs" file, even though this file doesn’t contain any test functions nor did we call the `setup` function from anywhere.
  * Having `common` appear in the test results with running **0 tests** displayed for it is not what we wanted. 
    * We just wanted to share some code with the other integration test files.
    * To avoid having `common` appear in the test output, instead of creating *tests/common.rs*, we’ll create *tests/common/mod.rs*.
    * This is the older naming convention that Rust also understands.
    * Naming the file this way tells Rust not to treat the `common` module as an integration test file.
    * When we move the `setup` function code into *tests/common/mod.rs* and delete the *tests/common.rs* file, the section in the test output will no longer appear. 
    * Files in subdirectories of the tests directory don’t get compiled as separate crates or have sections in the test output.
    * After we’ve created *tests/common/mod.rs*, we can use it from any of the integration test files as a module.

### Integration Tests for Binary Crates
* If our project is a binary crate that only contains a *src/main.rs* file and doesn’t have a *src/lib.rs* file, we can’t create integration tests in the tests directory and bring functions defined in the *src/main.rs* file into scope with a `use` statement. 
* Only library crates expose functions that other crates can use; binary crates are meant to be run on their own.

# Chapter 12

### Reading the Argument Values
* To enable `minigrep` to read the values of command line arguments we pass to it, we’ll need the `std::env::args` function provided in Rust’s standard library.
* This function returns an iterator of the command line arguments passed to `minigrep`, and we can call the `collect` method on an iterator to turn it into a collection, such as a vector.
* We can use the `collect` function to create many kinds of collections, so we explicitly annotate the type of `args` to specify that we want a vector of strings.

### Separation of Concerns for Binary Projects
* The Rust community has developed guidelines for splitting the separate concerns of a binary program when main starts getting large. This process has the following steps:
  * Split your program into a main.rs file and a lib.rs file and move your program’s logic to lib.rs.
  * As long as your command line parsing logic is small, it can remain in main.rs.
  * When the command line parsing logic starts getting complicated, extract it from main.rs and move it to lib.rs.
  * The responsibilities that remain in the `main` function after this process should be limited to the following:
    * Calling the command line parsing logic with the argument values
    * Setting up any other configuration
    * Calling a `run` function in lib.rs
    * Handling the error if `run` returns an error
* This pattern is about separating concerns: main.rs handles running the program and lib.rs handles all the logic of the task at hand. 
* Because you can’t test the `main` function directly, this structure lets you test all of your program’s logic by moving it into functions in lib.rs. 
* The code that remains in main.rs will be small enough to verify its correctness by reading it.

### Developing the Library’s Functionality with Test-Driven Development
* Test-driven development (TDD) process is the following steps:
  * Write a test that fails and run it to make sure it fails for the reason you expect.
  * Write or modify just enough code to make the new test pass.
  * Refactor the code you just added or changed and make sure the tests continue to pass.
  * Repeat from step 1!
  
### Fixing the Error Handling
#### Returning a Result Instead of Calling panic!
* Instead of `panic`, we can return a `Result` value that will contain a `Config` instance in the successful case and will describe the problem in the error case.
* We’re also going to change the function name from `new` to `build` because many programmers expect `new` functions to never fail.
* When `Config::build` is communicating to `main`, we can use the `Result` type to signal there was a problem.
* Then we can change `main` to convert an `Err` variant into a more practical error for our users without the surrounding text about thread `'main'` and `RUST_BACKTRACE` that a call to `panic!` causes.

#### Calling Config::build and Handling Errors
*  Using `unwrap_or_else` allows us to define some custom, non-panic! error handling. 
*  If the `Result` is an `Ok` value, this method’s behavior is similar to `unwrap`: it returns the inner value that `Ok` is wrapping. 
*  However, if the value is an `Err` value, this method calls the code in the closure, which is an anonymous function we define and pass as an argument to `unwrap_or_else`.
  
### Extracting Logic from main
#### Handling Errors Returned from run in main
* We use `if let` rather than `unwrap_or_else` to check whether run returns an `Err` value and to call `process::exit(1)` if it does. 
* The `run` function doesn’t return a value that we want to unwrap in the same way that `Config::build` returns the `Config` instance. 
* Because `run` returns `()` in the success case, we only care about detecting an error, so we don’t need `unwrap_or_else` to return the unwrapped value, which would only be `()`.

### Splitting Code into a Library Crate
* We split the *src/main.rs* file and put some code into the *src/lib.rs* file.
* That way, we can test the code and have a *src/main.rs* file with fewer responsibilities.

### Developing the Library’s Functionality with Test- Driven Development
#### Writing a Failing Test
* Notice that we need to define an explicit lifetime `'a` in the signature of `search` and use that lifetime with the `contents` argument and the return value. 
* The lifetime parameters specify which argument lifetime is connected to the lifetime of the return value. 
* In this case, we indicate that the returned vector should contain string slices that reference slices of the argument `contents` (rather than the argument `query`).
* In other words, we tell Rust that the data returned by the `search` function will live as long as the data passed into the `search` function in the `contents` argument. 
* This is important! The data referenced by a slice needs to be valid for the reference to be valid; if the compiler assumes we’re making string slices of `query` rather than `contents`, it will do its safety checking incorrectly.
* Rust can’t possibly know which of the two arguments we need, so we need to tell it explicitly.
* Because `contents` is the argument that contains all of our text and we want to return the parts of that text that match, we know `contents` is the argument that should be connected to the return value using the lifetime syntax.

#### Working with Environment Variables
* We’ll improve `minigrep` by adding an extra feature: an option for case-insensitive searching that the user can turn on via an environment variable. 
* We could make this feature a command line option and require that users enter it each time they want it to apply, but by instead making it an environment variable, we allow our users to set the environment variable once and have all their searches be case insensitive in that terminal session.

#### Implementing the search_case_insensitive Function
* We use the `var` function from the `env` module to check to see if any value has been set for an environment variable named `IGNORE_CASE` like: `env::var("IGNORE_CASE").is_ok()`.
* The `env::var` function returns a `Result` that will be the successful `Ok` variant that contains the value of the environment variable if the environment variable is set to any value. 
* It will return the `Err` variant if the environment variable is not set.
* We’re using the `is_ok` method on the `Result` to check whether the environment variable is set, which means the program should do a case-insensitive search. 
* If the `IGNORE_CASE` environment variable isn’t set to anything, `is_ok` will return `false` and the program will perform a case-sensitive search.
* We don’t care about the value of the environment variable, just whether it’s set or unset, so we’re checking `is_ok` rather than using `unwrap`, `expect`, or any of the other methods we’ve seen on `Result`.
* We can set the environment variable while running the program like: `$ IGNORE_CASE=1 cargo run -- to poem.txt`.
  
### Writing Error Messages to Standard Error Instead of Standard Output
* In most terminals, there are two kinds of output: standard output (`stdout`) for general information and standard error (`stderr`) for error messages. 
* This distinction enables users to choose to direct the successful output of a program to a file but still print error messages to the screen.
* The `println!` macro is only capable of printing to standard output.

### Checking Where Errors Are Written
* Command line programs are expected to send error messages to the standard error stream so we can still see error messages on the screen even if we redirect the standard output stream to a file.
* We can use `>` to redirect `stdout` output from the terminal to a file.
  
### Printing Errors to Standard Error
* The standard library provides the `eprintln!` macro that prints to the standard error stream.
* We still use `>` to redirect output to a file, but all errors will be printed to the screen while any other output goes to the file.

# Chapter 13
* Rust has some features that are similar to features in functional programming languages, like:
  * Closures: a function-like construct you can store in a variable
  * Iterators: a way of processing a series of elements
  * Pattern matching and Enums are also influenced by the functional style.

## Closures
### Closures: Anonymous Functions That Capture Their Environment
* Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other functions.
* You can create the closure in one place and then call the closure elsewhere to evaluate it in a different context.
* Unlike functions, closures can capture values from the scope in which they’re defined.

### Capturing the Environment with Closures
* `user_preference.unwrap_or_else(|| self.most_stocked())`
* If the `Option<T>` is the `Some` variant, `unwrap_or_else` returns the value from within the `Some`.
* If the `Option<T>` is the `None` variant, `unwrap_or_else` calls the closure and returns the value returned by the closure.
* We specify the closure expression `|| self.most_stocked()` as the argument to `unwrap_or_else`. 
  * This is a closure that takes no parameters itself (if the closure had parameters, they would appear between the `||`).
  * The body of the closure calls `self.most_stocked()`.
* One interesting aspect here is that we’ve passed a closure that calls `self.most_stocked()` on the current `Inventory` instance. 
* The standard library didn’t need to know anything about the `Inventory` or `ShirtColor` types we defined, or the logic we want to use in this scenario. 
* The closure captures an immutable reference to the `self Inventory` instance and passes it with the code we specify to the `unwrap_or_else` method. 
* Functions, on the other hand, are not able to capture their environment in this way.

### Closure Type Inference and Annotation
* Closures don’t usually require you to annotate the types of the parameters or the return value like functions do.
* Type annotations are required on functions because the types are part of an explicit interface exposed to your users. 
* Closures, on the other hand, aren’t used in an exposed interface like this: they’re stored in variables and used without naming them and exposing them to users of our library.
* Closures are typically short and relevant only within a narrow context rather than in any arbitrary scenario.
* Within these limited contexts, the compiler can infer the types of the parameters and the return type.
* If we want to be more explicit, we can add type annotations to closures:
  * ```Rust
      let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
      };
      
      // these are all the same
      fn  add_one_v1   (x: u32) -> u32 { x + 1 }

      let add_one_v2 = |x: u32| -> u32 { x + 1 };
      let add_one_v3 = |x|             { x + 1 };
      let add_one_v4 = |x|               x + 1  ;
    ```
* Closures that don't have type annotations can only be called for the types that were used with that closure for the first time:
  * ```Rust
      // simple closure that returns what's passed in
      let example_closure = |x| x;
      let s = example_closure(String::from("hello"));

      // since the first call to example_closure was with a String, that String type
      // was locked in for the closure and the following will give us an error
      let n = example_closure(5);
    ```

### Capturing References or Moving Ownership
* Closures can capture values from their environment in three ways, which directly map to the three ways a function can take a parameter: borrowing immutably, borrowing mutably, and taking ownership. 
* The closure will decide which of these to use based on what the body of the function does with the captured values.
* To force the closure to take ownership of the values it uses in the environment even though the body of the closure doesn’t strictly need ownership, you can use the `move` keyword before the parameter list.
  * This technique is mostly useful when passing a closure to a new thread to move the data so that it’s owned by the new thread.
  * ```Rust
      thread::spawn(move || println!("From thread: {:?}", list))
          .join()
          .unwrap();
    ```
* We spawn a new thread, giving the thread a closure to run as an argument. The closure body prints out the `list`. 
* In this example, even though the closure body still only needs an immutable reference, we need to specify that `list` should be moved into the closure by putting the `move` keyword at the beginning of the closure definition. 
* The new thread might finish before the rest of the main thread finishes, or the main thread might finish first. 
* If the main thread maintains ownership of `list` but ends before the new thread and drops `list`, the immutable reference in the thread would be invalid. 
* Therefore, the compiler requires that `list` be moved into the closure given to the new thread so the reference will be valid.

### Moving Captured Values Out of Closures and the Fn Traits
* Once a closure has captured a reference or captured ownership of a value from the environment where the closure is defined (thus affecting what, if anything, is moved *into* the closure), the code in the body of the closure defines what happens to the references or values when the closure is evaluated later (thus affecting what, if anything, is moved *out* of the closure).
* A closure body can do any of the following: 
  * move a captured value out of the closure
  * mutate the captured value
  * neither move nor mutate the value
  * capture nothing from the environment to begin with
* The way a closure captures and handles values from the environment affects which traits the closure implements, and traits are how functions and structs can specify what kinds of closures they can use. 
* Closures will automatically implement one, two, or all three of these `Fn` traits, in an additive fashion, depending on how the closure’s body handles the values:
  * `FnOnce` applies to closures that can be called once. All closures implement at least this trait because all closures can be called. A closure that moves captured values out of its body will only implement `FnOnce` and none of the other `Fn` traits because it can only be called once.
  * `FnMut` applies to closures that don’t move captured values out of their body, but that might mutate the captured values. These closures can be called more than once.
  * `Fn` applies to closures that don’t move captured values out of their body and that don’t mutate captured values, as well as closures that capture nothing from their environment. These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently.
  * **Note: Functions can implement all three of the `Fn` traits too. If what we want to do doesn’t require capturing a value from the environment, we can use the name of a function rather than a closure where we need something that implements one of the `Fn` traits. For example, on an `Option<Vec<T>>` value, we could call `unwrap_or_else(Vec::new)` to get a new, empty vector if the value is `None`.**

### FnOnce
  * ```Rust
      impl<T> Option<T> {
        pub fn unwrap_or_else<F>(self, f: F) -> T
        where
          F: FnOnce() -> T
        {
          match self {
            Some(x) => x,
            None => f(),
          }
        }
      }
    ```
* Recall that `T` is the generic type representing the type of the value in the `Some` variant of an `Option`. That type `T` is also the return type of the `unwrap_or_else` function: code that calls `unwrap_or_else` on an `Option<String>`, for example, will get a `String`.
* Next, notice that the `unwrap_or_else` function has the additional generic type parameter `F`. The `F` type is the type of the parameter named `f`, which is the closure we provide when calling `unwrap_or_else`.
* The trait bound specified on the generic type `F` is `FnOnce() -> T`, which means `F` must be able to be called once, take no arguments, and return a `T`. Using `FnOnce` in the trait bound expresses the constraint that `unwrap_or_else` is only going to call `f` one time, at most. In the body of `unwrap_or_else`, we can see that if the `Option` is `Some`, `f` won’t be called. If the `Option` is `None`, `f` will be called once. Because all closures implement `FnOnce`, `unwrap_or_else` accepts the largest variety of closures and is as flexible as it can be.

### FnMut
* Now let’s look at the standard library method `sort_by_key`, defined on slices, to see how that differs from `unwrap_or_else` and why `sort_by_key` uses `FnMut` instead of `FnOnce` for the trait bound.
* The closure gets one argument in the form of a reference to the current item in the slice being considered, and returns a value of type `K` that can be ordered. 
* This function is useful when you want to sort a slice by a particular attribute of each item.
  * ```Rust
      #[derive(Debug)]
      struct Rectangle {
        width: u32,
        height: u32,
      }
      
      fn main() {
        let mut list = [
            Rectangle { width: 10, height: 1 },
            Rectangle { width: 3, height: 5 },
            Rectangle { width: 7, height: 12 },
        ];
        list.sort_by_key(|r| r.width);
        println!("{:#?}", list);
      }
    ```
* The reason `sort_by_key` is defined to take an `FnMut` closure is that it calls the closure multiple times: once for each item in the slice. 
* The closure `|r| r.width` doesn’t capture, mutate, or move anything out from its environment, so it meets the trait bound requirements.
* The following example of a closure that implements just the `FnOnce` trait, because it moves a value out of the environment. The compiler won’t let us use this closure with `sort_by_key`:
  * ```Rust
      fn main() {
       let mut list = [
          Rectangle { width: 10, height: 1 },
          Rectangle { width: 3, height: 5 },
          Rectangle { width: 7, height: 12 },
        ];

        let mut sort_operations = vec![];
        let value = String::from("by key called");
        
        list.sort_by_key(|r| {
          sort_operations.push(value);
          r.width
        });
            
        println!("{:#?}", list);
      }
    ```
* The above is a contrived, convoluted way (that doesn’t work) to try and count the number of times `sort_by_key` gets called when sorting `list`. 
* The code attempts to do this counting by pushing `value` - a String from the closure’s environment - into the `sort_operations` vector. 
* The closure captures value and then moves value out of the closure by transferring ownership of value to the `sort_operations` vector. 
* This closure can be called once; trying to call it a second time wouldn’t work because `value` would no longer be in the environment to be pushed into `sort_operations` again!
* Therefore, this closure only implements `FnOnce`. When we try to compile this code, we get this error that value can’t be moved out of the closure because the closure must implement `FnMut`.

* The code above can be fixed by:
  * ```Rust
      fn main() {
        let mut num_sort_operations = 0;
        list.sort_by_key(|r| {
          // this works
          num_sort_operations += 1;
          r.width
        });
        
        println!("{:#?}, sorted in {num_sort_operations} operations", list);
      }
    ```
* The closure in the code above works with `sort_by_key` because it is only capturing a mutable reference to the `num_sort_operations` counter and can therefore be called more than once.

### Processing a Series of Items with Iterators
* The iterator pattern allows you to perform some task on a sequence of items in turn.
* An iterator is responsible for the logic of iterating over each item and determining when the sequence has finished.
* In Rust, iterators are *lazy*, meaning they have no effect until you call methods that consume the iterator to use it up.
* In the following example, we separate the creation of the iterator from the use of the iterator in the `for` loop. When the `for` loop is called using the iterator in `v1_iter`, each element in the iterator is used in one iteration of the loop, which prints out each value:
  * ```Rust
      let v1 = vec![1, 2, 3];
      let v1_iter = v1.iter();

      for val in v1_iter {
          println!("Got: {val}");
      }
    ```
* In languages that don’t have iterators provided by their standard libraries, you would likely write this same functionality by starting a variable at index 0, using that variable to index into the vector to get a value, and incrementing the variable value in a loop until it reached the total number of items in the vector.
* Iterators handle all of that logic for you, cutting down on repetitive code you could potentially mess up. Iterators give you more flexibility to use the same logic with many different kinds of sequences, not just data structures you can index into, like vectors.

### The Iterator Trait and the next Method
* All iterators implement a trait named `Iterator` that is defined in the standard library. 
* The definition of the trait looks like this:
  * ```Rust
      pub trait Iterator {
        type Item; // associated type explained in later chapters

        fn next(&mut self) -> Option<Self::Item>;
        
        // methods with default implementations elided
      }
    ```
* The above code says implementing the `Iterator` trait requires that you also define an `Item` type, and this `Item` type is used in the return type of the `next` method. 
* In other words, the `Item` type will be the type returned from the iterator.
* The `Iterator` trait only requires implementors to define one method: the `next` method, which returns one item of the iterator at a time, wrapped in `Some`, and, when iteration is over, returns `None`.
* Note that if we are calling `next` on an iterator, we needed to make it mutable: calling the `next` method on an iterator changes internal state that the iterator uses to keep track of where it is in the sequence.
* Each call to `next` eats up an item from the iterator. We didn’t need to make `v1_iter` mutable when we used a `for` loop because the loop took ownership of `v1_iter` and made it mutable behind the scenes.
* Also note that the values we get from the calls to `next` are immutable references to the values in the vector. 
* The `iter` method produces an iterator over immutable references. If we want to create an iterator that takes ownership of `v1` and returns owned values, we can call `into_iter` instead of `iter`. 
* Similarly, if we want to iterate over mutable references, we can call `iter_mut` instead of `iter`.

### Methods That Consume the Iterator
* The `Iterator` trait has a number of different methods with default implementations provided by the standard library.
* Some of these methods call the next method in their definition, which is why you’re required to implement the `next` method when implementing the `Iterator` trait.
* Methods that call `next` are called *consuming adapters* because calling them uses up the iterator. 
* One example is the `sum` method, which takes ownership of the iterator and iterates through the items by repeatedly calling `next`, thus consuming the iterator. 
* As it iterates through, it adds each item to a running total and returns the total when iteration is complete.
  * ```Rust
      let total: i32 = v1_iter.sum();
    ```
* We aren’t allowed to use `v1_iter` after the call to `sum` because sum takes ownership of the iterator we call it on.

### Methods That Produce Other Iterators
* *Iterator adapters* are methods defined on the `Iterator` trait that don’t consume the iterator. Instead, they produce different iterators by changing some aspect of the original iterator.
* Example of calling the iterator adapter method `map`, which takes a closure to call on each item as the items are iterated through. The `map` method returns a new iterator that produces the modified items. The closure here creates a new iterator in which each item from the vector will be incremented by 1:
  * ```Rust
         let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    ```
* If we don't call `collect`, we will get a warning and the code won't do anything.
* Since iterators are *lazy*, we need to call `collect` to consume the iterator.

### Using Closures That Capture Their Environment
* Many iterator adapters take closures as arguments, and commonly the closures we’ll specify as arguments to iterator adapters will be closures that capture their environment.
* The `shoes_in_size` function takes ownership of a vector of shoes and a shoe size as parameters. It returns a vector containing only shoes of the specified size.
* In the body of `shoes_in_size`, we call `into_iter` to create an iterator that takes ownership of the vector. Then we call `filter` to adapt that iterator into a new iterator that only contains elements for which the closure returns `true`.
* The closure captures the `shoe_size` parameter from the environment and compares the value with each shoe’s size, keeping only shoes of the size specified. 
* Finally, calling `collect` gathers the values returned by the adapted iterator into a vector that’s returned by the function.

### Improving minigrep
* With knowledge about iterators, we can use them in `Config::build` and `search` for clarity.

### Removing a clone Using an Iterator
* In minigrep, we added code that took a slice of String values and created an instance of the `Config` struct by indexing into the slice and cloning the values, allowing the `Config` struct to own those values.
* We needed `clone` there because we have a slice with `String` elements in the parameter `args`, but the build function doesn’t own `args`. 
* With our new knowledge about iterators, we can change the `build` function to take ownership of an iterator as its argument instead of borrowing a slice. 
  * We’ll use the iterator functionality instead of the code that checks the length of the slice and indexes into specific locations.
  * This will clarify what the `Config::build` function is doing because the iterator will access the values.
  * Once `Config::build` takes ownership of the iterator and stops using indexing operations that borrow, we can move the `String` values from the iterator into `Config` rather than calling clone and making a new allocation.
* The `env::args()` function returns an iterator.
  * Rather than collecting the iterator values into a vector and then passing a slice to `Config::build`, now we’re passing ownership of the iterator returned from `env::args` to `Config::build` directly.
  * We then need to change the signature of `Config::build`:
    * `pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &str> {` 
    * We’ve updated the signature so the parameter `args` has a generic type with the trait bounds `impl Iterator<Item = String>` instead of `&[String]`.
    * This usage of the `impl Trait` syntax means that `args` can be any type that implements the `Iterator` type and returns `String` items.
    * Because we’re taking ownership of `args` and we’ll be mutating `args` by iterating over it, we can add the `mut` keyword into the specification of the `args` parameter to make it mutable.

### Using Iterator Trait Methods Instead of Indexing
* We need to fix the body of `Config::build`. Because `args` implements the `Iterator` trait, we know we can call the `next` method on it.

### Making Code Clearer with Iterator Adapters
* We can also take advantage of iterators in the search function in minigrep.
* We can write the code in a more concise way using iterator adapter methods. 
  * Doing so also lets us avoid having a mutable intermediate `results` vector. 
  * The functional programming style prefers to minimize the amount of mutable state to make code clearer. 
  * Removing the mutable state might enable a future enhancement to make searching happen in parallel because we wouldn’t have to manage concurrent access to the `results` vector.

### Choosing Between Loops and Iterators
* Most Rust programmers prefer to use the iterator style. 
* It’s a bit tougher to get the hang of at first, but once you get a feel for the various iterator adapters and what they do, iterators can be easier to understand.
* Instead of fiddling with the various bits of looping and building new vectors, the code focuses on the high-level objective of the loop.

### Comparing Performance: Loops vs. Iterators
* Running a benchmark by loading the entire contents of "The Adventures of Sherlock Holmes" by Sir Arthur Conan Doyle into a String and looking for the word "the" in the contents showed the `iter` version of `search` to be faster than the `for` loop one.
* Iterators, although a high-level abstraction, get compiled down to roughly the same code as if you’d written the lower-level code yourself. 
* Iterators are one of Rust’s *zero-cost abstractions*, by which we mean that using the abstraction imposes no additional runtime overhead.
* Another example:
  * ```Rust
      let buffer: &mut [i32];
      let coefficients: [i64; 12];
      let qlp_shift: i16;

      for i in 12..buffer.len() {
        let prediction = coefficients.iter()
                                     .zip(&buffer[i - 12..i])
                                     .map(|(&c, &s)| c * s as i64
                                     .sum::<i64>() >> qlp_shift;
        let delta = buffer[i];
        buffer[i] = prediction as i32 + delta;
      }
    ```
* To calculate the value of `prediction`, this code iterates through each of the 12 values in `coefficients` and uses the `zip` method to pair the `coefficient` values with the previous 12 values in `buffer`. 
* Then, for each pair, it multiplies the values together, sums all the results, and shifts the bits in the sum `qlp_shift` bits to the right.
* Calculations in applications like audio decoders often prioritize performance most highly.
* Here, we’re creating an iterator, using two adapters, and then consuming the value.
* This code compiles down to the same assembly you’d write. There’s no loop at all corresponding to the iteration over the values in `coefficients`.
* Rust knows that there are 12 iterations, so it *“unrolls”* the loop.
  * *Unrolling* is an optimization that removes the overhead of the loop controlling code and instead generates repetitive code for each iteration of the loop.
* All of the coefficients get stored in registers, which means accessing the values is very fast. 
* There are no bounds checks on the array access at runtime. All of these optimizations that Rust is able to apply make the resultant code extremely efficient.