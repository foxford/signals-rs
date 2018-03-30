# List

## Request

```
A(a)u1 pub:
agents/A(a)u1/out/signals.netology-group.services/api/v1
```

```json
{
    "jsonrpc": "2.0",
    "method": "track.list",
    "params": [{
        // See details below
        "fq": ""
    }],
    "id": "qwerty"
}
```

#### Filters

Filters are optional. The agent will receive the data in accordance with the access rights.
You can use logical operators `AND`, `OR`

- List of tracks in a certain room
```
"fq": "room_id:050b7c6f-795c-4cb4-aeea-5ee3f9083de2"
```

- List of local tracks of a specific agent
```
"fq": "owner_id:1154b35c-e5b0-4a42-8ab2-d4967ce38c9e"
```

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
            "id": "915adba9-5586-4743-a22e-47cc57260e37",
            "data": {
                "owner_id": "1154b35c-e5b0-4a42-8ab2-d4967ce38c9e",
                "metadata": {}
            }
        },
        {
            "id": "470b537a-ec18-420b-81b5-04ba0ca8e014",
            "data": {
                "owner_id": "9923576c-1ee5-4987-a797-6cb4982d45de",
                "metadata": {}
            }
        }
    ],
    "id": "qwerty"
}
```
