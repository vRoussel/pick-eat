use crate::conf::DBConf;
use sqlx::{
    postgres::{PgPool, PgPoolOptions},
    query, Error,
};

pub async fn get_pool(db_conf: &DBConf) -> Result<PgPool, Box<dyn std::error::Error>> {
    let conn_str = format!(
        "postgres:///{}?host={}&port={}&user={}&password={}",
        db_conf.dbname,
        db_conf.host,
        db_conf.port.unwrap_or(5432).to_string(),
        db_conf.user,
        db_conf.password.clone().unwrap_or_default(),
    );

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&conn_str)
        .await?;

    Ok(pool)
}

pub async fn add_default_data(db_conn: &PgPool) -> Result<(), Error> {
    let mut transaction = db_conn.begin().await?;

    // Seasons
    query!(
        "
        INSERT INTO seasons (label,name) VALUES
            ('spring', 'Printemps')
            ,('summer', 'Été')
            ,('fall', 'Automne')
            ,('winter', 'Hiver')
        ON CONFLICT DO NOTHING
        ;
    "
    )
    .execute(&mut *transaction)
    .await?;

    // Diets
    query!(
        "
        INSERT INTO diets (label,name) VALUES
            ('vegetarian', 'Végétarien')
            ,('vegan', 'Vegan')
        ON CONFLICT DO NOTHING
        ;
    "
    )
    .execute(&mut *transaction)
    .await?;

    // Units
    query!(
        "
        INSERT INTO units (full_name,short_name) VALUES
            ('Grammes', 'g')
            ,('Kilogrammes', 'kg')
            ,('Litres', 'L')
            ,('Centilitres', 'cL')
            ,('Millilitres', 'mL')
            ,('Cuillères à soupe', 'cas')
            ,('Cuillères à café', 'cac')
            ,('Poignées', 'poignées')
            ,('Pincées', 'pincées')
            ,('Boules', 'boules')
            ,('Pots', 'pots')
        ON CONFLICT DO NOTHING
        ;
    "
    )
    .execute(&mut *transaction)
    .await?;

    query!(
        "
        INSERT INTO categories (name) VALUES
            ('Petit dej')
            ,('Repas')
            ,('Gouter')
            ,('Dessert')
            ,('Pâte')
        ON CONFLICT DO NOTHING
        ;
    "
    )
    .execute(&mut *transaction)
    .await?;

    query!(
        "
        INSERT INTO tags (name) VALUES
            ('Rapide')
            ,('Réconfortant')
            ,('Grosse faim')
        ON CONFLICT DO NOTHING
        ;
    "
    )
    .execute(&mut *transaction)
    .await?;

    transaction.commit().await?;
    Ok(())
}
