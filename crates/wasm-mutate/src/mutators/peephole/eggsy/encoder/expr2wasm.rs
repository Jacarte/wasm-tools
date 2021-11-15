use std::num::Wrapping;

use crate::info::ModuleInfo;
use crate::module::PrimitiveTypeInfo;
use crate::mutators::peephole::eggsy::encoder::TraversalEvent;
use crate::mutators::peephole::*;
use crate::{error::EitherType, mutators::peephole::EG};
use egg::{Id, RecExpr};
use rand::Rng;
use wasm_encoder::{Function, Instruction, MemArg};

pub(crate) fn expr2wasm(
    info: &ModuleInfo,
    rnd: &mut rand::prelude::SmallRng,
    expr: &RecExpr<Lang>,
    newfunc: &mut Function,
    egraph: &EG,
) -> crate::Result<()> {
    let nodes = expr.as_ref();
    // The last node is the root.
    let root = Id::from(nodes.len() - 1);

    struct Context {
        // Current visited node index in the tree traversing
        current_node: Id,
        // In which state of the traversing of the current node the process is
        traversal_event: TraversalEvent,
    }

    impl Context {
        fn new(current_node: Id, traversal_event: TraversalEvent) -> Self {
            Context {
                current_node,
                traversal_event,
            }
        }
    }

    // println!("{:?}", expr.as_ref());
    let mut type_stack = vec![egraph
        .analysis
        .get_returning_tpe(&nodes[usize::from(root)], expr.as_ref())?];
    let mut worklist = vec![
        Context::new(root, TraversalEvent::Exit),
        Context::new(root, TraversalEvent::Enter),
    ];
    // Enqueue the coding back nodes and infer types
    while let Some(context) = worklist.pop() {
        let rootlang = &nodes[usize::from(context.current_node)];

        match context.traversal_event {
            TraversalEvent::Enter => {
                // Push children
                match rootlang {
                    Lang::I64Add(operands)
                    | Lang::I64Shl(operands)
                    | Lang::I64ShrU(operands)
                    | Lang::I64Sub(operands)
                    | Lang::I64Mul(operands)
                    | Lang::I64And(operands)
                    | Lang::I64Or(operands)
                    | Lang::I64Xor(operands)
                    | Lang::I64ShrS(operands)
                    | Lang::I64DivS(operands)
                    | Lang::I64DivU(operands)
                    | Lang::I64RotR(operands)
                    | Lang::I64RotL(operands)
                    | Lang::I64RemS(operands)
                    | Lang::I64RemU(operands) => {
                        type_stack.push(PrimitiveTypeInfo::I64);
                        type_stack.push(PrimitiveTypeInfo::I64);

                        for operand in operands.iter().rev() {
                            worklist.push(Context::new(*operand, TraversalEvent::Exit));
                            worklist.push(Context::new(*operand, TraversalEvent::Enter));
                        }
                    }
                    Lang::I32Add(operands)
                    | Lang::I32Shl(operands)
                    | Lang::I32ShrU(operands)
                    | Lang::I32Sub(operands)
                    | Lang::I32Mul(operands)
                    | Lang::I32And(operands)
                    | Lang::I32Or(operands)
                    | Lang::I32Xor(operands)
                    | Lang::I32ShrS(operands)
                    | Lang::I32DivS(operands)
                    | Lang::I32DivU(operands)
                    | Lang::I32RotR(operands)
                    | Lang::I32RotL(operands)
                    | Lang::I32RemS(operands)
                    | Lang::I32RemU(operands) => {
                        type_stack.push(PrimitiveTypeInfo::I32);
                        type_stack.push(PrimitiveTypeInfo::I32);

                        for operand in operands.iter().rev() {
                            worklist.push(Context::new(*operand, TraversalEvent::Exit));
                            worklist.push(Context::new(*operand, TraversalEvent::Enter));
                        }
                    }
                    Lang::F32Eq(operands)
                    | Lang::F32Max(operands)
                    | Lang::F32Min(operands)
                    | Lang::F32Div(operands)
                    | Lang::F32Mul(operands)
                    | Lang::F32Sub(operands)
                    | Lang::F32Copysign(operands)
                    | Lang::F32Ne(operands)
                    | Lang::F32Lt(operands)
                    | Lang::F32Gt(operands)
                    | Lang::F32Le(operands)
                    | Lang::F32Ge(operands)
                    | Lang::F32Add(operands) => {
                        type_stack.push(PrimitiveTypeInfo::F32);
                        type_stack.push(PrimitiveTypeInfo::F32);

                        for operand in operands.iter().rev() {
                            worklist.push(Context::new(*operand, TraversalEvent::Exit));
                            worklist.push(Context::new(*operand, TraversalEvent::Enter));
                        }
                    }
                    Lang::F64Eq(operands)
                    | Lang::F64Ne(operands)
                    | Lang::F64Lt(operands)
                    | Lang::F64Gt(operands)
                    | Lang::F64Le(operands)
                    | Lang::F64Ge(operands)
                    | Lang::F64Copysign(operands)
                    | Lang::F64Max(operands)
                    | Lang::F64Min(operands)
                    | Lang::F64Div(operands)
                    | Lang::F64Mul(operands)
                    | Lang::F64Sub(operands)
                    | Lang::F64Add(operands) => {
                        type_stack.push(PrimitiveTypeInfo::F64);
                        type_stack.push(PrimitiveTypeInfo::F64);

                        for operand in operands.iter().rev() {
                            worklist.push(Context::new(*operand, TraversalEvent::Exit));
                            worklist.push(Context::new(*operand, TraversalEvent::Enter));
                        }
                    }
                    Lang::F32Trunc(operands)
                    | Lang::F32Floor(operands)
                    | Lang::F32Ceil(operands)
                    | Lang::F32Sqrt(operands)
                    | Lang::F32Neg(operands)
                    | Lang::F32Nearest(operands)
                    | Lang::F32Abs(operands) => {
                        type_stack.push(PrimitiveTypeInfo::F32);

                        for operand in operands.iter().rev() {
                            worklist.push(Context::new(*operand, TraversalEvent::Exit));
                            worklist.push(Context::new(*operand, TraversalEvent::Enter));
                        }
                    }
                    Lang::F64Nearest(operands)
                    | Lang::F64trunc(operands)
                    | Lang::F64Floor(operands)
                    | Lang::F64Ceil(operands)
                    | Lang::F64Sqrt(operands)
                    | Lang::F64Neg(operands)
                    | Lang::F64Abs(operands) => {
                        type_stack.push(PrimitiveTypeInfo::F64);

                        for operand in operands.iter().rev() {
                            worklist.push(Context::new(*operand, TraversalEvent::Exit));
                            worklist.push(Context::new(*operand, TraversalEvent::Enter));
                        }
                    }
                    Lang::I32Eq(operands)
                    | Lang::I32Ne(operands)
                    | Lang::I32LtS(operands)
                    | Lang::I32LtU(operands)
                    | Lang::I32GtS(operands)
                    | Lang::I32GtU(operands)
                    | Lang::I32LeS(operands)
                    | Lang::I32LeU(operands)
                    | Lang::I32GeS(operands)
                    | Lang::I32GeU(operands) => {
                        type_stack.push(PrimitiveTypeInfo::I32);
                        type_stack.push(PrimitiveTypeInfo::I32);
                        for operand in operands.iter().rev() {
                            // The type is one of the siblings
                            worklist.push(Context::new(*operand, TraversalEvent::Exit));
                            worklist.push(Context::new(*operand, TraversalEvent::Enter));
                        }
                    }
                    Lang::I64Eq(operands)
                    | Lang::I64Ne(operands)
                    | Lang::I64LtS(operands)
                    | Lang::I64LtU(operands)
                    | Lang::I64GtS(operands)
                    | Lang::I64GtU(operands)
                    | Lang::I64LeS(operands)
                    | Lang::I64LeU(operands)
                    | Lang::I64GeS(operands)
                    | Lang::I64GeU(operands) => {
                        type_stack.push(PrimitiveTypeInfo::I64);
                        type_stack.push(PrimitiveTypeInfo::I64);
                        for operand in operands.iter().rev() {
                            // The type is one of the siblings
                            worklist.push(Context::new(*operand, TraversalEvent::Exit));
                            worklist.push(Context::new(*operand, TraversalEvent::Enter));
                        }
                    }
                    Lang::I32Popcnt(operands) | Lang::I32Eqz(operands) => {
                        type_stack.push(PrimitiveTypeInfo::I32);
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));
                    }
                    Lang::I64Popcnt(operands) | Lang::I64Eqz(operands) => {
                        type_stack.push(PrimitiveTypeInfo::I64);
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));
                    }
                    Lang::I32Extend8S(operands) => {
                        type_stack.push(PrimitiveTypeInfo::I32);
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));
                    }
                    Lang::I32Extend16S(operands) => {
                        type_stack.push(PrimitiveTypeInfo::I32);
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));
                    }
                    Lang::I64Extend32S(operands) => {
                        type_stack.push(PrimitiveTypeInfo::I64);
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));
                    }
                    Lang::I64Extend16S(operands) => {
                        type_stack.push(PrimitiveTypeInfo::I64);
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));
                    }
                    Lang::I64ExtendI32S(operands) => {
                        type_stack.push(PrimitiveTypeInfo::I32);
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));
                    }
                    Lang::I64Extend8S(operands) => {
                        type_stack.push(PrimitiveTypeInfo::I64);
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));
                    }
                    Lang::I64ExtendI32U(operands) => {
                        type_stack.push(PrimitiveTypeInfo::I32);
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));
                    }
                    Lang::I32TruncF32S(operands) => {
                        type_stack.push(PrimitiveTypeInfo::F32);
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));
                    }
                    Lang::I32TruncF32U(operands) => {
                        type_stack.push(PrimitiveTypeInfo::F32);
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));
                    }
                    Lang::I32TruncF64S(operands) => {
                        type_stack.push(PrimitiveTypeInfo::F64);
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));
                    }
                    Lang::I32TruncF64U(operands) => {
                        type_stack.push(PrimitiveTypeInfo::F64);
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));
                    }
                    Lang::I64TruncF32S(operands) => {
                        type_stack.push(PrimitiveTypeInfo::F32);
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));
                    }
                    Lang::I64TruncF32U(operands) => {
                        type_stack.push(PrimitiveTypeInfo::F32);
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));
                    }
                    Lang::I64TruncF64S(operands) => {
                        type_stack.push(PrimitiveTypeInfo::F64);
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));
                    }
                    Lang::I64TruncF64U(operands) => {
                        type_stack.push(PrimitiveTypeInfo::F64);
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));
                    }
                    Lang::F32ConvertI32S(operands) => {
                        type_stack.push(PrimitiveTypeInfo::I32);
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));
                    }
                    Lang::F32ConvertI32U(operands) => {
                        type_stack.push(PrimitiveTypeInfo::I32);
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));
                    }
                    Lang::F32ConvertI64S(operands) => {
                        type_stack.push(PrimitiveTypeInfo::I64);
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));
                    }
                    Lang::F32ConvertI64U(operands) => {
                        type_stack.push(PrimitiveTypeInfo::I64);
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));
                    }
                    Lang::F32DemoteF64(operands) => {
                        type_stack.push(PrimitiveTypeInfo::F64);
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));
                    }
                    Lang::F64ConvertI32S(operands) => {
                        type_stack.push(PrimitiveTypeInfo::I32);
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));
                    }
                    Lang::F64ConvertI32U(operands) => {
                        type_stack.push(PrimitiveTypeInfo::I32);
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));
                    }
                    Lang::F64ConvertI64S(operands) => {
                        type_stack.push(PrimitiveTypeInfo::I64);
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));
                    }
                    Lang::F64ConvertI64U(operands) => {
                        type_stack.push(PrimitiveTypeInfo::I64);
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));
                    }
                    Lang::F64PromoteF32(operands) => {
                        type_stack.push(PrimitiveTypeInfo::F32);
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));
                    }
                    Lang::I32ReinterpretF32(operands) => {
                        type_stack.push(PrimitiveTypeInfo::F32);
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));
                    }
                    Lang::I64ReinterpretF64(operands) => {
                        type_stack.push(PrimitiveTypeInfo::F64);
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));
                    }
                    Lang::F32ReinterpretI32(operands) => {
                        type_stack.push(PrimitiveTypeInfo::I32);
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));
                    }
                    Lang::F64ReinterpretI64(operands) => {
                        type_stack.push(PrimitiveTypeInfo::I64);
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));
                    }
                    Lang::I32TruncSatF32S(operands) => {
                        type_stack.push(PrimitiveTypeInfo::F32);
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));
                    }
                    Lang::I32TruncSatF32U(operands) => {
                        type_stack.push(PrimitiveTypeInfo::F32);
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));
                    }
                    Lang::I32TruncSatF64S(operands) => {
                        type_stack.push(PrimitiveTypeInfo::F64);
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));
                    }
                    Lang::I32TruncSatF64U(operands) => {
                        type_stack.push(PrimitiveTypeInfo::F64);
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));
                    }
                    Lang::I64TruncSatF32S(operands) => {
                        type_stack.push(PrimitiveTypeInfo::F32);
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));
                    }
                    Lang::I64TruncSatF32U(operands) => {
                        type_stack.push(PrimitiveTypeInfo::F32);
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));
                    }
                    Lang::I64TruncSatF64S(operands) => {
                        type_stack.push(PrimitiveTypeInfo::F64);
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));
                    }
                    Lang::I64TruncSatF64U(operands) => {
                        type_stack.push(PrimitiveTypeInfo::F64);
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));
                    }
                    Lang::I32Clz(operands) => {
                        type_stack.push(PrimitiveTypeInfo::I32);
                        for operand in operands.iter().rev() {
                            // The type is one of the siblings
                            worklist.push(Context::new(*operand, TraversalEvent::Exit));
                            worklist.push(Context::new(*operand, TraversalEvent::Enter));
                        }
                    }
                    Lang::I64Clz(operands) => {
                        type_stack.push(PrimitiveTypeInfo::I64);
                        for operand in operands.iter().rev() {
                            // The type is one of the siblings
                            worklist.push(Context::new(*operand, TraversalEvent::Exit));
                            worklist.push(Context::new(*operand, TraversalEvent::Enter));
                        }
                    }
                    Lang::I32Ctz(operands) => {
                        type_stack.push(PrimitiveTypeInfo::I64);
                        for operand in operands.iter().rev() {
                            // The type is one of the siblings
                            worklist.push(Context::new(*operand, TraversalEvent::Exit));
                            worklist.push(Context::new(*operand, TraversalEvent::Enter));
                        }
                    }
                    Lang::I64Ctz(operands) => {
                        type_stack.push(PrimitiveTypeInfo::I64);
                        for operand in operands.iter().rev() {
                            // The type is one of the siblings
                            worklist.push(Context::new(*operand, TraversalEvent::Exit));
                            worklist.push(Context::new(*operand, TraversalEvent::Enter));
                        }
                    }
                    Lang::Select(operands) => {
                        type_stack.push(PrimitiveTypeInfo::I32);
                        type_stack.push(PrimitiveTypeInfo::I32);
                        type_stack.push(PrimitiveTypeInfo::I32);
                        for operand in operands.iter().rev() {
                            // The type is one of the siblings
                            worklist.push(Context::new(*operand, TraversalEvent::Exit));
                            worklist.push(Context::new(*operand, TraversalEvent::Enter));
                        }
                    }
                    Lang::LocalTee(operands) => {
                        // Skip operand 0 which is the symbol
                        let expectedvalue = &nodes[usize::from(operands[1])];
                        let tpe = egraph
                            .analysis
                            .get_returning_tpe(expectedvalue, expr.as_ref())?;
                        type_stack.push(tpe);
                        worklist.push(Context::new(operands[1], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[1], TraversalEvent::Enter));
                    }
                    Lang::LocalSet(operands) => {
                        // Skip operand 0 which is the symbol
                        let expectedvalue = &nodes[usize::from(operands[1])];
                        let tpe = egraph
                            .analysis
                            .get_returning_tpe(expectedvalue, expr.as_ref())?;
                        type_stack.push(tpe);
                        worklist.push(Context::new(operands[1], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[1], TraversalEvent::Enter));
                    }
                    Lang::GlobalSet(operands) => {
                        // Skip operand 0 which is the symbol
                        let expectedvalue = &nodes[usize::from(operands[1])];
                        let tpe = egraph
                            .analysis
                            .get_returning_tpe(expectedvalue, expr.as_ref())?;
                        type_stack.push(tpe);
                        worklist.push(Context::new(operands[1], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[1], TraversalEvent::Enter));
                    }
                    Lang::Wrap(operands) => {
                        type_stack.push(PrimitiveTypeInfo::I64);
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));
                    }
                    Lang::Drop(operands) => {
                        type_stack.push(PrimitiveTypeInfo::I64);
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));
                    }
                    Lang::Call(operands) => {
                        // The first operand is always the helper Arg to identify the function
                        let first = operands[0];
                        let firstnode = &nodes[usize::from(first)];
                        let functionindex = match firstnode {
                            Lang::Arg(val) => {
                                *val as u32
                            }
                            Lang::Const(val) => {
                                *val as u32
                            }
                            _ => unreachable!("The first argument for Call nodes should be an inmmediate node type (Arg)")
                        };
                        let typeinfo = info.get_functype_idx(functionindex as usize);
                        if let crate::module::TypeInfo::Func(tpe) = typeinfo {
                            // Push operands types in reverse order
                            for (idx, _) in operands[1..].iter().enumerate().rev() {
                                type_stack.push(tpe.params[idx].clone())
                            }
                        }
                        for operand in operands.iter().skip(1).rev() {
                            worklist.push(Context::new(*operand, TraversalEvent::Exit));
                            worklist.push(Context::new(*operand, TraversalEvent::Enter));
                        }
                    }
                    Lang::I32Load(operands)
                    | Lang::I64Load(operands)
                    | Lang::F32Load(operands)
                    | Lang::F64Load(operands)
                    | Lang::I32Load8S(operands)
                    | Lang::I32Load8U(operands)
                    | Lang::I32Load16S(operands)
                    | Lang::I32Load16U(operands)
                    | Lang::I64Load8S(operands)
                    | Lang::I64Load8U(operands)
                    | Lang::I64Load16S(operands)
                    | Lang::I64Load16U(operands)
                    | Lang::I64Load32S(operands)
                    | Lang::I64Load32U(operands) => {
                        type_stack.push(PrimitiveTypeInfo::I32);
                        // Only push the first argument, remaining are helpersÇ
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));
                    }
                    Lang::I32Store(operands) => {
                        type_stack.push(PrimitiveTypeInfo::I32);
                        type_stack.push(PrimitiveTypeInfo::I32);
                        // Only push the first argument, remaining are helpers
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));
                        worklist.push(Context::new(operands[1], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[1], TraversalEvent::Enter));
                    }
                    Lang::I64Store(operands) => {
                        type_stack.push(PrimitiveTypeInfo::I64);
                        type_stack.push(PrimitiveTypeInfo::I32);
                        // Only push the first argument, remaining are helpers
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));

                        worklist.push(Context::new(operands[1], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[1], TraversalEvent::Enter));
                    }
                    Lang::F32Store(operands) => {
                        type_stack.push(PrimitiveTypeInfo::F32);
                        type_stack.push(PrimitiveTypeInfo::I32);
                        // Only push the first argument, remaining are helpers
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));

                        worklist.push(Context::new(operands[1], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[1], TraversalEvent::Enter));
                    }
                    Lang::F64Store(operands) => {
                        type_stack.push(PrimitiveTypeInfo::F64);
                        type_stack.push(PrimitiveTypeInfo::I32);
                        // Only push the first argument, remaining are helpers
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));

                        worklist.push(Context::new(operands[1], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[1], TraversalEvent::Enter));
                    }
                    Lang::I32Store8(operands) => {
                        type_stack.push(PrimitiveTypeInfo::I32);
                        type_stack.push(PrimitiveTypeInfo::I32);
                        // Only push the first argument, remaining are helpers
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));

                        worklist.push(Context::new(operands[1], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[1], TraversalEvent::Enter));
                    }
                    Lang::I32Store16(operands) => {
                        type_stack.push(PrimitiveTypeInfo::I32);
                        type_stack.push(PrimitiveTypeInfo::I32);
                        // Only push the first argument, remaining are helpers
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));

                        worklist.push(Context::new(operands[1], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[1], TraversalEvent::Enter));
                    }
                    Lang::I64Store8(operands) => {
                        type_stack.push(PrimitiveTypeInfo::I64);
                        type_stack.push(PrimitiveTypeInfo::I32);
                        // Only push the first argument, remaining are helpers
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));

                        worklist.push(Context::new(operands[1], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[1], TraversalEvent::Enter));
                    }
                    Lang::I64Store16(operands) => {
                        type_stack.push(PrimitiveTypeInfo::I64);
                        type_stack.push(PrimitiveTypeInfo::I32);
                        // Only push the first argument, remaining are helpers
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));

                        worklist.push(Context::new(operands[1], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[1], TraversalEvent::Enter));
                    }
                    Lang::I64Store32(operands) => {
                        type_stack.push(PrimitiveTypeInfo::I64);
                        type_stack.push(PrimitiveTypeInfo::I32);
                        // Only push the first argument, remaining are helpers
                        worklist.push(Context::new(operands[0], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[0], TraversalEvent::Enter));

                        worklist.push(Context::new(operands[1], TraversalEvent::Exit));
                        worklist.push(Context::new(operands[1], TraversalEvent::Enter));
                    }
                    _ => { /* Do nothing */ }
                }
            }
            TraversalEvent::Exit => {
                let top = type_stack.pop();
                match rootlang {
                    Lang::LocalGet(operands) => {
                        let id = &nodes[usize::from(operands[0])];
                        match id {
                            Lang::Arg(val) => {
                                newfunc.instruction(&Instruction::LocalGet(*val as u32));
                            }
                            _ => unreachable!("Invalid local index {:?}", id),
                        }
                    }
                    Lang::GlobalGet(operands) => {
                        let id = &nodes[usize::from(operands[0])];
                        match id {
                            Lang::Arg(val) => {
                                newfunc.instruction(&Instruction::GlobalGet(*val as u32));
                            }
                            _ => unreachable!("Invalid local index {:?}", id),
                        }
                    }
                    Lang::LocalSet(operands) => {
                        let id = &nodes[usize::from(operands[0])];
                        match id {
                            Lang::Arg(val) => {
                                newfunc.instruction(&Instruction::LocalSet(*val as u32));
                            }
                            _ => unreachable!("Invalid local index {:?}", id),
                        }
                    }
                    Lang::GlobalSet(operands) => {
                        let id = &nodes[usize::from(operands[0])];
                        match id {
                            Lang::Arg(val) => {
                                newfunc.instruction(&Instruction::GlobalSet(*val as u32));
                            }
                            _ => unreachable!("Invalid local index {:?}", id),
                        }
                    }
                    Lang::LocalTee(operands) => {
                        let id = &nodes[usize::from(operands[0])];
                        match id {
                            Lang::Arg(val) => {
                                newfunc.instruction(&Instruction::LocalTee(*val as u32));
                            }
                            _ => unreachable!("Invalid local index {:?}", id),
                        }
                    }
                    Lang::Wrap(_) => {
                        newfunc.instruction(&Instruction::I32WrapI64);
                    }
                    Lang::Call(operands) => {
                        let first = operands[0];
                        let firstnode = &nodes[usize::from(first)];
                        match firstnode {
                            Lang::Arg(val) => {
                                newfunc.instruction(&Instruction::Call(*val as u32));
                            }
                            ////Lang::Num(val) => {
                             //   newfunc.instruction(&Instruction::Call(*val as u32));
                            //}
                            _ => unreachable!("The first argument for Call nodes should be an inmmediate node type (Arg)")
                        }
                    }
                    Lang::Drop(_) => {
                        newfunc.instruction(&Instruction::Drop);
                    }
                    Lang::I32Load(operands) => {
                        let offset_operand = &nodes[usize::from(operands[1])];
                        let align_operand = &nodes[usize::from(operands[2])];
                        let memidx_operand = &nodes[usize::from(operands[3])];

                        let toarg = |op: &Lang| match op {
                            Lang::Arg(val) => *val as u64,
                            Lang::Const(val) => *val as u64,
                            _ => unreachable!(
                                "This operand should be an Arg node. Current operand {:?}",
                                op
                            ),
                        };

                        let offset_value = toarg(offset_operand);

                        let align_value = toarg(align_operand);

                        let memidx_value = toarg(memidx_operand);

                        let memarg = MemArg {
                            offset: offset_value, // These can be mutated as well
                            align: align_value as u32,
                            memory_index: memidx_value as u32,
                        };

                        newfunc.instruction(&Instruction::I32Load(memarg));
                    }
                    Lang::I64Load(operands) => {
                        let offset_operand = &nodes[usize::from(operands[1])];
                        let align_operand = &nodes[usize::from(operands[2])];
                        let memidx_operand = &nodes[usize::from(operands[3])];

                        let toarg = |op: &Lang| match op {
                            Lang::Arg(val) => *val as u64,
                            Lang::Const(val) => *val as u64,
                            _ => unreachable!(
                                "This operand should be an Arg node. Current operand {:?}",
                                op
                            ),
                        };

                        let offset_value = toarg(offset_operand);

                        let align_value = toarg(align_operand);

                        let memidx_value = toarg(memidx_operand);

                        let memarg = MemArg {
                            offset: offset_value, // These can be mutated as well
                            align: align_value as u32,
                            memory_index: memidx_value as u32,
                        };

                        newfunc.instruction(&Instruction::I64Load(memarg));
                    }
                    Lang::RandI32 => {
                        newfunc.instruction(&Instruction::I32Const(rnd.gen()));
                    }
                    Lang::RandI64 => {
                        newfunc.instruction(&Instruction::I64Const(rnd.gen()));
                    }
                    Lang::Undef => { /* Do nothig */ }
                    Lang::UnfoldI32(value) => {
                        let child = &nodes[usize::from(*value)];
                        match child {
                            Lang::I32(value) => {
                                // Getting type from eclass.

                                let r: i32 = rnd.gen();
                                newfunc.instruction(&Instruction::I32Const(r));
                                newfunc.instruction(&Instruction::I32Const(
                                    (Wrapping(*value as i32) - Wrapping(r)).0,
                                ));
                                newfunc.instruction(&Instruction::I32Add);
                            }
                            _ => {
                                return Err(crate::Error::UnsupportedType(EitherType::EggError(
                                    format!("The current eterm cannot be unfolded {:?}", child,),
                                )))
                            }
                        }
                    }
                    Lang::UnfoldI64(value) => {
                        let child = &nodes[usize::from(*value)];
                        match child {
                            Lang::I64(value) => {
                                // Getting type from eclass.

                                let r: i64 = rnd.gen();
                                newfunc.instruction(&Instruction::I64Const(r));
                                newfunc.instruction(&Instruction::I64Const(
                                    (Wrapping(*value) - Wrapping(r)).0,
                                ));
                                newfunc.instruction(&Instruction::I64Add);
                            }
                            _ => {
                                return Err(crate::Error::UnsupportedType(EitherType::EggError(
                                    format!("The current eterm cannot be unfolded {:?}", child,),
                                )))
                            }
                        }
                    }
                    Lang::I32(v) => {
                        newfunc.instruction(&Instruction::I32Const(*v));
                    }
                    Lang::I64(v) => {
                        newfunc.instruction(&Instruction::I64Const(*v));
                    }
                    Lang::F32(v) => {
                        newfunc.instruction(&Instruction::F32Const(f32::from_bits(*v)));
                    }
                    Lang::F64(v) => {
                        newfunc.instruction(&Instruction::F64Const(f64::from_bits(*v)));
                    }
                    Lang::Arg(val) => {
                        let t = type_stack.pop().expect("Missing type information");
                        match t {
                            PrimitiveTypeInfo::I32 => {
                                newfunc.instruction(&Instruction::I32Const(*val as i32));
                            }
                            PrimitiveTypeInfo::I64 => {
                                newfunc.instruction(&Instruction::I64Const(*val as i64));
                            }
                            _ => unreachable!("Type cannot be encoded"),
                        }
                    }
                    Lang::I32Add(_) => {
                        newfunc.instruction(&Instruction::I32Add);
                    }
                    Lang::I64Add(_) => {
                        newfunc.instruction(&Instruction::I64Add);
                    }
                    Lang::I32Sub(_) => {
                        newfunc.instruction(&Instruction::I32Sub);
                    }
                    Lang::I64Sub(_) => {
                        newfunc.instruction(&Instruction::I64Sub);
                    }
                    Lang::I32Mul(_) => {
                        newfunc.instruction(&Instruction::I32Mul);
                    }
                    Lang::I64Mul(_) => {
                        newfunc.instruction(&Instruction::I64Mul);
                    }
                    Lang::I32And(_) => {
                        newfunc.instruction(&Instruction::I32And);
                    }
                    Lang::I64And(_) => {
                        newfunc.instruction(&Instruction::I64And);
                    }
                    Lang::I32Or(_) => {
                        newfunc.instruction(&Instruction::I32Or);
                    }
                    Lang::I64Or(_) => {
                        newfunc.instruction(&Instruction::I64Or);
                    }
                    Lang::I32Xor(_) => {
                        newfunc.instruction(&Instruction::I32Xor);
                    }
                    Lang::I64Xor(_) => {
                        newfunc.instruction(&Instruction::I64Xor);
                    }
                    Lang::I32Shl(_) => {
                        newfunc.instruction(&Instruction::I32Shl);
                    }
                    Lang::I64Shl(_) => {
                        newfunc.instruction(&Instruction::I64Shl);
                    }
                    Lang::I32ShrU(_) => {
                        newfunc.instruction(&Instruction::I32ShrU);
                    }
                    Lang::I64ShrU(_) => {
                        newfunc.instruction(&Instruction::I64ShrU);
                    }
                    Lang::I32DivU(_) => {
                        newfunc.instruction(&Instruction::I32DivU);
                    }
                    Lang::I64DivU(_) => {
                        newfunc.instruction(&Instruction::I64DivU);
                    }
                    Lang::I32DivS(_) => {
                        newfunc.instruction(&Instruction::I32DivS);
                    }
                    Lang::I64DivS(_) => {
                        newfunc.instruction(&Instruction::I64DivS);
                    }
                    Lang::I32ShrS(_) => {
                        newfunc.instruction(&Instruction::I32ShrS);
                    }
                    Lang::I64ShrS(_) => {
                        newfunc.instruction(&Instruction::I64ShrS);
                    }
                    Lang::I32RotR(_) => {
                        newfunc.instruction(&Instruction::I32Rotr);
                    }
                    Lang::I64RotR(_) => {
                        newfunc.instruction(&Instruction::I64Rotr);
                    }
                    Lang::I32RotL(_) => {
                        newfunc.instruction(&Instruction::I32Rotl);
                    }
                    Lang::I64RotL(_) => {
                        newfunc.instruction(&Instruction::I64Rotl);
                    }
                    Lang::I32RemS(_) => {
                        newfunc.instruction(&Instruction::I32RemS);
                    }
                    Lang::I64RemS(_) => {
                        newfunc.instruction(&Instruction::I64RemS);
                    }
                    Lang::I32RemU(_) => {
                        newfunc.instruction(&Instruction::I32RemU);
                    }
                    Lang::I64RemU(_) => {
                        newfunc.instruction(&Instruction::I64RemU);
                    }
                    Lang::I32Eqz(_) => {
                        newfunc.instruction(&Instruction::I32Eqz);
                    }
                    Lang::I64Eqz(_) => {
                        newfunc.instruction(&Instruction::I64Eqz);
                    }
                    Lang::I32Eq(_) => {
                        newfunc.instruction(&Instruction::I32Eq);
                    }
                    Lang::I64Eq(_) => {
                        newfunc.instruction(&Instruction::I64Eq);
                    }
                    Lang::I32Ne(_) => {
                        newfunc.instruction(&Instruction::I32Neq);
                    }
                    Lang::I64Ne(_) => {
                        newfunc.instruction(&Instruction::I64Neq);
                    }
                    Lang::I32LtS(_) => {
                        newfunc.instruction(&Instruction::I32LtS);
                    }
                    Lang::I64LtS(_) => {
                        newfunc.instruction(&Instruction::I64LtS);
                    }
                    Lang::I32LtU(_) => {
                        newfunc.instruction(&Instruction::I32LtU);
                    }
                    Lang::I64LtU(_) => {
                        newfunc.instruction(&Instruction::I64LtU);
                    }
                    Lang::I32GtS(_) => {
                        newfunc.instruction(&Instruction::I32GtS);
                    }
                    Lang::I64GtS(_) => {
                        newfunc.instruction(&Instruction::I64GtS);
                    }
                    Lang::I32GtU(_) => {
                        newfunc.instruction(&Instruction::I32GtU);
                    }
                    Lang::I64GtU(_) => {
                        newfunc.instruction(&Instruction::I64GtU);
                    }
                    Lang::I32LeS(_) => {
                        newfunc.instruction(&Instruction::I32LeS);
                    }
                    Lang::I64LeS(_) => {
                        newfunc.instruction(&Instruction::I64LeS);
                    }
                    Lang::I32LeU(_) => {
                        newfunc.instruction(&Instruction::I32LeU);
                    }
                    Lang::I64LeU(_) => {
                        newfunc.instruction(&Instruction::I64LeU);
                    }
                    Lang::I32GeS(_) => {
                        newfunc.instruction(&Instruction::I32GeS);
                    }
                    Lang::I64GeS(_) => {
                        newfunc.instruction(&Instruction::I64GeS);
                    }
                    Lang::I32GeU(_) => {
                        newfunc.instruction(&Instruction::I32GeU);
                    }
                    Lang::I64GeU(_) => {
                        newfunc.instruction(&Instruction::I64GeU);
                    }
                    Lang::I32Popcnt(_) => {
                        newfunc.instruction(&Instruction::I32Popcnt);
                    }
                    Lang::I64Popcnt(_) => {
                        newfunc.instruction(&Instruction::I64Popcnt);
                    }

                    Lang::F32Add(_) => {
                        newfunc.instruction(&Instruction::F32Add);
                    }
                    Lang::F64Add(_) => {
                        newfunc.instruction(&Instruction::F64Add);
                    }
                    Lang::F32Sub(_) => {
                        newfunc.instruction(&Instruction::F32Sub);
                    }
                    Lang::F64Sub(_) => {
                        newfunc.instruction(&Instruction::F64Sub);
                    }
                    Lang::F32Mul(_) => {
                        newfunc.instruction(&Instruction::F32Mul);
                    }
                    Lang::F64Mul(_) => {
                        newfunc.instruction(&Instruction::F64Mul);
                    }
                    Lang::F32Div(_) => {
                        newfunc.instruction(&Instruction::F32Div);
                    }
                    Lang::F64Div(_) => {
                        newfunc.instruction(&Instruction::F64Div);
                    }
                    Lang::F32Min(_) => {
                        newfunc.instruction(&Instruction::F32Min);
                    }
                    Lang::F64Min(_) => {
                        newfunc.instruction(&Instruction::F64Min);
                    }
                    Lang::F32Max(_) => {
                        newfunc.instruction(&Instruction::F32Max);
                    }
                    Lang::F64Max(_) => {
                        newfunc.instruction(&Instruction::F64Max);
                    }
                    Lang::F32Copysign(_) => {
                        newfunc.instruction(&Instruction::F32Copysign);
                    }
                    Lang::F64Copysign(_) => {
                        newfunc.instruction(&Instruction::F64Copysign);
                    }
                    Lang::F32Abs(_) => {
                        newfunc.instruction(&Instruction::F32Abs);
                    }
                    Lang::F64Abs(_) => {
                        newfunc.instruction(&Instruction::F64Abs);
                    }
                    Lang::F32Neg(_) => {
                        newfunc.instruction(&Instruction::F32Neg);
                    }
                    Lang::F64Neg(_) => {
                        newfunc.instruction(&Instruction::F64Neg);
                    }
                    Lang::F32Sqrt(_) => {
                        newfunc.instruction(&Instruction::F32Sqrt);
                    }
                    Lang::F64Sqrt(_) => {
                        newfunc.instruction(&Instruction::F64Sqrt);
                    }
                    Lang::F32Ceil(_) => {
                        newfunc.instruction(&Instruction::F32Ceil);
                    }
                    Lang::F64Ceil(_) => {
                        newfunc.instruction(&Instruction::F64Ceil);
                    }
                    Lang::F32Floor(_) => {
                        newfunc.instruction(&Instruction::F32Floor);
                    }
                    Lang::F64Floor(_) => {
                        newfunc.instruction(&Instruction::F64Floor);
                    }
                    Lang::F32Trunc(_) => {
                        newfunc.instruction(&Instruction::F32Trunc);
                    }
                    Lang::F64trunc(_) => {
                        newfunc.instruction(&Instruction::F64Trunc);
                    }
                    Lang::F32Nearest(_) => {
                        newfunc.instruction(&Instruction::F32Nearest);
                    }
                    Lang::F64Nearest(_) => {
                        newfunc.instruction(&Instruction::F64Nearest);
                    }
                    Lang::F32Eq(_) => {
                        newfunc.instruction(&Instruction::F32Eq);
                    }
                    Lang::F64Eq(_) => {
                        newfunc.instruction(&Instruction::F64Eq);
                    }
                    Lang::F32Ne(_) => {
                        newfunc.instruction(&Instruction::F32Neq);
                    }
                    Lang::F64Ne(_) => {
                        newfunc.instruction(&Instruction::F64Neq);
                    }
                    Lang::F32Lt(_) => {
                        newfunc.instruction(&Instruction::F32Lt);
                    }
                    Lang::F64Lt(_) => {
                        newfunc.instruction(&Instruction::F64Lt);
                    }
                    Lang::F32Gt(_) => {
                        newfunc.instruction(&Instruction::F32Gt);
                    }
                    Lang::F64Gt(_) => {
                        newfunc.instruction(&Instruction::F64Gt);
                    }
                    Lang::F32Le(_) => {
                        newfunc.instruction(&Instruction::F32Le);
                    }
                    Lang::F64Le(_) => {
                        newfunc.instruction(&Instruction::F64Le);
                    }
                    Lang::F32Ge(_) => {
                        newfunc.instruction(&Instruction::F32Ge);
                    }
                    Lang::F64Ge(_) => {
                        newfunc.instruction(&Instruction::F64Ge);
                    }
                    Lang::I32Extend8S(_) => {
                        newfunc.instruction(&Instruction::I32Extend8S);
                    }
                    Lang::I64Extend8S(_) => {
                        newfunc.instruction(&Instruction::I64Extend8S);
                    }
                    Lang::I32Extend16S(_) => {
                        newfunc.instruction(&Instruction::I32Extend16S);
                    }
                    Lang::I64Extend16S(_) => {
                        newfunc.instruction(&Instruction::I64Extend16S);
                    }
                    Lang::I64Extend32S(_) => {
                        newfunc.instruction(&Instruction::I64Extend32S);
                    }
                    Lang::I64ExtendI32S(_) => {
                        newfunc.instruction(&Instruction::I64ExtendI32S);
                    }
                    Lang::I64ExtendI32U(_) => {
                        newfunc.instruction(&Instruction::I64ExtendI32U);
                    }
                    Lang::I32Clz(_) => {
                        newfunc.instruction(&Instruction::I32Clz);
                    }
                    Lang::I64Clz(_) => {
                        newfunc.instruction(&Instruction::I32Clz);
                    }
                    Lang::I32Ctz(_) => {
                        newfunc.instruction(&Instruction::I32Ctz);
                    }
                    Lang::I64Ctz(_) => {
                        newfunc.instruction(&Instruction::I64Ctz);
                    }
                    Lang::Select(_) => {
                        newfunc.instruction(&Instruction::Select);
                    }
                    Lang::I32TruncF32S(_) => {
                        newfunc.instruction(&Instruction::I32TruncF32S);
                    }
                    Lang::I32TruncF32U(_) => {
                        newfunc.instruction(&Instruction::I32TruncF32U);
                    }
                    Lang::I32TruncF64S(_) => {
                        newfunc.instruction(&Instruction::I32TruncF64S);
                    }
                    Lang::I64TruncF32S(_) => {
                        newfunc.instruction(&Instruction::I64TruncF32S);
                    }
                    Lang::I64TruncF32U(_) => {
                        newfunc.instruction(&Instruction::I64TruncF32U);
                    }
                    Lang::I64TruncF64S(_) => {
                        newfunc.instruction(&Instruction::I64TruncF64S);
                    }
                    Lang::I64TruncF64U(_) => {
                        newfunc.instruction(&Instruction::I64TruncF64U);
                    }
                    Lang::F32ConvertI32S(_) => {
                        newfunc.instruction(&Instruction::F32ConvertI32S);
                    }
                    Lang::F32ConvertI32U(_) => {
                        newfunc.instruction(&Instruction::F32ConvertI32U);
                    }
                    Lang::F32ConvertI64S(_) => {
                        newfunc.instruction(&Instruction::F32ConvertI64S);
                    }
                    Lang::F32ConvertI64U(_) => {
                        newfunc.instruction(&Instruction::F32ConvertI64U);
                    }
                    Lang::F32DemoteF64(_) => {
                        newfunc.instruction(&Instruction::F32DemoteF64);
                    }
                    Lang::F64ConvertI32S(_) => {
                        newfunc.instruction(&Instruction::F64ConvertI32S);
                    }
                    Lang::F64ConvertI32U(_) => {
                        newfunc.instruction(&Instruction::F64ConvertI32U);
                    }
                    Lang::F64ConvertI64S(_) => {
                        newfunc.instruction(&Instruction::F64ConvertI64S);
                    }
                    Lang::F64ConvertI64U(_) => {
                        newfunc.instruction(&Instruction::F64ConvertI64U);
                    }
                    Lang::F64PromoteF32(_) => {
                        newfunc.instruction(&Instruction::F64PromoteF32);
                    }
                    Lang::I32ReinterpretF32(_) => {
                        newfunc.instruction(&Instruction::I32ReinterpretF32);
                    }
                    Lang::I64ReinterpretF64(_) => {
                        newfunc.instruction(&Instruction::I64ReinterpretF64);
                    }
                    Lang::F32ReinterpretI32(_) => {
                        newfunc.instruction(&Instruction::F32ReinterpretI32);
                    }
                    Lang::F64ReinterpretI64(_) => {
                        newfunc.instruction(&Instruction::F64ReinterpretI64);
                    }
                    Lang::I32TruncSatF32S(_) => {
                        newfunc.instruction(&Instruction::I32TruncSatF32S);
                    }
                    Lang::I32TruncSatF32U(_) => {
                        newfunc.instruction(&Instruction::I32TruncSatF32U);
                    }
                    Lang::I32TruncSatF64S(_) => {
                        newfunc.instruction(&Instruction::I32TruncSatF64S);
                    }
                    Lang::I32TruncSatF64U(_) => {
                        newfunc.instruction(&Instruction::I32TruncSatF64U);
                    }
                    Lang::I64TruncSatF32S(_) => {
                        newfunc.instruction(&Instruction::I64TruncSatF32S);
                    }
                    Lang::I64TruncSatF32U(_) => {
                        newfunc.instruction(&Instruction::I64TruncSatF32U);
                    }
                    Lang::I64TruncSatF64S(_) => {
                        newfunc.instruction(&Instruction::I64TruncSatF64S);
                    }
                    Lang::I64TruncSatF64U(_) => {
                        newfunc.instruction(&Instruction::I64TruncSatF64U);
                    }
                    Lang::I32TruncF64U(_) => {
                        newfunc.instruction(&Instruction::I32TruncF64U);
                    }
                    Lang::Const(v) => match top.clone().expect("Missing info") {
                        PrimitiveTypeInfo::I32 => {
                            newfunc.instruction(&Instruction::I32Const(*v as i32));
                        }
                        PrimitiveTypeInfo::I64 => {
                            newfunc.instruction(&Instruction::I64Const(*v));
                        }
                        PrimitiveTypeInfo::F32 => {
                            newfunc.instruction(&Instruction::F32Const(f32::from_bits(*v as u32)));
                        }
                        PrimitiveTypeInfo::F64 => {
                            newfunc.instruction(&Instruction::F64Const(f64::from_bits(*v as u64)));
                        }
                        _ => unreachable!("Invalid type {:?}", top),
                    },
                    Lang::I32Store(operands) => {
                        let offset_operand = &nodes[usize::from(operands[2])];
                        let align_operand = &nodes[usize::from(operands[3])];
                        let memidx_operand = &nodes[usize::from(operands[4])];

                        let toarg = |op: &Lang| match op {
                            Lang::Arg(val) => *val as u64,
                            Lang::Const(val) => *val as u64,
                            _ => unreachable!(
                                "This operand should be an Arg node. Current operand {:?}",
                                op
                            ),
                        };

                        let offset_value = toarg(offset_operand);

                        let align_value = toarg(align_operand);

                        let memidx_value = toarg(memidx_operand);

                        let memarg = MemArg {
                            offset: offset_value, // These can be mutated as well
                            align: align_value as u32,
                            memory_index: memidx_value as u32,
                        };

                        newfunc.instruction(&Instruction::I32Store(memarg));
                    }
                    Lang::I64Store(operands) => {
                        let offset_operand = &nodes[usize::from(operands[2])];
                        let align_operand = &nodes[usize::from(operands[3])];
                        let memidx_operand = &nodes[usize::from(operands[4])];

                        let toarg = |op: &Lang| match op {
                            Lang::Arg(val) => *val as u64,
                            Lang::Const(val) => *val as u64,
                            _ => unreachable!(
                                "This operand should be an Arg node. Current operand {:?}",
                                op
                            ),
                        };

                        let offset_value = toarg(offset_operand);

                        let align_value = toarg(align_operand);

                        let memidx_value = toarg(memidx_operand);

                        let memarg = MemArg {
                            offset: offset_value, // These can be mutated as well
                            align: align_value as u32,
                            memory_index: memidx_value as u32,
                        };

                        newfunc.instruction(&Instruction::I64Store(memarg));
                    }
                    Lang::MemoryGrow(_) => todo!(),
                    Lang::MemorySize(_) => todo!(),
                    Lang::F32Load(operands) => {
                        let offset_operand = &nodes[usize::from(operands[1])];
                        let align_operand = &nodes[usize::from(operands[2])];
                        let memidx_operand = &nodes[usize::from(operands[3])];

                        let toarg = |op: &Lang| match op {
                            Lang::Arg(val) => *val as u64,
                            Lang::Const(val) => *val as u64,
                            _ => unreachable!(
                                "This operand should be an Arg node. Current operand {:?}",
                                op
                            ),
                        };

                        let offset_value = toarg(offset_operand);

                        let align_value = toarg(align_operand);

                        let memidx_value = toarg(memidx_operand);

                        let memarg = MemArg {
                            offset: offset_value, // These can be mutated as well
                            align: align_value as u32,
                            memory_index: memidx_value as u32,
                        };

                        newfunc.instruction(&Instruction::F32Load(memarg));
                    }
                    Lang::F64Load(operands) => {
                        let offset_operand = &nodes[usize::from(operands[1])];
                        let align_operand = &nodes[usize::from(operands[2])];
                        let memidx_operand = &nodes[usize::from(operands[3])];

                        let toarg = |op: &Lang| match op {
                            Lang::Arg(val) => *val as u64,
                            Lang::Const(val) => *val as u64,
                            _ => unreachable!(
                                "This operand should be an Arg node. Current operand {:?}",
                                op
                            ),
                        };

                        let offset_value = toarg(offset_operand);

                        let align_value = toarg(align_operand);

                        let memidx_value = toarg(memidx_operand);

                        let memarg = MemArg {
                            offset: offset_value, // These can be mutated as well
                            align: align_value as u32,
                            memory_index: memidx_value as u32,
                        };

                        newfunc.instruction(&Instruction::F64Load(memarg));
                    }
                    Lang::I32Load8S(operands) => {
                        let offset_operand = &nodes[usize::from(operands[1])];
                        let align_operand = &nodes[usize::from(operands[2])];
                        let memidx_operand = &nodes[usize::from(operands[3])];

                        let toarg = |op: &Lang| match op {
                            Lang::Arg(val) => *val as u64,
                            Lang::Const(val) => *val as u64,
                            _ => unreachable!(
                                "This operand should be an Arg node. Current operand {:?}",
                                op
                            ),
                        };

                        let offset_value = toarg(offset_operand);

                        let align_value = toarg(align_operand);

                        let memidx_value = toarg(memidx_operand);

                        let memarg = MemArg {
                            offset: offset_value, // These can be mutated as well
                            align: align_value as u32,
                            memory_index: memidx_value as u32,
                        };

                        newfunc.instruction(&Instruction::I32Load8_U(memarg));
                    }
                    Lang::I32Load8U(operands) => {
                        let offset_operand = &nodes[usize::from(operands[1])];
                        let align_operand = &nodes[usize::from(operands[2])];
                        let memidx_operand = &nodes[usize::from(operands[3])];

                        let toarg = |op: &Lang| match op {
                            Lang::Arg(val) => *val as u64,
                            Lang::Const(val) => *val as u64,
                            _ => unreachable!(
                                "This operand should be an Arg node. Current operand {:?}",
                                op
                            ),
                        };

                        let offset_value = toarg(offset_operand);

                        let align_value = toarg(align_operand);

                        let memidx_value = toarg(memidx_operand);

                        let memarg = MemArg {
                            offset: offset_value, // These can be mutated as well
                            align: align_value as u32,
                            memory_index: memidx_value as u32,
                        };

                        newfunc.instruction(&Instruction::I32Load8_U(memarg));
                    }
                    Lang::I32Load16S(operands) => {
                        let offset_operand = &nodes[usize::from(operands[1])];
                        let align_operand = &nodes[usize::from(operands[2])];
                        let memidx_operand = &nodes[usize::from(operands[3])];

                        let toarg = |op: &Lang| match op {
                            Lang::Arg(val) => *val as u64,
                            Lang::Const(val) => *val as u64,
                            _ => unreachable!(
                                "This operand should be an Arg node. Current operand {:?}",
                                op
                            ),
                        };

                        let offset_value = toarg(offset_operand);

                        let align_value = toarg(align_operand);

                        let memidx_value = toarg(memidx_operand);

                        let memarg = MemArg {
                            offset: offset_value, // These can be mutated as well
                            align: align_value as u32,
                            memory_index: memidx_value as u32,
                        };

                        newfunc.instruction(&Instruction::I32Load16_S(memarg));
                    }
                    Lang::I32Load16U(operands) => {
                        let offset_operand = &nodes[usize::from(operands[1])];
                        let align_operand = &nodes[usize::from(operands[2])];
                        let memidx_operand = &nodes[usize::from(operands[3])];

                        let toarg = |op: &Lang| match op {
                            Lang::Arg(val) => *val as u64,
                            Lang::Const(val) => *val as u64,
                            _ => unreachable!(
                                "This operand should be an Arg node. Current operand {:?}",
                                op
                            ),
                        };

                        let offset_value = toarg(offset_operand);

                        let align_value = toarg(align_operand);

                        let memidx_value = toarg(memidx_operand);

                        let memarg = MemArg {
                            offset: offset_value, // These can be mutated as well
                            align: align_value as u32,
                            memory_index: memidx_value as u32,
                        };

                        newfunc.instruction(&Instruction::I32Load16_U(memarg));
                    }
                    Lang::I64Load8S(operands) => {
                        let offset_operand = &nodes[usize::from(operands[1])];
                        let align_operand = &nodes[usize::from(operands[2])];
                        let memidx_operand = &nodes[usize::from(operands[3])];

                        let toarg = |op: &Lang| match op {
                            Lang::Arg(val) => *val as u64,
                            Lang::Const(val) => *val as u64,
                            _ => unreachable!(
                                "This operand should be an Arg node. Current operand {:?}",
                                op
                            ),
                        };

                        let offset_value = toarg(offset_operand);

                        let align_value = toarg(align_operand);

                        let memidx_value = toarg(memidx_operand);

                        let memarg = MemArg {
                            offset: offset_value, // These can be mutated as well
                            align: align_value as u32,
                            memory_index: memidx_value as u32,
                        };

                        newfunc.instruction(&Instruction::I64Load8_S(memarg));
                    }
                    Lang::I64Load8U(operands) => {
                        let offset_operand = &nodes[usize::from(operands[1])];
                        let align_operand = &nodes[usize::from(operands[2])];
                        let memidx_operand = &nodes[usize::from(operands[3])];

                        let toarg = |op: &Lang| match op {
                            Lang::Arg(val) => *val as u64,
                            Lang::Const(val) => *val as u64,
                            _ => unreachable!(
                                "This operand should be an Arg node. Current operand {:?}",
                                op
                            ),
                        };

                        let offset_value = toarg(offset_operand);

                        let align_value = toarg(align_operand);

                        let memidx_value = toarg(memidx_operand);

                        let memarg = MemArg {
                            offset: offset_value, // These can be mutated as well
                            align: align_value as u32,
                            memory_index: memidx_value as u32,
                        };

                        newfunc.instruction(&Instruction::I64Load8_U(memarg));
                    }
                    Lang::I64Load16S(operands) => {
                        let offset_operand = &nodes[usize::from(operands[1])];
                        let align_operand = &nodes[usize::from(operands[2])];
                        let memidx_operand = &nodes[usize::from(operands[3])];

                        let toarg = |op: &Lang| match op {
                            Lang::Arg(val) => *val as u64,
                            Lang::Const(val) => *val as u64,
                            _ => unreachable!(
                                "This operand should be an Arg node. Current operand {:?}",
                                op
                            ),
                        };

                        let offset_value = toarg(offset_operand);

                        let align_value = toarg(align_operand);

                        let memidx_value = toarg(memidx_operand);

                        let memarg = MemArg {
                            offset: offset_value, // These can be mutated as well
                            align: align_value as u32,
                            memory_index: memidx_value as u32,
                        };

                        newfunc.instruction(&Instruction::I64Load16_S(memarg));
                    }
                    Lang::I64Load16U(operands) => {
                        let offset_operand = &nodes[usize::from(operands[1])];
                        let align_operand = &nodes[usize::from(operands[2])];
                        let memidx_operand = &nodes[usize::from(operands[3])];

                        let toarg = |op: &Lang| match op {
                            Lang::Arg(val) => *val as u64,
                            Lang::Const(val) => *val as u64,
                            _ => unreachable!(
                                "This operand should be an Arg node. Current operand {:?}",
                                op
                            ),
                        };

                        let offset_value = toarg(offset_operand);

                        let align_value = toarg(align_operand);

                        let memidx_value = toarg(memidx_operand);

                        let memarg = MemArg {
                            offset: offset_value, // These can be mutated as well
                            align: align_value as u32,
                            memory_index: memidx_value as u32,
                        };

                        newfunc.instruction(&Instruction::I64Load16_U(memarg));
                    }
                    Lang::I64Load32S(operands) => {
                        let offset_operand = &nodes[usize::from(operands[1])];
                        let align_operand = &nodes[usize::from(operands[2])];
                        let memidx_operand = &nodes[usize::from(operands[3])];

                        let toarg = |op: &Lang| match op {
                            Lang::Arg(val) => *val as u64,
                            Lang::Const(val) => *val as u64,
                            _ => unreachable!(
                                "This operand should be an Arg node. Current operand {:?}",
                                op
                            ),
                        };

                        let offset_value = toarg(offset_operand);

                        let align_value = toarg(align_operand);

                        let memidx_value = toarg(memidx_operand);

                        let memarg = MemArg {
                            offset: offset_value, // These can be mutated as well
                            align: align_value as u32,
                            memory_index: memidx_value as u32,
                        };

                        newfunc.instruction(&Instruction::I64Load32_S(memarg));
                    }
                    Lang::I64Load32U(operands) => {
                        let offset_operand = &nodes[usize::from(operands[1])];
                        let align_operand = &nodes[usize::from(operands[2])];
                        let memidx_operand = &nodes[usize::from(operands[3])];

                        let toarg = |op: &Lang| match op {
                            Lang::Arg(val) => *val as u64,
                            Lang::Const(val) => *val as u64,
                            _ => unreachable!(
                                "This operand should be an Arg node. Current operand {:?}",
                                op
                            ),
                        };

                        let offset_value = toarg(offset_operand);

                        let align_value = toarg(align_operand);

                        let memidx_value = toarg(memidx_operand);

                        let memarg = MemArg {
                            offset: offset_value, // These can be mutated as well
                            align: align_value as u32,
                            memory_index: memidx_value as u32,
                        };

                        newfunc.instruction(&Instruction::I64Load32_U(memarg));
                    }
                    Lang::F32Store(operands) => {
                        let offset_operand = &nodes[usize::from(operands[2])];
                        let align_operand = &nodes[usize::from(operands[3])];
                        let memidx_operand = &nodes[usize::from(operands[4])];

                        let toarg = |op: &Lang| match op {
                            Lang::Arg(val) => *val as u64,
                            Lang::Const(val) => *val as u64,
                            _ => unreachable!(
                                "This operand should be an Arg node. Current operand {:?}",
                                op
                            ),
                        };

                        let offset_value = toarg(offset_operand);

                        let align_value = toarg(align_operand);

                        let memidx_value = toarg(memidx_operand);

                        let memarg = MemArg {
                            offset: offset_value, // These can be mutated as well
                            align: align_value as u32,
                            memory_index: memidx_value as u32,
                        };

                        newfunc.instruction(&Instruction::F32Store(memarg));
                    }
                    Lang::F64Store(operands) => {
                        let offset_operand = &nodes[usize::from(operands[2])];
                        let align_operand = &nodes[usize::from(operands[3])];
                        let memidx_operand = &nodes[usize::from(operands[4])];

                        let toarg = |op: &Lang| match op {
                            Lang::Arg(val) => *val as u64,
                            Lang::Const(val) => *val as u64,
                            _ => unreachable!(
                                "This operand should be an Arg node. Current operand {:?}",
                                op
                            ),
                        };

                        let offset_value = toarg(offset_operand);

                        let align_value = toarg(align_operand);

                        let memidx_value = toarg(memidx_operand);

                        let memarg = MemArg {
                            offset: offset_value, // These can be mutated as well
                            align: align_value as u32,
                            memory_index: memidx_value as u32,
                        };

                        newfunc.instruction(&Instruction::F64Store(memarg));
                    }
                    Lang::I32Store8(operands) => {
                        let offset_operand = &nodes[usize::from(operands[2])];
                        let align_operand = &nodes[usize::from(operands[3])];
                        let memidx_operand = &nodes[usize::from(operands[4])];

                        let toarg = |op: &Lang| match op {
                            Lang::Arg(val) => *val as u64,
                            Lang::Const(val) => *val as u64,
                            _ => unreachable!(
                                "This operand should be an Arg node. Current operand {:?}",
                                op
                            ),
                        };

                        let offset_value = toarg(offset_operand);

                        let align_value = toarg(align_operand);

                        let memidx_value = toarg(memidx_operand);

                        let memarg = MemArg {
                            offset: offset_value, // These can be mutated as well
                            align: align_value as u32,
                            memory_index: memidx_value as u32,
                        };

                        newfunc.instruction(&Instruction::I32Store8(memarg));
                    }
                    Lang::I32Store16(operands) => {
                        let offset_operand = &nodes[usize::from(operands[2])];
                        let align_operand = &nodes[usize::from(operands[3])];
                        let memidx_operand = &nodes[usize::from(operands[4])];

                        let toarg = |op: &Lang| match op {
                            Lang::Arg(val) => *val as u64,
                            Lang::Const(val) => *val as u64,
                            _ => unreachable!(
                                "This operand should be an Arg node. Current operand {:?}",
                                op
                            ),
                        };

                        let offset_value = toarg(offset_operand);

                        let align_value = toarg(align_operand);

                        let memidx_value = toarg(memidx_operand);

                        let memarg = MemArg {
                            offset: offset_value, // These can be mutated as well
                            align: align_value as u32,
                            memory_index: memidx_value as u32,
                        };

                        newfunc.instruction(&Instruction::I32Store16(memarg));
                    }
                    Lang::I64Store8(operands) => {
                        let offset_operand = &nodes[usize::from(operands[2])];
                        let align_operand = &nodes[usize::from(operands[3])];
                        let memidx_operand = &nodes[usize::from(operands[4])];

                        let toarg = |op: &Lang| match op {
                            Lang::Arg(val) => *val as u64,
                            Lang::Const(val) => *val as u64,
                            _ => unreachable!(
                                "This operand should be an Arg node. Current operand {:?}",
                                op
                            ),
                        };

                        let offset_value = toarg(offset_operand);

                        let align_value = toarg(align_operand);

                        let memidx_value = toarg(memidx_operand);

                        let memarg = MemArg {
                            offset: offset_value, // These can be mutated as well
                            align: align_value as u32,
                            memory_index: memidx_value as u32,
                        };

                        newfunc.instruction(&Instruction::I64Store8(memarg));
                    }
                    Lang::I64Store16(operands) => {
                        let offset_operand = &nodes[usize::from(operands[2])];
                        let align_operand = &nodes[usize::from(operands[3])];
                        let memidx_operand = &nodes[usize::from(operands[4])];

                        let toarg = |op: &Lang| match op {
                            Lang::Arg(val) => *val as u64,
                            Lang::Const(val) => *val as u64,
                            _ => unreachable!(
                                "This operand should be an Arg node. Current operand {:?}",
                                op
                            ),
                        };

                        let offset_value = toarg(offset_operand);

                        let align_value = toarg(align_operand);

                        let memidx_value = toarg(memidx_operand);

                        let memarg = MemArg {
                            offset: offset_value, // These can be mutated as well
                            align: align_value as u32,
                            memory_index: memidx_value as u32,
                        };

                        newfunc.instruction(&Instruction::I64Store16(memarg));
                    }
                    Lang::I64Store32(operands) => {
                        let offset_operand = &nodes[usize::from(operands[2])];
                        let align_operand = &nodes[usize::from(operands[3])];
                        let memidx_operand = &nodes[usize::from(operands[4])];

                        let toarg = |op: &Lang| match op {
                            Lang::Arg(val) => *val as u64,
                            Lang::Const(val) => *val as u64,
                            _ => unreachable!(
                                "This operand should be an Arg node. Current operand {:?}",
                                op
                            ),
                        };

                        let offset_value = toarg(offset_operand);

                        let align_value = toarg(align_operand);

                        let memidx_value = toarg(memidx_operand);

                        let memarg = MemArg {
                            offset: offset_value, // These can be mutated as well
                            align: align_value as u32,
                            memory_index: memidx_value as u32,
                        };

                        newfunc.instruction(&Instruction::I64Store32(memarg));
                    }
                }
            }
        }
    }
    Ok(())
}
