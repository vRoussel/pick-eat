-- Database generated with pgModeler (PostgreSQL Database Modeler).
-- pgModeler version: 1.0.5
-- PostgreSQL version: 15.0
-- Project Site: pgmodeler.io
-- Model Author: ---
-- -- object: pickeat | type: ROLE --
-- -- DROP ROLE IF EXISTS pickeat;
-- CREATE ROLE pickeat WITH 
-- 	CREATEROLE
-- 	LOGIN;
-- -- ddl-end --
-- 
-- -- object: pickeat_model | type: ROLE --
-- -- DROP ROLE IF EXISTS pickeat_model;
-- CREATE ROLE pickeat_model WITH 
-- 	LOGIN;
-- -- ddl-end --
-- 
-- -- object: pickeat_app | type: ROLE --
-- -- DROP ROLE IF EXISTS pickeat_app;
-- CREATE ROLE pickeat_app WITH 
-- 	LOGIN;
-- -- ddl-end --
-- 

-- Database creation must be performed outside a multi lined SQL file. 
-- These commands were put in this file only as a convenience.
-- 
-- object: pickeat | type: DATABASE --
-- DROP DATABASE IF EXISTS pickeat;
CREATE DATABASE pickeat;
-- ddl-end --


SET check_function_bodies = false;
-- ddl-end --

-- object: public.categories | type: TABLE --
-- DROP TABLE IF EXISTS public.categories CASCADE;
CREATE TABLE public.categories (
	id integer NOT NULL GENERATED ALWAYS AS IDENTITY ,
	name text NOT NULL,
	CONSTRAINT categories_pk PRIMARY KEY (id),
	CONSTRAINT categories_uq_name UNIQUE (name)
);
-- ddl-end --
ALTER TABLE public.categories OWNER TO pickeat;
-- ddl-end --

-- object: public.tags | type: TABLE --
-- DROP TABLE IF EXISTS public.tags CASCADE;
CREATE TABLE public.tags (
	id integer NOT NULL GENERATED ALWAYS AS IDENTITY ,
	name text NOT NULL,
	CONSTRAINT tags_pk PRIMARY KEY (id),
	CONSTRAINT tags_uq_name UNIQUE (name)
);
-- ddl-end --
ALTER TABLE public.tags OWNER TO pickeat;
-- ddl-end --

-- object: public.recipes_tags | type: TABLE --
-- DROP TABLE IF EXISTS public.recipes_tags CASCADE;
CREATE TABLE public.recipes_tags (
	tag_id integer NOT NULL,
	recipe_id integer NOT NULL,
	CONSTRAINT recipes_tags_pk PRIMARY KEY (tag_id,recipe_id)
);
-- ddl-end --
ALTER TABLE public.recipes_tags OWNER TO pickeat;
-- ddl-end --

-- object: public.ingredients | type: TABLE --
-- DROP TABLE IF EXISTS public.ingredients CASCADE;
CREATE TABLE public.ingredients (
	id integer NOT NULL GENERATED ALWAYS AS IDENTITY ,
	name text NOT NULL,
	default_unit_id integer,
	CONSTRAINT ingredients_pk PRIMARY KEY (id),
	CONSTRAINT ingredients_uq_name UNIQUE (name)
);
-- ddl-end --
ALTER TABLE public.ingredients OWNER TO pickeat;
-- ddl-end --

-- object: public.recipes_ingredients | type: TABLE --
-- DROP TABLE IF EXISTS public.recipes_ingredients CASCADE;
CREATE TABLE public.recipes_ingredients (
	recipe_id integer NOT NULL,
	ingredient_id integer NOT NULL,
	quantity real,
	unit_id integer,
	CONSTRAINT recipes_ingredients_pk PRIMARY KEY (recipe_id,ingredient_id),
	CONSTRAINT recipes_ingredients_ck_qty CHECK (quantity > 0)
);
-- ddl-end --
ALTER TABLE public.recipes_ingredients OWNER TO pickeat;
-- ddl-end --

-- object: public.recipes_categories | type: TABLE --
-- DROP TABLE IF EXISTS public.recipes_categories CASCADE;
CREATE TABLE public.recipes_categories (
	recipe_id integer NOT NULL,
	category_id integer NOT NULL,
	CONSTRAINT recipes_categories_pk PRIMARY KEY (recipe_id,category_id)
);
-- ddl-end --
ALTER TABLE public.recipes_categories OWNER TO pickeat;
-- ddl-end --

