table! {
    agent (id) {
        id -> Uuid,
        label -> Text,
        room_id -> Uuid,
    }
}

table! {
    room (id) {
        id -> Uuid,
        label -> Text,
    }
}

joinable!(agent -> room (room_id));

allow_tables_to_appear_in_same_query!(agent, room,);
