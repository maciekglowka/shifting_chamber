use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum UpgradeKind {
    TileTransform(TransformKind),
    HealPlayer,
    IncreaseAP,
    IncreaseHP
}
impl UpgradeKind {
    pub fn is_single(&self) -> bool {
        match self {
            Self::HealPlayer | Self::IncreaseAP | Self::IncreaseHP => false,
            _ => true
        }
    }
    pub fn to_str(&self) -> &str {
        match self {
            Self::HealPlayer => "Heal 3HP",
            Self::IncreaseAP => "Increase Max AP +1",
            Self::IncreaseHP => "Increase Max HP +1",
            Self::TileTransform(a) => a.to_str()
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum TransformKind {
    TileShift,
    TileSwitch,
    TileRotate
}
impl TransformKind {
    pub fn to_str(&self) -> &str {
        match self {
            Self::TileShift => "Tile Shift",
            Self::TileSwitch => "Tile Switch",
            Self::TileRotate => "Tile Rotate",
        }
    }
}

pub fn get_initial_upgrades() -> HashSet<UpgradeKind> {
    HashSet::from_iter(
        vec![
            UpgradeKind::TileTransform(TransformKind::TileSwitch),
            UpgradeKind::TileTransform(TransformKind::TileRotate),
            UpgradeKind::HealPlayer,
            UpgradeKind::IncreaseAP,
            UpgradeKind::IncreaseHP
        ]
    )
}