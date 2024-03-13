use rbatis::Rbatis;
use crate::{
    Swap,
};

pub async fn update_swaps<'a>(db: &'a mut Rbatis, swaps: &'a Vec<Swap<'a>>) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
