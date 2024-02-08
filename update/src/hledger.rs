use std::collections::HashMap;

use shared::types::places;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Hledger(String),
    De(csv::Error),
    Ser(serde_json::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(error) => write!(f, "{error}"),
            Error::Hledger(error) => write!(f, "{error}"),
            Error::Ser(error) => write!(f, "{error}"),
            Error::De(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug, serde::Deserialize)]
struct Row {
    date: chrono::NaiveDate,
    amount: String,
    description: String,
}

#[derive(Debug, Clone)]
struct Entry {
    date: chrono::NaiveDate,
    amount: f64,
    payee: String,
}

impl From<Row> for Entry {
    fn from(row: Row) -> Self {
        let amount = row.amount.strip_suffix(" SEK").expect("SEK suffix");
        let amount = amount.parse().expect("parse amount");
        let payee = row
            .description
            .split('|')
            .next()
            .unwrap_or(&row.description)
            .trim()
            .to_string();
        Self {
            date: row.date,
            amount,
            payee,
        }
    }
}

pub async fn update<P: AsRef<std::path::Path>>(file: Option<P>, output: P) -> Result<(), Error> {
    use std::collections::BTreeMap;

    let output = output.as_ref();

    let year_ago = chrono::Local::now() - chrono::Duration::days(365);

    let mut command = std::process::Command::new("hledger");
    let mut command = command
        .arg("register")
        .arg("--value=then,SEK")
        .arg("--output-format=csv")
        .arg("--infer-market-prices")
        .arg("expenses:Food:Restaurants & Cafes$")
        .arg(format!("--begin={}", year_ago.format("%Y-%m-%d")));

    if let Some(file) = file {
        command = command.arg(format!("--file={}", file.as_ref().display()));
    }

    let hledger_output = command.output().map_err(Error::Io)?;

    if !hledger_output.status.success() {
        let stderr = String::from_utf8_lossy(&hledger_output.stderr);
        return Err(Error::Hledger(stderr.to_string()));
    }

    let mut reader = csv::Reader::from_reader(&hledger_output.stdout[..]);
    let rows = reader
        .deserialize()
        .map(|result| result.map_err(Error::De))
        .collect::<Result<Vec<Row>, _>>()?;

    let entries = rows.into_iter().map(Entry::from).collect::<Vec<_>>();

    let entries_by_payee_and_date = entries
        .into_iter()
        .fold(BTreeMap::new(), |mut map, entry| {
            map.entry((entry.payee.clone(), entry.date))
                .and_modify(|e: &mut Entry| e.amount += entry.amount)
                .or_insert(entry);
            map
        })
        .into_values();

    let entries_by_place =
        entries_by_payee_and_date
            .into_iter()
            .fold(BTreeMap::new(), |mut map, entry| {
                map.entry(entry.payee.clone())
                    .and_modify(|entries: &mut Vec<Entry>| entries.push(entry.clone()))
                    .or_insert_with(|| vec![entry]);
                map
            });

    let mut places = entries_by_place
        .into_iter()
        .filter(|(_, entries)| entries.len() > 1)
        .map(|(payee, entries)| places::Place {
            location: *LOCATIONS
                .get(&payee)
                .unwrap_or_else(|| panic!("no location for {payee}")),
            name: payee,
            times: entries.len().try_into().expect("usize to u8"),
            spent: entries.into_iter().map(|e| e.amount).sum(),
        })
        .collect::<Vec<_>>();
    places.sort_by(|a, b| b.times.cmp(&a.times));

    if output.exists() {
        async_std::fs::remove_dir_all(&output)
            .await
            .map_err(Error::Io)?;
    }

    async_std::fs::create_dir_all(&output)
        .await
        .map_err(Error::Io)?;

    for place in &places {
        let output = output.join(format!("{}.json", place.name));
        async_std::fs::write(
            &output,
            serde_json::to_string_pretty(&place).map_err(Error::Ser)?,
        )
        .await
        .map_err(Error::Io)?;
    }

    Ok(())
}

