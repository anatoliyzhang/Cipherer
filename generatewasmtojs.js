const fs = require('fs');
let file_path = './blackbox/pkg/blackbox_bg.wasm';
let file_buffer = new Buffer.from(fs.readFileSync(file_path));
fs.writeFileSync('wasm.js','let wasm_string = "'+file_buffer.toString('base64')+'";' );
// console.log(file_buffer.toString('base64'));