"use strict";

const path = require("path");

function platformCandidates() {
  const platform = process.platform;
  const arch = process.arch;

  if (platform === "win32") {
    return [`../index.${platform}-${arch}-msvc.node`];
  }

  if (platform === "darwin") {
    return [`../index.${platform}-${arch}.node`];
  }

  if (platform === "linux") {
    return [
      `../index.${platform}-${arch}-gnu.node`,
      `../index.${platform}-${arch}-musl.node`,
      `../index.${platform}-${arch}-gnueabihf.node`,
    ];
  }

  if (platform === "freebsd") {
    return [`../index.${platform}-${arch}.node`];
  }

  return [];
}

const candidates = [
  "../asic-js.node",
  "../asic_js.node",
  ...platformCandidates(),
  "../target/release/asic_js.node",
  "../target/debug/asic_js.node",
  "../../../target/release/asic_js.node",
  "../../../target/debug/asic_js.node",
];

for (const candidate of candidates) {
  try {
    module.exports = require(path.join(__dirname, candidate));
    return;
  } catch (err) {
    if (err.code !== "MODULE_NOT_FOUND") throw err;
  }
}

throw new Error(
  "The asic-js native module is not built. Run `npm run build` in javascript/asic-js.",
);
