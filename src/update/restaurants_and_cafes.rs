use crate::restaurands_and_cafes::Place;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Hledger(String),
    De(csv::Error),
    Ser(serde_json::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(error) => write!(f, "{error}"),
            Error::Hledger(error) => write!(f, "{error}"),
            Error::Ser(error) => write!(f, "{error}"),
            Error::De(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug, serde::Deserialize)]
struct Row {
    date: chrono::NaiveDate,
    amount: String,
    description: String,
}

#[derive(Debug, Clone)]
struct Entry {
    date: chrono::NaiveDate,
    amount: f64,
    payee: String,
}

impl From<Row> for Entry {
    fn from(row: Row) -> Self {
        let amount = row.amount.strip_suffix(" SEK").expect("SEK suffix");
        let amount = amount.parse().expect("parse amount");
        let payee = row
            .description
            .split('|')
            .next()
            .unwrap_or(&row.description)
            .to_string();
        Self {
            date: row.date,
            amount,
            payee,
        }
    }
}

pub async fn update<P: AsRef<std::path::Path>>(file: Option<P>, output: P) -> Result<(), Error> {
    use std::collections::BTreeMap;
    use async_std::prelude::*;

    let year_ago = chrono::Local::now() - chrono::Duration::days(365);

    let mut command = async_std::process::Command::new("hledger");
    let mut command = command
        .arg("register")
        .arg("--value=then,SEK")
        .arg("--output-format=csv")
        .arg("expenses:Food:Restaurants & Cafes$")
        .arg(format!("--begin={}", year_ago.format("%Y-%m-%d")));

    if let Some(file) = file {
        command = command.arg(format!("--file={}", file.as_ref().display()));
    }

    let hledger_output = command.output().await.map_err(Error::Io)?;

    if !hledger_output.status.success() {
        let stderr = String::from_utf8_lossy(&hledger_output.stderr);
        return Err(Error::Hledger(stderr.to_string()));
    }

    let mut reader = csv::Reader::from_reader(&hledger_output.stdout[..]);
    let rows = reader
        .deserialize()
        .map(|result| result.map_err(Error::De))
        .collect::<Result<Vec<Row>, _>>()?;

    let entries = rows.into_iter().map(Entry::from).collect::<Vec<_>>();

    let entries_by_payee_and_date = entries
        .into_iter()
        .fold(BTreeMap::new(), |mut map, entry| {
            map.entry((entry.payee.clone(), entry.date))
                .and_modify(|e: &mut Entry| e.amount += entry.amount)
                .or_insert(entry);
            map
        })
        .into_values();

    let entries_by_place =
        entries_by_payee_and_date
            .into_iter()
            .fold(BTreeMap::new(), |mut map, entry| {
                map.entry(entry.payee.clone())
                    .and_modify(|entries: &mut Vec<Entry>| entries.push(entry.clone()))
                    .or_insert_with(|| vec![entry]);
                map
            });

    let mut places = entries_by_place
        .into_iter()
        .map(|(payee, entries)| Place {
            name: payee,
            times: entries.len().try_into().expect("usize to u8"),
            spent: entries.into_iter().map(|e| e.amount).sum(),
        })
        .filter(|place| place.times > 1)
        .collect::<Vec<_>>();
    places.sort_by(|a, b| b.times.cmp(&a.times));

    let mut outputs = vec![];
    for place in &places {
        let output = output.as_ref().join(format!("{}.json", place.name));

        async_std::fs::write(
            &output,
            serde_json::to_string_pretty(&place).map_err(Error::Ser)?,
        )
        .await
        .map_err(Error::Io)?;

        outputs.push(output);
    }

    let mut entries = async_std::fs::read_dir(output.as_ref())
        .await
        .map_err(Error::Io)?;

    while let Some(res) = entries.next().await {
        let entry = res.map_err(Error::Io)?;
        let path = std::path::PathBuf::from(entry.path().display().to_string());
        if !outputs.contains(&path) {
            async_std::fs::remove_file(entry.path())
                .await
                .map_err(Error::Io)?;
        }
    }

    Ok(())
}
