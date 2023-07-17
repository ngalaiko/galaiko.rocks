import places from './data.json';

export type Place = {
  payee: string;
  count: number;
  currency: string;
  amount: number;
};

export const list = async () => places;
