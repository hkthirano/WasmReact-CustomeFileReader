# React

npm create vite@latest

devcontainerで起動したアプリにアクセスするための設定

起動コマンドに--hostを付ける "dev": "vite --host"

サーバの設定を行う

vite.config.tsに以下追加

```
server: {
    port: 5173,
    host: '127.0.0.1'
}
```

# WASM

cargo install wasm-pack

cargo new --lib my-wasm-lib
