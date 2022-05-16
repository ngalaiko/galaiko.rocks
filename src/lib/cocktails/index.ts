import Fraction from 'fraction.js';

const one = new Fraction(1);
const two = one.mul(2);
const three = one.mul(3);
const six = one.mul(6);
const ten = one.mul(10);
const oneAndHalfParts = three.div(two);
const third = one.div(three);
const quarter = one.div(4);
const half = one.div(two);
const dash = new Fraction(1, 32);

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

const coupe = {
	title: 'Coupe'
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

const coffeeLiqueur = {
	title: 'Coffee Liqueur'
};

const lowball = {
	title: 'Lowball'
};

type Ingridient = {
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

const hendriksGin = {
	title: "Hendrik's Gin"
};

const gin = {
	title: 'Gin'
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

const orangeTwist = {
	title: 'Orange Twist'
};

const lemonTwist = {
	title: 'Lemon Twist'
};

const lemonWheel = {
	title: 'Lemon Wheel'
};

const orangeWheel = {
	title: 'Orange Wheel'
};

const lemonPeel = {
	title: 'Lemon Peel'
};

const simpleSyrup = {
	title: 'Simple Syrup'
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
	// first level means and, second level means or
	ingridients?: ([Ingridient, Fraction][] | [Ingridient, Fraction])[];
	glass?: Glass;
	ice?: Ice;
};

type Ice = 'cubes' | 'large cube';

const stir = (...ingridients: ([Ingridient, Fraction] | [Ingridient, Fraction][])[]) => ({
	title: 'Stir',
	ingridients
});

const muddle = (...ingridients: ([Ingridient, Fraction] | [Ingridient, Fraction][])[]) => ({
	title: 'Muddle',
	ingridients
});

const dryShake = (...ingridients: ([Ingridient, Fraction] | [Ingridient, Fraction][])[]) => ({
	title: 'Dry Shake',
	ingridients
});

const shake = (...ingridients: ([Ingridient, Fraction] | [Ingridient, Fraction][])[]) => ({
	title: 'Shake',
	ingridients
});

const garnish = (...ingridients: ([Ingridient, Fraction] | [Ingridient, Fraction][])[]) => ({
	title: 'Garnish',
	ingridients
});

const doubleStrain = ({ glass, ice }: { glass: Glass; ice?: Ice }) => ({
	title: 'Double Strain',
	glass,
	ice
});

const strain = ({ glass, ice }: { glass: Glass; ice?: Ice }) => ({
	title: 'Strain',
	glass,
	ice
});

const topUp = (...ingridients: ([Ingridient, Fraction] | [Ingridient, Fraction][])[]) => ({
	title: 'Top Up',
	ingridients
});

const list: Cocktail[] = [
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
			stir(
				[londonDryGin, oneAndHalfParts],
				[sweetVermouth, oneAndHalfParts],
				[orangeBitter, dash.mul(2)]
			),
			strain({ glass: cocktail }),
			garnish([orangePeel, one])
		]
	},
	{
		title: 'Rogue 75',
		instructions: [
			muddle([lemonTwist, one], [raspberries, two], [simpleSyrup, one]),
			shake([gin, oneAndHalfParts]),
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
			shake([gin, oneAndHalfParts], [freshLemonJuice, half], [simpleSyrup, half]),
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
				[gin, oneAndHalfParts],
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
				[gin, oneAndHalfParts],
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
	}
];

export default list;
