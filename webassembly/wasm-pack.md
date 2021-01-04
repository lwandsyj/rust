1. 安装wasm-pack，wasm-pack 是用于构建，测试和发布Rust生成的WebAssembly。

            curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

2. cargo-generate 通过利用预先存在的git存储库作为模板，可以帮助您快速启动并运行新的Rust项目。

            cargo install cargo-generate

3.  npm是JavaScript的软件包管理器。我们将使用它来安装和运行JavaScript捆绑器和开发服务器。在本教程的最后，我们将把编译.wasm后的内容发布到npm注册表中
