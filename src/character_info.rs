use std::collections::HashMap;

#[derive(Debug)]
pub struct CharacterInfo {
    pub(crate) character_innates: CharacterInnates,
    pub(crate) attributes: Attributes,
    pub(crate) talents: Talents,
    pub(crate) mantras: Mantras,
}

#[derive(Debug)]
pub struct Attributes {
    pub(crate) stats: PlayerStats,
    pub(crate) attunements: Attunement,
    pub(crate) weapon_stats: WeaponStats
}

#[derive(Debug)]
pub struct CharacterInnates {
    pub(crate) name: String,
    pub(crate) level: String,
    pub(crate) race: String,
    pub(crate) oath: String,
    pub(crate) origin: String,
}

#[derive(Debug)]
pub struct Attunement {
    pub(crate) fir: u8,
    pub(crate) ltn: u8,
    pub(crate) wnd: u8,
    pub(crate) mtl: u8,
    pub(crate) ice: u8,
    pub(crate) sdw: u8,
    pub(crate) bld: u8
}

#[derive(Debug)]
pub struct WeaponStats {
    pub(crate) hvy: u8,
    pub(crate) med: u8,
    pub(crate) lht: u8,
}

#[derive(Debug)]
pub struct PlayerStats {
    pub(crate) str: u8,
    pub(crate) ftd: u8,
    pub(crate) agl: u8,
    pub(crate) int: u8,
    pub(crate) wll: u8,
    pub(crate) cha: u8,
}

#[derive(Debug)]
pub struct Talents(pub(crate) Vec<String>);

#[derive(Debug)]
pub struct Mantras(pub(crate) Vec<String>);


impl CharacterInnates {
    pub fn from_text(text: &str) -> CharacterInnates {

        let name = text.lines().nth(0).unwrap_or_default().to_string();

        let info_line = text.lines().nth(2).unwrap_or_default();

        let level = info_line
            .split_whitespace()
            .nth(1)
            .unwrap().to_string();

        let race = info_line.split_whitespace().nth(2).unwrap().to_string().replace(",", "").trim()
            .to_string();

        let origin = info_line.split_whitespace().nth(3).unwrap().replace(",", "").trim()
            .to_string();

        let oath = info_line.split_whitespace().nth(4).unwrap().to_string().replace(",", "").trim()
            .to_string();

        Self {
            name,
            level,
            race,
            origin,
            oath
        }
    }
}

impl Attunement {
    pub fn from_str(text: &str) -> Attunement {
        let info_line = text.lines().nth(8).unwrap_or_default();
        let mut att_map = HashMap::from([
            ("FIR".to_string(), 0),
            ("LTN".to_string(), 0),
            ("WND".to_string(), 0),
            ("MTL".to_string(), 0),
            ("ICE".to_string(), 0),
            ("SDW".to_string(), 0),
            ("BLD".to_string(), 0)
        ]);

        for split_token in info_line.split_whitespace() {
            let cleaned_token = split_token.replace(";", "").trim().to_string();
            if cleaned_token.parse::<u8>().is_ok() {
                continue;
            }
            if !att_map.contains_key(cleaned_token.as_str()) {
                continue;
            }
            let att_level: u8 = info_line.split_whitespace().nth(
                info_line.split_whitespace().position(|x| x.contains(split_token)).unwrap() - 1
            ).unwrap_or("0").parse().unwrap();
            att_map.insert(cleaned_token, att_level);
        }
        Self {
            fir : att_map.get("FIR").unwrap().clone(),
            ltn : att_map.get("LTN").unwrap().clone(),
            wnd : att_map.get("WND").unwrap().clone(),
            mtl : att_map.get("MTL").unwrap().clone(),
            ice : att_map.get("ICE").unwrap().clone(),
            sdw : att_map.get("SDW").unwrap().clone(),
            bld : att_map.get("BLD").unwrap().clone()
        }
    }
}

impl WeaponStats {
    pub fn from_str(text: &str) -> WeaponStats {
        let info_line = text.lines().nth(6).unwrap_or_default();
        let hvy: u8 = info_line.split_whitespace().nth(0).unwrap().parse().unwrap();
        let med: u8 = info_line.split_whitespace().nth(2).unwrap().parse().unwrap();
        let lht: u8 = info_line.split_whitespace().nth(4).unwrap().parse().unwrap();
        Self {
            hvy,
            med,
            lht
        }
    }
}

impl PlayerStats {
    pub fn from_str(text: &str) -> PlayerStats {
        let info_line = text.lines().nth(4).unwrap_or_default();

        let str: u8 = info_line.split_whitespace().nth(0).unwrap().parse().unwrap();
        let ftd: u8 = info_line.split_whitespace().nth(2).unwrap().parse().unwrap();
        let agl: u8 = info_line.split_whitespace().nth(4).unwrap().parse().unwrap();
        let int: u8 = info_line.split_whitespace().nth(6).unwrap().parse().unwrap();
        let wll: u8 = info_line.split_whitespace().nth(8).unwrap().parse().unwrap();
        let cha: u8 = info_line.split_whitespace().nth(10).unwrap().parse().unwrap();

        Self {
            str,
            ftd,
            agl,
            int,
            wll,
            cha
        }
    }
}

impl Talents {
    pub fn from_str(text: &str) -> Talents {
        let mut talents = Vec::new();
        for line in text.lines().skip(text.lines()
            .position(|x| x.contains("== TALENTS ==")).unwrap_or_default() + 1) {
            talents.push(line.to_string());
        }
        Self(talents)
    }
}

impl Mantras {
    pub fn from_str(text: &str) -> Mantras {
        let mut mantras = Vec::new();
        for line in text.lines().skip(10) {
            if line.contains("== TALENTS ==") {
                break;
            }
            mantras.push(line.to_string());
        }
        Self(mantras)
    }
}

impl Attributes {
    pub fn from_str(text: &str) -> Attributes {
        let stats = PlayerStats::from_str(text);
        let attunements = Attunement::from_str(text);
        let weapon_stats = WeaponStats::from_str(text);

        Self {
            stats,
            attunements,
            weapon_stats
        }
    }
}

impl CharacterInfo {
    pub fn from_str(text: &str) -> CharacterInfo {
        let character_innates = CharacterInnates::from_text(text);
        let attributes = Attributes::from_str(text);
        let talents = Talents::from_str(text);
        let mantras = Mantras::from_str(text);

        Self {
            character_innates,
            attributes,
            talents,
            mantras
        }
    }
}