# todolist-server
A demo todo list server using volo and axum

## 构建
```shell
cargo build --release
```

## 部署
运行 `target/release` 下的二进制文件即可

程序运行需要在同目录下的 `.env` 文件

请在部署前修改 `JWT_SECRET` 和 `DATABASE_URL` 的值

## 配置开发环境
建议运行以下命令，让 git 忽略对 `.env` 文件的更改，避免意外提交密钥。

来源：https://stackoverflow.com/a/13631525/1093511
```shell
git update-index --skip-worktree .env
```
参照 `init.sql` 初始化数据库。

修改 `.env` 文件，设置 `SQLX_OFFLINE=false`，并设置测试数据库链接 `DATABASE_URL`。


## 项目结构
```
sqlx：sqlx 的声明文件，用于在离线模式下构建项目时提供 sql 语法检查
idl: thrift 声明文件
src/bin: 二进制入口
src/http_server: HTTP 服务器的具体实现
src/rpc_server: RPC 服务器的具体实现
src/util.rs 一些零散的小工具
volo-gen: 由 volo-cli 生成的 crate，不应该手动修改
.env: 应用运行时的环境变量
init.sql: 初始化测试数据库使用的 sql
rust-toolchain.toml: 指定 rust 使用 nightly 版本，以启用 volo 所需的 impl_trait_in_assoc_type 特性
```
