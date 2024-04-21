import Cell from "./Components/Cell";

export default class Board {
    private static _instance: Board;

    public readonly board: HTMLDivElement;

    private _width: number = 0;
    private _height: number = 0;

    private _cellSize: number = 0;

    private _cells: Cell[] = [];

    public drawMode: DrawType = DrawType.Alive;

    private constructor() {
        const board: HTMLDivElement | null = document.querySelector("#board");

        if (!board) {
            throw new Error("Failed to get board element");
        } else {
            this.board = board;
        }

        let isMouseDown: boolean = false;

        this.board.addEventListener("mousedown", (event: MouseEvent) => {
            if (event.buttons === 1) {
                isMouseDown = true;

                const cell: Cell | undefined = this._cells.find(
                    (cell: Cell) => cell === event.target,
                );

                if (cell) {
                    const state: boolean = this.drawMode === DrawType.Alive;

                    if (state !== cell.alive) {
                        cell.alive = state;
                    } else {
                        cell.toggleAlive();
                    }
                }
            }
        });

        this.board.addEventListener("mouseup", () => {
            isMouseDown = false;
        });

        this.board.addEventListener("mouseover", (event: MouseEvent) => {
            if (isMouseDown && event.buttons === 1) {
                const cell: Cell | undefined = this._cells.find(
                    (cell: Cell) => cell === event.target,
                );

                if (cell) {
                    cell.alive = this.drawMode === DrawType.Alive;
                }
            }
        });

        this.board.addEventListener("mouseenter", (event: MouseEvent) => {
            if (event.buttons === 1) {
                isMouseDown = true;
            }
        });

        this.board.addEventListener("mouseleave", () => {
            isMouseDown = false;
        });
    }

    public static get instance(): Board {
        if (!this._instance) {
            this._instance = new Board();
        }
        return this._instance;
    }

    public get width(): number {
        return this._width;
    }

    public get height(): number {
        return this._height;
    }

    public get size(): Size {
        return {
            width: this._width,
            height: this._height,
        };
    }

    public get cellSize(): number {
        return this._cellSize;
    }

    public get cells(): Cell[] {
        return this._cells;
    }

    public get cellsIds(): string[] {
        return this._cells.map((cell: Cell) => cell.id);
    }

    public get aliveCells(): Cell[] {
        return this._cells.filter((cell: Cell) => cell.alive);
    }

    public get aliveCellsIds(): string[] {
        return this.aliveCells.map((cell: Cell) => cell.id);
    }

    public get deadCells(): Cell[] {
        return this._cells.filter((cell: Cell) => !cell.alive);
    }

    public get deadCellsIds(): string[] {
        return this.deadCells.map((cell: Cell) => cell.id);
    }

    public createBoard(size: Size, cellSize: number): void {
        if (size.width < 1) {
            throw new Error(
                `Width must be a positive number, got ${size.width}`,
            );
        }
        if (size.height < 1) {
            throw new Error(
                `Height must be a positive number, got ${size.height}`,
            );
        }

        if (cellSize < 1) {
            throw new Error(
                `Cell size must be a positive number, got ${cellSize}`,
            );
        }

        this._width = size.width;
        this._height = size.height;
        this._cellSize = cellSize;

        this.board.innerHTML = "";

        // Set grid-template-columns and grid-template-rows to repeat(size, 20px)
        this.board.style.setProperty(
            "grid-template-columns",
            `repeat(${size.width}, ${cellSize}px)`,
        );
        this.board.style.setProperty(
            "grid-template-rows",
            `repeat(${size.height}, ${cellSize}px)`,
        );

        // Create cells
        for (let y = 0; y < size.height; y++) {
            for (let x = 0; x < size.width; x++) {
                const cell: Cell = new Cell();

                this._cells.push(cell);

                this.board.appendChild(cell);
            }
        }
    }
}

export type Size = {
    width: number;
    height: number;
};

export enum DrawType {
    Dead = 0,
    Alive = 1,
}
