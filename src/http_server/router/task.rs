use crate::http_server::model::{
    CreateTaskPayload, DeleteTaskPayload, GetMultipleTaskPayload, GetTaskPayload, TaskData,
    UpdateTaskStatusPayload,
};
use crate::http_server::router::error::ApiError;
use crate::http_server::router::jwt::Claims;
use crate::http_server::router::{ok, ApiResult};
use crate::http_server::TASK_SERVICE_CLIENT;
use axum::Json;

pub async fn create_task(
    claim: Claims,
    Json(payload): Json<CreateTaskPayload>,
) -> ApiResult<TaskData> {
    let resp = TASK_SERVICE_CLIENT
        .create_task(payload.to_rpc_req(claim.userid))
        .await
        .unwrap();
    match resp.status.code {
        200 => ok(resp
            .data
            .expect("status code is 200 but data is None!")
            .into()),
        500 => Err(ApiError::InternalServerError(resp.status.msg.into_string())),
        code => unreachable!("unknown rpc api status code: {}", code),
    }
}
pub async fn update_task_status(
    claim: Claims,
    Json(payload): Json<UpdateTaskStatusPayload>,
) -> ApiResult<()> {
    let resp = TASK_SERVICE_CLIENT
        .update_task_status(payload.to_rpc_req(claim.userid))
        .await
        .unwrap();
    match resp.status.code {
        200 => ok(()),
        500 => Err(ApiError::InternalServerError(resp.status.msg.into_string())),
        code => unreachable!("unknown rpc api status code: {}", code),
    }
}
pub async fn delete_task(claim: Claims, Json(payload): Json<DeleteTaskPayload>) -> ApiResult<()> {
    let resp = TASK_SERVICE_CLIENT
        .delete_task(payload.to_rpc_req(claim.userid))
        .await
        .unwrap();
    match resp.status.code {
        200 => ok(()),
        500 => Err(ApiError::InternalServerError(resp.status.msg.into_string())),
        code => unreachable!("unknown rpc api status code: {}", code),
    }
}
pub async fn get_task(
    claim: Claims,
    Json(payload): Json<GetTaskPayload>,
) -> ApiResult<Option<TaskData>> {
    let resp = TASK_SERVICE_CLIENT
        .query_task(payload.to_rpc_req(claim.userid))
        .await
        .unwrap();
    match resp.status.code {
        200 => ok(resp.data.map(|it| it.into())),
        500 => Err(ApiError::InternalServerError(resp.status.msg.into_string())),
        code => unreachable!("unknown rpc api status code: {}", code),
    }
}
pub async fn get_multiple_task(
    claim: Claims,
    Json(payload): Json<GetMultipleTaskPayload>,
) -> ApiResult<Vec<TaskData>> {
    let resp = TASK_SERVICE_CLIENT
        .query_multiple_task(payload.to_rpc_req(claim.userid))
        .await
        .unwrap();
    match resp.status.code {
        200 => ok(resp.data.iter().map(|it| it.clone().into()).collect()),
        500 => Err(ApiError::InternalServerError(resp.status.msg.into_string())),
        code => unreachable!("unknown rpc api status code: {}", code),
    }
}
