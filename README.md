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
全ユーザー
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

ある記事
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

全記事
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

ある記事
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

ある書き主の記事を
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

