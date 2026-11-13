use std::collections::BTreeMap;

use uuid::Uuid;

use crate::model::{Order, OrderProduct, Product};

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Cart(BTreeMap<Uuid, i32>);

impl Cart {
    pub fn from_order(order: Option<&Order>) -> Self {
        let items = order
            .into_iter()
            .flat_map(|order| order.products.iter())
            .filter(|product| product.quantity > 0)
            .map(|product| (product.id, product.quantity))
            .collect();
        Self(items)
    }

    pub fn add(&mut self, id: Uuid, quantity: i32, stock: i32) {
        let current = self.0.get(&id).copied().unwrap_or(0);
        self.set(id, current.saturating_add(quantity), stock);
    }

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
            total: format_euro(0),
        }
    }

    pub fn build(products: &[Product], cart: &Cart) -> Self {
        let mut lines = Vec::new();
        let mut item_count = 0;
        let mut total = 0_i64;

        for (id, quantity) in &cart.0 {
            let Some(product) = products.iter().find(|product| product.id == *id) else {
                continue;
            };
            let line_total = i64::from(product.price) * i64::from(*quantity);
            total += line_total;
            item_count += quantity;
            lines.push(CartLine {
                id: *id,
                name: product.name.clone(),
                packing: product.packing.clone(),
                unit_price: format_euro(i64::from(product.price)),
                quantity: *quantity,
                stock: product.stock,
                line_total: format_euro(line_total),
            });
        }

        Self {
            lines,
            item_count,
            total: format_euro(total),
        }
    }
}

pub fn format_euro(amount: i64) -> String {
    format!("€{amount}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn product(id: Uuid, price: i32, stock: i32) -> Product {
        Product {
            id,
            name: "Beans".into(),
            description: String::new(),
            packing: "1kg".into(),
            price,
            stock,
        }
    }

    #[test]
    fn formats_euro() {
        assert_eq!(format_euro(10), "€10");
        assert_eq!(format_euro(0), "€0");
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
    fn summary_totals() {
        let a = Uuid::from_u128(4);
        let b = Uuid::from_u128(5);
        let products = vec![product(a, 10, 5), product(b, 8, 5)];
        let mut cart = Cart::default();
        cart.add(a, 2, 5);
        cart.add(b, 1, 5);
        let summary = CartSummary::build(&products, &cart);
        assert_eq!(summary.item_count, 3);
        assert_eq!(summary.total, "€28");
    }
}
