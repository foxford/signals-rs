# Candidate

## Request

Agent _A(a)u1_ sends a candidate to agent _A(a)u2_

```
A(a)u1 pub:
agents/A(a)u1/out/signals.netology-group.services/api/v1
```

```json
{
    "jsonrpc": "2.0",
    "method": "webrtc.candidate",
    "params": [{
        "room_id": "050b7c6f-795c-4cb4-aeea-5ee3f9083de2",
        "data": {
            "candidate": { "..." },
            "from": "A(a)u1",
            "to": "A(a)u2"
        }
    }],
    "id": "qwerty"
}
```

Agent _A(a)u2_ receives a candidate

```
A(a)signals pub: 
agents/A(a)u2/in/signals.netology-group.services/api/v1
```

```json
{
    "jsonrpc": "2.0",
    "method": "webrtc.candidate",
    "params": [{
        "room_id": "050b7c6f-795c-4cb4-aeea-5ee3f9083de2",
        "data": {
            "candidate": { "..." },
            "from": "A(a)u1"
        }
    }]
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
    "result": [],
    "id": "qwerty"
}
```
