use std::collections::BTreeMap;

use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use uuid::Uuid;

use crate::model::{OrderProduct, Product};

pub const CART_COOKIE: &str = "cart";

/// Shopping cart (product id -> quantity), persisted in a cookie as `id:qty|id:qty`.
#[derive(Default, Clone, Debug, PartialEq)]
pub struct Cart(BTreeMap<Uuid, i32>);

impl Cart {
    pub fn from_jar(jar: &CookieJar) -> Self {
        jar.get(CART_COOKIE)
            .map(|cookie| Self::parse(cookie.value()))
            .unwrap_or_default()
    }

    pub fn parse(value: &str) -> Self {
        let items = value
            .split('|')
            .filter_map(|entry| {
                let (id, quantity) = entry.split_once(':')?;
                let id = Uuid::parse_str(id).ok()?;
                let quantity: i32 = quantity.parse().ok()?;
                (quantity > 0).then_some((id, quantity))
            })
            .collect();
        Self(items)
    }

    pub fn encode(&self) -> String {
        self.0
            .iter()
            .map(|(id, quantity)| format!("{id}:{quantity}"))
            .collect::<Vec<_>>()
            .join("|")
    }

    pub fn to_cookie(&self) -> Cookie<'static> {
        Cookie::build((CART_COOKIE, self.encode()))
            .path("/")
            .http_only(true)
            .same_site(SameSite::Lax)
            .build()
    }

    /// Adds to the existing quantity, capped at `stock`.
    pub fn add(&mut self, id: Uuid, quantity: i32, stock: i32) {
        let current = self.0.get(&id).copied().unwrap_or(0);
        self.set(id, current.saturating_add(quantity), stock);
    }

    /// Sets the quantity, capped at `stock`; zero or less removes the product.
    pub fn set(&mut self, id: Uuid, quantity: i32, stock: i32) {
        let quantity = quantity.min(stock);
        if quantity > 0 {
            self.0.insert(id, quantity);
        } else {
            self.0.remove(&id);
        }
    }

    pub fn remove(&mut self, id: &Uuid) {
        self.0.remove(id);
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn item_count(&self) -> i32 {
        self.0.values().sum()
    }

    pub fn to_order_products(&self) -> Vec<OrderProduct> {
        self.0
            .iter()
            .map(|(id, quantity)| OrderProduct {
                id: *id,
                quantity: *quantity,
            })
            .collect()
    }
}

pub struct CartLine {
    pub id: Uuid,
    pub name: String,
    pub packing: String,
    pub unit_price: String,
    pub quantity: i32,
    pub stock: i32,
    pub line_total: String,
}

pub struct CartSummary {
    pub lines: Vec<CartLine>,
    pub item_count: i32,
    pub total: String,
}

impl CartSummary {
    pub fn empty() -> Self {
        Self {
            lines: Vec::new(),
            item_count: 0,
            total: format_money("€", 0),
        }
    }

    pub fn build(products: &[Product], cart: &Cart) -> Self {
        let mut lines = Vec::new();
        let mut item_count = 0;
        let mut total_cents = 0_i64;
        let mut currency = String::new();

        for (id, quantity) in &cart.0 {
            let Some(product) = products.iter().find(|product| product.id == *id) else {
                continue;
            };
            let (symbol, cents) = parse_price(&product.price).unwrap_or_default();
            if currency.is_empty() {
                currency = symbol.clone();
            }
            let line_cents = cents * i64::from(*quantity);
            total_cents += line_cents;
            item_count += quantity;
            lines.push(CartLine {
                id: *id,
                name: product.name.clone(),
                packing: product.packing.clone(),
                unit_price: product.price.clone(),
                quantity: *quantity,
                stock: product.stock,
                line_total: format_money(&symbol, line_cents),
            });
        }

        if currency.is_empty() {
            currency = "€".to_string();
        }
        Self {
            lines,
            item_count,
            total: format_money(&currency, total_cents),
        }
    }
}

/// Parses prices like "€10" or "$9.50" into (currency symbol, cents).
pub fn parse_price(price: &str) -> Option<(String, i64)> {
    let symbol: String = price
        .chars()
        .filter(|c| !c.is_ascii_digit() && !matches!(c, '.' | ',') && !c.is_whitespace())
        .collect();
    let number: String = price
        .chars()
        .filter(|c| c.is_ascii_digit() || matches!(c, '.' | ','))
        .collect::<String>()
        .replace(',', ".");
    let (whole, fraction) = number.split_once('.').unwrap_or((number.as_str(), ""));
    let whole: i64 = whole.parse().ok()?;
    let fraction: String = fraction.chars().take(2).collect();
    let cents = match fraction.len() {
        0 => 0,
        1 => fraction.parse::<i64>().ok()? * 10,
        _ => fraction.parse::<i64>().ok()?,
    };
    Some((symbol, whole * 100 + cents))
}

pub fn format_money(symbol: &str, cents: i64) -> String {
    format!("{symbol}{}.{:02}", cents / 100, cents % 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn product(id: Uuid, price: &str, stock: i32) -> Product {
        Product {
            id,
            name: "Beans".into(),
            description: String::new(),
            packing: "1kg".into(),
            price: price.into(),
            stock,
        }
    }

    #[test]
    fn parses_prices() {
        assert_eq!(parse_price("€10"), Some(("€".into(), 1000)));
        assert_eq!(parse_price("$9.5"), Some(("$".into(), 950)));
        assert_eq!(parse_price("€8,25"), Some(("€".into(), 825)));
        assert_eq!(parse_price("free"), None);
    }

    #[test]
    fn formats_money() {
        assert_eq!(format_money("€", 1005), "€10.05");
        assert_eq!(format_money("€", 0), "€0.00");
    }

    #[test]
    fn cart_add_set_remove_and_clamp() {
        let id = Uuid::from_u128(1);
        let mut cart = Cart::default();
        cart.add(id, 2, 5);
        cart.add(id, 10, 5);
        assert_eq!(cart.item_count(), 5);
        cart.set(id, 3, 5);
        assert_eq!(cart.item_count(), 3);
        cart.set(id, 0, 5);
        assert!(cart.is_empty());
        cart.add(id, 1, 0);
        assert!(cart.is_empty());
    }

    #[test]
    fn cart_roundtrip_ignores_garbage() {
        let id = Uuid::from_u128(2);
        let mut cart = Cart::default();
        cart.add(id, 2, 5);
        let encoded = format!("{}|garbage|{}:-1", cart.encode(), Uuid::from_u128(3));
        assert_eq!(Cart::parse(&encoded), cart);
    }

    #[test]
    fn summary_totals() {
        let a = Uuid::from_u128(4);
        let b = Uuid::from_u128(5);
        let products = vec![product(a, "€10", 5), product(b, "€8", 5)];
        let mut cart = Cart::default();
        cart.add(a, 2, 5);
        cart.add(b, 1, 5);
        let summary = CartSummary::build(&products, &cart);
        assert_eq!(summary.item_count, 3);
        assert_eq!(summary.total, "€28.00");
    }
}
