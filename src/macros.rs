macro_rules! generate_builder_method {
    // Build a plain builder method, with a real inner structure, a name of the function/parameter,
    // and a type of the parameter
    (
        $(#[$meta:meta])*
        plain INNER($inner:ident) $name:ident($t:ty)
    ) => {
        $(#[$meta])*
        pub fn $name(mut self, $name: $t) -> Self {
            self.$inner.$name = $name;

            self
        }
    };

    // Build an intoiter builder method for ConfigBuilder, with a real inner structure, a name of
    // the function/parameter, a generic name, and a type of the expected parameter
    (
        $(#[$meta:meta])*
        into INNER($inner:ident) $name:ident($t:ident: $into:ty)
    ) => {
        $(#[$meta])*
        pub fn $name<$t>(mut self, $name: $t) -> Self
        where
            $t: Into<$into>,
        {
            self.$inner.$name = $name.into();

            self
        }
    };

    // Build an optional builder method for ConfigBuilder, with a real inner structure, a name of
    // the function/parameter, a generic name, and a type of the expected parameter
    (
        $(#[$meta:meta])*
        option INNER($inner:ident) $name:ident($t:ident: $into:ty)
    ) => {
        $(#[$meta])*
        pub fn $name<$t: Into<$into>>(mut self, $name: $t) -> Self {
            self.$inner.$name = Some($name.into());

            self
        }
    };

    // Build a token builder method for EntryBuilder, with a real inner structure, a name of
    // the function/parameter, a generic name, and a type of the expected parameter
    (
        $(#[$meta:meta])*
        token $parent:ident::$token:ident INNER($inner:ident) $name:ident($t:ident: $into:ty)
    ) => {
        $(#[$meta])*
        pub fn $name<$t: Into<$into>>(mut self, $name: $t) -> Self {
            self.$inner.tokens.push($parent::$token($name.into()));

            self
        }
    };
}

pub(crate) use generate_builder_method;
