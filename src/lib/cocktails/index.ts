import type { Step, StepIngredient } from '@cooklang/cooklang-ts';

export type Ingredient = {
	name: string;
	quantity?: string;
};

export type Cocktail = {
	title: string;
	ingredients: Ingredient[];
	steps: string[];
};

const capitalize = (str: string) => str.charAt(0).toUpperCase() + str.slice(1);

export const list = () =>
	Promise.all(
		Object.entries(import.meta.glob('./**/*.cook')).map(
			async ([filename, module]): Promise<Cocktail> => {
				const m = (await module()).default;
				const title = filename.split('/').pop().replace('.cook', '');
				const ingredients = m.steps
					.flatMap((step: Step) => step.filter((s) => s.type === 'ingredient'))
					.map(
						(ingredient: StepIngredient): Ingredient => ({
							name: capitalize(ingredient.name),
							quantity:
								ingredient.quantity !== 1 && ingredient.quantity !== 'some'
									? `${ingredient.quantity}${ingredient.units}`
									: undefined
						})
					);
				const steps = m.steps.map((step: Step) =>
					step.reduce((acc, s) => {
						switch (s.type) {
							case 'ingredient':
								return `${acc}${s.name}`;
							case 'cookware':
								return `${acc}${s.name}`;
							case 'timer':
								return `${acc}${s.quantity} ${s.units}`;
							case 'text':
								return `${acc}${s.value}`;
						}
					}, '')
				);
				return { title, ingredients, steps };
			}
		)
	).then((cocktails) => cocktails.sort((a, b) => a.title.localeCompare(b.title)));
