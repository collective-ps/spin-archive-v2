CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY,
    username text NOT NULL UNIQUE,
    password_hash text NOT NULL,
    email text UNIQUE,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    role smallint DEFAULT 0 NOT NULL,
    daily_upload_limit integer DEFAULT 1 NOT NULL
);

CREATE TABLE IF NOT EXISTS api_tokens (
    id BIGSERIAL PRIMARY KEY,
    token text NOT NULL,
    user_id INTEGER REFERENCES users (id) NOT NULL,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE IF NOT EXISTS audit_log (
    id BIGSERIAL PRIMARY KEY,
    table_name text NOT NULL,
    column_name text NOT NULL,
    row_id integer NOT NULL,
    changed_date timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    changed_by INTEGER REFERENCES users (id) NOT NULL,
    old_value text NOT NULL,
    new_value text NOT NULL
);

CREATE TABLE IF NOT EXISTS tags (
    id BIGSERIAL PRIMARY KEY,
    name text NOT NULL,
    description text DEFAULT ''::text NOT NULL,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    upload_count integer DEFAULT 0 NOT NULL
);

CREATE TABLE IF NOT EXISTS uploads (
    id BIGSERIAL PRIMARY KEY,
    status smallint DEFAULT 0 NOT NULL,
    file_id text NOT NULL UNIQUE,
    file_size bigint,
    file_name text,
    md5_hash text,
    uploader_user_id INTEGER REFERENCES users (id),
    source text,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    file_ext text NOT NULL,
    tag_string text DEFAULT ''::text NOT NULL,
    tag_index tsvector DEFAULT ''::tsvector NOT NULL,
    video_encoding_key text NOT NULL,
    thumbnail_url text,
    video_url text,
    description text DEFAULT ''::text NOT NULL,
    original_upload_date date
);


CREATE TABLE IF NOT EXISTS upload_comments (
    id BIGSERIAL PRIMARY KEY,
    upload_id INTEGER REFERENCES uploads (id) ON DELETE CASCADE NOT NULL,
    user_id INTEGER REFERENCES users (id) ON DELETE CASCADE NOT NULL,
    comment text DEFAULT ''::text NOT NULL,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE IF NOT EXISTS upload_views (
    id BIGSERIAL PRIMARY KEY,
    upload_id integer NOT NULL,
    viewed_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


CREATE UNIQUE INDEX api_tokens_token_idx ON api_tokens(token);
CREATE INDEX audit_log_idx ON audit_log (table_name, row_id);
CREATE INDEX index_uploads_on_file_name_index ON uploads USING gin (file_name gin_trgm_ops);
CREATE INDEX index_uploads_on_tags_index ON uploads USING gin (tag_index);
CREATE UNIQUE INDEX tags_name_unique_idx ON tags USING btree (name);
CREATE UNIQUE INDEX users_username_idx ON users(username);
CREATE UNIQUE INDEX users_email_idx ON users(email);

SELECT sqlx_manage_updated_at('api_tokens');
SELECT sqlx_manage_updated_at('tags');
SELECT sqlx_manage_updated_at('upload_comments');
SELECT sqlx_manage_updated_at('uploads');
SELECT sqlx_manage_updated_at('users');

CREATE TRIGGER trigger_uploads_on_tag_index_update
BEFORE INSERT OR UPDATE ON uploads
FOR EACH ROW EXECUTE PROCEDURE tsvector_update_trigger('tag_index', 'pg_catalog.english', 'tag_string');

ALTER TABLE upload_views
ADD CONSTRAINT upload_views_upload_id_fkey
FOREIGN KEY (upload_id)
REFERENCES uploads (id)
ON DELETE CASCADE;