namespace rs volo.redis

enum RedisCommand{
    Get,
    Set,
    Del,
    Ping,
    Publish,
    Subscribe
}

struct RedisRequest{
    1: required RedisCommand cmd,
    2: optional list<string> arguments,
}

struct RedisResponse{
    1: required bool ok,
    2: optional string data,
}

service RedisService{
    RedisResponse RedisCommand(1: RedisRequest req),
}