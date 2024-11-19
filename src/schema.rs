// @generated automatically by Diesel CLI.

diesel::table! {
    animals (id) {
        id -> Uuid,
        name -> Text,
        description_post -> Nullable<Text>,
        score1 -> Nullable<Float8>,
        score2 -> Nullable<Float8>,
        score3 -> Nullable<Float8>,
        score4 -> Nullable<Float8>,
        score5 -> Nullable<Float8>,
        score6 -> Nullable<Float8>,
        score7 -> Nullable<Float8>,
        score8 -> Nullable<Float8>,
        score9 -> Nullable<Float8>,
        score10 -> Nullable<Float8>,
        n_of_votes -> Nullable<Int8>,
    }
}

diesel::table! {
    friends (id) {
        id -> Uuid,
        user1 -> Nullable<Uuid>,
        user2 -> Nullable<Uuid>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Text,
    }
}

diesel::table! {
    votes (id) {
        id -> Uuid,
        user_id -> Nullable<Uuid>,
        animal_id -> Nullable<Uuid>,
        score1 -> Nullable<Int2>,
        score2 -> Nullable<Int2>,
        score3 -> Nullable<Int2>,
        score4 -> Nullable<Int2>,
        score5 -> Nullable<Int2>,
        score6 -> Nullable<Int2>,
        score7 -> Nullable<Int2>,
        score8 -> Nullable<Int2>,
        score9 -> Nullable<Int2>,
        score10 -> Nullable<Int2>,
    }
}

diesel::joinable!(votes -> animals (animal_id));
diesel::joinable!(votes -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    animals,
    friends,
    users,
    votes,
);