-- object: public.units | type: TABLE --
-- DROP TABLE IF EXISTS public.units CASCADE;
CREATE TABLE public.units (
	id integer NOT NULL GENERATED ALWAYS AS IDENTITY ,
	full_name text NOT NULL,
	short_name text NOT NULL,
	CONSTRAINT units_pk PRIMARY KEY (id),
	CONSTRAINT units_uq_full_name UNIQUE (full_name),
	CONSTRAINT units_uq_short_name UNIQUE (short_name)
);
-- ddl-end --
ALTER TABLE public.units OWNER TO pickeat;
-- ddl-end --

-- object: public.seasons | type: TABLE --
-- DROP TABLE IF EXISTS public.seasons CASCADE;
CREATE TABLE public.seasons (
	id integer NOT NULL GENERATED ALWAYS AS IDENTITY ,
	name text NOT NULL,
	label text NOT NULL,
	CONSTRAINT seasons_pk PRIMARY KEY (id),
	CONSTRAINT seasons_uq_name UNIQUE (name),
	CONSTRAINT seasons_uq_label UNIQUE (label)
);
-- ddl-end --
ALTER TABLE public.seasons OWNER TO pickeat;
-- ddl-end --

-- object: public.recipes_seasons | type: TABLE --
-- DROP TABLE IF EXISTS public.recipes_seasons CASCADE;
CREATE TABLE public.recipes_seasons (
	recipe_id integer NOT NULL,
	season_id integer NOT NULL,
	CONSTRAINT recipes_seasons_pk PRIMARY KEY (recipe_id,season_id)
);
-- ddl-end --
ALTER TABLE public.recipes_seasons OWNER TO pickeat;
-- ddl-end --

-- object: pg_trgm | type: EXTENSION --
-- DROP EXTENSION IF EXISTS pg_trgm CASCADE;
CREATE EXTENSION pg_trgm
WITH SCHEMA public;
-- ddl-end --

-- object: unaccent | type: EXTENSION --
-- DROP EXTENSION IF EXISTS unaccent CASCADE;
CREATE EXTENSION unaccent
WITH SCHEMA public;
-- ddl-end --

-- -- object: public.gist_trgm_ops | type: OPERATOR CLASS --
-- -- DROP OPERATOR CLASS IF EXISTS public.gist_trgm_ops USING gist CASCADE;
-- CREATE OPERATOR CLASS public.gist_trgm_ops FOR TYPE smallint
--  USING gist AS
-- 	STORAGE	text;
-- -- ddl-end --
-- ALTER OPERATOR CLASS public.gist_trgm_ops USING gist OWNER TO pickeat;
-- -- ddl-end --
-- 
-- object: public.accounts | type: TABLE --
-- DROP TABLE IF EXISTS public.accounts CASCADE;
CREATE TABLE public.accounts (
	id integer NOT NULL GENERATED ALWAYS AS IDENTITY ,
	display_name text NOT NULL,
	creation_date date NOT NULL DEFAULT CURRENT_DATE,
	email text NOT NULL,
	password text NOT NULL,
	is_admin bool NOT NULL DEFAULT false,
	is_validated bool NOT NULL DEFAULT false,
	CONSTRAINT accounts_pk PRIMARY KEY (id),
	CONSTRAINT accounts_uq_display_name UNIQUE (display_name),
	CONSTRAINT accounts_uq_email UNIQUE (email)
);
-- ddl-end --
ALTER TABLE public.accounts OWNER TO pickeat;
-- ddl-end --

-- object: public.accounts_fav_recipes | type: TABLE --
-- DROP TABLE IF EXISTS public.accounts_fav_recipes CASCADE;
CREATE TABLE public.accounts_fav_recipes (
	account_id integer NOT NULL,
	recipe_id integer NOT NULL,
	CONSTRAINT accounts_fav_recipes_pk PRIMARY KEY (account_id,recipe_id)
);
-- ddl-end --
ALTER TABLE public.accounts_fav_recipes OWNER TO pickeat;
-- ddl-end --

