use super::BeeType;
use bevy::{prelude::*, utils::HashMap};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Eq, PartialEq, Hash, Component, Clone)]
pub enum CurrencyType {
    Honey,
    Wax,
    MagicWax,
}

impl CurrencyType {
    pub fn get_image_name(&self) -> &'static str {
        match self {
            CurrencyType::Honey => "images/Honey.png",
            CurrencyType::Wax => "images/Wax.png",
            CurrencyType::MagicWax => "images/Wax.png",
        }
    }
}

pub struct CurrencyValue {
    pub value: u64,
    pub inflow: u64,
    pub limit: u64,
}

impl Default for CurrencyValue {
    fn default() -> Self {
        Self {
            value: 0,
            inflow: 0,
            limit: 100,
        }
    }
}

#[derive(Resource)]
pub struct CurrencyStorage {
    pub storage: HashMap<CurrencyType, CurrencyValue>,
    pub last_update: u64,
}

impl Default for CurrencyStorage {
    fn default() -> Self {
        let mut storage = HashMap::new();
        for currency in CurrencyType::iter() {
            storage.insert(currency, CurrencyValue::default());
        }
        Self {
            storage,
            last_update: 0,
        }
    }
}

pub fn earn_currency(mut storage: ResMut<CurrencyStorage>, time: Res<Time>, bees: Query<&BeeType>) {
    if storage.last_update < time.elapsed().as_secs() {
        storage.last_update = time.elapsed().as_secs();
        let honey_inflow = (bees.iter().count() as u64)
            + (bees.iter().filter(|bee| **bee != BeeType::Baby).count() as u64) * 2;
        if let Some(honey) = storage.storage.get_mut(&CurrencyType::Honey) {
            honey.inflow = honey_inflow;
            honey.value = u64::min(honey.value + honey_inflow, honey.limit);
        }
        if let Some(wax) = storage.storage.get_mut(&CurrencyType::Wax) {
            wax.inflow = honey_inflow / 2;
            wax.value = u64::min(wax.value + wax.inflow, wax.limit);
        }
    }
}
