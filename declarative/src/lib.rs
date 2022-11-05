#[macro_export]
macro_rules! vivek {
    ($($x:expr), *) => {
        {
            let mut temp_map = HashMap::new();
            $(
                temp_map.insert($x.0, $x.1);
            )*
            temp_map
        }
    };

    ($key:ty, $val:ty) => {{
        let map: HashMap<$key, $val> = HashMap::new();
        map
    }};

    ($($x:expr => $y:expr), *) => {
                {
            let mut temp_map = HashMap::new();
            $(
                temp_map.insert($x, $y);
            )*
            temp_map
        }
    }
}

#[macro_export]
macro_rules! vector_of_tuples {
    ($key:ty, $val:ty) => {{
        let map: Vec<($key, $val)> = Vec::new();
        map
    }};
}
