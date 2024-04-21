import {invoke} from "@tauri-apps/api/tauri";
import Board from "./Board";

window.addEventListener("load", () => {
    handleGenerateBoard();

    Board.instance.createBoard({width: 20, height: 20}, 20);
    
});

function handleGenerateBoard(): void {
    const inputWidth: HTMLInputElement | null =
        document.querySelector("#boardWidth");

    if (!inputWidth) {
        throw new Error("Failed to get input element");
    }

    const inputHeight: HTMLInputElement | null =
        document.querySelector("#boardHeight");

    if (!inputHeight) {
        throw new Error("Failed to get input element");
    }

    const button: HTMLButtonElement | null =
        document.querySelector("#boardGenerator");

    if (!button) {
        throw new Error("Failed to get button element");
    }

    button.addEventListener("click", async () => {
        const width: number = parseInt(inputWidth.value, 10);

        if (Number.isNaN(width) || width < 1) {
            alert(`Please enter a positive number, got ${width}`);
            inputWidth.value = Board.instance.width.toString();
            return;
        }

        const height: number = parseInt(inputHeight.value, 10);

        if (Number.isNaN(height) || height < 1) {
            alert(`Please enter a positive number, got ${height}`);
            inputHeight.value = Board.instance.height.toString();
            return;
        }

        Board.instance.createBoard({width, height}, 20);
    });
}
