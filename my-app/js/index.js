

import("../pkg/index.js").then(module=>{
    module.greet('hello rust wasm1')
}).catch(console.error);
