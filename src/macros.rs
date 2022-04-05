#[macro_export]
macro_rules! generate_builder_method {
    // Build a plain builder method, with a real inner structure, a name of the function/parameter,
    // and a type of the parameter
    (plain $real:ident, $name:ident, $t:ty) => {
        pub fn $name(mut self, $name: $t) -> Self {
            self.$real.$name = $name;

            self
        }
    };

    // Build an optional builder method for ConfigBuilder, with a real inner structure, a name of
    // the function/parameter, and a type of the parameter
    (option $name:ident, $t:ty) => {
        pub fn $name(mut self, $name: $t) -> Self {
            self.config.$name = Some($name.into());

            self
        }
    };

    // Build a token builder method for EntryBuilder, with a real inner structure, a name of
    // the function/parameter, and a type of the parameter
    (token $name:ident, $t:ty, $token:ident) => {
        pub fn $name(mut self, $name: $t) -> Self {
            self.entry.tokens.push(Token::$token($name.into()));

            self
        }
    };
}
