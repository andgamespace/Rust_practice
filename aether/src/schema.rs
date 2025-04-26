// the schema that all recipes are to follow

pub mod schema {
    pub struct Package {
        pub name: String,
        pub version: String,
        pub hash: String,
        dependencies: Vec<String>, //List of all dependencies that a pa
    }
}
