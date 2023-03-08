use extendr_api::prelude::*;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}

#[extendr]
fn is_positive(robj: Robj) -> Result<Robj> {
    use extendr_api::prelude::Rtype;
    match robj.rtype() {
        Rtype::Doubles => {
            let doubles: Doubles = robj.try_into().unwrap();
            let bool_vector_robj = doubles
                .iter()
                .map(|rfloat: Rfloat| match &rfloat {
                    _ if rfloat.is_na() => None,
                    _ if rfloat.0 > 0.0 => Some(true),
                    _ => Some(false),
                })
                .collect_robj();
            Ok(bool_vector_robj)
        }
        Rtype::Integers => {
            let doubles: Integers = robj.try_into().unwrap();
            let bool_vector_robj = doubles
                .iter()
                .map(|rint: Rint| match &rint {
                    _ if rint.is_na() => None,
                    _ if rint.0 > 0 => Some(true),
                    _ => Some(false),
                })
                .collect_robj();
            Ok(bool_vector_robj)
        }
        _ => Err(format!("this input type is not yet supported: {:?}", robj))
            .map_err(|err| extendr_api::Error::TypeMismatch(err.into())),
    }
}

#[extendr]
fn all_positive(robj: Robj) -> Rbool {
    match robj.rtype() {
        Rtype::Integers => {
            let integers = Integers::try_from(robj).unwrap();
            let bool = integers.iter().any(|x| x.is_na());
            if bool {
                return Rbool::na_value();
            }
            let bool = integers.iter().all(|x| x.0 > 0);
            Rbool::from_bool(bool)
        }
        Rtype::Doubles => {
            let doubles = Doubles::try_from(robj).unwrap();
            let bool = doubles.iter().any(|x| x.is_na());
            if bool {
                return Rbool::na_value();
            }
            let bool = doubles.iter().all(|x| x.0 > 0.);
            Rbool::from_bool(bool)
        }
        _ => panic!(),
    }
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod helloextendr;
    fn hello_world;
    fn is_positive;
    fn all_positive;
}
