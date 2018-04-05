# List

## Request

```
A(a)u1 pub:
agents/A(a)u1/out/signals.netology-group.services/api/v1
```

```json
{
    "jsonrpc": "2.0",
    "method": "room.list",
    "params": [],
    "id": "qwerty"
}
```

## Response

```
A(a)signals pub:
agents/A(a)u1/in/signals.netology-group.services/api/v1
```

```json
{
    "jsonrpc": "2.0",
    "result": [
        {
            "id": "050b7c6f-795c-4cb4-aeea-5ee3f9083de2",
            "data": {
                "capacity": 3,
                "created_at": "2018-04-05T03:07:34.906228"
            }
        }
    ],
    "id": "qwerty"
}
```
