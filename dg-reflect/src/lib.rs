
#[derive(Debug)]
pub enum ReflectionType {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Bool,
    String,
    Container(&'static [Reflection]),
}

#[derive(Debug)]
pub struct Reflection {
    // index: usize,
    name: &'static str,
    ty: ReflectionType,
    // ty: ReflectionType,
}

#[allow(unused_variables)]
pub trait Reflect {
    fn get_reflections() -> &'static [Reflection];
}

#[cfg(test)]
mod tests {
    use crate::{Reflect, Reflection, ReflectionType};

    struct InnerThing {
        inside: String,
        also_inside: bool,
    }

    impl InnerThing {
        const REFLECTIONS: &'static [Reflection] = &[
            Reflection {
                name: "inside",
                ty: ReflectionType::String,
            },
            Reflection {
                name: "also_inside",
                ty: ReflectionType::Bool,
            },
        ];
    }

    impl Reflect for InnerThing {
        fn get_reflections() -> &'static [Reflection] {
            Self::REFLECTIONS
        }
    }


    struct ReflectTest {
        coolness: u32,
        sickness: String,
        amazing: bool,
        inner_thing: InnerThing,
    }

    impl ReflectTest {
        const REFLECTIONS: &'static [Reflection] = &[
            Reflection {
                name: "coolness",
                ty: ReflectionType::U32,
            },
            Reflection {
                name: "sickness",
                ty: ReflectionType::String,
            },
            Reflection {
                name: "amazing",
                ty: ReflectionType::Bool,
            },
            Reflection {
                name: "inner_thing",
                ty: ReflectionType::Container(InnerThing::REFLECTIONS),
            },
        ];
    }

    impl Reflect for ReflectTest {
        fn get_reflections() -> &'static [Reflection] {
            Self::REFLECTIONS
        }
    }

    #[test]
    fn print() {
        fn print_tree(depth: usize, refls: &[Reflection]) {
            for refl in refls {
                for _ in 0..depth {
                    print!("    ");
                }
                println!("{}", refl.name);

                match refl.ty {
                    ReflectionType::Container(container) => {
                        print_tree(depth + 1, container);
                    },
                    _ => {},
                }
            }
        }

        print_tree(0, ReflectTest::get_reflections());
    }
}