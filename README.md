# todolist-server
A demo todo list server using volo and axum

## 构建
```shell
cargo build --release
```

## 配置开发环境
修改 `.env` 文件，设置 `SQLX_OFFLINE=false`，并设置测试数据库链接 `DATABASE_URL`。
参照 `init.sql` 初始化数据库。
