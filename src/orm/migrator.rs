use rbatis::Rbatis;


pub async fn migrate_token_table(db: &Rbatis) -> Result<(), Box<dyn std::error::Error>> {
    // TOKEN
    db.query_decode("CREATE TABLE IF NOT EXISTS token (id SERIAL PRIMARY KEY)", vec![]).await?;
    db.query_decode("ALTER TABLE token
            ADD COLUMN IF NOT EXISTS ticker VARCHAR(100),
            ADD COLUMN IF NOT EXISTS chain VARCHAR(50),
            ADD COLUMN IF NOT EXISTS decimals smallint,
            ADD COLUMN IF NOT EXISTS name VARCHAR(100),
            ADD COLUMN IF NOT EXISTS address VARCHAR(150),
            ADD COLUMN IF NOT EXISTS market_provider_id VARCHAR(100),
            ADD COLUMN IF NOT EXISTS price_usd NUMERIC(13, 6),
            ADD COLUMN IF NOT EXISTS volume_24h_usd NUMERIC(14, 2),
            ADD COLUMN IF NOT EXISTS last_updated_at timestamp with time zone,
            ADD COLUMN IF NOT EXISTS is_active boolean
    ", vec![]).await?;
    db.query_decode("ALTER TABLE token
            ALTER COLUMN ticker SET DATA TYPE VARCHAR(100),
            ALTER COLUMN chain SET DATA TYPE VARCHAR(50),
            ALTER COLUMN decimals SET DATA TYPE smallint,
            ALTER COLUMN name SET DATA TYPE VARCHAR(100),
            ALTER COLUMN address SET DATA TYPE VARCHAR(150),
            ALTER COLUMN market_provider_id SET DATA TYPE VARCHAR(100),
            ALTER COLUMN price_usd SET DATA TYPE NUMERIC(13, 6),
            ALTER COLUMN volume_24h_usd SET DATA TYPE NUMERIC(14, 2),
            ALTER COLUMN last_updated_at SET DATA TYPE timestamp with time zone,
            ALTER COLUMN is_active SET DATA TYPE boolean
    ", vec![]).await?;
    db.query_decode("UPDATE token SET is_active = FALSE", vec![]).await?;
    // db.query_decode("UPDATE token SET last_updated_at = CURRENT_TIMESTAMP", vec![]).await?;
    db.query_decode("ALTER TABLE token
            ALTER COLUMN ticker SET NOT NULL,
            ALTER COLUMN chain SET NOT NULL,
            ALTER COLUMN decimals DROP NOT NULL,
            ALTER COLUMN name DROP NOT NULL,
            ALTER COLUMN address DROP NOT NULL,
            ALTER COLUMN market_provider_id DROP NOT NULL,
            ALTER COLUMN price_usd DROP NOT NULL,
            ALTER COLUMN volume_24h_usd DROP NOT NULL,
            ALTER COLUMN last_updated_at SET NOT NULL,
            ALTER COLUMN is_active SET NOT NULL
    ", vec![]).await?;

    Ok(())
}

pub async fn migrate_pair_table(db: &Rbatis) -> Result<(), Box<dyn std::error::Error>> {
    db.query_decode("CREATE TABLE IF NOT EXISTS pair (id SERIAL PRIMARY KEY)", vec![]).await?;
    db.query_decode("ALTER TABLE pair
            ADD COLUMN IF NOT EXISTS from_token_id integer,
            ADD COLUMN IF NOT EXISTS to_token_id integer,
            ADD COLUMN IF NOT EXISTS providers VARCHAR(150),
            ADD COLUMN IF NOT EXISTS pool_address VARCHAR(150),
            ADD COLUMN IF NOT EXISTS fee_percent NUMERIC(5, 3),
            ADD COLUMN IF NOT EXISTS last_updated_at timestamp with time zone,
            ADD COLUMN IF NOT EXISTS is_active boolean
    ", vec![]).await?;
    db.query_decode("ALTER TABLE pair
            DROP CONSTRAINT IF EXISTS from_token_fk,
            DROP CONSTRAINT IF EXISTS to_token_fk
    ", vec![]).await?;
    db.query_decode("ALTER TABLE pair
            ADD CONSTRAINT from_token_fk FOREIGN KEY (from_token_id) REFERENCES token (id),
            ADD CONSTRAINT to_token_fk FOREIGN KEY (to_token_id) REFERENCES token (id)
    ", vec![]).await?;
    db.query_decode("ALTER TABLE pair
            ALTER COLUMN from_token_id SET DATA TYPE integer,
            ALTER COLUMN to_token_id SET DATA TYPE integer,
            ALTER COLUMN providers SET DATA TYPE VARCHAR(150),
            ALTER COLUMN pool_address SET DATA TYPE VARCHAR(150),
            ALTER COLUMN fee_percent SET DATA TYPE NUMERIC(5, 3),
            ALTER COLUMN last_updated_at SET DATA TYPE timestamp with time zone,
            ALTER COLUMN is_active SET DATA TYPE boolean
    ", vec![]).await?;
    db.query_decode("ALTER TABLE pair
            ALTER COLUMN from_token_id SET NOT NULL,
            ALTER COLUMN to_token_id SET NOT NULL,
            ALTER COLUMN providers DROP NOT NULL,
            ALTER COLUMN pool_address DROP NOT NULL,
            ALTER COLUMN fee_percent DROP NOT NULL,
            ALTER COLUMN last_updated_at SET NOT NULL,
            ALTER COLUMN is_active SET NOT NULL
    ", vec![]).await?;

    Ok(())
}

pub async fn migrate(db: &Rbatis) -> Result<(), Box<dyn std::error::Error>> {
    migrate_token_table(db).await?;
    migrate_pair_table(db).await?;

    Ok(())
}
