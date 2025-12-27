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

function renderBoard() {
  const boardDiv = document.getElementById("board");
  boardDiv.innerHTML = "";

  const state = JSON.parse(game.get_state());
  const board = state.board;

  for (let i = 0; i < 9; i++) {
    const cell = document.createElement("button");
    cell.className = "cell";
    cell.textContent = board[i];
    if (board[i] === "X") cell.classList.add("x");
    if (board[i] === "O") cell.classList.add("o");

    const isGameOver = state.status !== "Ongoing";
    const isOccupied = board[i] !== "";
    cell.disabled = isGameOver || isOccupied;

    cell.onclick = () => makeMove(i);
    boardDiv.appendChild(cell);
  }
}

function updateStatus() {
  const state = JSON.parse(game.get_state());

  document.getElementById("currentPlayer").textContent = state.currentPlayer;
  document.getElementById("status").textContent = state.status;

  const statusMessage = document.getElementById("statusMessage");
  if (state.status === "Ongoing") {
    statusMessage.textContent = `${state.currentPlayer}'s turn`;
    statusMessage.className = "status-message ongoing";
  } else if (state.status === "Draw") {
    statusMessage.textContent = "🤝 It's a Draw!";
    statusMessage.className = "status-message draw";
  } else if (state.status === "WinX") {
    statusMessage.textContent = "🎉 X Wins!";
    statusMessage.className = "status-message win";
  } else if (state.status === "WinO") {
    statusMessage.textContent = "🎉 O Wins!";
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
