import {invoke} from "@tauri-apps/api";
import Cell from "./Cell";
import GameCell from "./Components/GameCell";

export default class Board {
    private static _instance: Board;

    public readonly board: HTMLDivElement;

    private _running: boolean = false;

    private _sleepTime: number = 100;

    private _width: number = 0;
    private _height: number = 0;

    private _cellSize: number = 0;

    private _cells: GameCell[] = [];

    public drawMode: DrawType = DrawType.Alive;

    private constructor() {
        const board: HTMLDivElement | null = document.querySelector("#board");

        if (!board) {
            throw new Error("Failed to get board element");
        } else {
            this.board = board;
        }

        let isMouseDown: boolean = false;

        this.board.addEventListener("mousedown", async (event: MouseEvent) => {
            if (event.buttons === 1) {
                isMouseDown = true;

                const cell: GameCell | undefined = this._cells.find(
                    (cell: GameCell) => cell === event.target,
                );

                if (cell) {
                    let state: boolean = this.drawMode === DrawType.Alive;

                    if (state === cell.alive) {
                        state = !state;
                    }

                    await this.updateCellState(cell, state);
                }
            }
        });

        this.board.addEventListener("mouseup", () => {
            isMouseDown = false;
        });

        this.board.addEventListener("mouseover", async (event: MouseEvent) => {
            if (isMouseDown && event.buttons === 1) {
                const cell: GameCell | undefined = this._cells.find(
                    (cell: GameCell) => cell === event.target,
                );

                const state: boolean = this.drawMode === DrawType.Alive;

                if (cell && cell.alive !== state) {
                    await this.updateCellState(
                        cell,
                        this.drawMode === DrawType.Alive,
                    );
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

    private async updateCellState(
        cell: GameCell,
        state: boolean,
    ): Promise<void> {
        await invoke("update_cell_state", {
            id: cell.id,
            newState: state,
        })
            .then((response) => {
                const result: [string, boolean] = response as [string, boolean];

                const id: string = result[0];
                const resultState: boolean = result[1];

                if (id === cell.id && resultState) {
                    cell.alive = state;
                }
            })
            .catch((error) => {
                console.error(error);
            });
    }

    public static get instance(): Board {
        if (!this._instance) {
            this._instance = new Board();
        }
        return this._instance;
    }

    public get running(): boolean {
        return this._running;
    }

    public get sleepTime(): number {
        return this._sleepTime;
    }

    public set sleepTime(value: number) {
        if (value < 0) {
            throw new Error(
                `Sleep time must be a positive number, got ${value}`,
            );
        }

        this._sleepTime = value;
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

    public cells(cellVer: CellVersion): GameCell[] | Cell[] {
        if (cellVer === CellVersion.html) {
            return this._cells;
        } else {
            return this._cells.map((cell: GameCell) => cell.cell);
        }
    }

    public get cellSize(): number {
        return this._cellSize;
    }

    public get cellsIds(): string[] {
        return this._cells.map((cell: GameCell) => cell.id);
    }

    public aliveCells(cellVer: CellVersion): GameCell[] | Cell[] {
        if (cellVer === CellVersion.html) {
            return this._cells.filter((cell: GameCell) => cell.alive);
        } else {
            return this._cells
                .filter((cell: GameCell) => cell.alive)
                .map((cell: GameCell) => cell.cell);
        }
    }

    public get aliveCellsIds(): string[] {
        return (this.aliveCells(CellVersion.html) as GameCell[]).map(
            (cell: GameCell) => cell.id,
        );
    }

    public deadCells(cellVer: CellVersion): GameCell[] | Cell[] {
        if (cellVer === CellVersion.html) {
            return this._cells.filter((cell: GameCell) => !cell.alive);
        } else {
            return this._cells
                .filter((cell: GameCell) => !cell.alive)
                .map((cell: GameCell) => cell.cell);
        }
    }

    public get deadCellsIds(): string[] {
        return (this.deadCells(CellVersion.html) as GameCell[]).map(
            (cell: GameCell) => cell.id,
        );
    }

    public async createBoard(size: Size, cellSize: number): Promise<void> {
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
        this._cells = [];

        let createdCells: [string, number, number][] = [];

        await invoke("create_board", {
            width: this._width,
            height: this._height,
        })
            .then((response) => {
                createdCells = response as [string, number, number][];
            })
            .catch((error) => {
                console.error(error);
            });

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
        for (const [id, x, y] of createdCells) {
            const cell: GameCell = new GameCell(id, x, y);

            cell.style.gridRowStart = (size.height - y).toString();
            cell.style.gridColumnStart = (x + 1).toString();

            this._cells.push(cell);

            this.board.appendChild(cell);
        }
    }

    public async killBoard(): Promise<void> {
        await invoke("kill_board");

        this._cells.forEach((cell: GameCell) => {
            cell.alive = false;
        });
    }

    public async step(): Promise<void> {
        let status: [string, boolean][] = [];

        await invoke("compute_next_gen")
            .then((response) => {
                status = response as [string, boolean][];
            })
            .catch((error) => {
                console.error(error);
            });

        console.log(status);

        for (const [id, alive] of status) {
            const cell: GameCell | undefined = this._cells.find(
                (cell: GameCell) => cell.id === id,
            );

            if (cell) {
                cell.alive = alive;
            }
        }
    }

    private async sleep(): Promise<void> {
        return new Promise((resolve) => setTimeout(resolve, this._sleepTime));
    }

    private async loop(): Promise<void> {
        while (this._running) {
            await this.step();

            await this.sleep();
        }
    }

    public run(): void {
        this._running = true;
        this.loop();
    }

    public stop(): void {
        this._running = false;
    }
}

export type Size = {
    width: number;
    height: number;
};

export enum DrawType {
    Dead,
    Alive,
}

export enum CellVersion {
    html,
    object,
}
