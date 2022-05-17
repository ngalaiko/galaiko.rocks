import Fraction from '$lib/fraction';

const some: Fraction = new Fraction(0);
const one = new Fraction(1);
const two = one.mul(2);
const three = one.mul(3);
const four = one.mul(4);
const five = one.mul(5);
const six = one.mul(6);
const eight = one.mul(8);
const ten = one.mul(10);
const oneAndHalf = three.div(two);
const third = one.div(three);
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

const iceCube = {
	title: 'Ice cube'
};

const sugarCube = {
	title: 'Sugar cube'
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

const bourbon = {
	title: 'Bourbon'
};

const ryeWhiskey = {
	title: 'Rye Whiskey'
};

const darkRum = {
	title: 'Dark Rum'
};

const lightRum = {
	title: 'Light Rum'
};

const coconutCream = {
	title: 'Coconut Cream'
};

const heavyCream = {
	title: 'Heavy Cream'
};

const pineappleJuice = {
	title: 'Pineapple Juice'
};

const coffeeLiqueur = {
	title: 'Coffee Liqueur'
};

const lowball = {
	title: 'Lowball'
};

type Measurement = [Ingredient, Fraction] | Measurement[];

type Ingredient = {
	title: string;
};

const londonDryGin = {
	title: 'London Dry Gin'
};

const eggWhite = {
	title: 'Egg White'
};

const clubSoda = {
	title: 'Club Soda'
};

const dryVermouth = {
	title: 'Dry Vermouth'
};

const sweetVermouth = {
	title: 'Sweet Vermouth'
};

const orangeBitter = {
	title: 'Orange Bitters'
};

const angosturaBitters = {
	title: 'Angostura Bitters'
};

const olive = {
	title: 'Olive'
};

const vodka = {
	title: 'Vodka'
};

const gingerBeer = {
	title: 'Ginger Beer'
};

const cognac = {
	title: 'Cognac'
};

const tequila = {
	title: 'Tequila'
};

const hendriksGin = {
	title: "Hendrik's Gin"
};

const gin = {
	title: 'Gin'
};

const tonic = {
	title: 'Tonic'
};

const campari = {
	title: 'Campari'
};

const mintLeaf = {
	title: 'Meant Leaf'
};

const cucumberSlice = {
	title: 'Cucumber slice'
};

const champagne = {
	title: 'Champagne'
};

const tripleSec = {
	title: 'Triple Sec'
};

const orangeLiqueur = {
	title: 'Orange Liqueur'
};

const agaveSyrup = {
	title: 'Agave Syrup'
};

const freshOrangeJuice = {
	title: 'Fresh Orange Juice'
};

const freshLimeJuice = {
	title: 'Fresh Lime Juice'
};

const freshLemonJuice = {
	title: 'Fresh Lemon Juice'
};

const pickledOnion = {
	title: 'Pickled Onion'
};

const orangePeel = {
	title: 'Orange Peel'
};

const maraschinoCherry = {
	title: 'Maraschino Cherry'
};

const gratedNutmeg = {
	title: 'Grated Nutmeg'
};

const pineappleWedge = {
	title: 'Pineapple Wedge'
};

const orangeTwist = {
	title: 'Orange Twist'
};

const lime = {
	title: 'Lime'
};

const limeWedge = {
	title: 'Lime Wedge'
};

const lemonTwist = {
	title: 'Lemon Twist'
};

const lemonWedge = {
	title: 'Lemon Wedge'
};

const sugarRim = {
	title: 'Sugar Rim'
};

const cherry = {
	title: 'Cherry'
};

const lemonWheel = {
	title: 'Lemon Wheel'
};

const orangeWheel = {
	title: 'Orange Wheel'
};

const simpleSyrup = {
	title: 'Simple Syrup'
};

const cola = {
	title: 'Cola'
};

const raspberries = {
	title: 'Raspberry'
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
			garnish([[pickledOnion, three]])
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
			dryShake([londonDryGin, two], [freshLemonJuice, one], [simpleSyrup, third], [eggWhite, half]),
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
			muddle([cucumberSlice, three], [simpleSyrup, third], [mintLeaf, six]),
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
				[freshLimeJuice, third],
				[simpleSyrup, third],
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
				[campari, third],
				[freshLemonJuice, third],
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
			garnish([gratedNutmeg, some], [pineappleWedge, one])
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
					[freshLemonJuice, third],
					[simpleSyrup, third]
				]
			}),
			topUp([cola, some])
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
			blend([tequila, two], [orangeLiqueur, third], [freshLimeJuice, one], [iceCube, five])
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
			muddle([lemonWedge, four], [simpleSyrup, third]),
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
			topUp([clubSoda, some])
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
				[sugarRim, some]
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

const isDefinitiveMeasurement = (measurement: Measurement) =>
	measurement.length == 2 && measurement[1] instanceof Fraction;

export const measurementToString = (measurement: Measurement): string => {
	if (isDefinitiveMeasurement(measurement)) {
		const [ingridient, amount] = measurement as [Ingredient, Fraction];
		return `${ingridient.title} ${amount.toString()}`;
	}
	return (measurement as [Ingredient, Fraction][]).map(measurementToString).join(' or ');
};
