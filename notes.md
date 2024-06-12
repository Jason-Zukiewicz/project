Notes to Joel:

1. in the project dir there is a .env

   - this is a file for environmental variables
   - in this case the address and ports of the front and back, and a database
   - you can change the frontend as needed

2. Rust uses the command cargo to build and run

   - cargo build
   - cargo run // will also build
   - I have it configured to use cargo watch, you may need to do
     - cargo install cargo-watch
   - cargo watch will automatically rerun the build when you save, you may need to turn off autosave for this

   - One Important type for you to learn is Result
     - you may see it as - Result<T, Err>
     - What this means is that it is a box that will either hold a value or an error
     - This is used to propogate the errors back up the call stack so you can track where it came from and posible solve it when you can
   - There is also - Option(t) where it can either be Some(T) or None
     - This is basically just a box of something or nothing, rather than NULL or nullptr

- the way moving between files and libs is handled is with ::
- and folder have bundlers if you will and that is mod.rs
  - so look at the web folder
    - todos.rs // handles routes with "/todos"
    - root.rs //handles routes with "/"
    - mod.rs // Combines all the routes of todos and root
  - at the top we have ... to tell where to find their functions
    - mod todos;
    - mod root;
  - then in all_routes() we use ... to get those specific functions
    - root::routes()
    - todos::routes()
  - and in main() we call web::all_routes() since that is in the web/mod.rs
  - Main.rs --> web::mod.rs --> ALL THE INDIVIDUAL ROUTES

3. I have set up the routes for index and todos

   - TODO json

     - "title" : String
     - This is what you need to pass to the backend

   - Working Routes

     - get "/todos/id" -// Get A single todo by id
     - delete "todos/id" -// Delete A single todo by id
     - get "/todos/ -// Get A List of tod
     - post "/todos -// Receives a todo and stores it in the database

   - In Model.rs
     - The struct for Todos and TodosForCreate
     - The "Database store" for Todos and TodosForCreate
   - In web/todos.rs
     - Routing
