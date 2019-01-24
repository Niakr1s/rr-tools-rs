#[macro_export]
macro_rules! contur {
    ( $( $x:expr ),* ) => {
        {
            let mut temp = Contur::new();
            $(
                temp.push($x);
            )*
            temp
        }
    };
}