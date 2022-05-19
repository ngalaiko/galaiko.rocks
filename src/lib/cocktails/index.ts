import Fraction from '$lib/fraction';

const one = new Fraction(1);
const two = one.mul(2);
const three = one.mul(3);
const four = one.mul(4);
const five = one.mul(5);
const six = one.mul(6);
const eight = one.mul(8);
const ten = one.mul(10);
const oneAndHalf = three.div(two);
const sixth = one.div(six);
const quarter = one.div(4);
const half = one.div(two);
const dash = new Fraction(1, 32);
const barspoon = one.div(six);

export type Glass = {
	title: string;
};

const nickAndNora = {
	title: 'Nick & Nora'
};

const flute = {
	title: 'Champagne flute'
};

const cocktail = {
	title: 'Cocktail'
};
const oldFashioned = {
	title: 'Old Fashioned'
};

const doubleOldFashioned = {
	title: 'Double Old Fashioned'
};

const copper = {
	title: 'Copper'
};

const hurricane = {
	title: 'Hurricane'
};

const coupe = {
	title: 'Coupe'
};

const tiki = {
	title: 'Tiki'
};

const highball = {
	title: 'Highball'
};

const iceCube = {
	title: 'Ice cube',
	plural: 'Ice cubes',
	unit: 'number' as const
};

const sugarCube = {
	title: 'Sugar cube',
	plural: 'Sugar cubes',
	unit: 'number' as const
};

const bourbon = {
	title: 'Bourbon',
	unit: 'parts' as const
};

const ryeWhiskey = {
	title: 'Rye Whiskey',
	unit: 'parts' as const
};

const darkRum = {
	title: 'Dark Rum',
	unit: 'parts' as const
};

const lightRum = {
	title: 'Light Rum',
	unit: 'parts' as const
};

const coconutCream = {
	title: 'Coconut Cream',
	unit: 'parts' as const
};

const heavyCream = {
	title: 'Heavy Cream',
	unit: 'parts' as const
};

const pineappleJuice = {
	title: 'Pineapple Juice',
	unit: 'parts' as const
};

const coffeeLiqueur = {
	title: 'Coffee Liqueur',
	unit: 'parts' as const
};

const lowball = {
	title: 'Lowball'
};

type Measurement = [Ingredient, Fraction] | Measurement[];

type Unit = 'parts' | 'number' | 'undefined';

type Ingredient = {
	title: string;
	plural?: string;
	unit: Unit;
};

const londonDryGin = {
	title: 'London Dry Gin',
	unit: 'parts' as const
};

const eggWhite = {
	title: 'Egg Whites',
	unit: 'parts' as const
};

const clubSoda = {
	title: 'Club Soda',
	unit: 'parts' as const
};

const dryVermouth = {
	title: 'Dry Vermouth',
	unit: 'parts' as const
};

const sweetVermouth = {
	title: 'Sweet Vermouth',
	unit: 'parts' as const
};

const orangeBitter = {
	title: 'Orange Bitters',
	unit: 'parts' as const
};

const angosturaBitters = {
	title: 'Angostura Bitters',
	unit: 'parts' as const
};

const olive = {
	title: 'Olive',
	plural: 'Olives',
	unit: 'number' as const
};

const vodka = {
	title: 'Vodka',
	unit: 'parts' as const
};

const gingerBeer = {
	title: 'Ginger Beer',
	unit: 'parts' as const
};

const cognac = {
	title: 'Cognac',
	unit: 'parts' as const
};

const tequila = {
	title: 'Tequila',
	unit: 'parts' as const
};

const hendriksGin = {
	title: "Hendrik's Gin",
	unit: 'parts' as const
};

const gin = {
	title: 'Gin',
	unit: 'parts' as const
};

const tonic = {
	title: 'Tonic',
	unit: 'parts' as const
};

const campari = {
	title: 'Campari',
	unit: 'parts' as const
};

const mintLeaf = {
	title: 'Meant Leaf',
	plural: 'Meant Leaves',
	unit: 'number' as const
};

const cucumberSlice = {
	title: 'Cucumber slice',
	plural: 'Cucumber slices',
	unit: 'number' as const
};

const champagne = {
	title: 'Champagne',
	unit: 'parts' as const
};

const tripleSec = {
	title: 'Triple Sec',
	unit: 'parts' as const
};

const orangeLiqueur = {
	title: 'Orange Liqueur',
	unit: 'parts' as const
};

const agaveSyrup = {
	title: 'Agave Syrup',
	unit: 'parts' as const
};

const freshOrangeJuice = {
	title: 'Fresh Orange Juice',
	unit: 'parts' as const
};

const freshLimeJuice = {
	title: 'Fresh Lime Juice',
	unit: 'parts' as const
};

const freshLemonJuice = {
	title: 'Fresh Lemon Juice',
	unit: 'parts' as const
};

const pickledOnion = {
	title: 'Pickled Onion',
	plural: 'Pickled Onions',
	unit: 'number' as const
};

const orangePeel = {
	title: 'Orange Peel',
	plural: 'Orange Peels',
	unit: 'number' as const
};

const maraschinoCherry = {
	title: 'Maraschino Cherry',
	plural: 'Maraschino Cherries',
	unit: 'number' as const
};

const gratedNutmeg = {
	title: 'Grated Nutmeg',
	unit: 'undefined' as const
};

const pineappleWedge = {
	title: 'Pineapple Wedge',
	plural: 'Pineapple Wedges',
	unit: 'number' as const
};

const orangeTwist = {
	title: 'Orange Twist',
	plural: 'Orange Twists',
	unit: 'number' as const
};

