use duchess::prelude::*;

duchess::java_package! {
    package passing_collections;

    class Arrays { * }
}

#[test]
fn pass_an_array() {
    let arrays = passing_collections::Arrays::new().global().execute();
}
