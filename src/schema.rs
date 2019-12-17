table! {
    accounts (id) {
        id -> Integer,
        address -> Varchar,
        charge_user_id -> Nullable<Integer>,
        created_at -> Datetime,
        intention -> Nullable<Unsigned<Smallint>>,
        is_cooperation -> Nullable<Unsigned<Smallint>>,
        is_follow -> Nullable<Unsigned<Smallint>>,
        is_know_all -> Unsigned<Smallint>,
        is_sign -> Unsigned<Smallint>,
        last_charge_user_id -> Nullable<Integer>,
        name -> Varchar,
        recently_activity_date -> Nullable<Date>,
        school_id -> Integer,
        sign_type -> Nullable<Unsigned<Smallint>>,
        signed_at -> Nullable<Datetime>,
        start_activity_date -> Nullable<Date>,
        updated_at -> Nullable<Datetime>,
        area_id -> Integer,
        recently_activity_id -> Nullable<Integer>,
        start_activity_id -> Nullable<Integer>,
        city -> Nullable<Varchar>,
        city_id -> Nullable<Varchar>,
        people_count -> Nullable<Integer>,
        province -> Nullable<Varchar>,
        province_id -> Nullable<Varchar>,
        level -> Nullable<Unsigned<Smallint>>,
        nature -> Nullable<Unsigned<Smallint>>,
    }
}

table! {
    account_activities (id) {
        id -> Integer,
        account_id -> Integer,
        activity_date -> Datetime,
        created_at -> Datetime,
        current_content -> Longtext,
        intention -> Unsigned<Smallint>,
        next_content -> Nullable<Longtext>,
        next_date -> Nullable<Date>,
        problem_content -> Nullable<Longtext>,
        updated_at -> Nullable<Datetime>,
        user_id -> Integer,
        notification_count -> Unsigned<Integer>,
        img_urls -> Nullable<Longtext>,
        location -> Nullable<Varchar>,
        latest_comment_id -> Nullable<Integer>,
    }
}

table! {
    account_activity_comments (id) {
        id -> Integer,
        activity_id -> Integer,
        commentor_id -> Integer,
        content -> Longtext,
        created_at -> Datetime,
    }
}

table! {
    account_assign_records (id) {
        id -> Integer,
        account_id -> Integer,
        created_at -> Datetime,
        old_user_id -> Nullable<Integer>,
        updated_at -> Nullable<Datetime>,
        user_id -> Nullable<Integer>,
        handle_user_id -> Nullable<Integer>,
    }
}

table! {
    account_operate_infos (id) {
        id -> Integer,
        account_id -> Integer,
        bid_date -> Nullable<Date>,
        building_count -> Nullable<Integer>,
        created_at -> Datetime,
        had_public_washer -> Nullable<Unsigned<Smallint>>,
        is_bid -> Nullable<Unsigned<Smallint>>,
        nature -> Nullable<Unsigned<Smallint>>,
        pay_platform -> Nullable<Varchar>,
        pay_way -> Nullable<Unsigned<Smallint>>,
        self_buy_washer -> Nullable<Unsigned<Smallint>>,
        updated_at -> Nullable<Datetime>,
        washer_brand -> Nullable<Varchar>,
        washer_count -> Nullable<Integer>,
        washer_env -> Nullable<Unsigned<Smallint>>,
        washer_old_degree -> Nullable<Unsigned<Smallint>>,
        washer_used_year -> Nullable<Unsigned<Smallint>>,
    }
}

