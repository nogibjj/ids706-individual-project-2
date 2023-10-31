# Rust CLI Binary with SQLite

## <u>Rust</u> Source Code

* ``my_project/src/database.rs`` - This Rust code handles database interactions, allowing the program to interact with an SQLite database for tasks like table <u>creation</u>, data <u>insertion</u>, <u>reading</u>, <u>updating</u>, and <u>deleting</u> within an SQLite database.<br>
* ``my_project/src/main.rs`` -  This Rust code is the starting point of the program, where it interprets user inputs from the command line and connects these inputs to the appropriate database functions defined in ``database.rs``.<br><br>


## Usage of <u>Github CoPilot</u>
<u><b>Using GitHub Copilot in ``main.rs``:</b></u><br>

<b>1. Enhancing Error Handling:</b><br>
Scenario: Faced difficulties in implementing robust error handling for database connections and operations.<br>
With Copilot: Copilot recommended practical examples of using Rust's Result and Option types for more effective error handling, which were seamlessly integrated into our codebase.<br><br>

<b>2. Refactoring Function Calls:</b><br>
Scenario: Needed a more streamlined and efficient way to structure calls to CRUD operations.<br>
With Copilot: Copilot provided a cleaner and more organized way to structure function calls from the database.rs module, enhancing both readability and maintainability.<br><br>


<u><b>Using GitHub Copilot in ``database.rs``:</b></u><br>
<b>1. Structuring CRUD Operations:</b><br>
Scenario: Unsure about the optimal structure and best practices for CRUD functions with SQLite in Rust.<br>
With Copilot: GitHub Copilot provided adaptable templates for CRUD operations, which were tailored to our specific requirements for SQLite database interactions.<br><br>

<b>2. Optimizing SQL Queries:</b><br>
Scenario: Writing SQL queries that are both efficient and secure posed a challenge.<br>
With Copilot: Copilot suggested improvements for query optimization and the use of prepared statements to mitigate SQL injection risks.<br><br>