-- object: public.recipes | type: TABLE --
-- DROP TABLE IF EXISTS public.recipes CASCADE;
CREATE TABLE public.recipes (
	id integer NOT NULL GENERATED ALWAYS AS IDENTITY ,
	name text NOT NULL,
	notes text NOT NULL,
	preparation_time_min smallint NOT NULL,
	cooking_time_min smallint NOT NULL,
	image text NOT NULL,
	publication_date date NOT NULL DEFAULT CURRENT_DATE,
	instructions text[] NOT NULL,
	n_shares smallint NOT NULL,
	author_id integer NOT NULL,
	is_private bool DEFAULT false,
	CONSTRAINT recipes_pk PRIMARY KEY (id),
	CONSTRAINT recipes_ck_times CHECK (preparation_time_min >= 0 AND cooking_time_min >= 0)
);
-- ddl-end --
ALTER TABLE public.recipes OWNER TO pickeat;
-- ddl-end --

-- object: public.get_recipe_tags | type: FUNCTION --
-- DROP FUNCTION IF EXISTS public.get_recipe_tags(integer) CASCADE;
CREATE FUNCTION public.get_recipe_tags (IN recipe_id_in integer)
	RETURNS public.tags[]
	LANGUAGE sql
	STABLE 
	CALLED ON NULL INPUT
	SECURITY INVOKER
	PARALLEL SAFE
	COST 1
	AS $$
SELECT
    coalesce(
		array_agg(
			(t.id, t.name)::tags
		),
		'{}'
	) as tags
FROM
    tags AS t INNER JOIN recipes_tags AS rt
    ON t.id = rt.tag_id
WHERE rt.recipe_id = recipe_id_in
$$;
-- ddl-end --
ALTER FUNCTION public.get_recipe_tags(integer) OWNER TO pickeat;
-- ddl-end --

-- object: public.get_recipe_categories | type: FUNCTION --
-- DROP FUNCTION IF EXISTS public.get_recipe_categories(integer) CASCADE;
CREATE FUNCTION public.get_recipe_categories (IN recipe_id_in integer)
	RETURNS public.categories[]
	LANGUAGE sql
	STABLE 
	CALLED ON NULL INPUT
	SECURITY INVOKER
	PARALLEL SAFE
	COST 1
	AS $$
SELECT
    coalesce(
		array_agg(
			(c.id, c.name)::categories
		),
		'{}'
	) as categories
FROM
    categories AS c INNER JOIN recipes_categories AS ct
    ON c.id = ct.category_id
WHERE ct.recipe_id = recipe_id_in
$$;
-- ddl-end --
ALTER FUNCTION public.get_recipe_categories(integer) OWNER TO pickeat;
-- ddl-end --

-- object: public.get_recipe_seasons | type: FUNCTION --
-- DROP FUNCTION IF EXISTS public.get_recipe_seasons(integer) CASCADE;
CREATE FUNCTION public.get_recipe_seasons (IN recipe_id_in integer)
	RETURNS public.seasons[]
	LANGUAGE sql
	STABLE 
	CALLED ON NULL INPUT
	SECURITY INVOKER
	PARALLEL SAFE
	COST 1
	AS $$
SELECT
    coalesce(
		array_agg(
			(s.id, s.name, s.label)::seasons
		),
		'{}'
	) as seasons
FROM
    seasons AS s INNER JOIN recipes_seasons AS rs
    ON s.id = rs.season_id
WHERE rs.recipe_id = recipe_id_in
$$;
-- ddl-end --
ALTER FUNCTION public.get_recipe_seasons(integer) OWNER TO pickeat;
-- ddl-end --

-- object: trgm_idx | type: INDEX --
-- DROP INDEX IF EXISTS public.trgm_idx CASCADE;
CREATE INDEX trgm_idx ON public.recipes
USING gist
(
	name public.gist_trgm_ops
);
-- ddl-end --

-- object: public.diets | type: TABLE --
-- DROP TABLE IF EXISTS public.diets CASCADE;
CREATE TABLE public.diets (
	id integer NOT NULL GENERATED ALWAYS AS IDENTITY ,
	name text NOT NULL,
	label text,
	CONSTRAINT diets_pk PRIMARY KEY (id),
	CONSTRAINT diets_uq_name UNIQUE (name),
	CONSTRAINT diets_uq_label UNIQUE (label)
);
-- ddl-end --
ALTER TABLE public.diets OWNER TO pickeat;
-- ddl-end --

