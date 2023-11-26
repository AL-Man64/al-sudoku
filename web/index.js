import init, { Sudoku } from "./al-sudoku/al_sudoku.js";

function main() {
  let buttons = document.querySelectorAll("button");
  buttons[0].addEventListener("click", () => {
    let sudokuInput = Array.from(document.querySelectorAll("input")).map(
      (x) => {
        return Number.parseInt(x.value);
      }
    );
    let game = new Sudoku(sudokuInput);
    game = game.solve();
    let result = game.get_fields();
    let inputs = Array.from(document.querySelectorAll("input"));
    for (let i = 0; i < 81; i++) {
      inputs[i].value = result[i] !== 0 ? result[i] : "";
    }
  });
  buttons[1].addEventListener("click", () => {
    let inputs = Array.from(document.querySelectorAll("input"));
    for (let i = 0; i < 81; i++) {
      inputs[i].value = "";
    }
  });
}

init().then(main);
