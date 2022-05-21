import type { Fraction } from './fraction';
import type { Glass } from './glasses';
import type { Ingredient } from './ingredients';

export type Measurement = [Ingredient, Fraction] | Measurement[];

export type Instruction = {
	title: string;
	ingridients?: (Measurement | Instruction)[];
	glass?: Glass | Glass[];
	ice?: Ice;
};

export type Ice = 'cubes' | 'large cube' | 'crushed';

export const stir = (...ingridients: Measurement[]) => ({
	title: 'Stir',
	ingridients
});

export const roll = (...ingridients: (Measurement | Instruction)[]) => ({
	title: 'Roll',
	ingridients
});

export const muddle = (...ingridients: Measurement[]) => ({
	title: 'Muddle',
	ingridients
});

export const dryShake = (...ingridients: Measurement[]) => ({
	title: 'Dry Shake',
	ingridients
});

export const shake = (...ingridients: Measurement[]) => ({
	title: 'Shake',
	ingridients
});

export const blend = (...ingridients: Measurement[]) => ({
	title: 'Blend',
	ingridients
});

export const garnish = (...ingridients: Measurement[]) => ({
	title: 'Garnish',
	ingridients
});

export const doubleStrain = ({ glass, ice }: { glass: Glass; ice?: Ice }) => ({
	title: 'Double Strain',
	glass,
	ice
});

export const dump = ({ glass, ice }: { glass: Glass | Glass[]; ice?: Ice }) => ({
	title: 'Dump',
	glass,
	ice
});

export const strain = ({ glass, ice }: { glass: Glass | Glass[]; ice?: Ice }) => ({
	title: 'Strain',
	glass,
	ice
});

export const fill = ({
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

export const squeeze = (...ingridients: Measurement[]) => ({
	title: 'Squeeze',
	ingridients
});

export const topUp = (...ingridients: Measurement[]) => ({
	title: 'Top Up',
	ingridients
});
