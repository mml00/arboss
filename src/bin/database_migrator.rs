use screener::{
    config::get_config,
    orm::{
        connection::get_connection,
        migrator::migrate,
    },
};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = get_config();
    let mut db = get_connection(config.database.clone()).await?;

    migrate(&db).await?;
    println!("Migrations executed successfully");

    // test(&mut db).await;

    Ok(())
}


use rbatis::Rbatis;
use screener::orm::model::PairModel;
use rbdc::datetime::FastDateTime;

async fn test(db: &mut Rbatis) {
    PairModel::insert(db, &PairModel {
        id: None,
        from_token_id: 7081,
        to_token_id: 7000,
        providers: "oneinch".into(),
        pool_address: None,
        fee_percent: None,
        last_updated_at: FastDateTime::now(),
        is_active: true,
    }).await;

    println!("{:?}", PairModel::select_all(db).await.unwrap());
}
