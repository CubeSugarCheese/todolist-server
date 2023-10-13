namespace rs user

include "model.thrift"

struct RegisterRequest {
1: required string username,
2: required string password,
}

struct RegisterResponse {
1: required model.StatusFragment status,
2: optional model.User data,
}

struct LoginRequest {
1: required string username,
2: required string password,
}

struct LoginResponse {
1: required model.StatusFragment status,
2: optional model.User data,
}

service UserService {
RegisterResponse register(1:RegisterRequest req) (api.post="/user/register"),
LoginResponse login(1:LoginRequest req) (api.post="/user/login"),
}