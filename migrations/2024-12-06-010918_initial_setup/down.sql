-- Drops the function used to configure the trigger.
DROP FUNCTION IF EXISTS manage_updated_at(_tbl REGCLASS);
-- Drops the trigger used for an automatic update.
DROP FUNCTION IF EXISTS on_updated_at();
