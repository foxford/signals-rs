# Concepts

The application defines several types of objects whith certain set of
operations.

## Agents
Every application instance is an agent, and every user agent is an agent.
Agents communicate with each other through MQTT. Typical user agents are: web
browsers, mobile apps, etc.

## Tracks
Agent can capture and share
[media streams](https://developer.mozilla.org/en-US/docs/Web/API/MediaStream).
Each media stream consists of a number of
[media tracks](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrack).
In order to share a media stream an agent should share information about it's
tracks. Then others can decide if they want to receive a whole stream (all
tracks) or just part of a stream (i.e. video track only).

There are **local** and **remote** tracks.
Local tracks are created once an agent captures a stream and share it. A local
track represents information about a track itself and it's owner. Once others
start to receive this local track a remote track is created. Essentially, remote
track is just a link between a local track and an agent which receives it.

## Rooms
A room is a list of agents who intend to exchange data with each other.
Virtual room is a way to group agents in order to start WebRTC signaling session.

## Topics
The operations are performed through interaction through the API, which is the
publication of messages with payload in JSON-RPC format.

Signals-rs is subscribed to following topics:

`agents/+/out/signals.netology-group.services/api/v1`, where '`+`' is the agent
id.

Signals-rs will publish messages at:

`agents/$AGENT_ID/in/signals.netology-group.services/api/v1`, where $AGENT_ID is
the identifier of the agent to which the application responds.

## Events and Subscriptions
Some operations on objects generate events - messages that signals-rs publishes
into topics like:

`apps/signals.netology-group.services/api/v1/rooms/$ROOM_ID/agents`

To subscribe to such a topic, the agent must create a
[subscription](./api.subscriptions.html).

