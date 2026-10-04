-- Extensions and helpers shared by every later migration.

CREATE FUNCTION set_updated_at() RETURNS trigger
LANGUAGE plpgsql AS $$
begin
  new.updated_at = now();
  return new;
end
$$;

