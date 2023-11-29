-- There is a lot of discussion on stackoverflow about the differences between text and
-- varchar on the link below. I've settled on text with constraints for this db, though
-- the reasons for which aren't entirely clear - particularly as modern PG (>=9.2 ?) should
-- be able to update the varchar bounds without a table rewrite.
-- https://stackoverflow.com/questions/4848964/difference-between-text-and-varchar-character-varying

-- Note also that there is a lot of constraint-checking done here that will also be done
-- in the application. That's fiddly and annoying, especially it invokes a lot of PG-specific
-- syntax that means this isn't trivially reproducible in a sqlite db for example. I think
-- it's better to have a last line of defense

-- Constraints that are likely to be changed later are name
create table if not exists solutions (
    id serial primary key not null,
    user_id int8,
    created timestamptz not null,
    -- This could be an 1 byte instead of a 2 byte integer, but that's treated as a char
    -- and doesn't like to be compared with ints trivially
    year int2 not null constraint year_check check ( year <= 2023 and year > 2015 ),
    day int2 not null check (day <= 31 and  day > 0),
    -- This should perhaps be a bool "is_part1". That would be more work in views though
    part int2 not null check (part > 0 and part <= 2),
    functioning bool not null,
    -- This will eventually be a foreign key
    server_side_run_id int8,
    -- 1000 characters should be sufficient for any description
    description text constraint description_len_check check ( description <> '' and length(description) < 1000 ),
    -- 100k characters seems a lot for a solution, but my previous run to 22k
    solution text constraint solution_len_check check ( solution <> '' and length(solution) < 100000),
    language text check (language <> '' and length(language) < 20),
    claim_wall_time_s float8,
    claim_processors_used int check (claim_processors_used > 0),
    machine_os text check (machine_os <> '' and length(machine_os) < 20),
    machine_arch text check (machine_arch <> '' and length(machine_arch) < 20),
    machine_cpu_name text check (machine_cpu_name <> '' and length(machine_cpu_name) < 200),
    machine_clock_freq int2,
    code_source_repository text check (code_source_repository <> '' and length(code_source_repository) < 200),
    code_source_filename text check (code_source_filename <> '' and length(code_source_filename) < 200),
    code_source_commit_id text check (code_source_commit_id <> '' and length(code_source_commit_id) < 200)
)