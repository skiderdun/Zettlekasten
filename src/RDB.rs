// relational database manager (RDB)


// macro for creating a new relational database
macro_rules! new_rdb {
    ($name:ident) => {
        pub struct $name {
            pub name: String,
            pub path: String,
            pub tables: Vec<Table>,
        }
    };
}