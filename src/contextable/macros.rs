#[macro_export]
macro_rules! context {
    (
        struct $name:ident {
            $(
                $fname:ident : Vec<$inner_ty:ty>
            ),* $(,)?
        }
    ) => {
        #[derive(Debug)]
        struct $name {
            $($fname : Vec<$inner_ty>),*
        }

        impl $crate::contextable::traits::Contextable for $name {
            fn flatten(&self) -> std::collections::HashMap<String, String> {
                let mut map = std::collections::HashMap::new();
                $(
                    for (i, item) in self.$fname.iter().enumerate() {
                        let sub_map = $crate::contextable::traits::Contextable::flatten(item);
                        for (k, v) in sub_map {
                            map.insert(format!("{}.{}[{}].{}", stringify!($name), stringify!($fname), i, k).trim_matches('"').to_string(), v);
                        }
                    }
                )*
                map
            }
        }
    };

    (
        struct $name:ident {
            $(
                $fname:ident : $ftype:ty
            ),* $(,)?
        }
    ) => {
        #[derive(Debug)]
        struct $name {
            $($fname : $ftype),*
        }

        impl $crate::contextable::traits::Contextable for $name {
            fn flatten(&self) -> std::collections::HashMap<String, String> {
                let mut map = std::collections::HashMap::new();
                $(
                    map.insert(
                        format!("{}.{}", stringify!($name), stringify!($fname)),
                        format!("{:?}", self.$fname).trim_matches('"').to_string()
                    );
                )*
                map
            }
        }
    };
}