static LOCATIONS: once_cell::sync::Lazy<HashMap<String, (f64, f64)>> =
    once_cell::sync::Lazy::new(|| {
        let mut locations = HashMap::new();

        locations.insert("Cafe Ragazzi".to_string(), (59.335_918, 18.059_252));
        locations.insert("Esperanza".to_string(), (59.337_541, 18.057_392));
        locations.insert("Weidao".to_string(), (59.331_949, 18.057_649));
        locations.insert("Le Kebab".to_string(), (59.34188, 18.0491));
        locations.insert("Mikkeller".to_string(), (59.314_857, 18.076_491));
        locations.insert("Amida".to_string(), (59.314_721, 18.078_432));
        locations.insert("Johan & Nyström".to_string(), (59.316_273, 18.063_965));
        locations.insert("Kafé Ritorno".to_string(), (59.336_319, 18.048_464));
        locations.insert("Indian Curry House".to_string(), (59.329_504, 18.043_632));
        locations.insert("La Neta".to_string(), (59.336_673, 18.058_082));
        locations.insert("Pizza Hut".to_string(), (59.331_83, 18.059_651));
        locations.insert("Levinskys".to_string(), (59.339_83, 18.033_982));
        locations.insert("Marley Café".to_string(), (59.313_615, 18.079_98));
        locations.insert("Omnipollos Hatt".to_string(), (59.318_041, 18.072_196));
        locations.insert("Yume Sushi".to_string(), (59.317_338, 18.057_767));
        locations.insert("The Good Gringo".to_string(), (59.332_369, 18.045_24));
        locations.insert("Tehran Grill".to_string(), (59.340_032, 18.032_727));
        locations.insert(
            "Tehran Grill - Timmermansgatan".to_string(),
            (59.319_632, 18.059_523),
        );
        locations.insert("Franky's".to_string(), (59.339_843, 18.060_667));
        locations.insert("McDonald's".to_string(), (59.317_889, 18.053_61));
        locations.insert("Saigon Baguette".to_string(), (59.333_528, 18.058_477));
        locations.insert("AI Ramen".to_string(), (59.333_392, 18.057_954));
        locations.insert("AI Ramen Sofia".to_string(), (59.314_408, 18.089_957));
        locations.insert(
            "Tony's Pizza and Salad".to_string(),
            (59.319_056, 18.064_821),
        );
        locations.insert("Oolong Tea House".to_string(), (59.331_99, 18.067_811));
        locations.insert(
            "Cafe Eurobar Goes Russian".to_string(),
            (59.309_799, 18.079_108),
        );
        locations.insert("Brunos Korvbar".to_string(), (59.316_7, 18.055_242));
        locations.insert("Indian King".to_string(), (59.337_053, 18.052_454));
        locations.insert("Kajsas Fisk".to_string(), (59.334_262, 18.062_618));
        locations.insert("Phil’s burger".to_string(), (59.337_722, 18.057_471));
        locations.insert("Lillebrors Bageri".to_string(), (59.340_093, 18.033_877));
        locations.insert("Eat Greenii".to_string(), (59.334_63, 18.056_129));
        locations.insert("Chic Konditori".to_string(), (59.317_398, 18.063_439));
        locations.insert("Magnolia".to_string(), (59.319_797, 18.062_845));
        locations.insert("Tokyo Diner".to_string(), (59.334_456, 18.062_486));
        locations.insert("Pong".to_string(), (59.336_459, 18.058_798));
        locations.insert("Reload Superfood".to_string(), (59.334_054, 18.063_428));
        locations.insert("K4 Pampas".to_string(), (59.343_774, 18.006_291));
        locations.insert("DropCoffee".to_string(), (59.316_889, 18.062_711));
        locations.insert("Spice Of India".to_string(), (59.338_36, 18.039_952));
        locations.insert("Aryam".to_string(), (59.318_45, 18.070_466));
        locations.insert("Hanoi Corner".to_string(), (59.335_554, 18.053_908));
        locations.insert("Prime Burger".to_string(), (59.336_478, 18.061_049));
        locations.insert("Gute Grill Bar".to_string(), (59.336_739, 18.069_433));
        locations.insert("Fullmoon Wook".to_string(), (59.342_143, 18.049_23));
        locations.insert("Falafelbaren".to_string(), (59.318_37, 18.059_678));
        locations.insert("Itamae".to_string(), (59.333_08, 18.044_915));
        locations.insert("Meno Male".to_string(), (59.328_81, 18.045_246));
        locations.insert("Falloumi".to_string(), (59.308_163, 18.0777_89));
        locations.insert("Tutto Bello".to_string(), (59.338_836, 18.055_326));
        locations.insert("Hemma Vasastan".to_string(), (59.339_585, 18.047_905));
        locations.insert("Omnipollos Kyrka".to_string(), (59.363_667, 17.966_166));
        locations.insert("ESA Sushi".to_string(), (59.341_436, 18.037_335));
        locations.insert("Bröd & Salt".to_string(), (59.339_866, 18.037_079));
        locations.insert("Tbilisi’s Hörna".to_string(), (59.298_094, 18.088_214));
        locations.insert("Holy Cow".to_string(), (59.308_404, 18.066_223));
        locations.insert("Il Forno".to_string(), (59.338_245, 18.038_156));
        locations.insert("Bano City".to_string(), (59.337_541, 18.057_392));
        locations.insert("La Neta Bar".to_string(), (59.333_599, 18.070_247));
        locations.insert("A bowl of poke".to_string(), (59.319_777, 18.062_516));
        locations.insert("Joe & The Juice".to_string(), (59.318_803, 18.071_292));
        locations.insert("Eatnam".to_string(), (59.341_795, 18.047_249));
        locations.insert("Jafaris Donuts".to_string(), (59.329_551, 18.065_829));
        locations.insert("Birkastans Pizzeria".to_string(), (59.340_846, 18.032_985));
        locations.insert("Ming Palace".to_string(), (59.318_524, 18.061_989));
        locations.insert("SEN Street Kitchen".to_string(), (59.333_376, 18.054_404));
        locations.insert("Max".to_string(), (59.315_597, 18.073_084));
        locations.insert("StikkiNikki".to_string(), (59.318_77, 18.063_54));
        locations.insert("Spisa hos Helena".to_string(), (59.331_007, 18.044_476));
        locations.insert("Villa Romana".to_string(), (59.331_73, 18.049_42));
        locations.insert("Delibriket".to_string(), (59.365_031, 17.968_327));
        locations.insert(
            "Kungsholmens Glassfabrik".to_string(),
            (59.330_233, 18.045_708),
        );
        locations.insert("Waan Thai".to_string(), (59.341_723, 18.038_336));
        locations.insert("Ki mama".to_string(), (59.340_34, 18.050_425));
        locations.insert("Kolgrill Sam".to_string(), (59.428_415, 17.938_238));
        locations.insert("Belgobaren".to_string(), (59.332_907, 18.058_313));
        locations.insert("Shanti Shukriya".to_string(), (59.338_36, 18.039_952));
        locations.insert("PWC".to_string(), (59.337_622, 18.040_86));
        locations.insert("Ta'ameya".to_string(), (59.339_728, 18.060_213));
        locations.insert("Brisket & Friends".to_string(), (59.336_87, 18.054_108));
        locations.insert("Vigårda".to_string(), (59.333_651, 18.056_886));
        locations.insert("Djingis Khan".to_string(), (59.336_377, 18.063_073));
        locations.insert("Carl's Deli".to_string(), (59.334_257, 18.063_021));
        locations.insert("Mormors Dumpling".to_string(), (59.313_618, 18.085_532));
        locations.insert("Haymarket".to_string(), (59.334_656, 18.061_438));
        locations.insert("Brödernas".to_string(), (59.331_213, 18.044_646));
        locations.insert("Washoku Tomo".to_string(), (59.318_771, 18.059_878));
        locations.insert("Petite France".to_string(), (59.328_02, 18.037_138));
        locations.insert("D'abramo".to_string(), (59.334_457, 18.056_224));
        locations.insert("Nem Nem Quan".to_string(), (59.334_247, 18.062_669));
        locations.insert("Snø".to_string(), (59.340_793, 18.041_66));
        locations.insert("Haikyuu Sushi & Poke".to_string(), (59.329_799, 18.045_03));
        locations.insert("The Market".to_string(), (59.331_216, 18.059_258));
        locations.insert("Xulo".to_string(), (59.341_915, 18.036_938));
        locations.insert("Take Ramen".to_string(), (59.3405_91, 18.040_607));
        locations.insert("An Nam".to_string(), (59.339_906, 18.061_644));
        locations.insert("Günter's korvar".to_string(), (59.342_537, 18.032_653));
        locations.insert(
            "M.O.A.S Rörstrandsgatan".to_string(),
            (59.339_997, 18.033_302),
        );
        locations.insert("Kimchistan".to_string(), (59.333_083, 18.064_476));
        locations.insert(
            "The Italian Cousins - Odengatan".to_string(),
            (59.341_505, 18.044_753),
        );
        locations.insert(
            "The Italian Cousins - Hornsgatan".to_string(),
            (59.318_534, 18.058_827),
        );
        locations.insert("Lao Lao".to_string(), (59.317_697, 18.054_071));
        locations.insert("Fang Yuan Shi Wu".to_string(), (59.310_311, 18.082_277));
        locations.insert("Ja Thai".to_string(), (59.319_702, 18.068_247));
        locations.insert(
            "Falafelbaren - Götgatan".to_string(),
            (59.309_401, 18.075_404),
        );
        locations.insert("Susherian".to_string(), (59.308_812, 18.077_144));
        locations.insert("Indiska Källaren".to_string(), (59.308_648, 18.077_166));
        locations.insert(
            "Caput Mundi - Hornsgatan".to_string(),
            (59.315_738, 18.036_4),
        );
        locations.insert("Indio Kitchen".to_string(), (59.314_665, 18.083_247));
        locations.insert("ilcaffè".to_string(), (59.312_661, 18.081_021));
        locations.insert("Caput Mundi".to_string(), (59.313_771, 18.081_182));
        locations.insert("Tonys Coffeebar".to_string(), (57.697_588, 11.986_249));
        locations.insert("Bar Centro".to_string(), (57.705_447, 11.969_766));
        locations.insert("EDO Sushi".to_string(), (57.705_322, 11.967_964));
        locations.insert("A43".to_string(), (57.697_989, 11.978));
        locations.insert("koie ramen".to_string(), (57.704_636, 11.961_785));
        locations.insert("Banh Mi Shop".to_string(), (57.703_682, 11.968_214));
        locations.insert(
            "Alexandras Grekiska Soppkök".to_string(),
            (57.703_553, 11.967_89),
        );
        locations.insert("Gelaterian Göteborg".to_string(), (57.703_553, 11.967_89));
        locations.insert("House of Vietnam".to_string(), (57.709_422, 11.980_116));
        locations.insert("Bombay Street".to_string(), (57.693_698, 11.989_474));
        locations.insert("Ullevi Thaikök".to_string(), (57.708_449, 11.986_409));
        locations.insert("Nordstan Kebab".to_string(), (57.707_136, 11.968_511));
        locations.insert(
            "Morgon Coffee Roasters".to_string(),
            (57.708_091, 11.951_323),
        );
        locations.insert("Gourmetkorv".to_string(), (57.702_785, 11.963_998));
        locations.insert("Mahogny".to_string(), (57.707_827, 11.965_388));
        locations.insert("Zozaki".to_string(), (57.707_053, 11.973_963));
        locations.insert(
            "Hasselsson - Saluhallen".to_string(),
            (57.703_598, 11.967_959),
        );
        locations.insert("Steamy Pho".to_string(), (57.709_426, 11.985_551));
        locations.insert("Hoze".to_string(), (57.699_405, 11.935_068));
        locations.insert(
            "Landvetters Stenugnsbageri".to_string(),
            (57.700_682, 11.982_264),
        );
        locations.insert("Helenes Smørrebrød".to_string(), (57.698_781, 11.983_269));
        locations.insert("Hagabions Cafe".to_string(), (57.696_563, 11.950_73));
        locations.insert(
            "Trattoria Da Pasquale".to_string(),
            (57.700_095, 11.984_407),
        );
        locations.insert("La Terrazza".to_string(), (57.699_998, 11.984_547));
        locations.insert("Gansu Köket".to_string(), (57.708_635, 11.965_104));
        locations.insert("Joe And The Juice".to_string(), (59.318_923, 18.071_062));
        locations.insert("Kuro - Tbilisi".to_string(), (41.706_772, 44.781_744));
        locations.insert("Evolushi".to_string(), (57.708_039, 11.985_960));
        locations.insert("Nonna".to_string(), (57.703_262, 11.959_582));
        locations.insert("Kastello".to_string(), (57.696_235, 11.952_310))

        locations
    });
