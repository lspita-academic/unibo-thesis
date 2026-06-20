#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    // This statement gets translated into:
    // let v = {
    //      let mut temp_vec = Vec::new();
    //      temp_vec.push(1);
    //      temp_vec.push(2);
    //      temp_vec.push(3);
    //      temp_vec
    // };
    let v = vec!(1, 2, 3);

    assert_eq!(v[0], 1);
    assert_eq!(v[1], 2);
    assert_eq!(v[2], 3);
}
