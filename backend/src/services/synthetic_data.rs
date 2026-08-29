use bigdecimal::BigDecimal;
use chrono::{Duration, Utc};
use num_traits::FromPrimitive;
use rand::Rng;
use rand_distr::{Distribution, Normal};
use sqlx::{PgPool, QueryBuilder};
use tracing::info;

use crate::models::errors::trade_error::TradeError;
use crate::models::stock_ticker::Ticker;

#[derive(Debug, Clone)]
pub struct SyntheticCompany {
    pub symbol: &'static str,
    pub name: &'static str,
    pub sector: &'static str,
    pub description: &'static str,
    pub base_price: f64,
    pub volatility: f64, // sigma (annualized)
    pub drift: f64,      // mu (annualized)
    pub total_shares: i64,
}

pub const SYNTHETIC_UNIVERSE: &[SyntheticCompany] = &[
    SyntheticCompany {
        symbol: "CYBR",
        name: "Cyberdyne Systems",
        sector: "Robotics & AI",
        description: "Pioneering neural net processors, automated defense solutions, and autonomous robotics.",
        base_price: 320.0,
        volatility: 0.38,
        drift: 0.12,
        total_shares: 50_000_000,
    },
    SyntheticCompany {
        symbol: "WAYN",
        name: "Wayne Enterprises",
        sector: "Defense & Tech Conglomerate",
        description: "Global diversified multinational in aerospace, advanced materials, and clean urban technology.",
        base_price: 540.0,
        volatility: 0.18,
        drift: 0.07,
        total_shares: 120_000_000,
    },
    SyntheticCompany {
        symbol: "ACME",
        name: "Acme Corporation",
        sector: "Consumer Goods & Manufacturing",
        description: "Ubiquitous manufacturer of specialized industrial tools, consumer products, and dynamic apparatuses.",
        base_price: 145.0,
        volatility: 0.24,
        drift: 0.06,
        total_shares: 80_000_000,
    },
    SyntheticCompany {
        symbol: "SOYL",
        name: "Soylent Corp",
        sector: "AgriTech & BioNutrition",
        description: "Global leader in high-efficiency sustainable food processing and agricultural biotechnology.",
        base_price: 68.0,
        volatility: 0.20,
        drift: 0.05,
        total_shares: 60_000_000,
    },
    SyntheticCompany {
        symbol: "HYPN",
        name: "Hyperion Energy",
        sector: "Renewable & Fusion Energy",
        description: "Next-generation fusion reactors, zero-emission grid storage, and industrial orbital solar arrays.",
        base_price: 215.0,
        volatility: 0.32,
        drift: 0.09,
        total_shares: 45_000_000,
    },
    SyntheticCompany {
        symbol: "TYRL",
        name: "Tyrell Genomics",
        sector: "Biotechnology & Genetics",
        description: "Advanced bio-synthetic engineering, cellular regeneration, and gene editing therapeutics.",
        base_price: 410.0,
        volatility: 0.35,
        drift: 0.11,
        total_shares: 35_000_000,
    },
    SyntheticCompany {
        symbol: "NUKA",
        name: "Nuka-Cola Beverages",
        sector: "Consumer Staples & Beverages",
        description: "Iconic worldwide beverage brand with vast international distribution channels and loyal consumer base.",
        base_price: 88.0,
        volatility: 0.14,
        drift: 0.05,
        total_shares: 150_000_000,
    },
    SyntheticCompany {
        symbol: "OCP",
        name: "Omni Consumer Products",
        sector: "Urban Development & Security",
        description: "Massive municipal infrastructure, smart city development, civic security, and tech conglomerate.",
        base_price: 175.0,
        volatility: 0.28,
        drift: 0.08,
        total_shares: 90_000_000,
    },
];

pub fn get_synthetic_universe() -> Vec<SyntheticCompany> {
    SYNTHETIC_UNIVERSE.to_vec()
}

pub fn get_company_by_symbol(symbol: &str) -> Option<&'static SyntheticCompany> {
    SYNTHETIC_UNIVERSE.iter().find(|c| c.symbol.eq_ignore_ascii_case(symbol))
}

pub fn get_universe_symbols() -> Vec<String> {
    SYNTHETIC_UNIVERSE.iter().map(|c| c.symbol.to_string()).collect()
}

