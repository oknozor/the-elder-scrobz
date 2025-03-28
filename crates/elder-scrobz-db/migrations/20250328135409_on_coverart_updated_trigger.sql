CREATE OR REPLACE FUNCTION notify_on_coverart_updated() RETURNS TRIGGER AS $$
BEGIN
    PERFORM pg_notify('coverart_updated', row_to_json(NEW)::text);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER releases_updated_trigger
    AFTER UPDATE OF cover_art_url ON releases
    FOR EACH ROW
    WHEN (OLD.cover_art_url IS DISTINCT FROM NEW.cover_art_url)
EXECUTE FUNCTION notify_on_coverart_updated();
