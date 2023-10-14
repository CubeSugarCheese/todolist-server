use crate::rpc_server::db::DB;
use volo_gen::model::StatusFragment;
use volo_gen::task::{
    CreateTaskRequest, CreateTaskResponse, DeleteTaskRequest, DeleteTaskResponse,
    QueryMulitpleTaskRequest, QueryMulitpleTaskResponse, QueryTaskRequest, QueryTaskResponse,
    UpdateTaskStatusRequest, UpdateTaskStatusResponse,
};
use volo_gen::user::{LoginResponse, RegisterResponse};
use volo_thrift::AnyhowError;

pub struct S;

#[volo::async_trait]
impl volo_gen::user::UserService for S {
    async fn register(
        &self,
        req: volo_gen::user::RegisterRequest,
    ) -> Result<RegisterResponse, AnyhowError> {
        let result = DB
            .get()
            .unwrap()
            .add_user(&req.username, &req.password)
            .await;
        match result {
            Ok(user) => Ok(RegisterResponse {
                status: StatusFragment {
                    code: 200,
                    msg: "".into(),
                },
                data: Some(user),
            }),
            Err(e) => Ok(RegisterResponse {
                status: StatusFragment {
                    code: 500,
                    msg: e.to_string().into(),
                },
                data: None,
            }),
        }
    }
    async fn login(&self, req: volo_gen::user::LoginRequest) -> Result<LoginResponse, AnyhowError> {
        let result = DB
            .get()
            .unwrap()
            .get_user(req.username.as_str(), req.password.as_str())
            .await;
        match result {
            Ok(user) => Ok(LoginResponse {
                status: StatusFragment {
                    code: 200,
                    msg: "".into(),
                },
                data: Some(user),
            }),
            Err(e) => Ok(LoginResponse {
                status: StatusFragment {
                    code: 500,
                    msg: e.to_string().into(),
                },
                data: None,
            }),
        }
    }
}

#[volo::async_trait]
impl volo_gen::task::TaskService for S {
    async fn create_task(&self, req: CreateTaskRequest) -> Result<CreateTaskResponse, AnyhowError> {
        let result = DB
            .get()
            .unwrap()
            .add_task(
                req.userid,
                req.title.as_str(),
                req.content.as_str(),
                req.start_time.as_str(),
                match req.end_time {
                    Some(ref a) => Some(a.as_str()),
                    None => None,
                },
            )
            .await;
        match result {
            Ok(task) => Ok(CreateTaskResponse {
                status: StatusFragment {
                    code: 200,
                    msg: "".into(),
                },
                data: Some(task),
            }),
            Err(e) => Ok(CreateTaskResponse {
                status: StatusFragment {
                    code: 500,
                    msg: e.to_string().into(),
                },
                data: None,
            }),
        }
    }

    async fn update_task_status(
        &self,
        req: UpdateTaskStatusRequest,
    ) -> Result<UpdateTaskStatusResponse, AnyhowError> {
        let finished = req.status == 1;
        let result = DB
            .get()
            .unwrap()
            .set_task_status(req.taskid, req.userid, finished)
            .await;
        match result {
            Ok(_) => Ok(UpdateTaskStatusResponse {
                status: StatusFragment {
                    code: 200,
                    msg: "".into(),
                },
            }),
            Err(e) => Ok(UpdateTaskStatusResponse {
                status: StatusFragment {
                    code: 500,
                    msg: e.to_string().into(),
                },
            }),
        }
    }

    async fn delete_task(&self, req: DeleteTaskRequest) -> Result<DeleteTaskResponse, AnyhowError> {
        let result = DB.get().unwrap().delete_task(req.id, req.userid).await;
        match result {
            Ok(_) => Ok(DeleteTaskResponse {
                status: StatusFragment {
                    code: 200,
                    msg: "".into(),
                },
            }),
            Err(e) => Ok(DeleteTaskResponse {
                status: StatusFragment {
                    code: 500,
                    msg: e.to_string().into(),
                },
            }),
        }
    }

    async fn query_task(&self, req: QueryTaskRequest) -> Result<QueryTaskResponse, AnyhowError> {
        let result = DB.get().unwrap().get_task(req.id, req.userid).await;
        match result {
            Ok(task) => Ok(QueryTaskResponse {
                status: StatusFragment {
                    code: 200,
                    msg: "".into(),
                },
                data: task,
            }),
            Err(e) => Ok(QueryTaskResponse {
                status: StatusFragment {
                    code: 500,
                    msg: e.to_string().into(),
                },
                data: None,
            }),
        }
    }

    async fn query_multiple_task(
        &self,
        req: QueryMulitpleTaskRequest,
    ) -> Result<QueryMulitpleTaskResponse, AnyhowError> {
        let result = DB
            .get()
            .unwrap()
            .get_multiple_task(
                req.userid,
                req.page.unwrap_or(0),
                req.page_size.unwrap_or(20),
            )
            .await;
        match result {
            Ok(tasks) => Ok(QueryMulitpleTaskResponse {
                status: StatusFragment {
                    code: 200,
                    msg: "".into(),
                },
                data: tasks,
            }),
            Err(e) => Ok(QueryMulitpleTaskResponse {
                status: StatusFragment {
                    code: 500,
                    msg: e.to_string().into(),
                },
                data: Vec::default(),
            }),
        }
    }
}
