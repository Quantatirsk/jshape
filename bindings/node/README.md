# @quanthub/jshape

`@quanthub/jshape` is the Node.js package for the Rust-powered JSON shape analyzer in this repository.

It is built as a WebAssembly package for Node.js, so consumers get a normal npm install experience without needing a local Rust toolchain.

## Install

```bash
npm install @quanthub/jshape
```

## Usage

```js
const { analyzeJson } = require("@quanthub/jshape");

const outline = analyzeJson(
  '{"user":{"name":"Ada"},"events":[{"id":1},{"id":2,"amount":19.9}]}',
  true,
);

console.log(outline);
```
