# Scripts

One can use scripts below to interact with the application throught command line.

## Usage
```bash
MQTT_HOST=mqtt.testing.netology-group.services ./your-script.sh
```

## Ping

```bash
#!/bin/bash

MSG=`cat <<EOF
{
    "jsonrpc": "2.0",
    "method": "ping",
    "params": [],
    "id": "qwetry"
}
EOF
`

MQTT_HOST=${MQTT_HOST:-$(docker-machine ip default)}
MQTT_CLIENT_ID=${MQTT_CLIENT_ID:-"$(uuidgen).$(uuidgen)"}

set -x
mosquitto_pub \
    -h ${MQTT_HOST} \
    -i ${MQTT_CLIENT_ID} \
    -t 'ping' \
    -m "${MSG}"
```


## Rooms

### Create

```bash
#!/bin/bash

if [ "$#" -ne 1 ]; then
    echo "Illegal number of parameters"
    exit 1
fi

MSG=`cat <<EOF
{
    "jsonrpc": "2.0",
    "method": "room.create",
    "params": [],
    "id": "qwerty"
}
EOF
`

MQTT_HOST=${MQTT_HOST:-$(docker-machine ip default)}
MQTT_CLIENT_ID=${MQTT_CLIENT_ID:-"$(uuidgen).$(uuidgen)"}

set -x
mosquitto_pub \
    -h ${MQTT_HOST} \
    -i ${MQTT_CLIENT_ID} \
    -t 'agents/55e813bc-0c9b-4270-9f7f-81e5ffcfc9ff/out/signals.netology-group.services/api/v1' \
    -m "${MSG}" \
    -q 1
```

### Read

```bash
#!/bin/bash

if [ "$#" -ne 1 ]; then
    echo "Illegal number of parameters"
    exit 1
fi

MSG=`cat <<EOF
{
    "jsonrpc": "2.0",
    "method": "room.read",
    "params": [{
        "room_id": "$1"
    }],
    "id": "qwerty"
}
EOF
`

MQTT_HOST=${MQTT_HOST:-$(docker-machine ip default)}
MQTT_CLIENT_ID=${MQTT_CLIENT_ID:-"$(uuidgen).$(uuidgen)"}

set -x
mosquitto_pub \
    -h ${MQTT_HOST} \
    -i ${MQTT_CLIENT_ID} \
    -t 'agents/55e813bc-0c9b-4270-9f7f-81e5ffcfc9ff/out/signals.netology-group.services/api/v1' \
    -m "${MSG}" \
    -q 1
```

### Update

```bash
#!/bin/bash

if [ "$#" -ne 2 ]; then
    echo "Illegal number of parameters"
    exit 1
fi

MSG=`cat <<EOF
{
    "jsonrpc": "2.0",
    "method": "room.update",
    "params": [{
        "room_id": "$1",
        "data": {
            "label": "$2"
        }
    }],
    "id": "qwerty"
}
EOF
`

MQTT_HOST=${MQTT_HOST:-$(docker-machine ip default)}
MQTT_CLIENT_ID=${MQTT_CLIENT_ID:-"$(uuidgen).$(uuidgen)"}

set -x
mosquitto_pub \
    -h ${MQTT_HOST} \
    -i ${MQTT_CLIENT_ID} \
    -t 'agents/55e813bc-0c9b-4270-9f7f-81e5ffcfc9ff/out/signals.netology-group.services/api/v1' \
    -m "${MSG}" \
    -q 1
```

### Delete

```bash
#!/bin/bash

if [ "$#" -ne 1 ]; then
    echo "Illegal number of parameters"
    exit 1
fi

MSG=`cat <<EOF
{
    "jsonrpc": "2.0",
    "method": "room.delete",
    "params": [{
        "room_id": "$1"
    }],
    "id": "qwerty"
}
EOF
`

MQTT_HOST=${MQTT_HOST:-$(docker-machine ip default)}
MQTT_CLIENT_ID=${MQTT_CLIENT_ID:-"$(uuidgen).$(uuidgen)"}

set -x
mosquitto_pub \
    -h ${MQTT_HOST} \
    -i ${MQTT_CLIENT_ID} \
    -t 'agents/55e813bc-0c9b-4270-9f7f-81e5ffcfc9ff/out/signals.netology-group.services/api/v1' \
    -m "${MSG}" \
    -q 1
```

### List

```bash
#!/bin/bash

MSG=`cat <<EOF
{
    "jsonrpc": "2.0",
    "method": "room.list",
    "params": [],
    "id": "qwerty"
}
EOF
`

MQTT_HOST=${MQTT_HOST:-$(docker-machine ip default)}
MQTT_CLIENT_ID=${MQTT_CLIENT_ID:-"$(uuidgen).$(uuidgen)"}

set -x
mosquitto_pub \
    -h ${MQTT_HOST} \
    -i ${MQTT_CLIENT_ID} \
    -t 'agents/55e813bc-0c9b-4270-9f7f-81e5ffcfc9ff/out/signals.netology-group.services/api/v1' \
    -m "${MSG}" \
    -q 1
```