-- object: public.recipes_diets | type: TABLE --
-- DROP TABLE IF EXISTS public.recipes_diets CASCADE;
CREATE TABLE public.recipes_diets (
	recipe_id integer NOT NULL,
	diet_id integer NOT NULL,
	CONSTRAINT recipes_diets_pk PRIMARY KEY (recipe_id,diet_id)
);
-- ddl-end --
ALTER TABLE public.recipes_diets OWNER TO pickeat;
-- ddl-end --

-- object: public.get_recipe_diets | type: FUNCTION --
-- DROP FUNCTION IF EXISTS public.get_recipe_diets(integer) CASCADE;
CREATE FUNCTION public.get_recipe_diets (IN recipe_id_in integer)
	RETURNS public.diets[]
	LANGUAGE sql
	STABLE 
	CALLED ON NULL INPUT
	SECURITY INVOKER
	PARALLEL SAFE
	COST 1
	AS $$
SELECT
    coalesce(
		array_agg(
			(d.id, d.name, d.label)::diets
		),
		'{}'
	) as diets
FROM
    diets AS d INNER JOIN recipes_diets AS rd
    ON d.id = rd.diet_id
WHERE rd.recipe_id = recipe_id_in
$$;
-- ddl-end --
ALTER FUNCTION public.get_recipe_diets(integer) OWNER TO pickeat;
-- ddl-end --

-- object: public.account_validation_tokens | type: TABLE --
-- DROP TABLE IF EXISTS public.account_validation_tokens CASCADE;
CREATE TABLE public.account_validation_tokens (
	account_id integer NOT NULL,
	token text NOT NULL,
	valid_until timestamptz NOT NULL

);
-- ddl-end --
ALTER TABLE public.account_validation_tokens OWNER TO pickeat;
-- ddl-end --

-- object: public.password_reset_tokens | type: TABLE --
-- DROP TABLE IF EXISTS public.password_reset_tokens CASCADE;
CREATE TABLE public.password_reset_tokens (
	account_id integer NOT NULL,
	token text NOT NULL,
	valid_until timestamptz NOT NULL

);
-- ddl-end --
ALTER TABLE public.password_reset_tokens OWNER TO pickeat;
-- ddl-end --

-- object: public.is_recipe_in_account_favs | type: FUNCTION --
-- DROP FUNCTION IF EXISTS public.is_recipe_in_account_favs(integer,integer) CASCADE;
CREATE FUNCTION public.is_recipe_in_account_favs (IN recipe_id_in integer, IN account_id_in integer)
	RETURNS bool
	LANGUAGE sql
	VOLATILE 
	CALLED ON NULL INPUT
	SECURITY INVOKER
	PARALLEL UNSAFE
	COST 1
	AS $$
SELECT EXISTS(
    SELECT FROM accounts_fav_recipes
    WHERE
        recipe_id = recipe_id_in
        AND account_id = account_id_in
);

$$;
-- ddl-end --
ALTER FUNCTION public.is_recipe_in_account_favs(integer,integer) OWNER TO postgres;
-- ddl-end --

-- object: public.get_unit | type: FUNCTION --
-- DROP FUNCTION IF EXISTS public.get_unit(integer) CASCADE;
CREATE FUNCTION public.get_unit (IN unit_id_in integer)
	RETURNS public.units
	LANGUAGE sql
	VOLATILE 
	RETURNS NULL ON NULL INPUT
	SECURITY INVOKER
	PARALLEL UNSAFE
	COST 1
	AS $$
SELECT *
FROM units u
WHERE u.id = unit_id_in
$$;
-- ddl-end --
ALTER FUNCTION public.get_unit(integer) OWNER TO postgres;
-- ddl-end --
COMMENT ON FUNCTION public.get_unit(integer) IS E'If unit_id is null return null\nelse return (select * from units)';
-- ddl-end --

-- object: public.qingredient | type: TYPE --
-- DROP TYPE IF EXISTS public.qingredient CASCADE;
CREATE TYPE public.qingredient AS
(
 id integer,
 name text,
 quantity real,
 unit public.units
);
-- ddl-end --
ALTER TYPE public.qingredient OWNER TO pickeat;
-- ddl-end --

