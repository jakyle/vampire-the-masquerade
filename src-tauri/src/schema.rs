// @generated automatically by Diesel CLI.

diesel::table! {
    action_logs (id) {
        id -> Text,
        difficulty -> Integer,
        modifier -> Integer,
        created_at -> Timestamp,
    }
}

diesel::table! {
    characters (id) {
        id -> Text,
        name -> Text,
        is_active -> Bool,
        strength -> Integer,
        dexterity -> Integer,
        stamina -> Integer,
        charisma -> Integer,
        manipulation -> Integer,
        composure -> Integer,
        intelligence -> Integer,
        wits -> Integer,
        resolve -> Integer,
        athletics -> Integer,
        brawl -> Integer,
        crafts -> Integer,
        drive -> Integer,
        firearms -> Integer,
        melee -> Integer,
        larceny -> Integer,
        stealth -> Integer,
        survival -> Integer,
        animal_ken -> Integer,
        etiquette -> Integer,
        insight -> Integer,
        intimidation -> Integer,
        leadership -> Integer,
        performance -> Integer,
        persuasion -> Integer,
        streetwise -> Integer,
        subterfuge -> Integer,
        academics -> Integer,
        awareness -> Integer,
        finance -> Integer,
        investigation -> Integer,
        medicine -> Integer,
        occult -> Integer,
        politics -> Integer,
        science -> Integer,
        technology -> Integer,
        hunger -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    dice_results (id) {
        id -> Text,
        character_id -> Nullable<Text>,
        log_id -> Nullable<Text>,
        successes -> Integer,
        criticals -> Integer,
        beastial_failure -> Bool,
        messy_critical -> Bool,
        succeeded -> Bool,
        rolls -> Text,
        hunger_rolls -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    passive_results (id) {
        id -> Text,
        character_id -> Nullable<Text>,
        log_id -> Nullable<Text>,
        succeeded -> Bool,
        hunger -> Integer,
        total -> Integer,
        created_at -> Timestamp,
    }
}

diesel::joinable!(dice_results -> action_logs (log_id));
diesel::joinable!(dice_results -> characters (character_id));
diesel::joinable!(passive_results -> action_logs (log_id));
diesel::joinable!(passive_results -> characters (character_id));

diesel::allow_tables_to_appear_in_same_query!(
    action_logs,
    characters,
    dice_results,
    passive_results,
);
