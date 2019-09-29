use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
    result::Error as DieselError
};

use panda_base::traits::*;
use crate::{models::*, schema};

pub fn insert_panda_from_traits(
    genesis_tx: &i64, 
    owner_tx: &i64, 
    owner_tx_idx: &i64, 
    panda_traits: &PandaTraits,
    secret_genes: &[u8; 12],
    conn: &PgConnection
) -> Result<i64, DieselError> {
    use self::schema::panda::dsl as panda_dsl;

    // Pull panda attributes from traits
    let pa = panda_traits.to_attributes();
    
    // Get public genes
    let public_genes = panda_traits.to_byte_public_genes();

    // Extend public genes
    let genes_full_vec = &[&public_genes[..], &secret_genes[..]]
        .concat();
    let mut genes_full = [0; 48];
    genes_full.copy_from_slice(genes_full_vec);

    // Create panda row
    let new_panda = NewPanda {
        genesis_tx,
        owner_tx,
        owner_tx_idx,
        physique: &pa.physique,
        pattern: &pa.pattern,
        eye_color: &pa.eye_color,
        eye_shape: &pa.eye_shape,
        base_color: &pa.base_color,
        highlight_color: &pa.highlight_color,
        accent_color: &pa.accent_color,
        wild_element: &pa.wild_element,
        mouth: &pa.mouth,
        genes: &genes_full[..]
    };

    // Insert record
    diesel::insert_into(panda_dsl::panda)
        .values(&new_panda)
        .returning(panda_dsl::id)
        .get_results(conn).map(|res_vec| res_vec[0])  
}

pub fn insert_panda_from_genes(
    genesis_tx: &i64, 
    owner_tx: &i64, 
    owner_tx_idx: &i64, 
    genes: &[u8; 48],
    conn: &PgConnection
) -> Result<i64, DieselError> {
    use self::schema::panda::dsl as panda_dsl;

    // Create attributes
    let pa = PandaAttributes::from_genes(genes).unwrap(); // TODO: ? error here

    let new_panda = NewPanda {
        genesis_tx,
        owner_tx,
        owner_tx_idx,
        physique: &pa.physique,
        pattern: &pa.pattern,
        eye_color: &pa.eye_color,
        eye_shape: &pa.eye_shape,
        base_color: &pa.base_color,
        highlight_color: &pa.highlight_color,
        accent_color: &pa.accent_color,
        wild_element: &pa.wild_element,
        mouth: &pa.mouth,
        genes: &genes[..]
    };

    // Insert record
    diesel::insert_into(panda_dsl::panda)
        .values(&new_panda)
        .returning(panda_dsl::id)
        .get_results(conn).map(|res_vec| res_vec[0])  
}