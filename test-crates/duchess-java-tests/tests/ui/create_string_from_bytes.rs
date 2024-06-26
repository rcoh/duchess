//@ run

use duchess::{java, prelude::*, Global, IntoRust};

fn main() -> duchess::GlobalResult<()> {
    let v = vec!['H' as u8, 'i' as u8];

    let n: Global<java::lang::String> = java::lang::String::new(v.to_java()).global().execute()?;

    let n: String = n.to_rust().execute()?;

    assert_eq!(&n[..], "Hi");

    Ok(())
}
