CREATE TABLE IF NOT EXISTS todos (
    id bigserial PRIMARY KEY,
    description text NOT NULL,
    done boolean NOT NULL DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS todo_things (
    id bigserial PRIMARY KEY,
    todo_id bigint NOT NULL
);
