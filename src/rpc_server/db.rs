pub mod error;
pub mod model;

use self::error::{AddUserError, LoginError};
use self::model::{TaskRow, UserRow};
use chrono::Local;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};
use tokio::sync::OnceCell;
use volo_gen::model::{Task, User};

pub struct DATABASE {
    instance: Pool<MySql>,
}

impl DATABASE {
    pub async fn new(url: &str) -> Result<DATABASE, sqlx::Error> {
        let instance = MySqlPoolOptions::new()
            .max_connections(1)
            .connect(url) //
            .await?;
        Ok(DATABASE { instance })
    }

    pub async fn add_user(&self, username: &str, password: &str) -> Result<User, AddUserError> {
        let result = sqlx::query!("select * from user where username=?", username)
            .fetch_optional(&self.instance)
            .await?;
        if result.is_some() {
            tracing::info!("exist same name user!");
            return Err(AddUserError::AlreadyHaveSameName);
        }

        sqlx::query!(
            "insert into user (username, password) values (?, ?)",
            username,
            password
        )
        .execute(&self.instance)
        .await?;
        let row = sqlx::query_as!(UserRow, "select * from user where id=last_insert_id()")
            .fetch_one(&self.instance)
            .await?;
        Ok(row.into())
    }

    pub async fn get_user(&self, username: &str, password: &str) -> Result<User, LoginError> {
        let result = sqlx::query_as!(
            UserRow,
            "select * from user where username=? and password=?",
            username,
            password
        )
        .fetch_optional(&self.instance)
        .await?;
        match result {
            Some(row) => Ok(row.into()),
            None => Err(LoginError::AccountOrPasswordWrong),
        }
    }

    pub async fn add_task(
        &self,
        userid: i64,
        title: &str,
        content: &str,
        start_time: &str,
        end_time: Option<&str>,
    ) -> Result<Task, sqlx::Error> {
        sqlx::query!(
            "insert into task (userid,title,content,start_time,end_time) values (?,?,?,?,?)",
            userid,
            title,
            content,
            start_time,
            end_time
        )
        .execute(&self.instance)
        .await?;
        let row = sqlx::query_as!(TaskRow, "select * from task where id=last_insert_id()")
            .fetch_one(&self.instance)
            .await?;
        Ok(row.into())
    }

    pub async fn set_task_status(
        &self,
        task_id: i64,
        userid: i64,
        status_finished: bool,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "update task set finished=? where id=? and userid=? and delete_time is null",
            if status_finished { Some("") } else { None },
            task_id,
            userid
        )
        .execute(&self.instance)
        .await?;
        Ok(())
    }

    pub async fn delete_task(&self, task_id: i64, user_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "update task set delete_time=? where id=? and userid=? and delete_time is null",
            Local::now().timestamp(),
            task_id,
            user_id
        )
        .execute(&self.instance)
        .await?;
        Ok(())
    }

    pub async fn get_task(&self, task_id: i64, user_id: i64) -> Result<Option<Task>, sqlx::Error> {
        tracing::debug!("get task, task_id: {task_id}, user_id: {user_id}");
        let row = sqlx::query_as!(
            TaskRow,
            "select * from task where id=? and userid=? and delete_time is null",
            task_id,
            user_id
        )
        .fetch_optional(&self.instance)
        .await?;

        Ok(row.map(|it| it.into()))
    }

    pub async fn get_multiple_task(
        &self,
        user_id: i64,
        page: i64,
        size: i64,
    ) -> Result<Vec<Task>, sqlx::Error> {
        tracing::debug!("get task, user_id: {user_id}, page: {page}, size: {size}");
        let rows = sqlx::query_as!(
            TaskRow,
r#"select * from task where userid=? and delete_time is null
order by id
limit ?,?"#,
            user_id,
            page * size,
            size
        )
        .fetch_all(&self.instance)
        .await?;

        let tasks: Vec<Task> = rows.iter().map(|it| it.clone().into()).collect();
        Ok(tasks)
    }
}

pub static DB: OnceCell<DATABASE> = OnceCell::const_new();
