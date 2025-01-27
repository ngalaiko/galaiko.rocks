# /// script
# requires-python = ">=3.13"
# dependencies = [
#     "python-slugify",
# ]
# ///
import csv
from slugify import slugify
import subprocess
import os
import argparse
import json
import datetime
from collections import defaultdict

LOCATIONS = {
    "Cafe Ragazzi": (59.335_918, 18.059_252),
    "Esperanza": (59.337_541, 18.057_392),
    "Weidao": (59.331_949, 18.057_649),
    "Le Kebab": (59.34188, 18.0491),
    "Mikkeller": (59.314_857, 18.076_491),
    "Amida": (59.314_721, 18.078_432),
    "Johan & Nyström": (59.316_273, 18.063_965),
    "Kafé Ritorno": (59.336_319, 18.048_464),
    "Indian Curry House": (59.329_504, 18.043_632),
    "La Neta": (59.336_673, 18.058_082),
    "Pizza Hut": (59.331_83, 18.059_651),
    "Levinskys": (59.339_83, 18.033_982),
    "Marley Café": (59.313_615, 18.079_98),
    "Omnipollos Hatt": (59.318_041, 18.072_196),
    "Yume Sushi": (59.317_338, 18.057_767),
    "The Good Gringo": (59.332_369, 18.045_24),
    "Tehran Grill": (59.340_032, 18.032_727),
    "Tehran Grill - Timmermansgatan": (59.319_632, 18.059_523),
    "Franky's": (59.339_843, 18.060_667),
    "McDonald's": (59.317_889, 18.053_61),
    "Saigon Baguette": (59.333_528, 18.058_477),
    "AI Ramen": (59.333_392, 18.057_954),
    "AI Ramen Sofia": (59.314_408, 18.089_957),
    "Tony's Pizza and Salad": (59.319_056, 18.064_821),
    "Oolong Tea House": (59.331_99, 18.067_811),
    "Cafe Eurobar Goes Russian": (59.309_799, 18.079_108),
    "Brunos Korvbar": (59.316_7, 18.055_242),
    "Indian King": (59.337_053, 18.052_454),
    "Kajsas Fisk": (59.334_262, 18.062_618),
    "Phil’s burger": (59.337_722, 18.057_471),
    "Lillebrors Bageri": (59.340_093, 18.033_877),
    "Eat Greenii": (59.334_63, 18.056_129),
    "Chic Konditori": (59.317_398, 18.063_439),
    "Magnolia": (59.319_797, 18.062_845),
    "Tokyo Diner": (59.334_456, 18.062_486),
    "Pong": (59.336_459, 18.058_798),
    "Reload Superfood": (59.334_054, 18.063_428),
    "K4 Pampas": (59.343_774, 18.006_291),
    "DropCoffee": (59.316_889, 18.062_711),
    "Spice Of India": (59.338_36, 18.039_952),
    "Aryam": (59.318_45, 18.070_466),
    "Hanoi Corner": (59.335_554, 18.053_908),
    "Prime Burger": (59.336_478, 18.061_049),
    "Gute Grill Bar": (59.336_739, 18.069_433),
    "Fullmoon Wook": (59.342_143, 18.049_23),
    "Falafelbaren": (59.318_37, 18.059_678),
    "Itamae": (59.333_08, 18.044_915),
    "Meno Male": (59.328_81, 18.045_246),
    "Falloumi": (59.308_163, 18.0777_89),
    "Tutto Bello": (59.338_836, 18.055_326),
    "Hemma Vasastan": (59.339_585, 18.047_905),
    "Omnipollos Kyrka": (59.363_667, 17.966_166),
    "ESA Sushi": (59.341_436, 18.037_335),
    "Bröd & Salt": (59.339_866, 18.037_079),
    "Tbilisi’s Hörna": (59.298_094, 18.088_214),
    "Holy Cow": (59.308_404, 18.066_223),
    "Il Forno": (59.338_245, 18.038_156),
    "Bano City": (59.337_541, 18.057_392),
    "La Neta Bar": (59.333_599, 18.070_247),
    "A bowl of poke": (59.319_777, 18.062_516),
    "Joe & The Juice": (59.318_803, 18.071_292),
    "Eatnam": (59.341_795, 18.047_249),
    "Jafaris Donuts": (59.329_551, 18.065_829),
    "Jafaris Donuts - Goteborg": (57.708128, 11.973687),
    "Birkastans Pizzeria": (59.340_846, 18.032_985),
    "Ming Palace": (59.318_524, 18.061_989),
    "SEN Street Kitchen": (59.333_376, 18.054_404),
    "Max": (59.315_597, 18.073_084),
    "StikkiNikki": (59.318_77, 18.063_54),
    "Spisa hos Helena": (59.331_007, 18.044_476),
    "Villa Romana": (59.331_73, 18.049_42),
    "Delibriket": (59.365_031, 17.968_327),
    "Kungsholmens Glassfabrik": (59.330_233, 18.045_708),
    "Waan Thai": (59.341_723, 18.038_336),
    "Ki mama": (59.340_34, 18.050_425),
    "Kolgrill Sam": (59.428_415, 17.938_238),
    "Belgobaren": (59.332_907, 18.058_313),
    "Shanti Shukriya": (59.338_36, 18.039_952),
    "PWC": (59.337_622, 18.040_86),
    "Ta'ameya": (59.339_728, 18.060_213),
    "Brisket & Friends": (59.336_87, 18.054_108),
    "Vigårda": (59.333_651, 18.056_886),
    "Djingis Khan": (59.336_377, 18.063_073),
    "Carl's Deli": (59.334_257, 18.063_021),
    "Mormors Dumpling": (59.313_618, 18.085_532),
    "Haymarket": (59.334_656, 18.061_438),
    "Brödernas": (59.331_213, 18.044_646),
    "Washoku Tomo": (59.318_771, 18.059_878),
    "Petite France": (59.328_02, 18.037_138),
    "D'abramo": (59.334_457, 18.056_224),
    "Nem Nem Quan": (59.334_247, 18.062_669),
    "Snø": (59.340_793, 18.041_66),
    "Haikyuu Sushi & Poke": (59.329_799, 18.045_03),
    "The Market": (59.331_216, 18.059_258),
    "Xulo": (59.341_915, 18.036_938),
    "Take Ramen": (59.3405_91, 18.040_607),
    "An Nam": (59.339_906, 18.061_644),
    "Günter's korvar": (59.342_537, 18.032_653),
    "M.O.A.S Rörstrandsgatan": (59.339_997, 18.033_302),
    "Kimchistan": (59.333_083, 18.064_476),
    "The Italian Cousins - Odengatan": (59.341_505, 18.044_753),
    "The Italian Cousins - Hornsgatan": (59.318_534, 18.058_827),
    "Lao Lao": (59.317_697, 18.054_071),
    "Fang Yuan Shi Wu": (59.310_311, 18.082_277),
    "Ja Thai": (59.319_702, 18.068_247),
    "Falafelbaren - Götgatan": (59.309_401, 18.075_404),
    "Susherian": (59.308_812, 18.077_144),
    "Indiska Källaren": (59.308_648, 18.077_166),
    "Caput Mundi - Hornsgatan": (59.315_738, 18.036_4),
    "Indio Kitchen": (59.314_665, 18.083_247),
    "ilcaffè": (59.312_661, 18.081_021),
    "Caput Mundi": (59.313_771, 18.081_182),
    "Tonys Coffeebar": (57.697_588, 11.986_249),
    "Bar Centro": (57.705_447, 11.969_766),
    "EDO Sushi": (57.705_322, 11.967_964),
    "A43": (57.697_989, 11.978),
    "koie ramen": (57.704_636, 11.961_785),
    "Banh Mi Shop": (57.703_682, 11.968_214),
    "Alexandras Grekiska Soppkök": (57.703_553, 11.967_89),
    "Gelaterian Göteborg": (57.703_553, 11.967_89),
    "House of Vietnam": (57.709_422, 11.980_116),
    "Bombay Street": (57.693_698, 11.989_474),
    "Ullevi Thaikök": (57.708_449, 11.986_409),
    "Nordstan Kebab": (57.707_136, 11.968_511),
    "Morgon Coffee Roasters": (57.708_091, 11.951_323),
    "Gourmetkorv": (57.702_785, 11.963_998),
    "Mahogny": (57.707_827, 11.965_388),
    "Zozaki": (57.707_053, 11.973_963),
    "Hasselsson - Saluhallen": (57.703_598, 11.967_959),
    "Steamy Pho": (57.709_426, 11.985_551),
    "Hoze": (57.699_405, 11.935_068),
    "Landvetters Stenugnsbageri": (57.700_682, 11.982_264),
    "Helenes Smørrebrød": (57.698_781, 11.983_269),
    "Hagabions Cafe": (57.696_563, 11.950_73),
    "Trattoria Da Pasquale": (57.700_095, 11.984_407),
    "La Terrazza": (57.699_998, 11.984_547),
    "Gansu Köket": (57.708_635, 11.965_104),
    "Joe And The Juice": (59.318_923, 18.071_062),
    "Kuro - Tbilisi": (41.706_772, 44.781_744),
    "Evolushi": (57.708_039, 11.985_960),
    "Nonna": (57.703_262, 11.959_582),
    "Kastello": (57.696_235, 11.952_310),
    "Viktors Kaffe": (57.697_449, 11.978_164),
    "Dirty Records": (57.699_326, 11.951_237),
    "Råda Gelato": (57.696_235, 11.952_310),
    "Umizu": (57.699_246, 11.977_780),
    "Bar Etzy": (57.696_018, 11.959_750),
    "Nooodle": (57.695_921, 11.959_661),
    "Hasselsson - Skanstorget": (57.695_921, 11.959_661),
    "Manniny Espresso": (57.698_252, 11.965_896),
    "McDonald's - Göteborg": (57.699_245, 11.977_162),
    "Che Argento": (57.698036, 11.960528),
    "Berlin Doner": (57.692734, 11.949853),
    "Bizen": (57.717110, 11.929223),
    "Pagoden": (57.715770, 11.973805),
    "Kaffe Labet": (57.698059, 11.952616),
    "Oizo": (57.713235, 12.024676),
    "Papi's Pirogi - Gronsakstorg": (57.713235, 12.024676),
    "Efessos": (57.695582, 11.922782),
    "Firos Crep": (57.697161, 11.957360),
    "Poppels Citybryggeri": (57.713640, 11.971329),
    "Waffles Goteborg": (57.699106, 11.969203),
    "Big Bird": (57.693569, 11.956427),
    "Com": (57.696215, 11.950183),
    "Salko Cevaboznica": (57.728254, 11.949048),
    "Geisha Sushi": (57.692677, 11.955287),
    "Gerd": (57.698302, 11.951306),
    "Monopolet": (57.715475, 11.973445),
    "Gyllene Prag": (57.692369, 11.954439),
    "Yi Li": (57.719522, 12.026042),
    "Sylvain Marron": (57.677306, 11.928185),
}


