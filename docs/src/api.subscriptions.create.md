# Create

## Request
```
A(a)u1 pub:
agents/A(a)u1/out/signals.netology-group.services/api/v1
```

```json
{
    "jsonrpc": "2.0",
    "method": "subscription.create",
    "params": [{
        "room_id": "050b7c6f-795c-4cb4-aeea-5ee3f9083de2",
        "agent_id": "1154b35c-e5b0-4a42-8ab2-d4967ce38c9e",
        "data": {
            "resource": "agents"
        }
    }],
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
    "result": {
        "data": {
            "topic": "apps/signals.netology-group.services/api/v1/rooms/050b7c6f-795c-4cb4-aeea-5ee3f9083de2/agents"
        }
    },
    "id": "qwerty"
}
```