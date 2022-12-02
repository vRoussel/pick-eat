-- Database generated with pgModeler (PostgreSQL Database Modeler).
-- pgModeler version: 1.0.0-beta1
-- PostgreSQL version: 13.0
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
	is_favorite boolean NOT NULL DEFAULT false,
	CONSTRAINT recipes_pk PRIMARY KEY (id),
	CONSTRAINT recipes_ck_times CHECK (preparation_time_min >= 0 AND cooking_time_min >= 0)
);
-- ddl-end --
ALTER TABLE public.recipes OWNER TO pickeat;
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
	CONSTRAINT seasons_pk PRIMARY KEY (id),
	CONSTRAINT seasons_uq_name UNIQUE (name)
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

-- object: public.sentence_case | type: FUNCTION --
-- DROP FUNCTION IF EXISTS public.sentence_case(text) CASCADE;
CREATE FUNCTION public.sentence_case (s text)
	RETURNS text
	LANGUAGE sql
	IMMUTABLE 
	RETURNS NULL ON NULL INPUT
	SECURITY INVOKER
	PARALLEL UNSAFE
	COST 1
	AS $$
select upper(left($1, 1)) || lower(right($1, -1));
$$;
-- ddl-end --
ALTER FUNCTION public.sentence_case(text) OWNER TO pickeat;
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
-- object: trgm_idx | type: INDEX --
-- DROP INDEX IF EXISTS public.trgm_idx CASCADE;
CREATE INDEX trgm_idx ON public.recipes
USING gist
(
	name public.gist_trgm_ops
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
ON DELETE NO ACTION ON UPDATE CASCADE;
-- ddl-end --

-- object: recipes_seasons_fk_recipe_id | type: CONSTRAINT --
-- ALTER TABLE public.recipes_seasons DROP CONSTRAINT IF EXISTS recipes_seasons_fk_recipe_id CASCADE;
ALTER TABLE public.recipes_seasons ADD CONSTRAINT recipes_seasons_fk_recipe_id FOREIGN KEY (recipe_id)
REFERENCES public.recipes (id) MATCH FULL
ON DELETE CASCADE ON UPDATE CASCADE;
-- ddl-end --


