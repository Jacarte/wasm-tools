use std::num::Wrapping;

use rand::{prelude::SmallRng, Rng};
use wasm_encoder::{CodeSection, Function, Instruction, Module, ValType};
use wasmparser::{CodeSectionReader, FunctionBody, Operator, SectionReader};

use crate::{error::EitherType, module::*, Error, ModuleInfo, Result, WasmMutate};

use super::{CodeMutator, TupleType};

pub struct UnfoldConstant;

impl UnfoldConstant {
    fn is_constant(&self, op: &Operator) -> bool {
        match op {
            Operator::I32Const { .. } | Operator::I64Const { .. } => true,
            _ => false,
        }
    }

    fn unfold(
        &self,
        function: &mut Function,
        operator: Operator,
        rnd: &mut SmallRng,
    ) -> Result<()> {
        match operator {
            Operator::I32Const { value } => {
                let randomc: i32 = rnd.gen();

                function.instruction(Instruction::I32Const(randomc));
                function.instruction(Instruction::I32Const(
                    (Wrapping(value) - Wrapping(randomc)).0,
                ));
                // add the complement
                function.instruction(Instruction::I32Add);
                Ok(())
            }
            Operator::I64Const { value } => {
                let randomc: i64 = rnd.gen();

                function.instruction(Instruction::I64Const(randomc));
                function.instruction(Instruction::I64Const(
                    (Wrapping(value) - Wrapping(randomc)).0,
                ));
                // add the complement
                function.instruction(Instruction::I64Add);
                Ok(())
            }
            _ => Err(Error::UnsupportedType(EitherType::Operator(format!(
                "{:?}",
                operator
            )))),
        }
    }
}

impl CodeMutator for UnfoldConstant {
    fn mutate(
        &self,
        _: &WasmMutate,
        rnd: &mut SmallRng,
        operator_index: usize,
        operators: Vec<TupleType>,
        funcreader: FunctionBody,
        body_range: wasmparser::Range,
        function_stream: &[u8],
    ) -> Result<Function> {
        let mut localreader = funcreader.get_locals_reader().unwrap();
        // Get current locals and map to encoder types
        let (_, locals) = map_locals(&mut localreader);
        // Lets unfold only once, if its needed the orchestrator would know if this should be applied many times
        let mut newf = Function::new(locals?);
        let mut idx = 0;

        let mut newoffset = 0;
        for (operator, offset) in operators {
            newoffset = offset;
            if idx == operator_index {
                log::debug!("Unfolding constant {:?}", operator);
                // Copy previous code to the body
                let previous = &function_stream[body_range.start..offset];
                newf.raw(previous.iter().copied());
                self.unfold(&mut newf, operator, rnd)?
            }
            if idx == operator_index + 1
            // Operation over the previous constant
            {
                let previous = &function_stream[newoffset..offset];
                newf.raw(previous.iter().copied());
                break; // this break allows to copy the remaining buffer of the current reader
            }
            idx += 1;
        }
        // Copy last part of the function body
        let remaining = &function_stream[newoffset..body_range.end];
        newf.raw(remaining.iter().copied());
        Ok(newf)
    }

    fn can_mutate<'a>(
        &self,
        _: &'a WasmMutate,
        operators: &Vec<TupleType<'a>>,
        at: usize,
    ) -> Result<bool> {
        let (operator, _) = &operators[at];
        Ok(self.is_constant(operator))
    }
}

#[cfg(test)]
mod tests {
    use rand::{rngs::SmallRng, SeedableRng};
    use wasm_encoder::{CodeSection, FunctionSection, Module, TypeSection, ValType};
    use wasmparser::{Chunk, Parser};

    use crate::mutators::peephole::unfold_constant::UnfoldConstant;
    use crate::mutators::peephole::TupleType;
    use crate::{
        mutators::{
            peephole::{CodeMutator, PeepholeMutator},
            Mutator,
        },
        WasmMutate,
    };
    use wasm_encoder::{RawSection, SectionId};
    use wasmparser::{Payload, SectionReader};

    #[test]
    fn test_unfold() {
        let original = r#"
        (module
            (func (result i32) (local i32 i32)
                i32.const 42
                i32.const 100
                i32.add
            )
            (start 0)
        )
        "#;

        let expected = r#"
        (module
            (func (result i32) (local i32 i32)
              i32.const 1081994402
              i32.const -1081994360
              i32.add
              i32.const 100
              i32.add))
        "#;

        crate::match_code_mutation!(
            original,
            move |config: &WasmMutate, operators, mut reader, range, function_stream: &[u8]| {
                let mutator = UnfoldConstant;
                let mut rnd = SmallRng::seed_from_u64(0);

                mutator
                    .mutate(
                        &config,
                        &mut rnd,
                        0,
                        operators,
                        reader,
                        range,
                        &function_stream,
                    )
                    .unwrap()
            },
            expected
        )
    }
}
