#[macro_export]
macro_rules! context {
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
                use $crate::contextable::traits::Contextable;
                let mut map = std::collections::HashMap::new();

                $(
                    let sub_map = self.$fname.flatten();
                    for (k, v) in sub_map {
                        if k.is_empty() {
                            map.insert(
                                format!("{}.{}", stringify!($name), stringify!($fname)),
                                v,
                            );
                        } else {
                            map.insert(
                                format!("{}.{}.{}", stringify!($name), stringify!($fname), k),
                                v,
                            );
                        }
                    }
                )*

                map
            }
        }
    };
}
