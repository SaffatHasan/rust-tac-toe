let game;

// Import the WASM module
import init, { WasmGameEngine } from "../pkg/rust_tac_toe_wasm.js";

async function initializeGame() {
  try {
    // Initialize the WASM module
    await init();

    // Create a new game instance
    game = new WasmGameEngine();

    // Hide loading and show content
    document.getElementById("loading").style.display = "none";
    document.getElementById("content").style.display = "block";

    // Render the initial board
    renderBoard();
    updateStatus();
  } catch (err) {
    console.error("Failed to initialize WASM:", err);
    document.getElementById("loading").innerHTML =
      '<p style="color: #e74c3c;">Error loading WebAssembly. Check console for details.</p>';
  }
}

// Store elements for reuse to avoid re-creating them
let cellElements = [];

function renderBoard() {
  const boardDiv = document.getElementById("board");
  const state = game.get_state();
  const board = state.board;
  const gameStatus = state.status.type;

  if (boardDiv.childElementCount === 0) {
    for (let i = 0; i < 9; i++) {
      const cell = document.createElement("button");
      cell.className = "cell";
      cell.onclick = () => makeMove(i);
      boardDiv.appendChild(cell);
      cellElements.push(cell);
    }
    return;
  }

  // Otherwise update existing
  cellElements.forEach((cell, i) => {
    const val = board[i];
    cell.textContent = val;
    cell.className = `cell`;
    if (val === "X") cell.classList.add("x");
    if (val === "O") cell.classList.add("o");

    const isGameOver = gameStatus !== "Ongoing";
    const isOccupied = val !== undefined;
    cell.disabled = isGameOver || isOccupied;
  });
}

function updateStatus() {
  const state = game.get_state();

  document.getElementById("currentPlayer").textContent = state.currentPlayer;
  document.getElementById("status").textContent = state.status.type;

  const statusMessage = document.getElementById("statusMessage");
  if (state.status.type === "Ongoing") {
    statusMessage.textContent = `${state.currentPlayer}'s turn`;
    statusMessage.className = "status-message ongoing";
  } else if (state.status.type === "Draw") {
    statusMessage.textContent = "🤝 It's a Draw!";
    statusMessage.className = "status-message draw";
  } else if (state.status.type === "Win") {
    const winner = state.status.value;
    statusMessage.textContent = `🎉 ${winner} Wins!`;
    statusMessage.className = "status-message win";
  }
}

function makeMove(position) {
  try {
    game.play_move(position);
    renderBoard();
    updateStatus();
  } catch (err) {
    console.error("Move failed:", err);
  }
}

window.resetGame = function () {
  try {
    game.reset();
    renderBoard();
    updateStatus();
  } catch (err) {
    console.error("Reset failed:", err);
  }
};

// Initialize the game when the page loads
initializeGame();
