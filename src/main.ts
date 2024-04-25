import Board, {DrawType} from "./Board";

window.addEventListener("load", async () => {
    menuHandler();

    await Board.instance.createBoard({width: 20, height: 20}, 20);
});

function menuHandler(): void {
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

    const buttonSize: HTMLButtonElement | null =
        document.querySelector("#sizeSetter");

    if (!buttonSize) {
        throw new Error("Failed to get size setter button element");
    }

    buttonSize.addEventListener("click", async () => {
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

        await Board.instance.createBoard({width, height}, 20);
    });

    const buttonKill: HTMLButtonElement | null =
        document.querySelector("#boardKill");

    if (!buttonKill) {
        throw new Error("Failed to get clear button element");
    }

    buttonKill.addEventListener("click", async () => {
        await Board.instance.killBoard();
    });

    const buttonDraw: HTMLButtonElement | null =
        document.querySelector("#boardDraw");

    if (!buttonDraw) {
        throw new Error("Failed to get draw button element");
    }

    buttonDraw.addEventListener("click", async () => {
        const type: string = buttonDraw.dataset.type ?? "";

        switch (type) {
            case "alive":
                buttonDraw.innerText = "Draw dead";
                buttonDraw.dataset.type = "dead";
                Board.instance.drawMode = DrawType.Dead;
                break;
            case "dead":
                buttonDraw.innerText = "Draw alive";
                buttonDraw.dataset.type = "alive";
                Board.instance.drawMode = DrawType.Alive;
                break;
            default:
                throw new Error(`Unknown type ${type} for draw button`);
        }
    });

    const buttonStep: HTMLButtonElement | null =
        document.querySelector("#step");

    if (!buttonStep) {
        throw new Error("Failed to get step button element");
    }

    buttonStep.addEventListener("click", async () => {
        await Board.instance.step();
    });

    const buttonRun: HTMLButtonElement | null = document.querySelector("#run");

    if (!buttonRun) {
        throw new Error("Failed to get run button element");
    }

    buttonRun.addEventListener("click", async () => {
        if (Board.instance.running) {
            Board.instance.stop();
            buttonRun.innerText = "Play";
        } else {
            Board.instance.run();
            buttonRun.innerText = "Playing...";
        }
    });

    const buttonSetBoardSleepTime: HTMLButtonElement | null =
        document.querySelector("#setBoardSleepTime");

    if (!buttonSetBoardSleepTime) {
        throw new Error("Failed to get set board sleep time button element");
    }

    const inputBoardSleepTime: HTMLInputElement | null =
        document.querySelector("#boardSleepTime");

    if (!inputBoardSleepTime) {
        throw new Error("Failed to get board sleep time input element");
    }

    buttonSetBoardSleepTime.addEventListener("click", async () => {
        const sleepTime: number = parseInt(inputBoardSleepTime.value, 10);

        if (Number.isNaN(sleepTime) || sleepTime < 0) {
            alert(`Please enter a positive number, got ${sleepTime}`);
            inputBoardSleepTime.value = Board.instance.sleepTime.toString();
            return;
        }

        Board.instance.sleepTime = sleepTime;
    });
}
