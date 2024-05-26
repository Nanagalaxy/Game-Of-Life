import {LitElement, html, css} from "lit";
import {customElement, property} from "lit/decorators.js";

@customElement("cell")
export default class Cell extends LitElement {
    static readonly styles = css`
        :host {
            background-color: rgba(255, 255, 255, 1);
        }
        :host([alive]) {
            background-color: rgba(0, 0, 0, 1);
        }
    `;

    @property({type: String}) readonly id;
    @property({type: Boolean, reflect: true}) alive = false;
    @property({type: Number}) readonly x;
    @property({type: Number}) readonly y;

    constructor(id: string, x: number, y: number) {
        super();

        this.id = id;
        this.x = x;
        this.y = y;
    }

    public toggleAlive(): void {
        this.alive = !this.alive;
    }

    render() {
        return html`<div></div>`;
    }
}
