use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use std::io::Cursor;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut, RangeInclusive};

use flate2::read::ZlibDecoder;
use itertools::Itertools;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use smartstring::{LazyCompact, SmartString};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, EnumIter)]
pub enum Pickup {
    RedHeart,
    SoulHeart,
    BlackHeart,
    EternalHeart,
    GoldHeart,
    BoneHeart,
    RottenHeart,
    Penny,
    Nickel,
    Dime,
    LuckyPenny,
    Key,
    GoldenKey,
    ChargedKey,
    Bomb,
    GoldenBomb,
    GigaBomb,
    MicroBattery,
    LilBattery,
    MegaBattery,
    Card,
    Pill,
    Rune,
    DiceShard,
    CrackedKey,
}

impl Pickup {
    fn weight(self) -> u32 {
        match self {
            Pickup::RedHeart => 1,
            Pickup::SoulHeart => 4,
            Pickup::BlackHeart => 5,
            Pickup::EternalHeart => 5,
            Pickup::GoldHeart => 5,
            Pickup::BoneHeart => 5,
            Pickup::RottenHeart => 1,
            Pickup::Penny => 1,
            Pickup::Nickel => 3,
            Pickup::Dime => 5,
            Pickup::LuckyPenny => 8,
            Pickup::Key => 2,
            Pickup::GoldenKey => 5,
            Pickup::ChargedKey => 5,
            Pickup::Bomb => 2,
            Pickup::GoldenBomb => 6,
            Pickup::GigaBomb => 10,
            Pickup::MicroBattery => 2,
            Pickup::LilBattery => 4,
            Pickup::MegaBattery => 8,
            Pickup::Card => 2,
            Pickup::Pill => 2,
            Pickup::Rune => 4,
            Pickup::DiceShard => 4,
            Pickup::CrackedKey => 2,
        }
    }

    fn shifts(self) -> (u32, u32, u32) {
        match self {
            Pickup::RedHeart => (0x00000001, 0x00000005, 0x00000013),
            Pickup::SoulHeart => (0x00000001, 0x00000009, 0x0000001D),
            Pickup::BlackHeart => (0x00000001, 0x0000000B, 0x00000006),
            Pickup::EternalHeart => (0x00000001, 0x0000000B, 0x00000010),
            Pickup::GoldHeart => (0x00000001, 0x00000013, 0x00000003),
            Pickup::BoneHeart => (0x00000001, 0x00000015, 0x00000014),
            Pickup::RottenHeart => (0x00000001, 0x0000001B, 0x0000001B),
            Pickup::Penny => (0x00000002, 0x00000005, 0x0000000F),
            Pickup::Nickel => (0x00000002, 0x00000005, 0x00000015),
            Pickup::Dime => (0x00000002, 0x00000007, 0x00000007),
            Pickup::LuckyPenny => (0x00000002, 0x00000007, 0x00000009),
            Pickup::Key => (0x00000002, 0x00000007, 0x00000019),
            Pickup::GoldenKey => (0x00000002, 0x00000009, 0x0000000F),
            Pickup::ChargedKey => (0x00000002, 0x0000000F, 0x00000011),
            Pickup::Bomb => (0x00000002, 0x0000000F, 0x00000019),
            Pickup::GoldenBomb => (0x00000002, 0x00000015, 0x00000009),
            Pickup::GigaBomb => (0x00000003, 0x00000001, 0x0000000E),
            Pickup::MicroBattery => (0x00000003, 0x00000003, 0x0000001A),
            Pickup::LilBattery => (0x00000003, 0x00000003, 0x0000001C),
            Pickup::MegaBattery => (0x00000003, 0x00000003, 0x0000001D),
            Pickup::Card => (0x00000003, 0x00000005, 0x00000014),
            Pickup::Pill => (0x00000003, 0x00000005, 0x00000016),
            Pickup::Rune => (0x00000003, 0x00000005, 0x00000019),
            Pickup::DiceShard => (0x00000003, 0x00000007, 0x0000001D),
            Pickup::CrackedKey => (0x00000003, 0x0000000D, 0x00000007),
        }
    }
}

impl Slotable for Pickup {
    fn largest() -> Self {
        Pickup::CrackedKey
    }
}

