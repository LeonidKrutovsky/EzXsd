use syn::{GenericArgument, Path, PathArguments, Type};

pub fn unpack_generic_argument(arguments: &PathArguments) -> &Path {
    if let PathArguments::AngleBracketed(generic_args) = arguments {
        if let GenericArgument::Type(ty) = generic_args
            .args
            .first()
            .expect("Generic arguments must exists")
        {
            if let Type::Path(type_path) = ty {
                &type_path.path
            } else {
                unreachable!("Not Path generic argument")
            }
        } else {
            unreachable!("Generic Argument is not Type")
        }
    } else {
        unreachable!("Generic arguments must be angle bracketed")
    }
}
