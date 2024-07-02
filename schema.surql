-- user table
REMOVE TABLE user;
DEFINE TABLE IF NOT EXISTS user;
DEFINE FIELD username ON TABLE user TYPE string;
// DEFINE FIELD email ON TABLE user TYPE string ASSERT string::is::email($value);
// DEFINE INDEX userEmailIndex ON TABLE user COLUMNS email UNIQUE;
CREATE user SET username = "john";

-- event table
REMOVE TABLE event;
DEFINE TABLE IF NOT EXISTS event;
DEFINE FIELD type ON TABLE event TYPE string;
DEFINE FIELD data ON TABLE event TYPE any;
DEFINE FIELD created_at ON TABLE event TYPE datetime;

-- chat table
REMOVE TABLE chat;
DEFINE TABLE IF NOT EXISTS chat;
CREATE chat:test;

-- chta_events relation
REMOVE TABLE chat_events;
DEFINE TABLE chat_events TYPE RELATION;
DEFINE FIELD in ON TABLE chat_events TYPE record<chat>;
DEFINE FIELD out ON TABLE chat_events TYPE record<event>;
DEFINE INDEX unique_chat_events 
    ON TABLE chat_events
    COLUMNS in, out UNIQUE;