impl From<usize> for Pickup {
    fn from(pickup: usize) -> Self {
        use Pickup::*;
        match pickup {
            0 => RedHeart,
            1 => SoulHeart,
            2 => BlackHeart,
            3 => EternalHeart,
            4 => GoldHeart,
            5 => BoneHeart,
            6 => RottenHeart,
            7 => Penny,
            8 => Nickel,
            9 => Dime,
            10 => LuckyPenny,
            11 => Key,
            12 => GoldenKey,
            13 => ChargedKey,
            14 => Bomb,
            15 => GoldenBomb,
            16 => GigaBomb,
            17 => MicroBattery,
            18 => LilBattery,
            19 => MegaBattery,
            20 => Card,
            21 => Pill,
            22 => Rune,
            23 => DiceShard,
            24 => CrackedKey,
            _ => panic!(),
        }
    }
}

impl From<Pickup> for usize {
    fn from(pickup: Pickup) -> Self {
        use Pickup::*;
        match pickup {
            RedHeart => 0,
            SoulHeart => 1,
            BlackHeart => 2,
            EternalHeart => 3,
            GoldHeart => 4,
            BoneHeart => 5,
            RottenHeart => 6,
            Penny => 7,
            Nickel => 8,
            Dime => 9,
            LuckyPenny => 10,
            Key => 11,
            GoldenKey => 12,
            ChargedKey => 13,
            Bomb => 14,
            GoldenBomb => 15,
            GigaBomb => 16,
            MicroBattery => 17,
            LilBattery => 18,
            MegaBattery => 19,
            Card => 20,
            Pill => 21,
            Rune => 22,
            DiceShard => 23,
            CrackedKey => 24,
        }
    }
}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_ITEM_ID: &'static str = r#"
type ItemId = number;
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Record<Pickup, number>")]
    pub type HeldPickups;

    pub type ItemId;

    #[wasm_bindgen(typescript_type = "Array<ItemId>")]
    pub type ItemIds;
}

const QUALITY_BOUNDS_LIST: [(u32, RangeInclusive<u32>); 8] = [
    (34, 4..=4),
    (30, 3..=4),
    (26, 2..=4),
    (22, 1..=4),
    (18, 1..=3),
    (14, 1..=2),
    (8, 0..=2),
    (0, 0..=1),
];

struct Rng {
    seed: u32,
    shifts: (u32, u32, u32),
}

impl Default for Rng {
    fn default() -> Self {
        Rng {
            seed: 0x77777770,
            shifts: (0, 0, 0),
        }
    }
}

impl Rng {
    fn next(&mut self) -> u32 {
        let mut num = self.seed;
        num ^= num >> self.shifts.0;
        num ^= num << self.shifts.1;
        num ^= num >> self.shifts.2;
        self.seed = num;
        self.seed
    }

    fn next_float(&mut self) -> f32 {
        const MULT_BYTES: [u8; 4] = [0x2f, 0x7f, 0xff, 0xfe];
        let mult: f32 = f32::from_be_bytes(MULT_BYTES);
        self.next() as f32 * mult
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct InternalItemId(u16);

impl Serialize for InternalItemId {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        u16::serialize(&self.0, serializer)
    }
}

impl<'de> Deserialize<'de> for InternalItemId {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(InternalItemId(u16::deserialize(deserializer)?))
    }
}

impl From<u16> for InternalItemId {
    fn from(id: u16) -> Self {
        InternalItemId(id)
    }
}

impl From<InternalItemId> for u16 {
    fn from(id: InternalItemId) -> Self {
        id.0
    }
}

impl From<usize> for InternalItemId {
    fn from(id: usize) -> Self {
        InternalItemId(id as u16)
    }
}

impl From<InternalItemId> for usize {
    fn from(id: InternalItemId) -> Self {
        id.0 as usize
    }
}

impl Slotable for InternalItemId {
    fn largest() -> Self {
        InternalItemId(730)
    }
}

#[derive(Debug, Deserialize)]
struct ItemPools {
    #[serde(rename = "Pool")]
    pools: Vec<Pool>,
}

#[derive(Debug, Deserialize)]
struct Pool {
    #[serde(rename = "Name")]
    name: SmartString<LazyCompact>,

    #[serde(rename = "Item")]
    items: Vec<Item>,
}

#[derive(Debug, Deserialize)]
struct Item {
    #[serde(rename = "Id")]
    id: InternalItemId,
    #[serde(rename = "Weight")]
    weight: f32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, EnumIter)]
enum ItemPool {
    Treasure,
    Shop,
    Boss,
    Devil,
    Angel,
    Secret,
    Library,
    ShellGame,
    GoldenChest,
    RedChest,
    Beggar,
    DemonBeggar,
    Curse,
    KeyMaster,
    BatteryBum,
    MomsChest,
    GreedTreasure,
    GreedBoss,
    GreedShop,
    GreedCurse,
    GreedDevil,
    GreedAngel,
    GreedSecret,
    CraneGame,
    Unused24,
    BombBum,
    Planetarium,
    OldChest,
    BabyShop,
    WoodenChest,
    RottenBeggar,
}

