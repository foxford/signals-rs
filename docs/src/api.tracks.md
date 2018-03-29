# Tracks

## Methods

- [Create](api.tracks.create.html)
- [Delete](api.tracks.delete.html)
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
                "owner_id": "1154b35c-e5b0-4a42-8ab2-d4967ce38c9e",
                "metadata": {}
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
                "owner_id": "1154b35c-e5b0-4a42-8ab2-d4967ce38c9e",
                "metadata": {}
            }
        }
    }]
}
```
