#[macro_export]
macro_rules! generate_builder_method {
    // Build a plain builder method, with a real inner structure, a name of the function/parameter,
    // and a type of the parameter
    (
        $(#[$meta:meta])*
        plain REAL($real:ident) $name:ident($t:ty)
    ) => {
        $(#[$meta])*
        pub fn $name(mut self, $name: $t) -> Self {
            self.$real.$name = $name;

            self
        }
    };

    // Build an intoiter builder method for ConfigBuilder, with a real inner structure, a name of
    // the function/parameter, a generic name, and a type of the expected parameter
    (
        $(#[$meta:meta])*
        intoiter REAL($real:ident) $name:ident($t:ident => $into:ty)
    ) => {
        $(#[$meta])*
        pub fn $name<$t>(mut self, $name: $t) -> Self
        where
            $t: IntoIterator<Item = $into>,
        {
            self.$real.$name = $name.into_iter().collect();

            self
        }
    };

    // Build an optional builder method for ConfigBuilder, with a real inner structure, a name of
    // the function/parameter, a generic name, and a type of the expected parameter
    (
        $(#[$meta:meta])*
        option REAL($real:ident) $name:ident($t:ident => $into:ty)
    ) => {
        $(#[$meta])*
        pub fn $name<$t: Into<$into>>(mut self, $name: $t) -> Self {
            self.$real.$name = Some($name.into());

            self
        }
    };

    // Build a token builder method for EntryBuilder, with a real inner structure, a name of
    // the function/parameter, a generic name, and a type of the expected parameter
    (
        $(#[$meta:meta])*
        token => $token:ident REAL($real:ident) $name:ident($t:ident => $into:ty)
    ) => {
        $(#[$meta])*
        pub fn $name<$t: Into<$into>>(mut self, $name: $t) -> Self {
            self.$real.tokens.push(Token::$token($name.into()));

            self
        }
    };
}
