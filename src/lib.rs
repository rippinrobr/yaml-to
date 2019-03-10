#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate serde;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DBYaml {
    dbname: String,
    tables: Vec<TableDesc>,
}

impl DBYaml {
    pub fn get_tables(self) -> Vec<TableDesc> {
        return self.tables;
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TableDesc {
    name: String,
    primarykey: String,
    foreignkeys: Vec<ForeignKey>,
    columns: Vec<ColumnDesc>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ForeignKey {
    table: String,
    column: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ColumnDesc {
    name: String,
    datatype: String,
    size: String,
    nullable: bool,
}

pub fn run(contents: String) -> Result<(), String> {
    let db_yaml: DBYaml = serde_yaml::from_str(contents.as_str()).unwrap();
    
    println!("{:?}", db_yaml);
    println!("# of tables: {}", db_yaml.get_tables().len());

    Ok(())
}