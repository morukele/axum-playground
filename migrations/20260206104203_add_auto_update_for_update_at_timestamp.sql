-- Add migration script here
-- A function to update the updated_at section of the table
CREATE OR REPLACE FUNCTION update_updated_at()
    RETURNS TRIGGER AS
$$
BEGIN
    NEW.updated_at = current_timestamp;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER todo_updated_at_trigger
    BEFORE UPDATE
    ON todos
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at();