use crate::utils;

use super::BeeType;
use bevy::{prelude::*, utils::HashMap};
use rand::{thread_rng, Rng};
use strum::IntoEnumIterator;

#[derive(Eq, PartialEq, Hash, Component, Clone)]
pub struct CurrencyType(pub usize);

pub const CURRENCY_HONEY: CurrencyType = CurrencyType(0);
pub const CURRENCY_WAX: CurrencyType = CurrencyType(1);
pub const CURRENCY_MAGIC_WAX: CurrencyType = CurrencyType(2);

pub const CURRENCY_NUM: usize = 3;

impl CurrencyType {
    pub fn get_image_name(&self) -> &'static str {
        match *self {
            CURRENCY_HONEY => "images/Honey.png",
            CURRENCY_WAX => "images/Wax.png",
            CURRENCY_MAGIC_WAX => "images/MagicWax.png",
            _ => "images/W.png",
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

pub type CurrencyValues = [u64; CURRENCY_NUM];

#[derive(Resource)]
pub struct CurrencyStorage {
    pub stored: CurrencyValues,
    pub max_stored: CurrencyValues,
    pub estimated_inflow: CurrencyValues,
}

impl CurrencyStorage {
    pub fn check_can_spend(&self, price: &CurrencyValues) -> bool {
        self.stored
            .iter()
            .enumerate()
            .all(|(i, stored)| *stored >= price[i])
    }

    pub fn spend(&mut self, price: &CurrencyValues) {
        self.stored
            .iter_mut()
            .enumerate()
            .for_each(|(i, stored)| *stored -= price[i])
    }

    pub fn gain(&mut self, price: &CurrencyValues) {
        self.stored
            .iter_mut()
            .enumerate()
            .for_each(|(i, stored)| *stored += price[i])
    }
}

impl Default for CurrencyStorage {
    fn default() -> Self {
        Self {
            stored: if utils::is_local_build() {
                [1000, 1000, 1000]
            } else {
                [0; CURRENCY_NUM]
            },
            max_stored: [5000, 1000000, 1000000],
            estimated_inflow: [0; CURRENCY_NUM],
        }
    }
}

#[derive(Component, Default)]
pub struct CurrencyGainPerMinute {
    pub gain: CurrencyValues,
    pub gained_this_minute: CurrencyValues,
    pub time_since_minute_start: f32,
}

pub fn gain_system(
    mut currency: ResMut<CurrencyStorage>,
    mut gainers: Query<&mut CurrencyGainPerMinute>,
    time: Res<Time>,
) {
    currency.estimated_inflow = [0; CURRENCY_NUM];
    for mut gainer in gainers.iter_mut() {
        currency
            .estimated_inflow
            .iter_mut()
            .enumerate()
            .for_each(|(i, v)| *v += gainer.gain[i]);

        gainer.time_since_minute_start += time.delta_seconds();
        let t = gainer.time_since_minute_start as f64;

        let new_gained_this_minute = gainer.gain.map(|g| ((g as f64) * t / 60.0) as u64);

        for i in 0..CURRENCY_NUM {
            let new_gain = new_gained_this_minute[i].min(gainer.gain[i]);
            if new_gain > gainer.gained_this_minute[i] {
                let gain = new_gain - gainer.gained_this_minute[i];
                currency.stored[i] = (currency.stored[i] + gain).min(currency.max_stored[i]);
                gainer.gained_this_minute[i] = new_gain;
            }
        }

        if gainer.time_since_minute_start > 60.0 {
            gainer.time_since_minute_start -= 60.0;
            gainer.gained_this_minute = [0; CURRENCY_NUM];
        }
    }
}

impl From<BeeType> for CurrencyGainPerMinute {
    fn from(value: BeeType) -> Self {
        match value {
            BeeType::Baby => CurrencyGainPerMinute {
                ..Default::default()
            },
            BeeType::Regular => CurrencyGainPerMinute {
                gain: [2, 0, 0],
                ..Default::default()
            },
            BeeType::Worker(lvl) => CurrencyGainPerMinute {
                // todo: depends on lvl
                gain: [[8, 2, 0], [12, 4, 0], [20, 6, 0]][lvl as usize],
                ..Default::default()
            },
            BeeType::Defender(lvl) => CurrencyGainPerMinute {
                ..Default::default()
            },
            BeeType::Queen => CurrencyGainPerMinute {
                gain: [4, 1, 0],
                ..Default::default()
            },
        }
    }
}
