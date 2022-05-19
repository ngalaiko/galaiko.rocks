export default class Fraction {
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

	public toString() {
		return `${this.numerator}/${this.denominator}`;
	}

	public toNumber() {
		return this.numerator / this.denominator;
	}
}
