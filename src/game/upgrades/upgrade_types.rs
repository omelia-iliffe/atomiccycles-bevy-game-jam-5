use crate::game::upgrades::UpgradeAction;

pub trait Upgrade {
    /// Depleted, fully purchased, purchased, cannot not be bought anymore ever
    fn purchased(&self) -> bool;
    fn upgrade_action(&self) -> Option<&UpgradeAction>;
    fn purchase(&mut self) -> Result<(), String>;
    fn next_cost(&self) -> Option<u32>;
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn cost(&self) -> String;
}

pub struct SingleUpgrade {
    name: String,
    description: Option<String>,
    purchased: bool,
    cost: u32,
    upgrade: UpgradeAction,
}

impl SingleUpgrade {
    pub fn new(name: &str, description: Option<&str>, cost: u32, upgrade: UpgradeAction) -> Self {
        Self {
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            purchased: false,
            cost,
            upgrade,
        }
    }
}

impl Upgrade for SingleUpgrade {
    fn purchased(&self) -> bool {
        self.purchased
    }
    fn upgrade_action(&self) -> Option<&UpgradeAction> {
        Some(&self.upgrade)
    }
    fn purchase(&mut self) -> Result<(), String> {
        if self.purchased {
            return Err("Already purchased".to_string());
        }
        self.purchased = true;
        Ok(())
    }
    fn next_cost(&self) -> Option<u32> {
        if self.purchased {
            return None;
        }
        Some(self.cost)
    }
    fn name(&self) -> String {
        self.name.clone()
    }
    fn description(&self) -> String {
        if let Some(d) = &self.description {
            return d.clone();
        }
        format!("{}", self.upgrade)
    }
    fn cost(&self) -> String {
        if self.purchased {
            return "Purchased".to_string();
        }
        format!("Cost: {}", self.cost)
    }
}

pub struct LevelUpgrade {
    name: String,
    description: Option<String>,
    purchased_level: usize,
    upgrades: Vec<(u32, UpgradeAction)>,
}

impl LevelUpgrade {
    pub fn new(name: &str, description: Option<&str>, u_type: Vec<(u32, UpgradeAction)>) -> Self {
        Self {
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            purchased_level: 0,
            upgrades: u_type,
        }
    }
}

impl Upgrade for LevelUpgrade {
    fn purchased(&self) -> bool {
        self.purchased_level == self.upgrades.len()
    }

    fn upgrade_action(&self) -> Option<&UpgradeAction> {
        self.upgrades.get(self.purchased_level).map(|(_, u)| u)
    }
    fn purchase(&mut self) -> Result<(), String> {
        if self.purchased() {
            return Err("Already purchased".to_string());
        }
        self.purchased_level += 1;

        Ok(())
    }
    fn next_cost(&self) -> Option<u32> {
        self.upgrades
            .get(self.purchased_level)
            .map(|(cost, _)| *cost)
    }
    fn name(&self) -> String {
        if self.purchased_level == self.upgrades.len() {
            format!("{} MAX", self.name)
        } else {
            format!("{} {}", self.name, self.purchased_level + 1)
        }
    }
    fn description(&self) -> String {
        if let Some(d) = &self.description {
            return d.clone();
        }
        if let Some((_, upgrade_type)) = self.upgrades.get(self.purchased_level) {
            return format!("{}", upgrade_type);
        }
        "No more upgrades".to_string()
    }

    fn cost(&self) -> String {
        if let Some(cost) = self.next_cost() {
            return format!("Cost: {}", cost);
        }
        "".to_string()
    }
}
