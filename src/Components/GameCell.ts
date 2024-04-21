import {LitElement, html, css} from "lit";
import {customElement, property} from "lit/decorators.js";
import {uuid} from "../Utils";
import Cell from "../Cell";

@customElement("game-cell")
export default class GameCell extends LitElement {
    static readonly styles = css`
        :host {
            background-color: rgba(255, 255, 255, 1);
        }
        :host([alive]) {
            background-color: rgba(0, 0, 0, 1);
        }
    `;

    @property({type: String}) readonly id = uuid();
    @property({type: Boolean, reflect: true}) alive = false;
    @property({type: Number}) readonly x;
    @property({type: Number}) readonly y;
    @property({type: Cell}) readonly cell;

    constructor(x: number, y: number) {
        super();
        this.x = x;
        this.y = y;

        this.cell = new Cell(this);
    }

    public toggleAlive(): void {
        this.alive = !this.alive;
    }

    render() {
        return html`<div></div>`;
    }

    updated() {
        this.cell.alive = this.alive;
    }
}
