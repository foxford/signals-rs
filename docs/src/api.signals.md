# WebRTC Signaling

## Actions

### Offer

*Request:*
```
A(a)u1 pub:
agents/A(a)u1/out/signals.netology-group.services/api/v1
```

```json
{
    "jsonrpc": "2.0",
    "method": "webrtc.offer",
    "params": [{
        "room_id": "050b7c6f-795c-4cb4-aeea-5ee3f9083de2",
        "data": {
            "jsep": { "type": "offer", "sdp": "..." },
            "to": "A(a)u2",
            "tracks": [
                {
                    "id": "915adba9-5586-4743-a22e-47cc57260e37"
                }
            ]
        }
    }],
    "id": "qwerty"
}
```

*Response:*

```
A(a)signals pub: 
agents/A(a)u1/in/signals.netology-group.services/api/v1
```

```json
{
    "jsonrpc": "2.0",
    "result": {},
    "id": "qwerty"
}
```

### Answer

*Request:*
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
            "to": "A(a)u1"
        }
    }],
    "id": "qwerty"
}
```

*Response:*

```
A(a)signals pub: 
agents/A(a)u2/in/signals.netology-group.services/api/v1
```

```json
{
    "jsonrpc": "2.0",
    "result": {},
    "id": "qwerty"
}
```

### Candidate

*Request:*
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
            "to": "A(a)u2"
        }
    }],
    "id": "qwerty"
}
```

*Response:*

```
A(a)signals pub:
agents/A(a)u1/in/signals.netology-group.services/api/v1
```

```json
{
    "jsonrpc": "2.0",
    "result": {},
    "id": "qwerty"
}
```

## Events

> TODO:
