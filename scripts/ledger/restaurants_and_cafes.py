#!/usr/bin/python3

import subprocess
import csv
import os
from datetime import date, timedelta
from decimal import Decimal, getcontext
import argparse
import json

parser = argparse.ArgumentParser()
parser.add_argument("-n", "--number", default=25)
parser.add_argument("-f", "--file", default="main.ledger")
args = parser.parse_args()
number = int(args.number)

getcontext().prec = 6

year_ago = date.today() - timedelta(days=365)

command=[
    'hledger',
    '-f', args.file,
    'register',
    'expenses:Food:Restaurants & Cafes$',
    '--value', 'then,SEK',
    '--output-format', 'csv',
    '--begin', year_ago.isoformat(),
]

output = subprocess.run(command,check=True, stdout=subprocess.PIPE, universal_newlines=True, encoding='utf-8')
transactions = csv.reader(output.stdout.splitlines()[1:])

def convert(row):
    return {
        'date': date.fromisoformat(row[1]),
        'payee': row[3].split('|')[0].strip(),
        'amount': Decimal(row[5].split(' ')[0]),
        'currency': row[5].split(' ')[1]
    }

transactions = map(convert, transactions)

# calculate top 50 payees by amount sum, and print them like this as json:
# payee, total amount, currency, number of transactions
def groupby(transactions, get_key):
    groups = {}
    for transaction in transactions:
        key = get_key(transaction)
        if key not in groups:
            groups[key] = {
                'payee': transaction['payee'],
                'amount': Decimal(0),
                'currency': transaction['currency'],
                'count': 0
            }
        groups[key]['amount'] += transaction['amount']
        groups[key]['count'] += 1
    return groups

groups = groupby(transactions, lambda t: t['payee'])

result = map(lambda key: groups[key] , sorted(groups, key=lambda g: groups[g]['count'], reverse=True)[:number])

class DecimalEncoder(json.JSONEncoder):
  def default(self, obj):
    if isinstance(obj, Decimal):
      return format(obj, "2,f").replace(',', ' ').replace('.', ',')
      
    return json.JSONEncoder.default(self, obj)

script_location = os.path.realpath(__file__)
script_dir = os.path.dirname(script_location)
dst = os.path.join(script_dir, "../../data/restaurants_and_cafes.json")
dst = os.path.normpath(dst)
content = json.dumps(list(result), indent=4, cls=DecimalEncoder, ensure_ascii=False )
with open(dst, 'w') as f:
    f.write(content)
