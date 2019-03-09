#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate serde;

use std::fs;

fn main() {
    let filename = "retrosheet-events.yaml";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
 
    //println!("contents: {}", contents);
    let db_yaml: DBYaml = serde_yaml::from_str(contents.as_str()).unwrap();
    println!("{:?}", db_yaml);
    println!("# of tables: {}", db_yaml.tables.len())
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct DBYaml {
    dbname: String,
    tables: Vec<TableDesc>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct TableDesc {
    name: String,
    primarykey: String,
    foreignkeys: Vec<ForeignKey>,
    columns: Vec<ColumnDesc>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ForeignKey {
    table: String,
    column: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ColumnDesc {
    name: String,
    datatype: String,
    size: String,
    nullable: bool,
}