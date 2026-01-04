use crate::common::{ Outcome, SLUG_URL, SPORT_URL };
use crate::bot_error::TokenError;
use chrono::Datelike;
use polyfill_rs::math::spread_pct;
use polyfill_rs::{ ClobClient, PolyfillClient };
use std::{ collections::HashSet, sync::Arc };
use std::collections::HashMap;
use serde_json::Value;
use crate::common::{ EVENT_URL, Result };
use crate::common::CRYPTO_PATTERNS;
use polyfill_rs::PolyfillError;
use crate::context::BotContext;
use anyhow::{ self, Error, Ok, anyhow };

#[derive(Debug, Clone, Hash)]
pub struct Token {
    market_name: String,
    token_id: u64,
    outcome: Outcome,
}

impl Token {
    fn new(name: String, id: u64, o: Outcome) -> Self {
        Self {
            market_name: name,
            token_id: id,
            outcome: o,
        }
    }
}

pub struct TokenFetcher {
    context: BotContext,
    tokens: Vec<Token>,
}

trait TokenApi {
    async fn get_events_by_params(&self, params: HashMap<String, String>) -> Result<Value>;
    async fn get_specified_tag_ids(
        &self,
        filtered_list: Option<HashSet<String>>
    ) -> Result<Vec<String>>;

    async fn get_market_by_slug(&self, event_slug: String) -> Result<Vec<String>>;
}

impl TokenApi for ClobClient {
    async fn get_events_by_params(&self, params: HashMap<String, String>) -> Result<Value> {
        let response = self.http_client
            .get(EVENT_URL)
            .json(&params)
            .send().await
            .map_err(|e| PolyfillError::network(format!("Request failed: {}", e), e))?;

        let ret = response.json::<Value>().await.map_err(|e| anyhow::anyhow!("{}", e));
        ret
    }

    async fn get_specified_tag_ids(
        &self,
        filtered_list: Option<HashSet<String>>
    ) -> Result<Vec<String>> {
        let mut tags_set: HashSet<String> = HashSet::new();
        let mut ret: Vec<String> = Vec::new();

        // 获取引用
        let filtered_list_ref = filtered_list.as_ref();

        let sports_json: Value = self.http_client
            .get(SPORT_URL)
            .send().await
            .map_err(|e| PolyfillError::network(format!("Request failed: {}", e), e))?
            .json().await?;

        sports_json
            .as_array()
            .into_iter()
            .flatten()
            .filter(|entry| {
                entry
                    .get("sport")
                    .and_then(|v| v.as_str())
                    .map_or(false, |s| filtered_list_ref.map_or(true, |set| set.contains(s)))
            })
            .filter_map(|entry| entry.get("tags")?.as_str())
            .flat_map(|s| s.split(','))
            .map(|s| s.trim())
            .filter(|s| !s.is_empty() && s.chars().all(|c| c.is_ascii_digit()))
            .for_each(|s| {
                let tag = s.to_string();
                if tags_set.insert(tag.clone()) {
                    ret.push(tag);
                }
            });

        Ok(ret)
    }

    async fn get_market_by_slug(&self, event_slug: String) -> Result<Vec<String>> {
        let slug_url = format!("{}/{}", SLUG_URL, event_slug);
        let resp_json: Value = self.http_client
            .get(slug_url)
            .send().await
            .map_err(|e| PolyfillError::network(format!("Request failed: {}", e), e))?
            .json().await?;

        let markets = resp_json
            .as_object()
            .ok_or_else(|| anyhow!("expect markets meta data but got nothing"))?;
        let market_ids: Vec<String> = markets
            .get("markets")
            .and_then(|m: &Value| m.as_array())
            .into_iter()
            .flatten()
            .filter_map(|m: &Value|
                m
                    .get("id")
                    .and_then(|id| id.as_str())
                    .filter(|s| !s.is_empty())
            )
            .collect();
        Ok(market_ids)
    }
}

impl TokenFetcher {
    pub fn new(ctx: BotContext) -> Self {
        Self {
            context: ctx,
            tokens: Vec::with_capacity(200),
        }
    }

    async fn get_crypto_markets(&self, slugs: Vec<String>) -> Result<Vec<String>> {
        let slugs = TokenFetcher::generate_crypto_slugs()?;

        for slug in slugs {
            tokio::spawn()
        }

    }

    async fn get_market_by_slug(&self, event_slug: String) -> Result<Vec<String>> {
        self.context.inner.client.get_market_by_slug(event_slug).await
    }

    fn generate_crypto_slugs() -> Result<Vec<String>> {
        let mut slugs: Vec<String> = Vec::new();
        let base_now = chrono::Local::now();

        for i in 0..1 {
            let future_date = base_now + chrono::Duration::days(i);
            let f_month = future_date.format("%B").to_string().to_lowercase();
            let f_day = future_date.day();
            let f_suffix = format!("-on-{}-{}", f_month, f_day);
            for p in CRYPTO_PATTERNS {
                slugs.push(format!("{}{}", p, f_suffix));
            }
        }
        println!("{:?}", slugs);
        Ok(slugs)
    }

    async fn get_specified_tag_ids(
        &mut self,
        filtered_list: Option<HashSet<String>>
    ) -> Result<Vec<String>> {
        let res = self.context.inner.client.get_specified_tag_ids(filtered_list).await;
        return res;
    }

    pub async fn get_live_sports_tokens(&mut self) -> Result<Vec<String>> {
        let params = HashMap::from([
            ("closed".to_string(), "false".to_string()),
            ("active".to_string(), "true".to_string()),
        ]);

        let val = self.context.inner.client.get_events_by_params(params).await?;
        let events: Vec<String> = serde_json
            ::from_value(val)
            .map_err(|e| anyhow::anyhow!("here should be string of vec [{}]", e))?;
        println!("{:?}", events);
        Ok(events)
    }

    async fn get_crypto_tokens(&mut self) {}
}
