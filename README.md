# Rust CLI Binary with SQLite

## Rust Source Code

* ``my_project/src/database.rs`` - This Rust code handles database interactions, allowing the program to interact with an SQLite database for tasks like table <u>creation</u>, data <u>insertion</u>, <u>reading</u>, <u>updating</u>, and <u>deleting</u> within an SQLite database.<br>
* ``my_project/src/main.rs`` -  This Rust code is the starting point of the program, where it interprets user inputs from the command line and connects these inputs to the appropriate database functions defined in ``database.rs``.<br><br>


## Usage of Github CoPilot
Using GitHub Copilot in main.rs:
1. Simplifying Command-Line Argument Parsing:

Scenario: Struggling to remember the exact syntax for implementing command-line argument parsing using the clap crate.
With Copilot: GitHub Copilot suggested a concise and effective template for argument parsing tailored to our CLI's specific CRUD operations.
2. Enhancing Error Handling:

Scenario: Faced difficulties in implementing robust error handling for database connections and operations.
With Copilot: Copilot recommended practical examples of using Rust's Result and Option types for more effective error handling, which were seamlessly integrated into our codebase.
3. Refactoring Function Calls:

Scenario: Needed a more streamlined and efficient way to structure calls to CRUD operations.
With Copilot: Copilot provided a cleaner and more organized way to structure function calls from the database.rs module, enhancing both readability and maintainability.
Using GitHub Copilot in database.rs:
1. Structuring CRUD Operations:

Scenario: Unsure about the optimal structure and best practices for CRUD functions with SQLite in Rust.
With Copilot: GitHub Copilot provided adaptable templates for CRUD operations, which were tailored to our specific requirements for SQLite database interactions.
2. Optimizing SQL Queries:

Scenario: Writing SQL queries that are both efficient and secure posed a challenge.
With Copilot: Copilot suggested improvements for query optimization and the use of prepared statements to mitigate SQL injection risks.
3. Effective Database Connection Management:

Scenario: Concerns about efficiently managing and closing database connections.
With Copilot: Copilot proposed a robust pattern for managing database connections, ensuring they are properly closed after operations, thus improving overall resource management.