/// Generates multi-year historical daily and hourly candles for a synthetic company
pub fn generate_historical_candles(company: &SyntheticCompany) -> Vec<Ticker> {
    let mut rng = rand::thread_rng();
    let mut candles: Vec<Ticker> = Vec::new();

    let now = Utc::now();
    let years_back = 5;
    let total_days = years_back * 365;

    let dt_daily = 1.0 / 252.0;
    let normal_daily = Normal::new(
        (company.drift - 0.5 * company.volatility.powi(2)) * dt_daily,
        company.volatility * dt_daily.sqrt(),
    )
    .unwrap();

    let mut current_price = company.base_price;
    let start_date = now - Duration::days(total_days as i64);

    // Generate daily candles up to yesterday
    for day_idx in 0..total_days {
        let date = start_date + Duration::days(day_idx as i64);
        let return_shock = normal_daily.sample(&mut rng);
        let next_price = (current_price * return_shock.exp()).max(1.0);

        let open = current_price * (1.0 + (rng.gen::<f64>() - 0.5) * 0.005);
        let close = next_price;
        let intraday_vol = (company.volatility / (252.0_f64).sqrt()) * (1.0 + rng.gen::<f64>());
        let high = open.max(close) * (1.0 + rng.gen::<f64>() * intraday_vol);
        let low = (open.min(close) * (1.0 - rng.gen::<f64>() * intraday_vol)).max(0.5);

        let base_vol = 500_000.0;
        let volume = (base_vol * (1.0 + (high - low) / close * 20.0) * (0.8 + rng.gen::<f64>() * 0.4)) as i64;

        candles.push(Ticker {
            ticker: company.symbol.to_string(),
            date,
            close: BigDecimal::from_f64(round_cents(close)).unwrap_or_else(|| BigDecimal::from(100)),
            volume: Some(volume),
            open: Some(BigDecimal::from_f64(round_cents(open)).unwrap_or_else(|| BigDecimal::from(100))),
            high: Some(BigDecimal::from_f64(round_cents(high)).unwrap_or_else(|| BigDecimal::from(100))),
            low: Some(BigDecimal::from_f64(round_cents(low)).unwrap_or_else(|| BigDecimal::from(100))),
        });

        current_price = next_price;
    }

    // Generate intraday 15-minute candles for today up to current time
    let today_start = now.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc();
    let hours_today = (now - today_start).num_hours().max(1) as u32;
    let intervals = (hours_today * 4).max(4); // 15-minute intervals

    let dt_intraday = dt_daily / (intervals as f64);
    let normal_intraday = Normal::new(
        (company.drift - 0.5 * company.volatility.powi(2)) * dt_intraday,
        company.volatility * dt_intraday.sqrt(),
    )
    .unwrap();

    for i in 0..intervals {
        let date = today_start + Duration::minutes((i * 15) as i64);
        if date > now {
            break;
        }
        let return_shock = normal_intraday.sample(&mut rng);
        let next_price = (current_price * return_shock.exp()).max(1.0);

        let open = current_price;
        let close = next_price;
        let high = open.max(close) * (1.0 + rng.gen::<f64>() * 0.003);
        let low = (open.min(close) * (1.0 - rng.gen::<f64>() * 0.003)).max(0.5);
        let volume = (50_000.0 * (0.8 + rng.gen::<f64>() * 0.4)) as i64;

        candles.push(Ticker {
            ticker: company.symbol.to_string(),
            date,
            close: BigDecimal::from_f64(round_cents(close)).unwrap_or_else(|| BigDecimal::from(100)),
            volume: Some(volume),
            open: Some(BigDecimal::from_f64(round_cents(open)).unwrap_or_else(|| BigDecimal::from(100))),
            high: Some(BigDecimal::from_f64(round_cents(high)).unwrap_or_else(|| BigDecimal::from(100))),
            low: Some(BigDecimal::from_f64(round_cents(low)).unwrap_or_else(|| BigDecimal::from(100))),
        });

        current_price = next_price;
    }

    candles
}

fn round_cents(val: f64) -> f64 {
    (val * 100.0).round() / 100.0
}

/// Automatically seeds synthetic stock data if not present in the database
pub async fn seed_synthetic_data_if_needed(pool: &PgPool) -> Result<(), TradeError> {
    info!("Checking synthetic market data in PostgreSQL database...");

    for company in SYNTHETIC_UNIVERSE {
        let count_row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM stock_prices WHERE ticker = $1")
            .bind(company.symbol)
            .fetch_one(pool)
            .await
            .map_err(TradeError::DatabaseError)?;

        if count_row.0 < 100 {
            info!(
                "Seeding synthetic historical price series for {} ({}) - {} total shares...",
                company.symbol, company.name, company.total_shares
            );

            let candles = generate_historical_candles(company);
            let chunks = candles.chunks(500);

            for chunk in chunks {
                let mut query_builder: QueryBuilder<sqlx::Postgres> = QueryBuilder::new(
                    "INSERT INTO stock_prices (ticker, date, close, volume, open, high, low) ",
                );

                query_builder.push_values(chunk, |mut b, candle| {
                    b.push_bind(&candle.ticker)
                        .push_bind(candle.date)
                        .push_bind(&candle.close)
                        .push_bind(candle.volume)
                        .push_bind(&candle.open)
                        .push_bind(&candle.high)
                        .push_bind(&candle.low);
                });

                query_builder.push(" ON CONFLICT (ticker, date) DO UPDATE SET close = EXCLUDED.close, volume = EXCLUDED.volume, open = EXCLUDED.open, high = EXCLUDED.high, low = EXCLUDED.low");

                let query = query_builder.build();
                query.execute(pool).await.map_err(TradeError::DatabaseError)?;
            }

            info!("Successfully seeded {} records for {}", candles.len(), company.symbol);
        } else {
            info!("Synthetic data for {} is already populated ({} records).", company.symbol, count_row.0);
        }
    }

    info!("Synthetic market universe is fully initialized and operational.");
    Ok(())
}