def main(file, output):
    output_dir = os.path.dirname(output)
    os.makedirs(output_dir, exist_ok=True)

    year_ago = datetime.datetime.now() - datetime.timedelta(days=365)
    year_ago_str = year_ago.strftime("%Y-%m-%d")

    command = [
        "hledger",
        "register",
        "cur:SEK",
        "--value=then,SEK",
        "--output-format=csv",
        "--infer-market-prices",
        "expenses:Food:Restaurants & Cafes$",
        "expenses:Food:Lunch$",
        "expenses:Food:Eating Out$",
        f"--begin={year_ago_str}",
    ]

    if file:
        command.append(f"--file={file}")

    try:
        hledger_output = subprocess.check_output(command, stderr=subprocess.STDOUT)
    except subprocess.CalledProcessError as e:
        raise Exception(e.output.decode())

    rows = list(csv.DictReader(hledger_output.decode().splitlines()))

    entries = []
    for row in rows:
        amount = float(row["amount"].replace(" SEK", ""))
        payee = row["description"].split("|")[0].strip()
        entry = {"date": row["date"], "amount": amount, "payee": payee}
        entries.append(entry)

    entries_by_place = defaultdict(list)
    for entry in entries:
        entries_by_place[entry["payee"]].append(entry)

    places = []
    for payee, entries in entries_by_place.items():
        if len(entries) > 1 and payee in LOCATIONS:
            location = LOCATIONS[payee]
            place = {
                "location": location,
                "name": payee,
                "times": len(entries),
                "spent": sum(entry["amount"] for entry in entries),
            }
            places.append((payee, place))
        elif len(entries) > 1:
            raise Exception(f"Location for '{payee}' is missing")

    places.sort(key=lambda x: x[1]["times"], reverse=True)

    for _, place in places:
        output_file = os.path.join(output_dir, f"{slugify(place['name'])}.json")
        with open(output_file, "w") as f:
            json.dump(place, f, indent=4)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Scrape Letterboxd diary entries.")
    parser.add_argument("-f", "--file", help="Output directory")
    parser.add_argument(
        "-o", "--output", help="Output directory", default="./assets/places/"
    )
    args = parser.parse_args()

    main(args.file, args.output)
