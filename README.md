# Rust CLI Binary with SQLite

![Rust CLI](https://github.com/nogibjj/ids706-individual-project-2/actions/workflows/rust_build.yml/badge.svg)

## 🛠️ <u>Rust</u> Source Code

* ``my_project/src/database.rs`` - This Rust code orchestrates database activities, enabling the program to interact with an SQLite database for tasks like table <u>creation</u>🔨, data <u>insertion</u>📌, <u>reading</u>🔍, <u>updating</u>🔄, and <u>deleting</u>🗑️.<br>
* ``my_project/src/main.rs`` - This Rust code acts as the entry point, interpreting user inputs from the command line and directing them to the corresponding database functions in ``database.rs``. 🚀 <br><br> 

## 🧑‍💻 Usage of <u>Github CoPilot</u>

### <u><b>Using GitHub Copilot in `main.rs`:</b></u>

<b>1. Enhancing Error Handling:</b><br>
  <i>Scenario:</i> Encountered challenges in robust error handling for database connections and operations.<br>
  <i>With Copilot:</i> Recommended practical examples of using Rust's Result and Option types for more effective error handling, seamlessly integrated into the codebase. <br><br>
      
<b>2. Refactoring Function Calls:</b><br>
  <i>Scenario:</i> Sought a more streamlined approach to structure calls to CRUD operations.<br>
  <i>With Copilot:</i> Provided a cleaner and more organized way to structure function calls from the `database.rs` module, enhancing readability and maintainability. <br><br>


### <u><b>Using GitHub Copilot in `database.rs`:</b></u>

<b>1. Structuring CRUD Operations:</b><br>
  <i>Scenario:</i> Uncertain about the optimal structure and best practices for CRUD functions with SQLite in Rust.<br>
  <i>With Copilot:</i> Provided adaptable templates for CRUD operations, tailored to our specific requirements for SQLite database interactions. <br><br>
      
<b>2. Optimizing SQL Queries:</b><br>
  <i>Scenario:</i> Faced challenges in crafting SQL queries that are both efficient and secure.<br>
  <i>With Copilot:</i> Suggested improvements for query optimization and the use of prepared statements to mitigate SQL injection risks. <br><br>


