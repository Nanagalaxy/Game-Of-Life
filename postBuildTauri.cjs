const fs = require("fs");
const path = require("path");

const rootPath = path.resolve(__dirname);

const releasePath = path.resolve(rootPath, "release");

const tauriBundlePath = path.resolve(
    rootPath,
    "src-tauri",
    "target",
    "release",
    "bundle",
);

if (!fs.existsSync(tauriBundlePath)) {
    console.error(`Bundle path ${tauriBundlePath} does not exist.`);
    process.exit(1);
}

if (!fs.existsSync(releasePath)) {
    fs.mkdirSync(releasePath);
}

// Copies all folders and files from src-tauri/target/release/bundle to release/bundle
const copyFiles = (src, dest) => {
    const stats = fs.statSync(src);

    if (stats.isDirectory()) {
        if (!fs.existsSync(dest)) {
            fs.mkdirSync(dest);
        }

        fs.readdirSync(src).forEach((file) => {
            copyFiles(path.resolve(src, file), path.resolve(dest, file));
        });
    } else {
        // If it's a file, replace it
        if (fs.existsSync(dest) && fs.statSync(dest).isFile()) {
            fs.rmSync(dest);
        }

        fs.copyFileSync(src, dest);

        console.log(`Copied ${src} to ${dest}`);
    }
};

copyFiles(tauriBundlePath, releasePath);
