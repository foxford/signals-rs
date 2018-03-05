# Register (AKA, create remote track)

## Request

```
A(a)u1 pub:
agents/A(a)u1/out/signals.netology-group.services/api/v1
```

```json
{
    "jsonrpc": "2.0",
    "method": "track.register",
    "params": [{
        "room_id": "050b7c6f-795c-4cb4-aeea-5ee3f9083de2",
        "data": {
            "stream_id": "b16724bb-e970-4f4e-93c6-ac0649d43bc9",
            "track_id": "0ba2daf8-dfa3-4e64-b535-681508b6834d",
            "agent_id": "9839b399-e5f5-4f1c-acb8-bad33db615a9"
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
        "id": "915adba9-5586-4743-a22e-47cc57260e37",
        "data": {
            "stream_id": "b16724bb-e970-4f4e-93c6-ac0649d43bc9",
            "track_id": "0ba2daf8-dfa3-4e64-b535-681508b6834d",
            "device": "camera",
            "kind": "video",
            "label": "human readable",
            "owner_id": "1154b35c-e5b0-4a42-8ab2-d4967ce38c9e",
            "holders": [
                {
                    "id": "9839b399-e5f5-4f1c-acb8-bad33db615a9"
                }
            ]
        }
    },
    "id": "qwerty"
}
```
