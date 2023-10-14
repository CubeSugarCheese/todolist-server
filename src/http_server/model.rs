use serde::{Deserialize, Serialize};
use volo_gen::model::{Task, User};
use volo_gen::task::{
    CreateTaskRequest, DeleteTaskRequest, QueryMulitpleTaskRequest, QueryTaskRequest,
    UpdateTaskStatusRequest,
};
use volo_gen::user::{LoginRequest, RegisterRequest};

#[derive(Serialize, Debug)]
pub struct ApiResponse<T: Serialize> {
    msg: String,
    data: T,
}

impl<T: Serialize> ApiResponse<T> {
    pub const fn new(data: T) -> ApiResponse<T> {
        ApiResponse {
            msg: String::new(),
            data,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct UserData {
    pub id: i64,
    pub username: String,
    pub create_time: String,
    pub update_time: String,
}

impl From<User> for UserData {
    fn from(value: User) -> UserData {
        Self {
            id: value.id,
            username: value.username.to_string(),
            create_time: value.create_time.to_string(),
            update_time: value.update_time.to_string(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct RegisterPayload {
    pub username: String,
    pub password: String,
}

impl Into<RegisterRequest> for RegisterPayload {
    fn into(self) -> RegisterRequest {
        RegisterRequest {
            username: self.username.into(),
            password: self.password.into(),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct RegisterData {
    pub user: UserData,
}

impl RegisterData {
    pub const fn new(user: UserData) -> Self {
        Self { user }
    }
}

#[derive(Deserialize, Debug)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

impl Into<LoginRequest> for LoginPayload {
    fn into(self) -> LoginRequest {
        LoginRequest {
            username: self.username.into(),
            password: self.password.into(),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct LoginData {
    pub user: UserData,
    pub token: String,
}

impl LoginData {
    pub const fn new(user: UserData, token: String) -> Self {
        Self { user, token }
    }
}

#[derive(Serialize, Debug)]
pub struct TaskData {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub start_time: String,
    pub end_time: Option<String>,
}

impl From<Task> for TaskData {
    fn from(value: Task) -> Self {
        Self {
            id: value.id,
            title: value.title.to_string(),
            content: value.content.to_string(),
            start_time: value.start_time.to_string(),
            end_time: value.end_time.map(|it| it.to_string()),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct CreateTaskPayload {
    pub title: String,
    pub content: String,
    pub start_time: String,
    pub end_time: Option<String>,
}

impl CreateTaskPayload {
    pub fn to_rpc_req(self, userid: i64) -> CreateTaskRequest {
        CreateTaskRequest {
            title: self.title.into(),
            userid,
            content: self.content.into(),
            start_time: self.start_time.into(),
            end_time: self.end_time.map(|it| it.into()),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct UpdateTaskStatusPayload {
    pub task_id: i64,
    pub status: i64,
}

impl UpdateTaskStatusPayload {
    pub const fn to_rpc_req(self, userid: i64) -> UpdateTaskStatusRequest {
        UpdateTaskStatusRequest {
            taskid: self.task_id,
            userid,
            status: self.status,
        }
    }
}
#[derive(Deserialize, Debug)]
pub struct DeleteTaskPayload {
    pub task_id: i64,
}

impl DeleteTaskPayload {
    pub const fn to_rpc_req(self, userid: i64) -> DeleteTaskRequest {
        DeleteTaskRequest {
            id: self.task_id,
            userid,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct GetTaskPayload {
    task_id: i64,
}

impl GetTaskPayload {
    pub const fn to_rpc_req(self, userid: i64) -> QueryTaskRequest {
        QueryTaskRequest {
            id: self.task_id,
            userid,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct GetMultipleTaskPayload {
    page: Option<i64>,
    page_size: Option<i64>,
}

impl GetMultipleTaskPayload {
    pub const fn to_rpc_req(self, userid: i64) -> QueryMulitpleTaskRequest {
        QueryMulitpleTaskRequest {
            userid,
            page: self.page,
            page_size: self.page_size,
        }
    }
}
