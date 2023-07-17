const digits = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

const operations = {
  '+': (a: number, b: number) => a + b,
  '-': (a: number, b: number) => a - b,
  '×': (a: number, b: number) => a * b
};

const random = <T>(items: T[]) => items[Math.floor(Math.random() * items.length)];

export const generate = () => {
  let a = random(digits);
  let b = random(digits);
  if (b > a) {
    a = a + b;
    b = a - b;
    a = a - b;
  }
  return `${a} ${random(Object.keys(operations))} ${b}`;
};

export const solve = (challange: string) => {
  const [a, op, b] = challange.split(' ');
  return `${operations[op](parseInt(a), parseInt(b))}`;
};
