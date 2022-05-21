const isInt = (value: number) => value % 1 === 0;

export type Unit = 'parts' | 'number' | 'undefined';

export class Fraction {
	private readonly numerator: number;
	private readonly denominator: number;

	public constructor(numerator: number, denominator?: number) {
		this.numerator = numerator;
		this.denominator = denominator ?? 1;
	}

	public compare(target: Fraction | number) {
		if (target instanceof Fraction) {
			return this.numerator / this.denominator - target.numerator / target.denominator;
		} else {
			return this.numerator / this.denominator - target;
		}
	}

	public mul(target: Fraction | number) {
		if (target instanceof Fraction) {
			return new Fraction(this.numerator * target.numerator, this.denominator * target.denominator);
		} else {
			return new Fraction(this.numerator * target, this.denominator);
		}
	}

	public div(target: Fraction | number) {
		if (target instanceof Fraction) {
			return new Fraction(this.numerator * target.denominator, this.denominator * target.numerator);
		} else {
			return new Fraction(this.numerator, this.denominator * target);
		}
	}

	public add(target: Fraction | number) {
		if (target instanceof Fraction) {
			return new Fraction(
				this.numerator * target.denominator + this.denominator * target.numerator,
				this.denominator * target.denominator
			);
		} else {
			return new Fraction(this.numerator + target * this.denominator, this.denominator);
		}
	}

	public toString(unit: Unit) {
		switch (unit) {
			case 'undefined':
				return '';
			case 'parts':
				if (isInt(this.div(one).toNumber())) {
					return `${this.mul(30).toNumber()}ml`;
				} else if (isInt(this.div(third).toNumber())) {
					return `${this.mul(30).toNumber()}ml`;
				} else if (isInt(this.div(half).toNumber())) {
					return `${this.mul(30).toNumber()}ml`;
				} else if (isInt(this.div(quarter).toNumber())) {
					return `${this.mul(30).toNumber()}ml`;
				} else if (isInt(this.div(barspoon).toNumber())) {
					const number = this.div(barspoon).toNumber();
					return number == 1 ? `Single barspoon` : `${number}`;
				} else if (isInt(this.div(dash).toNumber())) {
					const number = this.div(dash).toNumber();
					return number == 1 ? `Single dash` : `${number} dashes`;
				} else {
					throw new Error(`Unsupported ${this} ${unit}`);
				}
			case 'number':
				if (isInt(this.div(one).toNumber())) {
					const number = this.div(one).toNumber();
					return number == 1 ? `One` : `${number}`;
				} else if (isInt(this.div(half).toNumber())) {
					const number = this.div(half).toNumber();
					return number == 1 ? `Half of` : `${number}`;
				} else if (isInt(this.div(quarter).toNumber())) {
					const number = this.div(quarter).toNumber();
					return number == 1 ? `Quarter of` : `${number}`;
				} else {
					throw new Error(`Unsupported ${this} ${unit}`);
				}
			default:
				throw new Error(`Unsupported ${this} ${unit}`);
		}
	}

	public toNumber() {
		return this.numerator / this.denominator;
	}
}

export const one = new Fraction(1);
export const two = one.mul(2);
export const three = one.mul(3);
export const four = one.mul(4);
export const five = one.mul(5);
export const six = one.mul(6);
export const eight = one.mul(8);
export const ten = one.mul(10);
export const oneAndHalf = three.div(two);
export const third = one.div(3);
export const quarter = one.div(4);
export const half = one.div(two);
export const dash = new Fraction(1, 32);
export const barspoon = one.div(six);
