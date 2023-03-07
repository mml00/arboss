use rbatis::rbatis::Rbatis;
use rbdc_pg::driver::PgDriver;
use crate::config::CDatabase;


pub async fn get_connection(config: CDatabase) -> Result<Rbatis, Box<dyn std::error::Error>>{
    let rb = Rbatis::new();
    rb.init(
        PgDriver {},
        &format!("postgres://{}:{}@{}:{}/{}", config.user, config.password, config.url, config.port, config.database)
    ).unwrap();
    Ok(rb)
}
