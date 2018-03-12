# Tracks

## Methods

- [Create](api.tracks.create.html)
- [Delete](api.tracks.delete.html)
- [Register](api.tracks.register.html)
- [Unregister](api.tracks.unregister.html)
- [List](api.tracks.list.html)

## Events

```
A(a)u1 sub:
apps/signals.netology-group.services/api/v1/rooms/050b7c6f-795c-4cb4-aeea-5ee3f9083de2/tracks
```

### Create

```json
{
    "jsonrpc": "2.0",
    "method": "event",
    "params": [{
        "type": "track.create",
        "payload": {
            "id": "915adba9-5586-4743-a22e-47cc57260e37",
            "data": {
                "stream_id": "b16724bb-e970-4f4e-93c6-ac0649d43bc9",
                "track_id": "0ba2daf8-dfa3-4e64-b535-681508b6834d",
                "device": "camera",
                "kind": "video",
                "label": "human readable",
                "owner_id": "1154b35c-e5b0-4a42-8ab2-d4967ce38c9e",
                "holders": []
            }
        }
    }]
}
```

### Register (kind of update)

```json
{
    "jsonrpc": "2.0",
    "method": "event",
    "params": [{
        "type": "track.update",
        "payload": {
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
        }
    }]
}
```

### Delete

```json
{
    "jsonrpc": "2.0",
    "method": "event",
    "params": [{
        "type": "track.delete",
        "payload": {
            "id": "915adba9-5586-4743-a22e-47cc57260e37",
            "data": {
                "stream_id": "b16724bb-e970-4f4e-93c6-ac0649d43bc9",
                "track_id": "0ba2daf8-dfa3-4e64-b535-681508b6834d",
                "device": "camera",
                "kind": "video",
                "label": "human readable",
                "owner_id": "1154b35c-e5b0-4a42-8ab2-d4967ce38c9e",
                "holders": []
            }
        }
    }]
}
```

### Unregister (kind of update)

### Register

```json
{
    "jsonrpc": "2.0",
    "method": "event",
    "params": [{
        "type": "track.update",
        "payload": {
            "id": "915adba9-5586-4743-a22e-47cc57260e37",
            "data": {
                "stream_id": "b16724bb-e970-4f4e-93c6-ac0649d43bc9",
                "track_id": "0ba2daf8-dfa3-4e64-b535-681508b6834d",
                "device": "camera",
                "kind": "video",
                "label": "human readable",
                "owner_id": "1154b35c-e5b0-4a42-8ab2-d4967ce38c9e",
                "holders": [
                    // Agents with this track
                ]
            }
        }
    }]
}
```
