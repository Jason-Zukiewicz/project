// Get the element with the class 'container'
const containerElement = document.querySelector('.container');

// Change the text content of the container element
containerElement.textContent = 'index.html, styles.css, and main.js are communicating.';

async function postRequest() {
    try {
        const response = await fetch('http://127.0.0.1:3000/todos', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ title: 'test' }) // JSON stringified data
        });
        const data = await response.json();
        console.log('Response from backend:', data);
    } catch (error) {
        console.error('Error connecting to backend:', error);
    }
}

document.getElementById('postButton').addEventListener('click', postRequest);