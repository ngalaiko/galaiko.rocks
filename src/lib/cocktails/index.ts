import {
	type Fraction,
	dash,
	barspoon,
	quarter,
	half,
	third,
	one,
	oneAndHalf,
	two,
	three,
	four,
	five,
	six,
	eight,
	ten
} from './fraction';

import {
	nickAndNora,
	flute,
	cocktail,
	oldFashioned,
	doubleOldFashioned,
	copper,
	hurricane,
	coupe,
	tiki,
	highball,
	lowball
} from './glasses';

import {
	type Ingredient,
	iceCube,
	sugarCube,
	bourbon,
	ryeWhiskey,
	darkRum,
	lightRum,
	coconutCream,
	heavyCream,
	pineappleJuice,
	coffeeLiqueur,
	vodka,
	tripleSec,
	freshLimeJuice,
	orangePeel,
	orangeBitter,
	dryVermouth,
	londonDryGin,
	olive,
	pickledOnion,
	sweetVermouth,
	lemonTwist,
	raspberries,
	simpleSyrup,
	gin,
	champagne,
	freshLemonJuice,
	eggWhite,
	clubSoda,
	cucumberSlice,
	hendriksGin,
	mintLeaf,
	angosturaBitters,
	campari,
	lemonWheel,
	orangeTwist,
	orangeWheel,
	orangeLiqueur,
	maraschinoCherry,
	freshOrangeJuice,
	gratedNutmeg,
	pineappleWedge,
	tequila,
	cola,
	cognac,
	cherry,
	lime,
	gingerBeer,
	limeWedge,
	tonic,
	lemonWedge,
	agaveSyrup,
	sugarRim
} from './ingredients';
import {
	type Instruction,
	shake,
	strain,
	dryShake,
	dump,
	stir,
	garnish,
	muddle,
	topUp,
	doubleStrain,
	blend,
	fill,
	squeeze,
	roll,
	type Measurement
} from './instruction';

type Cocktail = {
	title: string;
	instructions: Instruction[];
	note?: string;
};