-- object: public.get_recipe_ingredients | type: FUNCTION --
-- DROP FUNCTION IF EXISTS public.get_recipe_ingredients(integer) CASCADE;
CREATE FUNCTION public.get_recipe_ingredients (IN recipe_id_in integer)
	RETURNS public.qingredient[]
	LANGUAGE sql
	STABLE 
	CALLED ON NULL INPUT
	SECURITY INVOKER
	PARALLEL SAFE
	COST 1
	AS $$
SELECT
    coalesce(
    array_agg(
        (
            i.id,
            i.name,
            ri.quantity,
            get_unit(ri.unit_id)
        )::qingredient
    ),
    '{}'
) as ingredients
FROM
    ingredients AS i INNER JOIN recipes_ingredients AS ri
    ON i.id = ri.ingredient_id
WHERE ri.recipe_id = recipe_id_in
$$;
-- ddl-end --
ALTER FUNCTION public.get_recipe_ingredients(integer) OWNER TO pickeat;
-- ddl-end --

-- object: public.ingredients_full | type: VIEW --
-- DROP VIEW IF EXISTS public.ingredients_full CASCADE;
CREATE VIEW public.ingredients_full
AS 

SELECT
   i.id,
   i.name,
   get_unit(i.default_unit_id) AS default_unit
FROM
   public.ingredients AS i;
-- ddl-end --
ALTER VIEW public.ingredients_full OWNER TO pickeat;
-- ddl-end --

-- Appended SQL commands --
update pg_attribute pa
set attnotnull = 't'
from pg_class pg
where
    pa.attrelid = pg.oid
    and relname = 'ingredients_full'
    and attname in ('id', 'name');
-- ddl-end --

-- object: public.recipes_full | type: VIEW --
-- DROP VIEW IF EXISTS public.recipes_full CASCADE;
CREATE VIEW public.recipes_full
AS 

SELECT
   r.*,
   get_recipe_tags(r.id) AS tags,
   get_recipe_categories(r.id) AS categories,
   get_recipe_diets(r.id) AS diets,
   get_recipe_seasons(r.id) AS seasons,
   get_recipe_ingredients(r.id) AS ingredients
FROM
   public.recipes AS r;
-- ddl-end --
ALTER VIEW public.recipes_full OWNER TO pickeat;
-- ddl-end --

-- Appended SQL commands --
update pg_attribute pa
set attnotnull = 't'
from pg_class pg
where
    pa.attrelid = pg.oid
    and relname = 'recipes_full'
    and attname in (
        'id',
        'name',
        'notes',
        'preparation_time_min',
        'cooking_time_min',
        'image',
        'publication_date',
        'instructions',
        'n_shares',
        'author_id',
        'is_private',
        'tags',
		'categories',
		'diets',
		'seasons',
		'ingredients'
	);

-- ddl-end --

-- object: public.recipes_full_overview | type: VIEW --
-- DROP VIEW IF EXISTS public.recipes_full_overview CASCADE;
CREATE VIEW public.recipes_full_overview
AS 

SELECT
   get_recipe_diets(r.id) AS diets,
   get_recipe_ingredients(r.id) AS ingredients,
   r.id,
   r.name,
   r.image,
   r.n_shares,
   r.is_private
FROM
   public.recipes AS r;
-- ddl-end --
ALTER VIEW public.recipes_full_overview OWNER TO pickeat;
-- ddl-end --

-- Appended SQL commands --
update pg_attribute pa
set attnotnull = 't'
from pg_class pg
where
    pa.attrelid = pg.oid
    and relname = 'recipes_full_overview'
    and attname in (
        'id',
        'name',
        'image',
        'n_shares',
        'is_private',
		'diets',
		'ingredients'
	);

-- ddl-end --

-- object: recipes_tags_fk_tag_id | type: CONSTRAINT --
-- ALTER TABLE public.recipes_tags DROP CONSTRAINT IF EXISTS recipes_tags_fk_tag_id CASCADE;
ALTER TABLE public.recipes_tags ADD CONSTRAINT recipes_tags_fk_tag_id FOREIGN KEY (tag_id)
REFERENCES public.tags (id) MATCH FULL
ON DELETE CASCADE ON UPDATE CASCADE;
-- ddl-end --

