import GameCell from "./Components/GameCell";

export default class Cell {
    public readonly id;
    public alive: boolean;
    public readonly x: number;
    public readonly y: number;

    constructor(gameCell: GameCell) {
        this.id = gameCell.id;
        this.x = gameCell.x;
        this.y = gameCell.y;
        this.alive = gameCell.alive;
    }
}