impl ItemPool {
    fn name(self) -> &'static str {
        match self {
            ItemPool::Treasure => "treasure",
            ItemPool::Shop => "shop",
            ItemPool::Boss => "boss",
            ItemPool::Devil => "devil",
            ItemPool::Angel => "angel",
            ItemPool::Secret => "secret",
            ItemPool::Library => "library",
            ItemPool::ShellGame => "shellGame",
            ItemPool::GoldenChest => "goldenChest",
            ItemPool::RedChest => "redChest",
            ItemPool::Beggar => "beggar",
            ItemPool::DemonBeggar => "demonBeggar",
            ItemPool::Curse => "curse",
            ItemPool::KeyMaster => "keyMaster",
            ItemPool::BatteryBum => "batteryBum",
            ItemPool::MomsChest => "momsChest",
            ItemPool::GreedTreasure => "greedTreasure",
            ItemPool::GreedBoss => "greedBoss",
            ItemPool::GreedShop => "greedShop",
            ItemPool::GreedCurse => "greedCurse",
            ItemPool::GreedDevil => "greedDevil",
            ItemPool::GreedAngel => "greedAngel",
            ItemPool::GreedSecret => "greedSecret",
            ItemPool::CraneGame => "craneGame",
            ItemPool::Unused24 => "unused24",
            ItemPool::BombBum => "bombBum",
            ItemPool::Planetarium => "planetarium",
            ItemPool::OldChest => "oldChest",
            ItemPool::BabyShop => "babyShop",
            ItemPool::WoodenChest => "woodenChest",
            ItemPool::RottenBeggar => "rottenBeggar",
        }
    }
}

fn get_pool_item_weights() -> HashMap<ItemPool, HashMap<InternalItemId, f32>> {
    const ITEM_POOLS_DATA_ZLIB_DEFLATE: &[u8] = include_bytes!("itempools.xml.zz");
    let reader = ZlibDecoder::new(Cursor::new(ITEM_POOLS_DATA_ZLIB_DEFLATE));
    let pools: ItemPools = serde_xml_rs::from_reader(reader).unwrap();
    let item_pool_lookup: HashMap<&'static str, ItemPool> =
        ItemPool::iter().map(|pool| (pool.name(), pool)).collect();
    pools
        .pools
        .into_iter()
        .map(|pool| {
            (
                item_pool_lookup.get(pool.name.as_str()).copied().unwrap(),
                pool.items
                    .into_iter()
                    .map(|item| (item.id, item.weight))
                    .collect(),
            )
        })
        .collect()
}

#[derive(Debug, Deserialize)]
struct ItemsMetadata {
    #[serde(rename = "item")]
    items: Vec<ItemMetadata>,
}

#[derive(Debug, Deserialize)]
struct ItemMetadata {
    id: InternalItemId,
    quality: u32,
}

fn get_item_qualities() -> SlotMap<InternalItemId, u32> {
    const ITEM_POOLS_DATA_ZLIB_DEFLATE: &[u8] = include_bytes!("items_metadata.xml.zz");
    let reader = ZlibDecoder::new(Cursor::new(ITEM_POOLS_DATA_ZLIB_DEFLATE));
    let metadata: ItemsMetadata = serde_xml_rs::from_reader(reader).unwrap();
    let mut result = SlotMap::default();
    for item in metadata.items {
        result[item.id] = item.quality;
    }
    result
}

trait Slotable: Clone + From<usize> + Into<usize> {
    fn largest() -> Self;
}

struct SlotMap<S: Slotable, T> {
    data: Vec<T>,
    _p: PhantomData<S>,
}

impl<S: Slotable, T> SlotMap<S, T> {
    fn into_iter(self) -> impl Iterator<Item = (S, T)> {
        self.data
            .into_iter()
            .enumerate()
            .map(|(idx, t)| (S::from(idx), t))
    }

    fn iter(&self) -> impl Iterator<Item = (S, &T)> {
        self.data
            .iter()
            .enumerate()
            .map(|(idx, t)| (S::from(idx), t))
    }
}

impl<S: Slotable, T: Default + Clone> Default for SlotMap<S, T> {
    fn default() -> Self {
        SlotMap {
            data: vec![Default::default(); S::largest().into() + 1],
            _p: PhantomData::default(),
        }
    }
}

impl<S: Slotable, T> Index<S> for SlotMap<S, T> {
    type Output = T;

