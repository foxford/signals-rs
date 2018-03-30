# Agents

## Methods
- [Create](./api.agents.create.html)
- [Read](./api.agents.read.html)
- [Update](./api.agents.update.html)
- [Delete](./api.agents.delete.html)
- [List](./api.agents.list.html)
- [Join room](./api.agents.join_room.html)
- [Leave room](./api.agents.leave_room.html)

## Events
```
A(a)signals pub:
apps/signals.netology-group.services/api/v1/rooms/050b7c6f-795c-4cb4-aeea-5ee3f9083de2/agents
```

### Join room
```json
{
    "jsonrpc": "2.0",
    "method": "event",
    "params": [{
        "type": "agent.join_room",
        "payload": {
            "agent_id": "1154b35c-e5b0-4a42-8ab2-d4967ce38c9e",
            "room_id": "050b7c6f-795c-4cb4-aeea-5ee3f9083de2"
        }
    }]
}
```

### Leave room
```json
{
    "jsonrpc": "2.0",
    "method": "event",
    "params": [{
        "type": "agent.leave_room",
        "payload": {
            "agent_id": "1154b35c-e5b0-4a42-8ab2-d4967ce38c9e",
            "room_id": "050b7c6f-795c-4cb4-aeea-5ee3f9083de2"
        }
    }]
}
```
