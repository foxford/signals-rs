table! {
    agent (id) {
        id -> Uuid,
        label -> Text,
        room_id -> Uuid,
    }
}

table! {
    local_track (id) {
        id -> Uuid,
        stream_id -> Text,
        track_id -> Text,
        device -> Text,
        kind -> Text,
        label -> Text,
        owner_id -> Uuid,
    }
}

table! {
    remote_track (id) {
        id -> Uuid,
        local_track_id -> Uuid,
        agent_id -> Uuid,
    }
}

table! {
    room (id) {
        id -> Uuid,
        created_at -> Timestamp,
    }
}

joinable!(agent -> room (room_id));
joinable!(local_track -> agent (owner_id));
joinable!(remote_track -> agent (agent_id));
joinable!(remote_track -> local_track (local_track_id));

allow_tables_to_appear_in_same_query!(agent, local_track, remote_track, room,);
