const newTodoInput = document.getElementById('new-todo');
const addBtn = document.getElementById('add-btn');
const todoList = document.getElementById('todo-list');

let todos = JSON.parse(localStorage.getItem('todos')) || [];

function renderTodos() {
  todoList.innerHTML = '';
  todos.forEach((todo, index) => {
    const todoCard = document.createElement('div');
    todoCard.className = 'todo-card';
    todoCard.innerHTML = `
      <h3>${todo.title}</h3>
      <p>Created: ${new Date(todo.createdAt).toLocaleString()}</p>
      <div class="todo-actions">
        <button class="complete-btn" onclick="completeTodo(${index})">Complete</button>
        <button class="delete-btn" onclick="deleteTodo(${index})">Delete</button>
      </div>
    `;
    todoList.appendChild(todoCard);
  });
}

function addTodo() {
  const title = newTodoInput.value.trim();
  if (title) {
    const newTodo = {
      title,
      createdAt: new Date().toISOString()
    };
    todos.unshift(newTodo); // Add new todo at the beginning
    localStorage.setItem('todos', JSON.stringify(todos));
    newTodoInput.value = '';
    renderTodos();
  }
}

function completeTodo(index) {
  const todoCard = todoList.children[index];
  todoCard.classList.add('pop');


  setTimeout(() => {
    todos.splice(index, 1);
    localStorage.setItem('todos', JSON.stringify(todos));
    renderTodos();
  }, 300); // Match this duration with the CSS animation duration
}

function deleteTodo(index) {
  const todoCard = todoList.children[index];
  todoCard.classList.add('pop');


  setTimeout(() => {
    todos.splice(index, 1);
    localStorage.setItem('todos', JSON.stringify(todos));
    renderTodos();
  }, 300); // Match this duration with the CSS animation duration
}
if (addBtn) {
  addBtn.addEventListener('click', addTodo);
  newTodoInput.addEventListener('keypress', function(e) {
    if (e.key === 'Enter') {
      addTodo();
    }
  });
}

renderTodos();

