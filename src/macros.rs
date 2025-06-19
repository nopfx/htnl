macro_rules! Contextable {
    (struct $name:ident { $($fname:ident : $ftype:ty),* $(,)? }) => {
        struct $name {
            $($fname : $ftype),*
        }

        impl $name {
            fn flatten(&self) -> std::collections::HashMap<String, String> {
                let mut map = std::collections::HashMap::new();
                $(
                    map.insert(
                        format!("{}.{}", stringify!($name), stringify!($fname)),
                        self.$fname.to_string()
                    );
                )*
                map
            }
        }
    }
}
