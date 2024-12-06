-- Updates a column called `updated_at` whenever the row is modified
-- (unless `updated_at` was included in the modified columns).
CREATE OR REPLACE FUNCTION refresh_updated_at() RETURNS TRIGGER AS
$$
BEGIN
    IF (
        new IS DISTINCT FROM old AND
        new.updated_at IS NOT DISTINCT FROM old.updated_at
        ) THEN
        new.updated_at := current_timestamp;
    END IF;
    RETURN new;
END;
$$ LANGUAGE plpgsql;

-- Sets up a trigger for the given table to automatically set a column called
-- `updated_at` whenever the row is modified (unless `updated_at` was included
-- in the modified columns).
CREATE OR REPLACE FUNCTION manage_updated_at(_tbl REGCLASS) RETURNS VOID AS
$$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE refresh_updated_at()', _tbl);
END;
$$ LANGUAGE plpgsql;
