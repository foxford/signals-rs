table! {
    agent (id) {
        id -> Uuid,
    }
}

table! {
    room (id) {
        id -> Uuid,
        created_at -> Timestamp,
        capacity -> Int2,
        available_from -> Timestamp,
        available_to -> Timestamp,
    }
}

table! {
    room_agent (agent_id, room_id) {
        agent_id -> Uuid,
        room_id -> Uuid,
        label -> Text,
        created_at -> Timestamp,
    }
}

table! {
    track (id) {
        id -> Uuid,
        owner_id -> Uuid,
        metadata -> Jsonb,
    }
}

joinable!(room_agent -> agent (agent_id));
joinable!(room_agent -> room (room_id));
joinable!(track -> agent (owner_id));

allow_tables_to_appear_in_same_query!(agent, room, room_agent, track,);