export const list: Cocktail[] = [
	{
		title: 'Kamikadze',
		instructions: [
			shake([vodka, one], [tripleSec, one], [freshLimeJuice, one]),
			strain({ glass: nickAndNora })
		]
	},
	{
		title: 'Gin Martini',
		instructions: [
			stir([londonDryGin, two], [dryVermouth, one], [orangeBitter, dash.mul(2)]),
			strain({ glass: cocktail }),
			garnish([
				[olive, one],
				[olive, three]
			])
		],
		note: 'Make sure glass is ice cold'
	},
	{
		title: 'Gibson',
		instructions: [
			stir([londonDryGin, two], [dryVermouth, one]),
			strain({ glass: cocktail }),
			garnish([pickledOnion, three])
		],
		note: 'Make sure glass is ice cold'
	},
	{
		title: 'Artillery',
		instructions: [
			stir([londonDryGin, oneAndHalf], [sweetVermouth, oneAndHalf], [orangeBitter, dash.mul(2)]),
			strain({ glass: cocktail }),
			garnish([orangePeel, one])
		]
	},
	{
		title: 'Rogue 75',
		instructions: [
			muddle([lemonTwist, one], [raspberries, two], [simpleSyrup, one]),
			shake([gin, oneAndHalf]),
			strain({ glass: nickAndNora }),
			topUp([champagne, one]),
			garnish([lemonTwist, one])
		]
	},
	{
		title: 'Gin Fizz',
		instructions: [
			dryShake(
				[londonDryGin, two],
				[freshLemonJuice, one],
				[simpleSyrup, quarter.mul(3)],
				[eggWhite, half]
			),
			strain({ glass: lowball, ice: 'cubes' }),
			topUp([clubSoda, one])
		]
	},
	{
		title: 'Frensh 75',
		instructions: [
			shake([gin, oneAndHalf], [freshLemonJuice, half], [simpleSyrup, half]),
			strain({ glass: flute }),
			topUp([champagne, three])
		]
	},
	{
		title: 'East Side',
		instructions: [
			muddle([cucumberSlice, three], [simpleSyrup, quarter.mul(3)], [mintLeaf, six]),
			shake([hendriksGin, two], [freshLimeJuice, one]),
			doubleStrain({ glass: nickAndNora }),
			garnish([mintLeaf, one])
		]
	},
	{
		title: 'South side',
		instructions: [
			shake([gin, two], [freshLemonJuice, one], [simpleSyrup, half], [mintLeaf, six]),
			doubleStrain({ glass: nickAndNora }),
			garnish([mintLeaf, one])
		]
	},
	{
		title: 'Chicago South Side',
		instructions: [
			shake(
				[gin, oneAndHalf],
				[freshLimeJuice, quarter.mul(3)],
				[simpleSyrup, quarter.mul(3)],
				[angosturaBitters, dash.mul(7)],
				[mintLeaf, ten]
			),
			doubleStrain({ glass: nickAndNora }),
			garnish([mintLeaf, one])
		]
	},
	{
		title: 'Gin Campari Sour',
		instructions: [
			dryShake(
				[gin, oneAndHalf],
				[campari, quarter.mul(3)],
				[freshLemonJuice, quarter.mul(3)],
				[simpleSyrup, half],
				[orangeBitter, dash],
				[eggWhite, half]
			),
			strain({ glass: coupe }),
			garnish([
				[lemonWheel, one],
				[orangeWheel, one]
			])
		]
	},
	{
		title: 'White Lady',
		instructions: [
			dryShake([gin, two], [orangeLiqueur, half], [freshLemonJuice, half], [eggWhite, half]),
			strain({ glass: coupe })
		]
	},
	{
		title: 'Whiskey Sour',
		instructions: [
			shake(
				[bourbon, two],
				[freshLemonJuice, one],
				[simpleSyrup, half],
				[orangeBitter, dash.mul(2)],
				[eggWhite, half]
			),
			strain({ glass: oldFashioned, ice: 'large cube' }),
			garnish([orangePeel, one], [maraschinoCherry, one])
		]
	},
	{
		title: 'Old Fashioned',
		instructions: [
			stir(
				[bourbon, two],
				[simpleSyrup, quarter],
				[angosturaBitters, dash.mul(2)],
				[orangeBitter, dash.mul(2)]
			),
			strain({ glass: oldFashioned, ice: 'large cube' }),
			garnish([orangePeel, one])
		]
	},
	{
		title: 'Boulevandier',
		instructions: [
			stir([ryeWhiskey, one.add(quarter)], [campari, one], [sweetVermouth, one]),
			strain({ glass: oldFashioned }),
			garnish([orangePeel, one])
		],
		note: 'Make sure glass is ice cold'
	},
	{
		title: 'Perfect Manhattan',
		instructions: [
			stir(
				[ryeWhiskey, two],
				[sweetVermouth, half],
				[dryVermouth, half],
				[angosturaBitters, dash.mul(2)]
			),
			strain({ glass: cocktail }),
			garnish(
				[maraschinoCherry, one],
				[
					[lemonTwist, one],
					[orangeTwist, one]
				]
			)
		],
		note: 'Make sure glass is ice cold'
	},
	{
		title: 'Revolver',
		instructions: [
			stir([ryeWhiskey, two], [coffeeLiqueur, half], [orangeBitter, dash.mul(2)]),
			strain({ glass: nickAndNora })
		],
		note: "This drink needs the dilution and chill down time, don't skimp on the stir."
	},
	{
		title: 'Bamboo Screwdriver',
		instructions: [
			shake(
				[darkRum, one],
				[freshOrangeJuice, one],
				[orangeLiqueur, one],
				[freshLimeJuice, quarter],
				[angosturaBitters, dash.mul(4)]
			),
			strain({ glass: oldFashioned, ice: 'cubes' })
		]
	},
	{
		title: 'Piña Colada',
		instructions: [
			blend(
				[lightRum, two],
				[coconutCream, one],
				[heavyCream, one],
				[pineappleJuice, six],
				[iceCube, five]
			),
			strain({ glass: [tiki, highball] }),
			garnish([maraschinoCherry, one])
		]
	},
	{
		title: 'Painkiller',
		instructions: [
			shake([darkRum, two], [pineappleJuice, four], [freshOrangeJuice, one], [coconutCream, one]),
			strain({ glass: hurricane, ice: 'crushed' }),
			garnish([gratedNutmeg, undefined], [pineappleWedge, one])
		]
	},
	{
		title: 'Long Island Iced Tea',
		instructions: [
			fill({
				glass: highball,
				ice: 'cubes',
				ingridients: [
					[gin, half],
					[vodka, half],
					[tequila, half],
					[lightRum, half],
					[tripleSec, half],
					[freshLemonJuice, quarter.mul(3)],
					[simpleSyrup, quarter.mul(3)]
				]
			}),
			topUp([cola, undefined])
		]
	},
	{
		title: 'Champagne cocktail',
		instructions: [
			fill({
				glass: flute,
				ingridients: [
					[sugarCube, one],
					[angosturaBitters, dash.mul(2)]
				]
			}),
			topUp([cognac, third]),
			topUp([champagne, three]),
			garnish([lemonTwist, one], [cherry, one])
		]
	},
	{
		title: 'Frozen Margarita',
		instructions: [
			blend([tequila, two], [orangeLiqueur, quarter.mul(3)], [freshLimeJuice, one], [iceCube, five])
		]
	},
	{
		title: 'Moscow Mule',
		instructions: [
			fill({
				glass: copper,
				ice: 'crushed',
				ingridients: [
					[vodka, two],
					[freshLimeJuice, one]
				]
			}),
			squeeze([lime, quarter]),
			topUp([gingerBeer, four]),
			garnish([limeWedge, one])
		]
	},
	{
		title: 'Gin & Tonic',
		instructions: [
			fill({
				glass: highball,
				ice: 'cubes',
				ingridients: [[gin, two]]
			}),
			squeeze([lime, quarter]),
			garnish([lime, quarter]),
			topUp([tonic, five])
		]
	},
	{
		title: 'Whiskey Smash',
		instructions: [
			muddle([lemonWedge, four], [simpleSyrup, quarter.mul(3)]),
			muddle([mintLeaf, eight]),
			shake([bourbon, two]),
			dump({ glass: doubleOldFashioned, ice: 'cubes' })
		]
	},
	{
		title: 'Mojito',
		instructions: [
			fill({
				glass: highball,
				ice: 'cubes',
				ingridients: [
					[clubSoda, one],
					roll(
						[lightRum, two],
						muddle([limeWedge, four], [mintLeaf, eight], [simpleSyrup, barspoon.mul(2)])
					)
				]
			}),
			topUp([clubSoda, undefined])
		]
	},
	{
		title: 'Dulce De Tequila',
		instructions: [
			shake(
				[tequila, two],
				[cognac, one],
				[orangeLiqueur, one],
				[freshLimeJuice, half],
				[agaveSyrup, barspoon]
			),
			strain({ glass: cocktail }),
			garnish(
				[
					[limeWedge, one],
					[lemonWedge, one]
				],
				[sugarRim, undefined]
			)
		]
	}
];

const isMeasurement = (value: Measurement | Instruction) => value instanceof Array;

const instructionToIngredients = (instruction: Instruction): Measurement[] =>
	(instruction.ingridients ?? []).flatMap((ingridient) =>
		isMeasurement(ingridient)
			? [ingridient as Measurement]
			: instructionToIngredients(ingridient as Instruction)
	);

export const measurements = (cocktail: Cocktail): Measurement[] =>
	(cocktail.instructions ?? []).flatMap(instructionToIngredients);

const isDefinitiveMeasurement = (measurement: Measurement) => !(measurement[1] instanceof Array);

export const measurementToString = (measurement: Measurement): string => {
	if (!isDefinitiveMeasurement(measurement)) {
		return (measurement as [Ingredient, Fraction][]).map(measurementToString).join(' or ');
	}

	const [ingridient, amount] = measurement as [Ingredient, Fraction];
	if (amount === undefined) {
		return ingridient.title;
	}

	return amount.compare(one) > 0
		? `${amount.toString(ingridient.unit)} of ${ingridient.plural ?? ingridient.title}`
		: `${amount.toString(ingridient.unit)} of ${ingridient.title}`;
};
