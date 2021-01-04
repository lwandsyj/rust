1. wasm_bindgen 连接rust 和 JavaScript 

        #[wasm_bindgen]
        #[repr(u8)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum Cell {
            Dead = 0,
            Alive = 1,
        }

        1. wasm_bindgen  声明wasm_bindgen ，可以导出给JavaScript使用

        2. repr(u8) 因此每个单元都表示为一个字节

        3. Clone 实现克隆
           Copy 拷贝
           Debug 可以{:?} 打印
           PartialEq,Eq 可以使用== 判断
    

2. js 使用

        import { Universe, Cell } from "wasm-game-of-life";

3. 只有绑定了wasm_bindgen 才对外公开