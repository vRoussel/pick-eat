-- Database generated with pgModeler (PostgreSQL Database Modeler).
-- pgModeler  version: 0.9.2
-- PostgreSQL version: 12.0
-- Project Site: pgmodeler.io
-- Model Author: ---

-- object: valentin | type: ROLE --
-- DROP ROLE IF EXISTS valentin;
CREATE ROLE valentin WITH 
	CREATEDB
	LOGIN;
-- ddl-end --


-- Database creation must be done outside a multicommand file.
-- These commands were put in this file only as a convenience.
-- -- object: pick_eat | type: DATABASE --
-- -- DROP DATABASE IF EXISTS pick_eat;
-- CREATE DATABASE pick_eat;
-- -- ddl-end --
-- 

-- object: public.recipes | type: TABLE --
-- DROP TABLE IF EXISTS public.recipes CASCADE;
CREATE TABLE public.recipes (
	id integer NOT NULL GENERATED ALWAYS AS IDENTITY ,
	name text NOT NULL,
	cook_time interval HOUR TO MINUTE  NOT NULL,
	description text NOT NULL,
	rating smallint NOT NULL,
	image bytea NOT NULL,
	publication_date date NOT NULL DEFAULT CURRENT_DATE,
	category text,
	CONSTRAINT recipes_ck_rating_range CHECK (rating between 0 and 5),
	CONSTRAINT recipes_pkey PRIMARY KEY (id)

);
-- ddl-end --
ALTER TABLE public.recipes OWNER TO valentin;
-- ddl-end --

-- object: public.categories | type: TABLE --
-- DROP TABLE IF EXISTS public.categories CASCADE;
CREATE TABLE public.categories (
	name text NOT NULL,
	CONSTRAINT categories_pley PRIMARY KEY (name)

);
-- ddl-end --
ALTER TABLE public.categories OWNER TO valentin;
-- ddl-end --

-- object: public.tags | type: TABLE --
-- DROP TABLE IF EXISTS public.tags CASCADE;
CREATE TABLE public.tags (
	name text NOT NULL,
	CONSTRAINT tags_pkey PRIMARY KEY (name)

);
-- ddl-end --
ALTER TABLE public.tags OWNER TO valentin;
-- ddl-end --

-- object: public.recipes_tags | type: TABLE --
-- DROP TABLE IF EXISTS public.recipes_tags CASCADE;
CREATE TABLE public.recipes_tags (
	tag text NOT NULL,
	recipe_id integer NOT NULL,
	CONSTRAINT recipes_tags_pkey PRIMARY KEY (tag,recipe_id)

);
-- ddl-end --
ALTER TABLE public.recipes_tags OWNER TO valentin;
-- ddl-end --

-- object: public.ingredients | type: TABLE --
-- DROP TABLE IF EXISTS public.ingredients CASCADE;
CREATE TABLE public.ingredients (
	name text NOT NULL,
	CONSTRAINT ingredients_pkey PRIMARY KEY (name)

);
-- ddl-end --
ALTER TABLE public.ingredients OWNER TO valentin;
-- ddl-end --

-- object: public.recipes_ingredients | type: TABLE --
-- DROP TABLE IF EXISTS public.recipes_ingredients CASCADE;
CREATE TABLE public.recipes_ingredients (
	recipe_id integer NOT NULL,
	ingredient text NOT NULL,
	quantity text NOT NULL,
	CONSTRAINT recipes_ingredients_pkey PRIMARY KEY (recipe_id,ingredient)

);
-- ddl-end --
ALTER TABLE public.recipes_ingredients OWNER TO valentin;
-- ddl-end --

-- object: public.recipes_instructions | type: TABLE --
-- DROP TABLE IF EXISTS public.recipes_instructions CASCADE;
CREATE TABLE public.recipes_instructions (
	recipe_id integer NOT NULL,
	step int2 NOT NULL,
	instruction text NOT NULL,
	CONSTRAINT recipes_instructions_pkey PRIMARY KEY (recipe_id,step),
	CONSTRAINT recipes_instructions_ck_step_strict_positive CHECK (step > 0)

);
-- ddl-end --
ALTER TABLE public.recipes_instructions OWNER TO valentin;
-- ddl-end --

-- object: recipes_fkey_categry | type: CONSTRAINT --
-- ALTER TABLE public.recipes DROP CONSTRAINT IF EXISTS recipes_fkey_categry CASCADE;
ALTER TABLE public.recipes ADD CONSTRAINT recipes_fkey_categry FOREIGN KEY (category)
REFERENCES public.categories (name) MATCH FULL
ON DELETE NO ACTION ON UPDATE CASCADE;
-- ddl-end --

-- object: recipes_tags_fkey_tag | type: CONSTRAINT --
-- ALTER TABLE public.recipes_tags DROP CONSTRAINT IF EXISTS recipes_tags_fkey_tag CASCADE;
ALTER TABLE public.recipes_tags ADD CONSTRAINT recipes_tags_fkey_tag FOREIGN KEY (tag)
REFERENCES public.tags (name) MATCH FULL
ON DELETE NO ACTION ON UPDATE CASCADE;
-- ddl-end --

-- object: recipes_tags_fkey_recipe_id | type: CONSTRAINT --
-- ALTER TABLE public.recipes_tags DROP CONSTRAINT IF EXISTS recipes_tags_fkey_recipe_id CASCADE;
ALTER TABLE public.recipes_tags ADD CONSTRAINT recipes_tags_fkey_recipe_id FOREIGN KEY (recipe_id)
REFERENCES public.recipes (id) MATCH FULL
ON DELETE CASCADE ON UPDATE CASCADE;
-- ddl-end --

-- object: recipes_ingredients_fkey_recipe_id | type: CONSTRAINT --
-- ALTER TABLE public.recipes_ingredients DROP CONSTRAINT IF EXISTS recipes_ingredients_fkey_recipe_id CASCADE;
ALTER TABLE public.recipes_ingredients ADD CONSTRAINT recipes_ingredients_fkey_recipe_id FOREIGN KEY (recipe_id)
REFERENCES public.recipes (id) MATCH FULL
ON DELETE CASCADE ON UPDATE CASCADE;
-- ddl-end --

-- object: recipes_ingredients_fkey_ingredient | type: CONSTRAINT --
-- ALTER TABLE public.recipes_ingredients DROP CONSTRAINT IF EXISTS recipes_ingredients_fkey_ingredient CASCADE;
ALTER TABLE public.recipes_ingredients ADD CONSTRAINT recipes_ingredients_fkey_ingredient FOREIGN KEY (ingredient)
REFERENCES public.ingredients (name) MATCH FULL
ON DELETE NO ACTION ON UPDATE CASCADE;
-- ddl-end --

-- object: recipes_instructions_fkey_recipe_id | type: CONSTRAINT --
-- ALTER TABLE public.recipes_instructions DROP CONSTRAINT IF EXISTS recipes_instructions_fkey_recipe_id CASCADE;
ALTER TABLE public.recipes_instructions ADD CONSTRAINT recipes_instructions_fkey_recipe_id FOREIGN KEY (recipe_id)
REFERENCES public.recipes (id) MATCH FULL
ON DELETE CASCADE ON UPDATE CASCADE;
-- ddl-end --


