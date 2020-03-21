Wiki For The First Time

## DB

```
users {
    uid: int 
    nickname: string
}

articles {
    aid: int
    uid: int
    created: timestamp
    content : string
}
```

## API

### GET
(GET) 全ユーザーを取得
```
/api/users
```

```json
[
    {
        "id": 1,
        "user": "asano",
        "created": "2020/20/20",
        "content": "turedurenarumamani",
        "published": false,
    },
    {
        "id": 2,
        "user": "kasano",
        "created": "20201/20/20",
        "content": "turedurenarumamani",
        "published": true,
    },
]
```

(GET) ユーザーをIdで取得
```
/api/users/{user-id}
```

```json
[
    {
        "id": 2,
        "user": "asano",
        "created": "2020/20/20",
        "content": "turedurenarumamani",
        "published": false,
    },
]
```

(GET) 全記事取得
```
/api/articles
```

```json
[
    {
        "id": 1,
        "user": "asano",
        "created": "2020/20/20",
        "content": "turedurenarumamani",
        "published": false,
    },
    {
        "id": 2,
        "user": "kasano",
        "created": "20201/20/20",
        "content": "turedurenarumamani",
        "published": true,
    },
]
```

(GET) IDで記事を取得
```
/api/articles/{article-id}
```

```json
[
    {
        "id": 2,
        "user": "asano",
        "created": "2020/20/20",
        "content": "turedurenarumamani",
        "published": false,
    },
]
```

(GET) ある書き主の記事を全て取得
```
/api/user-articles/{user-id}
```

```json
[
    {
        "id": 1,
        "user": "asano",
        "created": "2020/20/20",
        "content": "turedurenarumamani",
        "published": false,
    },
]
```
### POST

(POST) User追加 json
```
/api/signin
```

```json
{
    "nickname": "asako",
}
```

(POST) 記事執筆 json
```
/api/write
```

```json
{
        "id": 4,
        "user": "asano",
        "content": "turedurenarumamani",
}
```


## API テスト
GET
```bash
curl localhost:3000/api/users
curl localhost:3000/api/users/123
curl localhost:3000/api/articles
curl localhost:3000/api/articles/111
```
POST
```bash
curl -X POST localhost:3000/api/write -H "Content-Type:application/json" -d "{\"author\":\"asako\", \"content\":\"ukiyoni\", \"published\":false}"
curl -X POST localhost:3000/api/signin -H "Content-Type:application/json" -d "{\"name\":\"asako\", \"id\":123}"
```