table! {
    account_operators (id) {
        id -> Integer,
        account_id -> Integer,
        contract_deadline -> Nullable<Date>,
        cooperation_year -> Nullable<Integer>,
        created_at -> Datetime,
        kp_name -> Nullable<Varchar>,
        kp_tel -> Nullable<Varchar>,
        name -> Nullable<Varchar>,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    account_state_changes (id) {
        id -> Integer,
        account_id -> Integer,
        change_content -> Varchar,
        created_at -> Datetime,
        new_value -> Integer,
        old_value -> Nullable<Integer>,
        #[sql_name = "type"]
        type_ -> Unsigned<Smallint>,
        updated_at -> Nullable<Datetime>,
        user_id -> Integer,
    }
}

table! {
    areas (id) {
        id -> Integer,
        created_at -> Datetime,
        name -> Varchar,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    cities (id) {
        id -> Integer,
        city_id -> Varchar,
        city -> Varchar,
        province_id -> Varchar,
    }
}

table! {
    contacts (id) {
        id -> Integer,
        account_id -> Integer,
        created_at -> Datetime,
        department -> Nullable<Varchar>,
        name -> Nullable<Varchar>,
        nature -> Nullable<Unsigned<Smallint>>,
        position -> Nullable<Varchar>,
        tel -> Nullable<Varchar>,
        #[sql_name = "type"]
        type_ -> Unsigned<Smallint>,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    info_change_fields (id) {
        id -> Integer,
        created_at -> Datetime,
        field_name -> Varchar,
        new_value -> Nullable<Varchar>,
        old_value -> Nullable<Varchar>,
        record_id -> Integer,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    info_change_records (id) {
        id -> Integer,
        account_id -> Nullable<Integer>,
        change_content -> Varchar,
        change_type -> Unsigned<Smallint>,
        content_id -> Integer,
        content_type -> Unsigned<Smallint>,
        created_at -> Datetime,
        updated_at -> Nullable<Datetime>,
        user_id -> Integer,
    }
}

table! {
    jurisdictions (id) {
        id -> Integer,
        created_at -> Datetime,
        name -> Varchar,
        updated_at -> Nullable<Datetime>,
        value -> Varchar,
    }
}

table! {
    operate_wash_modes (id) {
        id -> Integer,
        account_id -> Integer,
        created_at -> Datetime,
        operate_id -> Integer,
        price -> Nullable<Unsigned<Decimal>>,
        updated_at -> Nullable<Datetime>,
        mode -> Smallint,
    }
}

table! {
    provinces (id) {
        id -> Integer,
        province_id -> Varchar,
        province -> Varchar,
    }
}

table! {
    roles (id) {
        id -> Integer,
        name -> Varchar,
        created_at -> Datetime,
        #[sql_name = "type"]
        type_ -> Unsigned<Smallint>,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    role_jurisdictions (id) {
        id -> Integer,
        created_at -> Datetime,
        jurisdiction_id -> Integer,
        role_id -> Integer,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    schools (id) {
        id -> Integer,
        area_id -> Integer,
        charge_user_id -> Nullable<Integer>,
        city -> Nullable<Varchar>,
        city_id -> Nullable<Varchar>,
        created_at -> Datetime,
        level -> Nullable<Unsigned<Smallint>>,
        name -> Varchar,
        nature -> Nullable<Unsigned<Smallint>>,
        number -> Nullable<Varchar>,
        people_count -> Nullable<Integer>,
        province -> Nullable<Varchar>,
        province_id -> Nullable<Varchar>,
        reference_type -> Nullable<Unsigned<Smallint>>,
        remark -> Nullable<Varchar>,
        subordinate -> Nullable<Varchar>,
        teach_type -> Nullable<Unsigned<Smallint>>,
        #[sql_name = "type"]
        type_ -> Nullable<Unsigned<Smallint>>,
        updated_at -> Nullable<Datetime>,
        last_charge_user_id -> Nullable<Integer>,
    }
}

table! {
    school_assign_records (id) {
        id -> Integer,
        created_at -> Datetime,
        old_user_id -> Nullable<Integer>,
        school_id -> Integer,
        updated_at -> Nullable<Datetime>,
        user_id -> Nullable<Integer>,
        handle_user_id -> Nullable<Integer>,
    }
}

table! {
    users (id) {
        id -> Integer,
        avatar_url -> Nullable<Varchar>,
        created_at -> Datetime,
        email -> Nullable<Varchar>,
        name -> Varchar,
        phone -> Varchar,
        status -> Unsigned<Smallint>,
        uid -> Varchar,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    user_messages (id) {
        id -> Integer,
        account_id -> Integer,
        activity_id -> Nullable<Integer>,
        content -> Varchar,
        created_at -> Datetime,
        is_read -> Unsigned<Smallint>,
        #[sql_name = "type"]
        type_ -> Unsigned<Smallint>,
        updated_at -> Nullable<Datetime>,
        user_id -> Integer,
        activity_comment_id -> Nullable<Integer>,
    }
}

table! {
    user_roles (id) {
        id -> Integer,
        area_id -> Nullable<Integer>,
        created_at -> Datetime,
        role_id -> Integer,
        updated_at -> Nullable<Datetime>,
        user_id -> Integer,
    }
}

table! {
    user_role_change_records (id) {
        id -> Integer,
        area_id -> Nullable<Integer>,
        created_at -> Datetime,
        handle_user_id -> Integer,
        old_area_id -> Nullable<Integer>,
        old_role_id -> Nullable<Integer>,
        role_id -> Integer,
        updated_at -> Nullable<Datetime>,
        user_id -> Integer,
    }
}

table! {
    user_tokens (id) {
        id -> Integer,
        created_at -> Nullable<Datetime>,
        expire_at -> Datetime,
        token -> Varchar,
        uid -> Varchar,
        updated_at -> Nullable<Datetime>,
    }
}

allow_tables_to_appear_in_same_query!(
    accounts,
    account_activities,
    account_activity_comments,
    account_assign_records,
    account_operate_infos,
    account_operators,
    account_state_changes,
    areas,
    cities,
    contacts,
    info_change_fields,
    info_change_records,
    jurisdictions,
    operate_wash_modes,
    provinces,
    roles,
    role_jurisdictions,
    schools,
    school_assign_records,
    users,
    user_messages,
    user_roles,
    user_role_change_records,
    user_tokens,
);
