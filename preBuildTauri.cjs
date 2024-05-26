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

// Remove the previous bundles
if (fs.existsSync(tauriBundlePath)) {
    fs.rmSync(tauriBundlePath, {recursive: true});
}

// Remove the previous releases
if (fs.existsSync(releasePath)) {
    fs.rmSync(releasePath, {recursive: true});
}
