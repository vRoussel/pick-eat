#!/usr/bin/python3

import logging
import xml.etree.ElementTree as ET
from argparse import ArgumentParser
from configparser import ConfigParser

import psycopg2


def fetch_public_recipe_ids(db_conf):
    db_conn_uri = "postgres:///{}?host={}&port={}&user={}&password={}".format(
        db_conf["dbname"],
        db_conf["host"],
        db_conf.get("port", "5432"),
        db_conf["user"],
        db_conf.get("password", ""),
    )

    db_conn = psycopg2.connect(db_conn_uri)
    with db_conn.cursor() as c:
        c.execute(
            "SELECT id, coalesce(update_date, publication_date)::timestamptz from recipes where is_private = 'f'"
        )
        return c.fetchall()


def setup_logging(logfile, verbose):
    if verbose:
        log_lvl = logging.DEBUG
    else:
        log_lvl = logging.INFO

    if logfile:
        logging.basicConfig(filename=logfile, level=log_lvl)
    else:
        logging.basicConfig(level=log_lvl)


def url_elem(website_url, url_path, last_mod=None):
    elem = ET.Element("url")

    loc = ET.Element("loc")
    loc.text = f"{website_url}{url_path}"
    elem.append(loc)

    if last_mod:
        lastmod = ET.Element("lastmod")
        lastmod.text = last_mod.isoformat(sep="T")
        elem.append(lastmod)
    return elem


def gen_sitemap(website_url, recipe_ids):
    ns = {
        "xsi:schemaLocation": "http://www.sitemaps.org/schemas/sitemap/0.9 http://www.sitemaps.org/schemas/sitemap/0.9/sitemap.xsd",
        "xmlns:xsi": "http://www.w3.org/2001/XMLSchema-instance",
    }
    root = ET.Element("urlset", ns)
    root.append(url_elem(website_url, "/recipes"))
    for r in recipe_ids:
        e = url_elem(website_url, f"/recipe/{r[0]}", r[1])
        root.append(e)
    logging.debug(ET.tostring(root, encoding="utf8"))
    return ET.ElementTree(root)


if __name__ == "__main__":
    parser = ArgumentParser()
    parser.add_argument("--conf", "-c", required=True)
    parser.add_argument("--logfile", "-l")
    parser.add_argument("--verbose", "-v", action="store_true")
    args = parser.parse_args()

    setup_logging(args.logfile, args.verbose)

    config = ConfigParser()
    config.read(args.conf)

    db_conf = config["database"]
    recipe_ids = fetch_public_recipe_ids(db_conf)

    app_conf = config["app"]
    output_path = app_conf["sitemap_output_path"]
    website_url = app_conf["website_url"]

    xml = gen_sitemap(website_url, recipe_ids)
    xml.write(output_path)
