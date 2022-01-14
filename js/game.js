import { GameState } from 'wasm';
 
let game_state = GameState.new();

const handlePlayerMove = (r, c) => { 
    console.log('move ', r, c);
    const result = game_state.player_move(r, c);
    const success = result[0];
    const playerLast = result[1];
    const isGameOver = result[2]; 
    if (success){
        return [playerLast, isGameOver];
    }
}

const handleReset = () => {
    game_state = GameState.new();
    document.querySelectorAll("#grid div").forEach(e => {
        e.innerHTML = '';
    })
}

window.handleReset = handleReset;
window.handlePlayerMove = handlePlayerMove; 