-- object: recipes_tags_fk_recipe_id | type: CONSTRAINT --
-- ALTER TABLE public.recipes_tags DROP CONSTRAINT IF EXISTS recipes_tags_fk_recipe_id CASCADE;
ALTER TABLE public.recipes_tags ADD CONSTRAINT recipes_tags_fk_recipe_id FOREIGN KEY (recipe_id)
REFERENCES public.recipes (id) MATCH FULL
ON DELETE CASCADE ON UPDATE CASCADE;
-- ddl-end --

-- object: ingredients_fk_default_unit | type: CONSTRAINT --
-- ALTER TABLE public.ingredients DROP CONSTRAINT IF EXISTS ingredients_fk_default_unit CASCADE;
ALTER TABLE public.ingredients ADD CONSTRAINT ingredients_fk_default_unit FOREIGN KEY (default_unit_id)
REFERENCES public.units (id) MATCH FULL
ON DELETE NO ACTION ON UPDATE CASCADE;
-- ddl-end --

-- object: recipes_ingredients_fk_recipe_id | type: CONSTRAINT --
-- ALTER TABLE public.recipes_ingredients DROP CONSTRAINT IF EXISTS recipes_ingredients_fk_recipe_id CASCADE;
ALTER TABLE public.recipes_ingredients ADD CONSTRAINT recipes_ingredients_fk_recipe_id FOREIGN KEY (recipe_id)
REFERENCES public.recipes (id) MATCH FULL
ON DELETE CASCADE ON UPDATE CASCADE;
-- ddl-end --

-- object: recipes_ingredients_fk_ingredient_id | type: CONSTRAINT --
-- ALTER TABLE public.recipes_ingredients DROP CONSTRAINT IF EXISTS recipes_ingredients_fk_ingredient_id CASCADE;
ALTER TABLE public.recipes_ingredients ADD CONSTRAINT recipes_ingredients_fk_ingredient_id FOREIGN KEY (ingredient_id)
REFERENCES public.ingredients (id) MATCH FULL
ON DELETE NO ACTION ON UPDATE CASCADE;
-- ddl-end --

-- object: recipes_ingredients_fk_unit_id | type: CONSTRAINT --
-- ALTER TABLE public.recipes_ingredients DROP CONSTRAINT IF EXISTS recipes_ingredients_fk_unit_id CASCADE;
ALTER TABLE public.recipes_ingredients ADD CONSTRAINT recipes_ingredients_fk_unit_id FOREIGN KEY (unit_id)
REFERENCES public.units (id) MATCH FULL
ON DELETE NO ACTION ON UPDATE CASCADE;
-- ddl-end --

-- object: recipes_categories_fk_recipe_id | type: CONSTRAINT --
-- ALTER TABLE public.recipes_categories DROP CONSTRAINT IF EXISTS recipes_categories_fk_recipe_id CASCADE;
ALTER TABLE public.recipes_categories ADD CONSTRAINT recipes_categories_fk_recipe_id FOREIGN KEY (recipe_id)
REFERENCES public.recipes (id) MATCH FULL
ON DELETE CASCADE ON UPDATE CASCADE;
-- ddl-end --

-- object: recipes_categories_fk_category_id | type: CONSTRAINT --
-- ALTER TABLE public.recipes_categories DROP CONSTRAINT IF EXISTS recipes_categories_fk_category_id CASCADE;
ALTER TABLE public.recipes_categories ADD CONSTRAINT recipes_categories_fk_category_id FOREIGN KEY (category_id)
REFERENCES public.categories (id) MATCH FULL
ON DELETE CASCADE ON UPDATE CASCADE;
-- ddl-end --

-- object: recipes_seasons_fk_season_id | type: CONSTRAINT --
-- ALTER TABLE public.recipes_seasons DROP CONSTRAINT IF EXISTS recipes_seasons_fk_season_id CASCADE;
ALTER TABLE public.recipes_seasons ADD CONSTRAINT recipes_seasons_fk_season_id FOREIGN KEY (season_id)
REFERENCES public.seasons (id) MATCH FULL
ON DELETE CASCADE ON UPDATE CASCADE;
-- ddl-end --

