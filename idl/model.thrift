namespace rs model

# 时间统一规定为 UTC+0 时间

struct User {
1: required i64 id,
2: required string username,
3: required string password,
4: required string create_time,
5: required string update_time,
}

struct Task {
1: required i64 id,
2: required string title,
3: required string content,
4: required i64 status,          # 0为未完成 1为完成
5: required string start_time,
6: optional string end_time,
7: required string create_time,
8: required string update_time,
}

struct StatusFragment {
1: required i64 code,
2: required string msg,
}