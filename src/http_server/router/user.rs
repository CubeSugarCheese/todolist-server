use crate::http_server::model::{LoginData, LoginPayload, RegisterData, RegisterPayload};
use crate::http_server::router::error::ApiError;
use crate::http_server::router::jwt::{Claims, KEYS};
use crate::http_server::router::{ok, ApiResult};
use crate::http_server::USER_SERVICE_CLIENT;
use axum::Json;
use chrono::Months;
use jsonwebtoken::Header;

pub async fn register(Json(payload): Json<RegisterPayload>) -> ApiResult<RegisterData> {
    let resp = USER_SERVICE_CLIENT.register(payload.into()).await.unwrap();
    match resp.status.code {
        200 => ok(RegisterData::new(
            resp.data
                .expect("status code is 200 but data is None!")
                .into(),
        )),
        500 => Err(ApiError::InternalServerError(resp.status.msg.into_string())),
        code => unreachable!("unknown rpc api status code: {}", code),
    }
}

pub async fn login(Json(payload): Json<LoginPayload>) -> ApiResult<LoginData> {
    let resp = USER_SERVICE_CLIENT.login(payload.into()).await.unwrap();
    match resp.status.code {
        200 => {
            let user = resp.data.expect("status code is 200 but data is None!");
            let exp = chrono::Utc::now() + Months::new(1);
            let claims = Claims::new(user.id, exp.timestamp());
            let token = jsonwebtoken::encode(&Header::default(), &claims, &KEYS.encoding).unwrap();
            ok(LoginData::new(user.into(), token))
        }
        500 => Err(ApiError::InternalServerError(resp.status.msg.into_string())),
        code => unreachable!("unknown rpc api status code: {}", code),
    }
}
