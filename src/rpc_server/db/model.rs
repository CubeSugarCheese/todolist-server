use crate::util::SQL_FORMAT;
use pilota::FastStr;
use time::OffsetDateTime;
use volo_gen::model::{Task, User};

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct UserRow {
    pub(crate) id: i64,
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) register_time: OffsetDateTime,
    pub(crate) update_time: OffsetDateTime,
    pub(crate) delete_time: Option<OffsetDateTime>,
}

impl Into<User> for UserRow {
    fn into(self: UserRow) -> User {
        User {
            id: self.id,
            username: self.username.into(),
            password: self.password.into(),
            create_time: self
                .register_time
                .format(SQL_FORMAT)
                .expect("format time failed")
                .into(),
            update_time: self
                .update_time
                .format(SQL_FORMAT)
                .expect("format time failed")
                .into(),
        }
    }
}

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct TaskRow {
    pub(crate) id: i64,
    pub(crate) userid: i64,
    pub(crate) title: String,
    pub(crate) content: String,
    pub(crate) finished: Option<String>,
    pub(crate) start_time: OffsetDateTime,
    pub(crate) end_time: Option<OffsetDateTime>,
    pub(crate) create_time: OffsetDateTime,
    pub(crate) update_time: OffsetDateTime,
    pub(crate) delete_time: Option<OffsetDateTime>,
}

impl Into<Task> for TaskRow {
    fn into(self) -> Task {
        Task {
            id: self.id,
            title: self.title.into(),
            content: FastStr::new(self.content),
            status: if self.finished.is_some() { 1 } else { 0 },
            start_time: self
                .start_time
                .format(SQL_FORMAT)
                .expect("format time failed")
                .into(),
            end_time: self
                .end_time
                .map(|it| it.format(SQL_FORMAT).expect("format time failed").into()),
            create_time: self
                .create_time
                .format(SQL_FORMAT)
                .expect("format time failed")
                .into(),
            update_time: self
                .update_time
                .format(SQL_FORMAT)
                .expect("format time failed")
                .into(),
        }
    }
}
