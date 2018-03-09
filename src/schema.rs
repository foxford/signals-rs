table! {
    agents (id) {
        id -> Uuid,
        label -> Text,
        room_id -> Uuid,
    }
}

table! {
    rooms (id) {
        id -> Uuid,
        label -> Text,
    }
}

joinable!(agents -> rooms (room_id));

allow_tables_to_appear_in_same_query!(agents, rooms,);
