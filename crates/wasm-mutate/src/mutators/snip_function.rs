//! Mutator that replaces the body of a function with an empty body

use super::Mutator;
use crate::module::{PrimitiveTypeInfo, TypeInfo};
use crate::{ModuleInfo, Result, WasmMutate};
use rand::prelude::SmallRng;
use rand::{Rng, RngCore};
use wasm_encoder::{CodeSection, Export, ExportSection, Function, Instruction, Module};
use wasmparser::{CodeSectionReader, ExportSectionReader};

/// Mutator that replaces the body of a function with an empty body
pub struct SnipMutator;

impl Mutator for SnipMutator {
    fn mutate(
        &self,
        config: &WasmMutate,
        rnd: &mut SmallRng,
        info: &mut ModuleInfo,
    ) -> Result<Module> {
        let mut codes = CodeSection::new();
        let code_section = info.get_code_section();
        let mut reader = CodeSectionReader::new(code_section.data, 0)?;
        let count = reader.get_count();
        let function_to_mutate = rnd.gen_range(0, count);
        let ftype = info.get_functype_idx(function_to_mutate as usize);

        (0..count).for_each(|i| {
            if i == function_to_mutate {
                log::debug!("Snip function idx {:?}", function_to_mutate);
                let locals = vec![];
                let mut f = Function::new(locals);

                match ftype {
                    TypeInfo::Func(t) => {
                        t.returns.iter().for_each(|primitive| match primitive {
                            PrimitiveTypeInfo::I32 => {
                                f.instruction(Instruction::I32Const(0));
                            }
                            PrimitiveTypeInfo::I64 => {
                                f.instruction(Instruction::I64Const(0));
                            }
                            PrimitiveTypeInfo::F32 => {
                                f.instruction(Instruction::F32Const(0.0));
                            }
                            PrimitiveTypeInfo::F64 => {
                                f.instruction(Instruction::F64Const(0.0));
                            }
                        });
                    }
                    _ => panic!("Unconsistent function type"),
                };

                f.instruction(Instruction::End);

                codes.function(&f);
            } else {
                let f = reader.read().unwrap();
                codes.raw(&code_section.data[f.range().start..f.range().end]);
            }
        });
        Ok(info.replace_section(info.code.unwrap(), &codes))
    }

    fn can_mutate<'a>(&self, config: &'a WasmMutate, info: &ModuleInfo) -> Result<bool> {
        Ok(!config.preserve_semantics && info.has_code())
    }
}

#[cfg(test)]
mod tests {
    use crate::WasmMutate;
    use rand::{rngs::SmallRng, SeedableRng};

    use super::{Mutator, SnipMutator};

    #[test]
    fn test_code_snip_mutator() {
        crate::mutators::match_mutation(
            r#"
        (module
            (func (result i64)
                i64.const 42
            )
        )
        "#,
            &SnipMutator,
            r#"
        (module
            (type (;0;) (func (result i64)))
            (func (;0;) (type 0) (result i64)
              i64.const 0)
        )
        "#,
        );
    }
}
