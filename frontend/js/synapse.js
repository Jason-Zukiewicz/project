document.getElementById('easy-btn').addEventListener('click', function() {
    setDifficulty('easy', 'chartreuse');
});

document.getElementById('medium-btn').addEventListener('click', function() {
    setDifficulty('medium', 'gold');
});

document.getElementById('hard-btn').addEventListener('click', function() {
    setDifficulty('hard', 'rgb(255, 153, 0)');
});

document.getElementById('master-btn').addEventListener('click', function() {
    setDifficulty('master', 'crimson');
});

function setDifficulty(difficulty, color) {
    const gameBoard = document.querySelector('.game-board');
    gameBoard.className = 'game-board'; // Reset classes
    gameBoard.classList.add(difficulty); // Add the new difficulty class

    document.documentElement.style.setProperty('--difficulty-neon', color);
}