const lime = {
	title: 'Lime',
	unit: 'number' as const
};

const limeWedge = {
	title: 'Lime Wedge',
	plural: 'Lime Wedges',
	unit: 'number' as const
};

const lemonTwist = {
	title: 'Lemon Twist',
	plural: 'Lemon Twists',
	unit: 'number' as const
};

const lemonWedge = {
	title: 'Lemon Wedge',
	plural: 'Lemon Wedges',
	unit: 'number' as const
};

const sugarRim = {
	title: 'Sugar Rim',
	unit: 'undefined' as const
};

const cherry = {
	title: 'Cherry',
	plural: 'Cherries',
	unit: 'number' as const
};

const lemonWheel = {
	title: 'Lemon Wheel',
	plural: 'Lemon Wheels',
	unit: 'number' as const
};

const orangeWheel = {
	title: 'Orange Wheel',
	plural: 'Orange Wheels',
	unit: 'number' as const
};

const simpleSyrup = {
	title: 'Simple Syrup',
	unit: 'parts' as const
};

const cola = {
	title: 'Cola',
	unit: 'parts' as const
};

const raspberries = {
	title: 'Raspberry',
	plural: 'Raspberries',
	unit: 'number' as const
};

type Cocktail = {
	title: string;
	instructions: Instruction[];
	note?: string;
};

type Instruction = {
	title: string;
	ingridients?: (Measurement | Instruction)[];
	glass?: Glass | Glass[];
	ice?: Ice;
};

type Ice = 'cubes' | 'large cube' | 'crushed';

const stir = (...ingridients: Measurement[]) => ({
	title: 'Stir',
	ingridients
});

const roll = (...ingridients: (Measurement | Instruction)[]) => ({
	title: 'Roll',
	ingridients
});

const muddle = (...ingridients: Measurement[]) => ({
	title: 'Muddle',
	ingridients
});

const dryShake = (...ingridients: Measurement[]) => ({
	title: 'Dry Shake',
	ingridients
});

const shake = (...ingridients: Measurement[]) => ({
	title: 'Shake',
	ingridients
});

const blend = (...ingridients: Measurement[]) => ({
	title: 'Blend',
	ingridients
});

const garnish = (...ingridients: Measurement[]) => ({
	title: 'Garnish',
	ingridients
});

const doubleStrain = ({ glass, ice }: { glass: Glass; ice?: Ice }) => ({
	title: 'Double Strain',
	glass,
	ice
});

const dump = ({ glass, ice }: { glass: Glass | Glass[]; ice?: Ice }) => ({
	title: 'Dump',
	glass,
	ice
});

const strain = ({ glass, ice }: { glass: Glass | Glass[]; ice?: Ice }) => ({
	title: 'Strain',
	glass,
	ice
});

const fill = ({
	glass,
	ice,
	ingridients
}: {
	glass: Glass;
	ice?: Ice;
	ingridients?: (Measurement | Instruction)[];
}) => ({
	title: 'Fill',
	glass,
	ice,
	ingridients
});

const squeeze = (...ingridients: Measurement[]) => ({
	title: 'Squeeze',
	ingridients
});

const topUp = (...ingridients: Measurement[]) => ({
	title: 'Top Up',
	ingridients
});

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
			topUp([cognac, sixth]),
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

const isInt = (value: number) => value % 1 === 0;

export const measurementToString = (measurement: Measurement): string => {
	if (isDefinitiveMeasurement(measurement)) {
		const [ingridient, amount] = measurement as [Ingredient, Fraction];
		if (amount === undefined) {
			return ingridient.title;
		}
		switch (ingridient.unit) {
			case 'undefined':
				throw new Error(`Expected undefined amount for ${ingridient}, got ${amount}`);
			case 'parts':
				if (isInt(amount.div(one).toNumber())) {
					return `${amount.mul(30).toNumber()}ml of ${ingridient.title}`;
				} else if (isInt(amount.div(half).toNumber())) {
					return `${amount.mul(30).toNumber()}ml of ${ingridient.title}`;
				} else if (isInt(amount.div(quarter).toNumber())) {
					return `${amount.mul(30).toNumber()}ml of ${ingridient.title}`;
				} else if (isInt(amount.div(barspoon).toNumber())) {
					const number = amount.div(barspoon).toNumber();
					return number == 1
						? `Single barspoon of ${ingridient.title}`
						: `${amount.div(barspoon).toNumber()} barspoons of ${ingridient.title}`;
				} else if (isInt(amount.div(dash).toNumber())) {
					const number = amount.div(dash).toNumber();
					return number == 1
						? `Single dash of ${ingridient.title}`
						: `${amount.div(dash).toNumber()} dashes of ${ingridient.title}`;
				} else {
					throw new Error(`Unsupported measurement ${measurement}`);
				}
			case 'number':
				if (isInt(amount.div(one).toNumber())) {
					const number = amount.div(one).toNumber();
					return number == 1
						? `One ${ingridient.title}`
						: `${number} ${ingridient.plural ?? ingridient.title}`;
				} else if (isInt(amount.div(half).toNumber())) {
					const number = amount.div(half).toNumber();
					return number == 1 ? `Half of ${ingridient.title}` : `${number} ${ingridient.title}`;
				} else if (isInt(amount.div(quarter).toNumber())) {
					const number = amount.div(quarter).toNumber();
					return number == 1 ? `Quarter of ${ingridient.title}` : `${number} ${ingridient.title}`;
				} else {
					throw new Error(`Unsupported measurement ${measurement}`);
				}
		}
	} else {
		return (measurement as [Ingredient, Fraction][]).map(measurementToString).join(' or ');
	}
};
