CREATE OR REPLACE FUNCTION notify_on_insert() RETURNS TRIGGER AS $$
BEGIN
    PERFORM pg_notify('new_insert', row_to_json(NEW)::text);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER scrobbles_raw_insert_trigger
    AFTER INSERT ON scrobbles_raw
    FOR EACH ROW
EXECUTE FUNCTION notify_on_insert();
