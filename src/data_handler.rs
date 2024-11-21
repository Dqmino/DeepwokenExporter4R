use std::io::Read;
use curl::easy::Easy;
use crate::character_info::CharacterInfo;

pub(crate) fn send_http_request(data_as_struct: CharacterInfo) {

    let character_innates = data_as_struct.character_innates;
    let attributes = data_as_struct.attributes;
    let mantras = data_as_struct.mantras;
    let talents = data_as_struct.talents;

    let weapon_stats = attributes.weapon_stats;
    let attunements = attributes.attunements;
    let stats = attributes.stats;

    let data = "{\n".to_owned() +
        "    \"version\": 3,\n" +
        "    \"stats\": {\n" +
        "        \"buildName\": \"" + &*character_innates.name + "\",\n" +
        "        \"buildDescription\": \"" + &*character_innates.name + "\",\n" +
        "        \"buildAuthor\": \"" + "@dominocodes on Discord." + "\",\n" +
        "        \"power\": " + &*character_innates.level + ",\n" +
        "        \"pointsUntilNextPower\": 0,\n" +
        "        \"points\": 0,\n" +
        "        \"pointSpent\": 330,\n" +
        "        \"traitsPoints\": 12,\n" +
        "        \"traits\": {\n" +
        "            \"Vitality\": 0,\n" +
        "            \"Erudition\": 0,\n" +
        "            \"Proficiency\": 0,\n" +
        "            \"Songchant\": 0\n" +
        "        },\n" +
        "        \"meta\": {\n" +
        "            \"Race\": \"" + &*character_innates.race + "\",\n" +
        "            \"Oath\": \"" + &*character_innates.oath + "\",\n" +
        "            \"Murmur\": \"Ardour\",\n" +
        "            \"Origin\": \"" + &*character_innates.origin + "\",\n" +
        "            \"Bell\": \"Blood Scourge\",\n" +
        "            \"Outfit\": \"Black Diver\"\n" +
        "        },\n" +
        "        \"boon1\": \"None\",\n" +
        "        \"boon2\": \"None\",\n" +
        "        \"flaw1\": \"None\",\n" +
        "        \"flaw2\": \"None\"\n" +
        "    },\n" +
        "    \"attributes\": {\n" +
        "        \"weapon\": {\n" +
        "            \"Heavy Wep\": " + &*weapon_stats.hvy.to_string() + ",\n" +
        "            \"Medium Wep\": " + &*weapon_stats.med.to_string() + ",\n" +
        "            \"Light Wep\": " + &*weapon_stats.lht.to_string() + "\n" +
        "        },\n" +
        "        \"attunement\": {\n" +
        "            \"Flamecharm\": " + &*attunements.fir.to_string() + ",\n" +
        "            \"Frostdraw\": " + &*attunements.ice.to_string() + ",\n" +
        "            \"Thundercall\": " + &*attunements.ltn.to_string() + ",\n" +
        "            \"Galebreathe\": " + &*attunements.wnd.to_string() + ",\n" +
        "            \"Shadowcast\": " + &*attunements.sdw.to_string() + ",\n" +
        "            \"Ironsing\": " + &*attunements.mtl.to_string() + ",\n" +
        "            \"Bloodrend\": " + &*attunements.bld.to_string() + "\n" +
        "        },\n" +
        "        \"base\": {\n" +
        "            \"Strength\": " + &*stats.str.to_string() + ",\n" +
        "            \"Fortitude\": " + &*stats.ftd.to_string() + ",\n" +
        "            \"Agility\": " + &*stats.agl.to_string() + ",\n" +
        "            \"Intelligence\": " + &*stats.int.to_string() + ",\n" +
        "            \"Willpower\": " + &*stats.wll.to_string() + ",\n" +
        "            \"Charisma\": " + &*stats.cha.to_string() + "\n" +
        "        }\n" +
        "    },\n" +
        "    \"content\": {\n" +
        "        \"mantraModifications\": {},\n" +
        "        \"notes\": \"\"\n" +
        "    },\n" +
        "    \"meta\": {\n" +
        "        \"tags\": [\"Advanced\"],\n" +
        "        \"isPrivate\": true\n" +
        "    },\n" +
        "    \"talents\": " + format!(
        "[{}]",
        talents.0.iter()
            .map(|s| format!("\"{}\"", s))
            .collect::<Vec<_>>()
            .join(", ")
    ).as_str() + ",\n" +
        "    \"mantras\": " + format!(
        "[{}]",
        mantras.0.iter()
            .map(|s| format!("\"{}\"", s))
            .collect::<Vec<_>>()
            .join(", ")
    ).as_str() + ",\n" +
        "    \"weapons\": \"\",\n" +
        "    \"preShrine\": {\n" +
        "        \"base\": {\n" +
        "            \"Strength\": 0,\n" +
        "            \"Fortitude\": 0,\n" +
        "            \"Agility\": 0,\n" +
        "            \"Intelligence\": 0,\n" +
        "            \"Willpower\": 0,\n" +
        "            \"Charisma\": 0\n" +
        "        },\n" +
        "        \"weapon\": {\n" +
        "            \"Heavy Wep\": 0,\n" +
        "            \"Medium Wep\": 0,\n" +
        "            \"Light Wep\": 0\n" +
        "        },\n" +
        "        \"attunement\": {\n" +
        "            \"Flamecharm\": 0,\n" +
        "            \"Frostdraw\": 0,\n" +
        "            \"Thundercall\": 0,\n" +
        "            \"Galebreathe\": 0,\n" +
        "            \"Shadowcast\": 0,\n" +
        "            \"Ironsing\": 0,\n" +
        "            \"Bloodrend\": 0\n" +
        "        }\n" +
        "    },\n" +
        "    \"postShrine\": {\n" +
        "        \"weapon\": {\n" +
        "            \"Heavy Wep\": " + &*weapon_stats.hvy.to_string() + ",\n" +
        "            \"Medium Wep\": " + &*weapon_stats.med.to_string() + ",\n" +
        "            \"Light Wep\": " + &*weapon_stats.lht.to_string() + "\n" +
        "        },\n" +
        "        \"attunement\": {\n" +
        "            \"Flamecharm\": " + &*attunements.fir.to_string() + ",\n" +
        "            \"Frostdraw\": " + &*attunements.ice.to_string() + ",\n" +
        "            \"Thundercall\": " + &*attunements.ltn.to_string() + ",\n" +
        "            \"Galebreathe\": " + &*attunements.wnd.to_string() + ",\n" +
        "            \"Shadowcast\": " + &*attunements.sdw.to_string() + ",\n" +
        "            \"Ironsing\": " + &*attunements.mtl.to_string() + ",\n" +
        "            \"Bloodrend\": " + &*attunements.bld.to_string() + "\n" +
        "        },\n" +
        "        \"base\": {\n" +
        "            \"Strength\": " + &*stats.str.to_string() + ",\n" +
        "            \"Fortitude\": " + &*stats.ftd.to_string() + ",\n" +
        "            \"Agility\": " + &*stats.agl.to_string() + ",\n" +
        "            \"Intelligence\": " + &*stats.int.to_string() + ",\n" +
        "            \"Willpower\": " + &*stats.wll.to_string() + ",\n" +
        "            \"Charisma\": " + &*stats.cha.to_string() + "\n" +
        "        }\n" +
        "    },\n" +
        "    \"favoritedTalents\": []\n" +
        "}";

    let mut easy = Easy::new();
    easy.url("https://api.deepwoken.co/build").unwrap();
    easy.post(true).unwrap();
    easy.post_field_size(data.len() as u64).unwrap();
    easy.write_function(|data| {
        println!("{}", String::from_utf8_lossy(data));
        Ok(data.len())
    }).unwrap();
    let mut transfer = easy.transfer();
    transfer.read_function(|buf| {
        println!("{}", String::from_utf8_lossy(data.as_bytes()));
        Ok(data.as_bytes().read(buf).unwrap_or(0))
    }).unwrap();
    transfer.perform().unwrap();
}