-- object: recipes_seasons_fk_recipe_id | type: CONSTRAINT --
-- ALTER TABLE public.recipes_seasons DROP CONSTRAINT IF EXISTS recipes_seasons_fk_recipe_id CASCADE;
ALTER TABLE public.recipes_seasons ADD CONSTRAINT recipes_seasons_fk_recipe_id FOREIGN KEY (recipe_id)
REFERENCES public.recipes (id) MATCH FULL
ON DELETE CASCADE ON UPDATE CASCADE;
-- ddl-end --

-- object: accounts_fav_recipes_fk_account_id | type: CONSTRAINT --
-- ALTER TABLE public.accounts_fav_recipes DROP CONSTRAINT IF EXISTS accounts_fav_recipes_fk_account_id CASCADE;
ALTER TABLE public.accounts_fav_recipes ADD CONSTRAINT accounts_fav_recipes_fk_account_id FOREIGN KEY (account_id)
REFERENCES public.accounts (id) MATCH SIMPLE
ON DELETE CASCADE ON UPDATE CASCADE;
-- ddl-end --

-- object: accounts_fav_recipes_fk_recipe_id | type: CONSTRAINT --
-- ALTER TABLE public.accounts_fav_recipes DROP CONSTRAINT IF EXISTS accounts_fav_recipes_fk_recipe_id CASCADE;
ALTER TABLE public.accounts_fav_recipes ADD CONSTRAINT accounts_fav_recipes_fk_recipe_id FOREIGN KEY (recipe_id)
REFERENCES public.recipes (id) MATCH SIMPLE
ON DELETE CASCADE ON UPDATE CASCADE;
-- ddl-end --

-- object: recipes_fk_author_id | type: CONSTRAINT --
-- ALTER TABLE public.recipes DROP CONSTRAINT IF EXISTS recipes_fk_author_id CASCADE;
ALTER TABLE public.recipes ADD CONSTRAINT recipes_fk_author_id FOREIGN KEY (author_id)
REFERENCES public.accounts (id) MATCH SIMPLE
ON DELETE NO ACTION ON UPDATE CASCADE;
-- ddl-end --

-- object: recipes_diets_fk_recipe_id | type: CONSTRAINT --
-- ALTER TABLE public.recipes_diets DROP CONSTRAINT IF EXISTS recipes_diets_fk_recipe_id CASCADE;
ALTER TABLE public.recipes_diets ADD CONSTRAINT recipes_diets_fk_recipe_id FOREIGN KEY (recipe_id)
REFERENCES public.recipes (id) MATCH SIMPLE
ON DELETE CASCADE ON UPDATE CASCADE;
-- ddl-end --

-- object: recipes_diets_fk_diet_id | type: CONSTRAINT --
-- ALTER TABLE public.recipes_diets DROP CONSTRAINT IF EXISTS recipes_diets_fk_diet_id CASCADE;
ALTER TABLE public.recipes_diets ADD CONSTRAINT recipes_diets_fk_diet_id FOREIGN KEY (diet_id)
REFERENCES public.diets (id) MATCH SIMPLE
ON DELETE CASCADE ON UPDATE CASCADE;
-- ddl-end --

-- object: account_validation_tokens_fk_account_id | type: CONSTRAINT --
-- ALTER TABLE public.account_validation_tokens DROP CONSTRAINT IF EXISTS account_validation_tokens_fk_account_id CASCADE;
ALTER TABLE public.account_validation_tokens ADD CONSTRAINT account_validation_tokens_fk_account_id FOREIGN KEY (account_id)
REFERENCES public.accounts (id) MATCH SIMPLE
ON DELETE CASCADE ON UPDATE CASCADE;
-- ddl-end --

-- object: password_reset_tokens_fk_account_id | type: CONSTRAINT --
-- ALTER TABLE public.password_reset_tokens DROP CONSTRAINT IF EXISTS password_reset_tokens_fk_account_id CASCADE;
ALTER TABLE public.password_reset_tokens ADD CONSTRAINT password_reset_tokens_fk_account_id FOREIGN KEY (account_id)
REFERENCES public.accounts (id) MATCH SIMPLE
ON DELETE CASCADE ON UPDATE CASCADE;
-- ddl-end --


