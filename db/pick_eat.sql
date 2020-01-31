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
	CONSTRAINT check_rating CHECK (rating between 0 and 5),
	CONSTRAINT pk_name_recipe PRIMARY KEY (id)

);
-- ddl-end --
ALTER TABLE public.recipes OWNER TO valentin;
-- ddl-end --

-- object: public.categories | type: TABLE --
-- DROP TABLE IF EXISTS public.categories CASCADE;
CREATE TABLE public.categories (
	name text NOT NULL,
	CONSTRAINT pk_name_categores PRIMARY KEY (name)

);
-- ddl-end --
ALTER TABLE public.categories OWNER TO valentin;
-- ddl-end --

-- object: public.tags | type: TABLE --
-- DROP TABLE IF EXISTS public.tags CASCADE;
CREATE TABLE public.tags (
	name text NOT NULL,
	CONSTRAINT pk_name_tags PRIMARY KEY (name)

);
-- ddl-end --
ALTER TABLE public.tags OWNER TO valentin;
-- ddl-end --

-- object: public.recipes_tags | type: TABLE --
-- DROP TABLE IF EXISTS public.recipes_tags CASCADE;
CREATE TABLE public.recipes_tags (
	tag text NOT NULL,
	recipe_id integer NOT NULL,
	CONSTRAINT pk_tag_recipe_id PRIMARY KEY (tag,recipe_id)

);
-- ddl-end --
ALTER TABLE public.recipes_tags OWNER TO valentin;
-- ddl-end --

-- object: fk_category | type: CONSTRAINT --
-- ALTER TABLE public.recipes DROP CONSTRAINT IF EXISTS fk_category CASCADE;
ALTER TABLE public.recipes ADD CONSTRAINT fk_category FOREIGN KEY (category)
REFERENCES public.categories (name) MATCH FULL
ON DELETE NO ACTION ON UPDATE CASCADE;
-- ddl-end --

-- object: fk_tag | type: CONSTRAINT --
-- ALTER TABLE public.recipes_tags DROP CONSTRAINT IF EXISTS fk_tag CASCADE;
ALTER TABLE public.recipes_tags ADD CONSTRAINT fk_tag FOREIGN KEY (tag)
REFERENCES public.tags (name) MATCH FULL
ON DELETE NO ACTION ON UPDATE CASCADE;
-- ddl-end --

-- object: fk_recipe_id | type: CONSTRAINT --
-- ALTER TABLE public.recipes_tags DROP CONSTRAINT IF EXISTS fk_recipe_id CASCADE;
ALTER TABLE public.recipes_tags ADD CONSTRAINT fk_recipe_id FOREIGN KEY (recipe_id)
REFERENCES public.recipes (id) MATCH FULL
ON DELETE CASCADE ON UPDATE CASCADE;
-- ddl-end --


