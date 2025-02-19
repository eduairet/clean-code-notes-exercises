# Code Structure, Comments & Formatting

## Comments

- Most of the time comments are not necessary if the code is clean and self-explanatory
- Here are some signals of bad comments:
  - Redundant information
  - Dividers and block markers
  - Misleading comments
  - Commented-out code
- Good comments is text that actually adds up to the code, here are some examples:

  - Legal information

    ```Solidity
    // SPDX-License-Identifier: MIT

    pragma solidity ^0.8.0;

    // . . .
    ```

  - Information that can't be expressed in code

    ```Rust
    // This regular expression matches a valid Ethereum address
    let re = Regex::new(r"^0x[a-fA-F0-9]{40}$").unwrap();
    ```

  - Documentation comments

    ````Rust
    /// Adds two numbers together
    ///
    /// # Examples
    ///
    /// ```
    /// let result = add(1, 2);
    /// assert_eq!(result, 3);
    /// ```
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    ````

  - TODO comments

    ```Rust
    pub fn add(a: i32, b: i32) -> i32 {
        // TODO: Implement subtraction
    }

    ```

## Code Formatting

- Code formatting is important to make the code readable
- There are two types of code formatting:
  - Vertical formatting: How the code is structured in vertical space (e.g. line breaks, indentation)
    - Split gigantic files into smaller files, for example if you're creating classes in a single file, split them into separate files
    - Separate concepts in files that are related to each other
    - Keep related methods close to each other to avoid making the reader jump around the file
  - Horizontal formatting: How the code is structured in horizontal space (e.g. spaces, tabs)
    - Use indentation to show the hierarchy of the code (languages like Python and Rust rely on indentation to define the code structure)
    - Avoid long lines, if a line is too long, split it into multiple lines
    - Use clear but not unreadable long names for variables, functions, classes, etc.
- Good formatting can be achieved by following the language's style guide or by using a code formatter like `rustfmt` for Rust, `black` for Python, `prettier` for JavaScript, etc.
