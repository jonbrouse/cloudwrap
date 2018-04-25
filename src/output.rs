use chrono::NaiveDateTime;
use prettytable::{format, Table};
use rusoto_ssm::{Parameter, ParameterMetadata};

pub trait Printable {
    fn get_table(&self) -> Table;
    fn export(&self) -> Option<Vec<(String, String)>>;
}

impl Printable for Vec<Parameter> {
    fn get_table(&self) -> Table {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        table.set_titles(row!["KEY", "VALUE"]);

        for p in self.into_iter() {
            let key = p.name.clone().unwrap();
            let key: String = key.split("/").collect::<Vec<&str>>().pop().unwrap().into();
            let value = p.value.clone().unwrap();

            table.add_row(row![key, value]);
        }

        table
    }

    fn export(&self) -> Option<Vec<(String, String)>> {
        let mut pairs = Vec::new();

        for p in self.into_iter() {
            let key = p.name.clone().unwrap();
            let key: String = key.split("/").collect::<Vec<&str>>().pop().unwrap().into();
            let key = key.to_uppercase().replace("-", "_");
            let value = p.value.clone().unwrap();

            pairs.push((key.into(), value.into()));
        }

        Some(pairs)
    }
}

impl Printable for Vec<ParameterMetadata> {
    fn get_table(&self) -> Table {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        table.set_titles(row![c => "KEY", "VERSION", "LAST_MODIFIED_USER", "LAST_MODIFIED_DATE"]);

        for p in self.into_iter() {
            let key = p.name.clone().unwrap();
            let key: String = key.split("/").collect::<Vec<&str>>().pop().unwrap().into();
            let user = p.last_modified_user.clone().unwrap();
            let user: String = user.split("/").collect::<Vec<&str>>().pop().unwrap().into();
            let version = p.version.unwrap_or(0);
            let date = p.last_modified_date.unwrap();
            let date = NaiveDateTime::from_timestamp(date.floor() as i64, 0);

            table.add_row(row![r => key, format!("{}", version), user, date]);
        }

        table
    }

    fn export(&self) -> Option<Vec<(String, String)>> {
        None
    }
}
