# Answer

## Request

Agent _A(a)u2_ sends an answer to agent _A(a)u1_

```
A(a)u2 pub:
agents/A(a)u2/out/signals.netology-group.services/api/v1
```

```json
{
    "jsonrpc": "2.0",
    "method": "webrtc.answer",
    "params": [{
        "room_id": "050b7c6f-795c-4cb4-aeea-5ee3f9083de2",
        "data": {
            "jsep": { "type": "answer", "sdp": "..." },
            "from": "A(a)u2",
            "to": "A(a)u1"
        }
    }],
    "id": "qwerty"
}
```

Agent _A(a)u1_ receives an answer

```
A(a)signals pub: 
agents/A(a)u1/in/signals.netology-group.services/api/v1
```

```json
{
    "jsonrpc": "2.0",
    "method": "webrtc.answer",
    "params": [{
        "room_id": "050b7c6f-795c-4cb4-aeea-5ee3f9083de2",
        "data": {
            "jsep": { "type": "answer", "sdp": "..." },
            "from": "A(a)u2"
        }
    }]
}
```

## Response

```
A(a)signals pub: 
agents/A(a)u2/in/signals.netology-group.services/api/v1
```

```json
{
    "jsonrpc": "2.0",
    "result": [],
    "id": "qwerty"
}
```
