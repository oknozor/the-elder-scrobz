ALTER TABLE artists ADD COLUMN thumbnail_url TEXT;

CREATE OR REPLACE FUNCTION notify_on_release_inserted() RETURNS TRIGGER AS $$
BEGIN
    PERFORM pg_notify('release_inserted', row_to_json(NEW)::text);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER release_insert_trigger
    AFTER INSERT ON releases
    FOR EACH ROW
EXECUTE FUNCTION notify_on_release_inserted();


CREATE OR REPLACE FUNCTION notify_on_artist_inserted() RETURNS TRIGGER AS $$
BEGIN
    PERFORM pg_notify('artist_inserted', row_to_json(NEW)::text);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER artist_insert_trigger
    AFTER INSERT ON artists
    FOR EACH ROW
EXECUTE FUNCTION notify_on_artist_inserted();