---
title: "from hledger to ledger"
date: 2025-03-13
---

yesterday i have switched from hleder, that i have been using for over three yesrs to ledger.
all because of one new flow i have to model.

me and my wife are using the same credit card, which is linked to my bank account.
meaning every month i close the debt, and my wife transferres me some part of that debt.

which means when we have visited some cafe, and payed from a credit card, i will log it like that:

```
2025/03/08 Cafe Pupp
    expenses:Food:Eating Out              508.00 CZK @@ 224.72 SEK
    liabilities:shared:nordea:gold
```

what i really want is to log say 35% into my wife's liability

```
2025/03/08 Cafe Pupp
    expenses:Food:Eating Out              508.00 CZK @@ 224.72 SEK
    liabilities:shared:nordea:gold
    expenses:Food:Eating Out             -177.80 CZK @@ 78,65 SEK
    liabilities:wife                      78.65 SEK
```

i don't want to manually calculate that every time, so naturally i've reached out to automated postings.

that's where that problem starts. hedger doesn't support [references in auto-posting rules](https://github.com/simonmichael/hledger/issues/1975)

but ledger allows me to do this:

```
; transactions tagged with shared:: 35% will be shared with wife
; a - The posting’s amount; the balance of an account, without considering children.
; b - The cost of a posting; the cost of an account, without its children.
= expenses and %shared
	$account                                                           (a * -tag("shared") * 0.01)
	liabilities:wife                                                   (b *  tag("shared") * 0.01)
	equity:conversion:%(commodity(a))-%(commodity(b)):%(commodity(b))  (b * -tag("shared") * 0.01)
	equity:conversion:%(commodity(a))-%(commodity(b)):%(commodity(a))  (a *  tag("shared") * 0.01)

```

which does exactly what i want automatically if i tag transaction with new 'shared' tag like so:


```
2025/03/08 Cafe Pupp
    ; shared:: 35%
    expenses:Food:Eating Out              508.00 CZK @@ 224.72 SEK
    liabilities:shared:nordea:gold
```

which is perfect. 

also now with ledger i can finally properly track investment lots
