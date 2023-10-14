namespace rs task

include "model.thrift"

struct CreateTaskRequest {
    1: required string title,
    2: required i64 userid
    3: required string content,
    4: required string start_time,
    5: optional string end_time,
}

struct CreateTaskResponse {
    1: required model.StatusFragment status
    2: optional model.Task data
}

struct UpdateTaskStatusRequest {
    1: required i64 taskid,
    2: required i64 userid
    3: required i64 status, # 0为未完成 1为完成
}

struct UpdateTaskStatusResponse {
    1: required model.StatusFragment status
}

struct DeleteTaskRequest {
    1: required i64 id,
    2: required i64 userid
}

struct DeleteTaskResponse {
    1: required model.StatusFragment status
}

struct QueryTaskRequest {
    1: required i64 id,
    2: required i64 userid
}

struct QueryTaskResponse {
    1: required model.StatusFragment status
    2: optional model.Task data
}

struct QueryMulitpleTaskRequest {
    1: required i64 userid
    2: optional i64 page // 默认为0
    3: optional i64 page_size // 默认为20
}

struct QueryMulitpleTaskResponse {
    1: required model.StatusFragment status
    2: required list<model.Task> data
}

service TaskService {
    CreateTaskResponse create_task(1:CreateTaskRequest req) (api.post="/task/create"),
    UpdateTaskStatusResponse update_task_status(1:UpdateTaskStatusRequest req) (api.post="/task/update/single-status"),
    DeleteTaskResponse delete_task(1:DeleteTaskRequest req) (api.delete="/task/delete/single"),
    QueryTaskResponse query_task(1:QueryTaskRequest req) (api.get="/task/get/single"),
    QueryMulitpleTaskResponse query_multiple_task(1:QueryMulitpleTaskRequest req) (api.get="/task/get/multiple"),
}