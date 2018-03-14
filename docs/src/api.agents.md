# Agents

## Methods
- [Create](./api.agents.create.html)
- [Read](./api.agents.read.html)
- [Update](./api.agents.update.html)
- [Delete](./api.agents.delete.html)
- [List](./api.agents.list.html)

## Events
```
A(a)signals pub:
apps/signals.netology-group.services/api/v1/rooms/050b7c6f-795c-4cb4-aeea-5ee3f9083de2/agents
```

### Create
```json
{
    "jsonrpc": "2.0",
    "method": "event",
    "params": [{
        "type": "agent.create",
        "payload": {
            "id": "1154b35c-e5b0-4a42-8ab2-d4967ce38c9e",
            "data": {
                "label": "John Doe",
                "created_at": "2018-03-14T08:03:33.923744"
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
        "type": "agent.delete",
        "payload": {
            "id": "1154b35c-e5b0-4a42-8ab2-d4967ce38c9e",
            "data": {
                "label": "John Doe",
                "created_at": "2018-03-14T08:03:33.923744"
            }
        }
    }]
}
```
