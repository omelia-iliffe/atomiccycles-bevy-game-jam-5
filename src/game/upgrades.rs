use bevy::prelude::*;

use super::{cycles::CycleCount, movement::Revolve};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, add_upgrades);
    app.observe(purchase_upgrade);
}

fn add_upgrades(mut commands: Commands) {
    let upgrades = Upgrades(vec![
        Upgrade::new(
            "Speed Add",
            None,
            vec![
                (1, UpgradeType::SpeedAdd(1.0)),
                (2, UpgradeType::SpeedAdd(1.0)),
                (2, UpgradeType::SpeedAdd(1.0)),
            ],
        ),
        Upgrade::new(
            "Speed Multi",
            None,
            vec![
                (2, UpgradeType::SpeedMult(1.5)),
                (4, UpgradeType::SpeedMult(2.0)),
                (8, UpgradeType::SpeedMult(2.5)),
            ],
        ),
    ]);
    commands.insert_resource(upgrades);
}

#[derive(Debug, Clone)]
pub enum UpgradeType {
    SpeedAdd(f32),
    SpeedMult(f32),
    None,
}

impl std::fmt::Display for UpgradeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UpgradeType::SpeedAdd(v) => write!(f, "Speed +{}", v),
            UpgradeType::SpeedMult(v) => write!(f, "Speed x{}", v),
            UpgradeType::None => write!(f, "None"),
        }
    }
}

#[derive(Resource)]
pub struct Upgrades(pub Vec<Upgrade>);

#[derive(Component, Clone)]
pub struct Upgrade {
    name: String,
    description: Option<String>,
    purchased_level: usize,
    upgrades: Vec<(u32, UpgradeType)>,
}

impl Upgrade {
    pub fn new(name: &str, description: Option<&str>, u_type: Vec<(u32, UpgradeType)>) -> Self {
        Self {
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            purchased_level: 0,
            upgrades: u_type,
        }
    }

    pub fn next_cost(&self) -> Option<u32> {
        self.upgrades
            .get(self.purchased_level)
            .map(|(cost, _)| *cost)
    }

    pub fn name(&self) -> String {
        if self.purchased_level == self.upgrades.len() {
            format!("{} MAX", self.name)
        } else {
            format!("{} {}", self.name, self.purchased_level + 1)
        }
    }

    pub fn description(&self) -> String {
        if let Some(d) = &self.description {
            return d.clone();
        }
        if let Some((_, upgrade_type)) = self.upgrades.get(self.purchased_level) {
            return format!("{}", upgrade_type);
        }
        "No more upgrades".to_string()
    }

    pub fn cost(&self) -> String {
        if let Some(cost) = self.next_cost() {
            return format!("Cost: {}", cost);
        }
        "".to_string()
    }
}

#[derive(Event)]
pub struct PurchaseUpgrade(pub usize);

fn purchase_upgrade(
    trigger: Trigger<PurchaseUpgrade>,
    mut q: Query<&mut Revolve>,
    mut cycle_count: ResMut<CycleCount>,
    mut upgrades: ResMut<Upgrades>,
) {
    let upgrade_index = trigger.event().0;
    let upgrade = upgrades.0.get_mut(upgrade_index).unwrap();
    if upgrade.purchased_level == upgrade.upgrades.len() {
        log::info!("Upgrade already purchased");
        return;
    }
    let (cost, upgrade_type) = &upgrade.upgrades[upgrade.purchased_level];
    let can_purchase = cycle_count.0 >= *cost;

    if !can_purchase {
        log::info!("Cannot purchase upgrade: not enough cycles");
        return;
    }

    cycle_count.0 -= cost;

    log::info!("Purchased upgrade: {}", upgrade.name);
    upgrade.purchased_level += 1;
    match upgrade_type {
        UpgradeType::SpeedAdd(speed) => {
            for mut r in &mut q {
                r.speed += speed;
            }
        }
        UpgradeType::SpeedMult(mult) => {
            for mut r in &mut q {
                r.multiplier = *mult;
            }
        }
        UpgradeType::None => {}
    }
}
