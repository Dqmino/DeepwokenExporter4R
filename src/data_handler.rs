use http::Method;
use crate::character_info::CharacterInfo;
use reqwest;

pub(crate) fn send_http_request(data_as_struct: CharacterInfo) {
    let character_innates = data_as_struct.character_innates;
    let attributes = data_as_struct.attributes;
    let mantras = data_as_struct.mantras;
    let talents = data_as_struct.talents;

    let weapon_stats = attributes.weapon_stats;
    let attunements = attributes.attunements;
    let stats = attributes.stats;

    let data = "{".to_owned() +
        "\"version\":3," +
        "\"stats\":{" +
        "\"buildName\":\"" + &*character_innates.name + "\"," +
        "\"buildDescription\":\"" + "Auto-generated build from the ingame exportation feature, coded by @dominocodes on Discord." + "\"," +
        "\"buildAuthor\":\"" + "@dominocodes on Discord." + "\"," +
        "\"power\":" + &*character_innates.level + "," +
        "\"pointsUntilNextPower\":0," +
        "\"points\":0," +
        "\"pointSpent\":330," +
        "\"traitsPoints\":12," +
        "\"traits\":{" +
        "\"Vitality\":0," +
        "\"Erudition\":0," +
        "\"Proficiency\":0," +
        "\"Songchant\":0" +
        "}," +
        "\"meta\":{" +
        "\"Race\":\"" + &*character_innates.race + "\"," +
        "\"Oath\":\"" + &*character_innates.oath + "\"," +
        "\"Murmur\":\"Ardour\"," +
        "\"Origin\":\"" + &*character_innates.origin + "\"," +
        "\"Bell\":\"Blood Scourge\"," +
        "\"Outfit\":\"Black Diver\"" +
        "}," +
        "\"boon1\":\"None\"," +
        "\"boon2\":\"None\"," +
        "\"flaw1\":\"None\"," +
        "\"flaw2\":\"None\"" +
        "}," +
        "\"attributes\":{" +
        "\"weapon\":{" +
        "\"Heavy Wep.\":" + &*weapon_stats.hvy.to_string() + "," +
        "\"Medium Wep.\":" + &*weapon_stats.med.to_string() + "," +
        "\"Light Wep.\":" + &*weapon_stats.lht.to_string() + "" +
        "}," +
        "\"attunement\":{" +
        "\"Flamecharm\":" + &*attunements.fir.to_string() + "," +
        "\"Frostdraw\":" + &*attunements.ice.to_string() + "," +
        "\"Thundercall\":" + &*attunements.ltn.to_string() + "," +
        "\"Galebreathe\":" + &*attunements.wnd.to_string() + "," +
        "\"Shadowcast\":" + &*attunements.sdw.to_string() + "," +
        "\"Ironsing\":" + &*attunements.mtl.to_string() + "," +
        "\"Bloodrend\":" + &*attunements.bld.to_string() + "" +
        "}," +
        "\"base\":{" +
        "\"Strength\":" + &*stats.str.to_string() + "," +
        "\"Fortitude\":" + &*stats.ftd.to_string() + "," +
        "\"Agility\":" + &*stats.agl.to_string() + "," +
        "\"Intelligence\":" + &*stats.int.to_string() + "," +
        "\"Willpower\":" + &*stats.wll.to_string() + "," +
        "\"Charisma\":" + &*stats.cha.to_string() + "" +
        "}" +
        "}," +
        "\"content\":{" +
        "\"mantraModifications\":{}," +
        "\"notes\":\"\"" +
        "}," +
        "\"meta\":{" +
        "\"tags\":[\"Advanced\"]," +
        "\"isPrivate\":true" +
        "}," +
        "\"talents\":" + format_vector(talents.0).as_str() + "," +
        "\"mantras\":" + format_vector(mantras.0).as_str() + "," +
        "\"weapons\":\"\"," +
        "\"preShrine\":{" +
        "\"base\":{" +
        "\"Strength\":0," +
        "\"Fortitude\":0," +
        "\"Agility\":0," +
        "\"Intelligence\":0," +
        "\"Willpower\":0," +
        "\"Charisma\":0" +
        "}," +
        "\"weapon\":{" +
        "\"Heavy Wep.\":0," +
        "\"Medium Wep.\":0," +
        "\"Light Wep.\":0" +
        "}," +
        "\"attunement\":{" +
        "\"Flamecharm\":0," +
        "\"Frostdraw\":0," +
        "\"Thundercall\":0," +
        "\"Galebreathe\":0," +
        "\"Shadowcast\":0," +
        "\"Ironsing\":0," +
        "\"Bloodrend\":0" +
        "}" +
        "}," +
        "\"postShrine\":{" +
        "\"weapon\":{" +
        "\"Heavy Wep.\":" + &*weapon_stats.hvy.to_string() + "," +
        "\"Medium Wep.\":" + &*weapon_stats.med.to_string() + "," +
        "\"Light Wep.\":" + &*weapon_stats.lht.to_string() + "" +
        "}," +
        "\"attunement\":{" +
        "\"Flamecharm\":" + &*attunements.fir.to_string() + "," +
        "\"Frostdraw\":" + &*attunements.ice.to_string() + "," +
        "\"Thundercall\":" + &*attunements.ltn.to_string() + "," +
        "\"Galebreathe\":" + &*attunements.wnd.to_string() + "," +
        "\"Shadowcast\":" + &*attunements.sdw.to_string() + "," +
        "\"Ironsing\":" + &*attunements.mtl.to_string() + "," +
        "\"Bloodrend\":" + &*attunements.bld.to_string() + "" +
        "}," +
        "\"base\":{" +
        "\"Strength\":" + &*stats.str.to_string() + "," +
        "\"Fortitude\":" + &*stats.ftd.to_string() + "," +
        "\"Agility\":" + &*stats.agl.to_string() + "," +
        "\"Intelligence\":" + &*stats.int.to_string() + "," +
        "\"Willpower\":" + &*stats.wll.to_string() + "," +
        "\"Charisma\":" + &*stats.cha.to_string() + "" +
        "}" +
        "}," +
        "\"favoritedTalents\":[]" +
        "}";

    println!("https://deepwoken.co/builder?id={}", reqwest::blocking::Client::new()
        .request(Method::POST, "https://api.deepwoken.co/build")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:132.0) Gecko/20100101 Firefox/132.0")
        .header("content-type", "application/json")
        .body(data)
        .send().unwrap().text().unwrap().replace("{\"id\":\"", "").replace("\"}", ""));
}

fn format_vector(vector: Vec<String>) -> String {
    format!(
        "[{}]",
        vector.iter()
            .map(|s| format!("\"{}\"", s))
            .collect::<Vec<_>>()
            .join(",")
    ).trim().to_string()
}