    fn index(&self, index: S) -> &Self::Output {
        &self.data[index.into()]
    }
}

impl<S: Slotable, T> IndexMut<S> for SlotMap<S, T> {
    fn index_mut(&mut self, index: S) -> &mut Self::Output {
        &mut self.data[index.into()]
    }
}

impl<S: Slotable, T: Clone> Clone for SlotMap<S, T> {
    fn clone(&self) -> Self {
        SlotMap {
            data: self.data.clone(),
            _p: Default::default(),
        }
    }
}

impl<S: Slotable, T: PartialEq> PartialEq for SlotMap<S, T> {
    fn eq(&self, other: &Self) -> bool {
        self.data.eq(&other.data)
    }
}

impl<S: Slotable, T: PartialEq + Eq> Eq for SlotMap<S, T> {}

impl<S: Slotable, T: Debug> Debug for SlotMap<S, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SlotMap{{data: {:?}}}", self.data)
    }
}

#[wasm_bindgen]
pub struct DeltaCrafter {
    crafter: BasicCrafter,
    methods: SlotMap<InternalItemId, HashSet<InternalPickups>>,
    held: SlotMap<Pickup, u8>,
}

#[wasm_bindgen]
impl DeltaCrafter {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pickups(&self) -> Result<HeldPickups, JsValue> {
        let obj = js_sys::Object::new();

        for (pickup, count) in self.held.iter() {
            js_sys::Reflect::set(
                &obj,
                &JsValue::from(wasm_bindgen::convert::IntoWasmAbi::into_abi(pickup)),
                &JsValue::from(*count),
            )?;
        }

        Ok(obj.unchecked_into())
    }

    pub fn items(&self) -> ItemIds {
        let items: Vec<InternalItemId> = self
            .methods
            .iter()
            .filter(|(_, method)| !method.is_empty())
            .map(|(id, _)| id)
            .collect();
        item_ids_to_js_item_ids(items.as_slice())
    }

    pub fn add_pickup(&mut self, pickup: Pickup) {
        self.held[pickup] = self.held[pickup].saturating_add(1);
        let held_now = self.held[pickup];

        if held_now >= 8 {
            if held_now == 8 {
                let pickups = InternalPickups::new([pickup; 8]);
                assert!(self.methods[self.crafter.craft(pickups)].insert(pickups));
            }
            return;
        }

        let other = 8 - held_now as usize;

        self.for_each_crafting_method(pickup, held_now as usize, other, |methods, method| {
            assert!(methods.insert(method));
        })
    }

    pub fn remove_pickup(&mut self, pickup: Pickup) {
        let held_before = self.held[pickup];
        if held_before == 0 {
            return;
        }

        self.held[pickup] = held_before.saturating_sub(1);
        if held_before >= 8 {
            if held_before == 8 {
                let pickups = InternalPickups::new([pickup; 8]);
                assert!(self.methods[self.crafter.craft(pickups)].remove(&pickups));
            }
            return;
        }

        let other = 8 - held_before as usize;

        self.for_each_crafting_method(pickup, held_before as usize, other, |methods, method| {
            assert!(methods.remove(&method));
        })
    }

    fn for_each_crafting_method<F: FnMut(&mut HashSet<InternalPickups>, InternalPickups)>(
        &mut self,
        pickup: Pickup,
        held_count: usize,
        other_count: usize,
        mut f: F,
    ) {
        for pickups in self
            .held
            .iter()
            .filter(move |(p, _)| *p != pickup)
            .flat_map(|(p, count)| std::iter::repeat(p).take(*count as usize))
            .combinations(other_count)
            .map(|comb| {
                let mut k = [pickup; 8];
                k[held_count..].copy_from_slice(comb.as_slice());
                InternalPickups::new(k)
            })
            .unique()
        {
            f(&mut self.methods[self.crafter.craft(pickups)], pickups);
        }
    }
}

impl Default for DeltaCrafter {
    fn default() -> Self {
        Self {
            crafter: Default::default(),
            methods: SlotMap::default(),
            held: Default::default(),
        }
    }
}

trait Crafter {
    fn craft(&self, pickups: InternalPickups) -> InternalItemId;
}

#[derive(Debug, Clone)]
pub struct BasicCrafter {
    pool_item_weights: HashMap<ItemPool, HashMap<InternalItemId, f32>>,
    item_qualities: SlotMap<InternalItemId, u32>,
}

impl Default for BasicCrafter {
    fn default() -> Self {
        set_panic_hook();
        BasicCrafter {
            pool_item_weights: get_pool_item_weights(),
            item_qualities: get_item_qualities(),
        }
    }
}

