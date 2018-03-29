table! {
    agent (id) {
        id -> Uuid,
        label -> Text,
        room_id -> Uuid,
        created_at -> Timestamp,
    }
}

table! {
    room (id) {
        id -> Uuid,
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

joinable!(agent -> room (room_id));
joinable!(track -> agent (owner_id));

allow_tables_to_appear_in_same_query!(agent, room, track,);
