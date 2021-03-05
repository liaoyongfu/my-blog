# My Blog

Actix-web + Yew

## 目录说明

- client: Yew 端
- server: Actix-web 端
- static: index.html 等
- build: client => wasm
- scripts: webpack 等配置

## 流程说明

1. server 端提供后端服务
2. client 端提供前端页面/路由，打包后生成 wasm（pkg 目录）
3. static 端静态文件，由 webpack 引入 wasm 文件，并提供 hot reload 功能

## 启动

````
yarn start
````

## 编译

````
yarn build
````

## 部署 gh-pages

````
yarn deploy
````
