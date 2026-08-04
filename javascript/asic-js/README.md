# asic-js

Node.js bindings for `asic-rs`.

The package mirrors the Python binding's top-level shape:

- `MinerFactory` discovers miners and constructs `Miner` handles.
- `Miner` exposes telemetry and control methods.
- Rust data models are returned as plain JavaScript objects.

## Build

```sh
npm install
npm run build
```

## Example

```js
"use strict";

const { MinerFactory } = require("asic-js");

async function main() {
  const miner = await new MinerFactory().getMiner("192.168.1.10");
  if (!miner) return;

  console.log(miner.make, miner.model, miner.ip);
  console.log(miner.deviceInfo);
  console.log(await miner.getData());

  if (miner.supportsRestart) {
    await miner.restart();
  }
}

main().catch((err) => {
  console.error(err);
  process.exitCode = 1;
});
```

Snake-case aliases are provided for parity with `pyasic_rs`, for example `get_miner`, `get_data`, and `supports_restart`.
