# Game of Life

This project implements Conway's Game of Life using a Tauri application with Rust for the backend and a frontend powered by HTML, TypeScript, and Vite.

## Installation

To set up and run this project, follow these steps:

### Prerequisites

Ensure you have the following installed on your system:

-   Node.js (v20.11.0 or higher)
-   PNPM (v8.15.5 or higher)
-   Rust (2021 edition)

You can check if these are installed and find the versions by running:

```bash
node -v
pnpm -v
rustc --version
```

### Setup

1. First, clone the repository and navigate into the project directory.

2. Install the necessary Node.js dependencies:

```bash
pnpm install
```

### Running the Development Version

To run the development version of the project:

```bash
pnpm run dev:tauri
```

This command starts the Vite development server and the Tauri development mode. The Vite server will run on `http://localhost:1420`, and the Tauri window will open automatically.

### Building the Project

To build the project for production:

```bash
pnpm build
```

This command builds the frontend and backend components of the project. An executable file will be created in the `src-tauri/target/release` directory and 3 installers for Windows (msi in english and french and nsis in english) will be created in the `release` directory.

## Usage

Once the application is running, you can interact with the Game of Life grid displayed in the Tauri window. Click on cells to toggle their state between alive and dead, and use the provided controls to start, stop, and reset the simulation.
