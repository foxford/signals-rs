# Create

> TODO: Add a description of errors

## Request

```
A(a)u1 pub:
agents/A(a)u1/out/signals.netology-group.services/api/v1
```

```json
{
    "jsonrpc": "2.0",
    "method": "track.create",
    "params": [{
        "data": {
            "owner_id": "1154b35c-e5b0-4a42-8ab2-d4967ce38c9e",
            "metadata": {}
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
            "owner_id": "1154b35c-e5b0-4a42-8ab2-d4967ce38c9e",
            "metadata": {}
        }
    },
    "id": "qwerty"
}
```