impl Crafter for BasicCrafter {
    fn craft(&self, pickups: InternalPickups) -> InternalItemId {
        let mut rng = Rng::default();
        let pickup_counts = pickups.pickups.iter().fold(BTreeMap::new(), |mut acc, p| {
            *acc.entry(*p).or_default() += 1;
            acc
        });
        let pickup_weight_total: u32 = pickups.pickups.iter().map(|pickup| pickup.weight()).sum();
        for pickup in pickups.pickups.iter() {
            rng.shifts = pickup.shifts();
            rng.next();
        }

        fn get_pickup_count(pickup_counts: &BTreeMap<Pickup, u32>, pickup: Pickup) -> u32 {
            pickup_counts.get(&pickup).copied().unwrap_or(0)
        }

        rng.shifts = (1, 21, 20);
        let mut pool_weights: Vec<(ItemPool, f32)> = vec![
            (ItemPool::Treasure, 1.),
            (ItemPool::Shop, 2.),
            (ItemPool::Boss, 2.),
            (
                ItemPool::Angel,
                get_pickup_count(&pickup_counts, Pickup::EternalHeart) as f32 * 10.,
            ),
            (
                ItemPool::Devil,
                get_pickup_count(&pickup_counts, Pickup::BlackHeart) as f32 * 10.,
            ),
            (
                ItemPool::Secret,
                get_pickup_count(&pickup_counts, Pickup::BoneHeart) as f32 * 5.,
            ),
            (
                ItemPool::GoldenChest,
                get_pickup_count(&pickup_counts, Pickup::GoldHeart) as f32 * 10.,
            ),
            (
                ItemPool::Curse,
                get_pickup_count(&pickup_counts, Pickup::RottenHeart) as f32 * 10.,
            ),
            (
                ItemPool::RedChest,
                get_pickup_count(&pickup_counts, Pickup::CrackedKey) as f32 * 10.,
            ),
        ];

        let mut combined = 0;
        for i in [Pickup::RedHeart, Pickup::Penny, Pickup::Key, Pickup::Bomb].iter() {
            combined += get_pickup_count(&pickup_counts, *i);
        }
        if combined == 0 {
            pool_weights.push((
                ItemPool::Planetarium,
                get_pickup_count(&pickup_counts, Pickup::Rune) as f32 * 10.,
            ));
        }

        let mut item_weights = SlotMap::<InternalItemId, f32>::default();
        let mut weight_total = 0_f32;

        for (pool, pool_weight) in pool_weights.iter() {
            if *pool_weight <= 0. {
                continue;
            }

            let mut quality_check_val = pickup_weight_total;
            match pool {
                ItemPool::Devil | ItemPool::Angel | ItemPool::Secret => quality_check_val -= 5,
                _ => (),
            }

            let mut quality_bounds = 0..=0;
            for (min, bounds) in QUALITY_BOUNDS_LIST.iter() {
                if quality_check_val > *min {
                    quality_bounds = bounds.clone();
                    break;
                }
            }
            let items_with_weights = self.pool_item_weights.get(pool).unwrap();
            for (item, weight) in items_with_weights.iter() {
                if quality_bounds.contains(&self.item_qualities[*item]) {
                    let final_weight = *pool_weight * weight;
                    item_weights[*item] += final_weight;
                    weight_total += final_weight;
                }
            }
        }

        let mut target = rng.next_float() * weight_total;
        for (item, weight) in item_weights.into_iter() {
            target -= weight;
            if target < 0. {
                return item;
            }
        }

        InternalItemId::from(25_u16)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct InternalPickups {
    pickups: [Pickup; 8],
}

impl InternalPickups {
    fn new(pickups: [Pickup; 8]) -> Self {
        InternalPickups {
            pickups: sort_pickups(pickups),
        }
    }
}

fn sort_pickups(mut pickups: [Pickup; 8]) -> [Pickup; 8] {
    for i in 0..pickups.len() {
        for j in i..pickups.len() {
            if pickups[j] < pickups[i] {
                pickups.swap(i, j);
            }
        }
    }
    pickups
}

fn item_ids_to_js_item_ids(item_ids: &[InternalItemId]) -> ItemIds {
    let array = js_sys::Array::new_with_length(item_ids.len() as u32);
    for (i, item_id) in item_ids.iter().copied().enumerate() {
        array.set(i as u32, JsValue::from(item_id.0));
    }
    array.unchecked_into()
}

#[cfg(test)]
mod tests {
    use once_cell::sync::Lazy;

    use Pickup::*;

    use super::*;

    static SIMPLE_CACHE: Lazy<BasicCrafter> = Lazy::new(BasicCrafter::default);

    #[test]
    fn craft_moms_knife() {
        assert_eq!(
            InternalItemId(114),
            SIMPLE_CACHE.craft(InternalPickups::new([
                SoulHeart, SoulHeart, SoulHeart, SoulHeart, SoulHeart, SoulHeart, SoulHeart,
                SoulHeart,
            ]))
        )
    }

    #[test]
    fn craft_sworn_protector() {
        assert_eq!(
            InternalItemId(363),
            SIMPLE_CACHE.craft(InternalPickups::new([
                SoulHeart,
                SoulHeart,
                SoulHeart,
                SoulHeart,
                SoulHeart,
                SoulHeart,
                EternalHeart,
                LuckyPenny,
            ]))
        )
    }

    #[test]
    fn craft_rotten_meat() {
        assert_eq!(
            InternalItemId(26),
            SIMPLE_CACHE.craft(InternalPickups::new([
                RedHeart, RedHeart, SoulHeart, Penny, Penny, Nickel, LuckyPenny, Key,
            ]))
        )
    }

    #[test]
    fn craft_luna() {
        assert_eq!(
            InternalItemId(589),
            SIMPLE_CACHE.craft(InternalPickups::new([
                SoulHeart, SoulHeart, Nickel, Card, Card, Rune, Rune, Rune,
            ]))
        )
    }

    #[test]
    fn delta_crafter() {
        let mut delta_crafter = DeltaCrafter::default();
        let mut methods = SlotMap::<InternalItemId, HashSet<InternalPickups>>::default();
        let mut held = SlotMap::<Pickup, u8>::default();
        assert_eq!(delta_crafter.held, held);
        assert_eq!(delta_crafter.methods, methods);

        delta_crafter.add_pickup(RedHeart);
        held[RedHeart] += 1;
        assert_eq!(delta_crafter.held, held);
        assert_eq!(delta_crafter.methods, methods);

        delta_crafter.add_pickup(RedHeart);
        held[RedHeart] += 1;
        assert_eq!(delta_crafter.held, held);
        assert_eq!(delta_crafter.methods, methods);

        delta_crafter.add_pickup(SoulHeart);
        held[SoulHeart] += 1;
        assert_eq!(delta_crafter.held, held);
        assert_eq!(delta_crafter.methods, methods);

        delta_crafter.add_pickup(Penny);
        held[Penny] += 1;
        assert_eq!(delta_crafter.held, held);
        assert_eq!(delta_crafter.methods, methods);

        delta_crafter.add_pickup(Penny);
        held[Penny] += 1;
        assert_eq!(delta_crafter.held, held);
        assert_eq!(delta_crafter.methods, methods);

        delta_crafter.add_pickup(Nickel);
        held[Nickel] += 1;
        assert_eq!(delta_crafter.held, held);
        assert_eq!(delta_crafter.methods, methods);

        delta_crafter.add_pickup(LuckyPenny);
        held[LuckyPenny] += 1;
        assert_eq!(delta_crafter.held, held);
        assert_eq!(delta_crafter.methods, methods);

        delta_crafter.add_pickup(Key);
        held[Key] += 1;
        methods[InternalItemId(26)].insert(InternalPickups::new([
            RedHeart, RedHeart, SoulHeart, Penny, Penny, Nickel, LuckyPenny, Key,
        ]));
        assert_eq!(delta_crafter.held, held);
        assert_eq!(delta_crafter.methods, methods);

        delta_crafter.remove_pickup(Penny);
        held[Penny] -= 1;
        methods[InternalItemId(26)].remove(&InternalPickups::new([
            RedHeart, RedHeart, SoulHeart, Penny, Penny, Nickel, LuckyPenny, Key,
        ]));
        assert_eq!(delta_crafter.held, held);
        assert_eq!(delta_crafter.methods, methods);

        delta_crafter.add_pickup(Penny);
        held[Penny] += 1;
        methods[InternalItemId(26)].insert(InternalPickups::new([
            RedHeart, RedHeart, SoulHeart, Penny, Penny, Nickel, LuckyPenny, Key,
        ]));
        assert_eq!(delta_crafter.held, held);
        assert_eq!(delta_crafter.methods, methods);

        delta_crafter.add_pickup(LilBattery);
        held[LilBattery] += 1;
        methods[InternalItemId(26)].insert(InternalPickups::new([
            RedHeart, RedHeart, SoulHeart, Penny, Penny, Nickel, LuckyPenny, Key,
        ]));
        methods[InternalItemId(368)].insert(InternalPickups::new([
            RedHeart, RedHeart, Penny, Penny, Nickel, LuckyPenny, Key, LilBattery,
        ]));
        methods[InternalItemId(456)].insert(InternalPickups::new([
            RedHeart, RedHeart, SoulHeart, Penny, Penny, Nickel, LuckyPenny, LilBattery,
        ]));
        methods[InternalItemId(514)].insert(InternalPickups::new([
            RedHeart, RedHeart, SoulHeart, Penny, Nickel, LuckyPenny, Key, LilBattery,
        ]));
        methods[InternalItemId(539)].insert(InternalPickups::new([
            RedHeart, RedHeart, SoulHeart, Penny, Penny, Nickel, Key, LilBattery,
        ]));
        methods[InternalItemId(539)].insert(InternalPickups::new([
            RedHeart, SoulHeart, Penny, Penny, Nickel, LuckyPenny, Key, LilBattery,
        ]));
        methods[InternalItemId(555)].insert(InternalPickups::new([
            RedHeart, RedHeart, SoulHeart, Penny, Penny, LuckyPenny, Key, LilBattery,
        ]));
        assert_eq!(delta_crafter.held, held);
        assert_eq!(delta_crafter.methods, methods);

        delta_crafter.remove_pickup(RedHeart);
        held[RedHeart] -= 1;
        methods[InternalItemId(26)].remove(&InternalPickups::new([
            RedHeart, RedHeart, SoulHeart, Penny, Penny, Nickel, LuckyPenny, Key,
        ]));
        methods[InternalItemId(368)].remove(&InternalPickups::new([
            RedHeart, RedHeart, Penny, Penny, Nickel, LuckyPenny, Key, LilBattery,
        ]));
        methods[InternalItemId(456)].remove(&InternalPickups::new([
            RedHeart, RedHeart, SoulHeart, Penny, Penny, Nickel, LuckyPenny, LilBattery,
        ]));
        methods[InternalItemId(514)].remove(&InternalPickups::new([
            RedHeart, RedHeart, SoulHeart, Penny, Nickel, LuckyPenny, Key, LilBattery,
        ]));
        methods[InternalItemId(539)].remove(&InternalPickups::new([
            RedHeart, RedHeart, SoulHeart, Penny, Penny, Nickel, Key, LilBattery,
        ]));
        methods[InternalItemId(555)].remove(&InternalPickups::new([
            RedHeart, RedHeart, SoulHeart, Penny, Penny, LuckyPenny, Key, LilBattery,
        ]));
        assert_eq!(delta_crafter.held, held);
        assert_eq!(delta_crafter.methods, methods);

        delta_crafter.add_pickup(RedHeart);
        held[RedHeart] += 1;
        methods[InternalItemId(26)].insert(InternalPickups::new([
            RedHeart, RedHeart, SoulHeart, Penny, Penny, Nickel, LuckyPenny, Key,
        ]));
        methods[InternalItemId(368)].insert(InternalPickups::new([
            RedHeart, RedHeart, Penny, Penny, Nickel, LuckyPenny, Key, LilBattery,
        ]));
        methods[InternalItemId(456)].insert(InternalPickups::new([
            RedHeart, RedHeart, SoulHeart, Penny, Penny, Nickel, LuckyPenny, LilBattery,
        ]));
        methods[InternalItemId(514)].insert(InternalPickups::new([
            RedHeart, RedHeart, SoulHeart, Penny, Nickel, LuckyPenny, Key, LilBattery,
        ]));
        methods[InternalItemId(539)].insert(InternalPickups::new([
            RedHeart, RedHeart, SoulHeart, Penny, Penny, Nickel, Key, LilBattery,
        ]));
        methods[InternalItemId(555)].insert(InternalPickups::new([
            RedHeart, RedHeart, SoulHeart, Penny, Penny, LuckyPenny, Key, LilBattery,
        ]));
        assert_eq!(delta_crafter.held, held);
        assert_eq!(delta_crafter.methods, methods);

        delta_crafter.remove_pickup(LuckyPenny);
        held[LuckyPenny] -= 1;
        methods[InternalItemId(26)].remove(&InternalPickups::new([
            RedHeart, RedHeart, SoulHeart, Penny, Penny, Nickel, LuckyPenny, Key,
        ]));
        methods[InternalItemId(368)].remove(&InternalPickups::new([
            RedHeart, RedHeart, Penny, Penny, Nickel, LuckyPenny, Key, LilBattery,
        ]));
        methods[InternalItemId(456)].remove(&InternalPickups::new([
            RedHeart, RedHeart, SoulHeart, Penny, Penny, Nickel, LuckyPenny, LilBattery,
        ]));
        methods[InternalItemId(514)].remove(&InternalPickups::new([
            RedHeart, RedHeart, SoulHeart, Penny, Nickel, LuckyPenny, Key, LilBattery,
        ]));
        methods[InternalItemId(555)].remove(&InternalPickups::new([
            RedHeart, RedHeart, SoulHeart, Penny, Penny, LuckyPenny, Key, LilBattery,
        ]));
        methods[InternalItemId(539)].remove(&InternalPickups::new([
            RedHeart, SoulHeart, Penny, Penny, Nickel, LuckyPenny, Key, LilBattery,
        ]));
        assert_eq!(delta_crafter.held, held);
        assert_eq!(delta_crafter.methods, methods);
    }

    #[test]
    fn delta_crafter_at_8() {
        let mut delta_crafter = DeltaCrafter::default();
        let mut methods = SlotMap::<InternalItemId, HashSet<InternalPickups>>::default();
        let mut held = SlotMap::<Pickup, u8>::default();
        assert_eq!(delta_crafter.held, held);
        assert_eq!(delta_crafter.methods, methods);

        delta_crafter.add_pickup(RedHeart);
        held[RedHeart] += 1;
        assert_eq!(delta_crafter.held, held);
        assert_eq!(delta_crafter.methods, methods);

        delta_crafter.add_pickup(RedHeart);
        held[RedHeart] += 1;
        assert_eq!(delta_crafter.held, held);
        assert_eq!(delta_crafter.methods, methods);

        delta_crafter.add_pickup(RedHeart);
        held[RedHeart] += 1;
        assert_eq!(delta_crafter.held, held);
        assert_eq!(delta_crafter.methods, methods);

        delta_crafter.add_pickup(RedHeart);
        held[RedHeart] += 1;
        assert_eq!(delta_crafter.held, held);
        assert_eq!(delta_crafter.methods, methods);

        delta_crafter.add_pickup(RedHeart);
        held[RedHeart] += 1;
        assert_eq!(delta_crafter.held, held);
        assert_eq!(delta_crafter.methods, methods);

        delta_crafter.add_pickup(RedHeart);
        held[RedHeart] += 1;
        assert_eq!(delta_crafter.held, held);
        assert_eq!(delta_crafter.methods, methods);

        delta_crafter.add_pickup(RedHeart);
        held[RedHeart] += 1;
        assert_eq!(delta_crafter.held, held);
        assert_eq!(delta_crafter.methods, methods);

        delta_crafter.add_pickup(RedHeart);
        held[RedHeart] += 1;
        methods[InternalItemId(599)].insert(InternalPickups::new([
            RedHeart, RedHeart, RedHeart, RedHeart, RedHeart, RedHeart, RedHeart, RedHeart,
        ]));
        assert_eq!(delta_crafter.held, held);
        assert_eq!(delta_crafter.methods, methods);

        delta_crafter.add_pickup(Pill);
        held[Pill] += 1;
        methods[InternalItemId(233)].insert(InternalPickups::new([
            RedHeart, RedHeart, RedHeart, RedHeart, RedHeart, RedHeart, RedHeart, Pill,
        ]));
        assert_eq!(delta_crafter.held, held);
        assert_eq!(delta_crafter.methods, methods);

        delta_crafter.add_pickup(RedHeart);
        held[RedHeart] += 1;
        assert_eq!(delta_crafter.held, held);
        assert_eq!(delta_crafter.methods, methods);

        delta_crafter.remove_pickup(RedHeart);
        held[RedHeart] -= 1;
        assert_eq!(delta_crafter.held, held);
        assert_eq!(delta_crafter.methods, methods);

        delta_crafter.remove_pickup(RedHeart);
        held[RedHeart] -= 1;
        methods[InternalItemId(599)].remove(&InternalPickups::new([
            RedHeart, RedHeart, RedHeart, RedHeart, RedHeart, RedHeart, RedHeart, RedHeart,
        ]));
        assert_eq!(delta_crafter.held, held);
        assert_eq!(delta_crafter.methods, methods);

        delta_crafter.remove_pickup(RedHeart);
        held[RedHeart] -= 1;
        methods[InternalItemId(233)].remove(&InternalPickups::new([
            RedHeart, RedHeart, RedHeart, RedHeart, RedHeart, RedHeart, RedHeart, Pill,
        ]));
        assert_eq!(delta_crafter.held, held);
        assert_eq!(delta_crafter.methods, methods);
    }
}
