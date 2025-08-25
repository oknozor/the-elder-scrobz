-- Add migration script here
CREATE OR REPLACE FUNCTION notify_on_thumbnail_updated() RETURNS TRIGGER AS $$
BEGIN
    PERFORM pg_notify('thumbnail_updated', row_to_json(NEW)::text);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER artist_updated_trigger
    AFTER UPDATE OF thumbnail_url ON artists
    FOR EACH ROW
    WHEN (OLD.thumbnail_url DISTINCT FROM NEW.thumbnail_url)
EXECUTE FUNCTION notify_on_thumbnail_updated();
