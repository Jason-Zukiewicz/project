// Get the element with the class 'container'
const containerElement = document.querySelector('.container');

// Change the text content of the container element
containerElement.textContent = 'index.html, styles.css, and main.js are communicating.';

async function postRequest() {
    try {
        const response = await fetch('http://127.0.0.1:9999/login', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ username: 'user',  password: "pass"}) // JSON stringified data
        });

        const data = await response.json();
        console.log('Response from backend: Login: ', data);

        response = await fetch('http://127.0.0.1:9999/todos', {
           method: 'POST',
             headers: {
                 'Content-Type': 'application/json'
             },
             body: JSON.stringify({ title: 'test' }) // JSON stringified data
         });
         data = await response.json();
        console.log('Response from backend: TODOS: ', data);
    } catch (error) {
        console.error('Error connecting to backend:', error);
    }
}

// document.getElementById('postButton').addEventListener('click', postRequest);

document.getElementById('postButton').addEventListener('click', addTodo);

function addTodo() {
    // Create the new todo card elements
    const newTodoCard = document.createElement('div');
    newTodoCard.className = 'todo-cards';

    const todoTitle = document.createElement('strong');
    todoTitle.className = 'todo-title';
    todoTitle.innerHTML = '<u>TODO HEADER</u>';

    const todoIdNum = document.createElement('strong');
    todoIdNum.innerHTML = '<i class="todo-id-num">ID: </i>';

    const br1 = document.createElement('br');
    const br2 = document.createElement('br');

    const todoMsg = document.createElement('p');
    todoMsg.className = 'todo-msg';
    todoMsg.textContent = 'This is a new todo card.';

    // Append the elements to the new todo card
    newTodoCard.appendChild(todoTitle);
    newTodoCard.appendChild(todoIdNum);
    newTodoCard.appendChild(br1);
    newTodoCard.appendChild(br2);
    newTodoCard.appendChild(todoMsg);

    // Append the new todo card to the todos div
    document.getElementById('todos').appendChild(newTodoCard);
}
