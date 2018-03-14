# List
## Request
```
A(a)u1 pub:
agents/A(a)u1/out/signals.netology-group.services/api/v1
```
```json
{
    "jsonrpc": "2.0",
    "method": "agent.list",
    "params": [{
        "fq": "room_id:050b7c6f-795c-4cb4-aeea-5ee3f9083de2"
    }],
    "id": "qwerty"
}
```
Filters are optional. The agent will receive the data in accordance with the access rights.
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
            "id": "1154b35c-e5b0-4a42-8ab2-d4967ce38c9e",
            "data": {
                "label": "John Doe",
                "created_at": "2018-03-14T08:03:33.923744"
            }
        },
        {
            "id": "ad0dbebd-a685-48a1-85e3-d01cda9d98e4",
            "data": {
                "label": "Johnny Appleseed",
                "created_at": "2018-03-14T08:04:44.923744"
            }
        }
    ],
    "id": "qwerty"
}
```
