use egg::{define_language, Id};

// Language definition for a piece of Wasm
define_language! {
    pub enum Lang {
        // Define Wasm language here progressively
        "i32add" = I32Add([Id; 2]),
        "i64add" = I64Add([Id; 2]),
        "i32sub" = I32Sub([Id; 2]),
        "i32sub" = I64Sub([Id; 2]),
        "i32mul" = I32Mul([Id; 2]),
        "i64mul" = I64Mul([Id; 2]),
        "i32and" = I32And([Id; 2]),
        "i64and" = I64And([Id; 2]),
        "i32or" = I32Or([Id; 2]),
        "i64or" = I64Or([Id; 2]),
        "i32xor" = I32Xor([Id; 2]),
        "i64xor" = I64Xor([Id; 2]),
        "i32shl" = I32Shl([Id; 2]),
        "i64shl" = I64Shl([Id; 2]),
        "i32shr_u" = I32ShrU([Id; 2]),
        "i64shr_u" = I64ShrU([Id; 2]),
        "i32div_u" = I32DivU([Id; 2]),
        "i64div_u" = I64DivU([Id; 2]),
        "i32div_s" = I32DivS([Id; 2]),
        "i64div_s" = I64DivS([Id; 2]),
        "i32shr_s" = I32ShrS([Id; 2]),
        "i64shr_s" = I64ShrS([Id; 2]),
        "i32rotr" = I32RotR([Id; 2]),
        "i64rotr" = I64RotR([Id; 2]),
        "i32rotl" = I32RotL([Id; 2]),
        "i64rotl" = I64RotL([Id; 2]),
        "i32rem_s" = I32RemS([Id; 2]),
        "i64rem_s" = I64RemS([Id; 2]),
        "i32rem_u" = I32RemU([Id; 2]),
        "i64rem_u" = I64RemU([Id; 2]),
        // floats
        // binops
        "f32add" = F32Add([Id; 2]),
        "f64add" = F64Add([Id; 2]),
        "f32sub" = F32Sub([Id; 2]),
        "f64sub" = F64Sub([Id; 2]),
        "f32mul" = F32Mul([Id; 2]),
        "f64mul" = F64Mul([Id; 2]),
        "f32div" = F32Div([Id; 2]),
        "f64div" = F64Div([Id; 2]),
        "f32min" = F32Min([Id; 2]),
        "f64min" = F64Min([Id; 2]),
        "f32max" = F32Max([Id; 2]),
        "f64max" = F64Max([Id; 2]),
        "f32copysign" = F32Copysign([Id; 2]),
        "f64copysign" = F64Copysign([Id; 2]),
        // unops
        "f32abs" = F32Abs([Id; 1]),
        "f64abs" = F64Abs([Id; 1]),
        "f32neg" = F32Neg([Id; 1]),
        "f64neg" = F64Neg([Id; 1]),
        "f32sqrt" = F32Sqrt([Id; 1]),
        "f64sqrt" = F64Sqrt([Id; 1]),
        "f32ceil" = F32Ceil([Id; 1]),
        "f64ceil" = F64Ceil([Id; 1]),
        "f32floor" = F32Floor([Id; 1]),
        "f64floor" = F64Floor([Id; 1]),
        "f32trunc" = F32Trunc([Id; 1]),
        "f64trunc" = F64trunc([Id; 1]),
        "f32nearest" = F32Nearest([Id; 1]),
        "f64nearest" = F64Nearest([Id; 1]),
        // frelops
        "f32eq" = F32Eq([Id; 2]),
        "f64eq" = F64Eq([Id; 2]),
        "f32ne" = F32Ne([Id; 2]),
        "f64ne" = F64Ne([Id; 2]),
        "f32lt" = F32Lt([Id; 2]),
        "f64lt" = F64Lt([Id; 2]),
        "f32gt" = F32Gt([Id; 2]),
        "f64gt" = F64Gt([Id; 2]),
        "f32le" = F32Le([Id; 2]),
        "f64le" = F64Le([Id; 2]),
        "f32ge" = F32Ge([Id; 2]),
        "f64ge" = F64Ge([Id; 2]),

        // testop
        "i32eqz" = I32Eqz([Id; 1]),
        "i64eqz" = I64Eqz([Id; 1]),
        // relop
        "i32eq" = I32Eq([Id; 2]),
        "i64eq" = I64Eq([Id; 2]),
        "i32ne" = I32Ne([Id; 2]),
        "i64ne" = I64Ne([Id; 2]),

        "i32lt_s" = I32LtS([Id; 2]),
        "i64lt_s" = I64LtS([Id; 2]),
        "i32lt_u" = I32LtU([Id; 2]),
        "i64lt_u" = I64LtU([Id; 2]),

        "i32gt_s" = I32GtS([Id; 2]),
        "i64gt_s" = I64GtS([Id; 2]),

        "i32gt_u" = I32GtU([Id; 2]),
        "i64gt_u" = I64GtU([Id; 2]),
        "i32le_s" = I32LeS([Id; 2]),
        "i64le_s" = I64LeS([Id; 2]),

        "i32le_u" = I32LeU([Id; 2]),
        "i64le_u" = I64LeU([Id; 2]),
        "i32ge_s" = I32GeS([Id; 2]),
        "i64ge_s" = I64GeS([Id; 2]),
        "i32ge_u" = I32GeU([Id; 2]),
        "i64ge_u" = I64GeU([Id; 2]),

        "i32popcnt" = I32Popcnt([Id; 1]),
        "i64popcnt" = I64Popcnt([Id; 1]),

        "i32clz" = I32Clz([Id; 1]),
        "i32ctz" = I32Ctz([Id; 1]),
        "i64ctz" = I64Ctz([Id; 1]),
        "i64clz" = I64Clz([Id; 1]),
        "select" = Select([Id; 3]),

        // Locals
        // Idx and value
        "local_tee" = LocalTee([Id; 2]),
        // Idx and value
        "local_set" = LocalSet([Id; 2]),
        "local_get" = LocalGet([Id; 1]),

        // Globals
        // Idx and value
        "global_set" = GlobalSet([Id; 2]),
        "global_get" = GlobalGet([Id; 1]),
        // conversion operators
        "wrap" = Wrap([Id; 1]),

        // more conversion
        "i32extend8s" = I32Extend8S([Id; 1]),
        "i64extend8s" = I64Extend8S([Id; 1]),
        "i32extend16s" = I32Extend16S([Id; 1]),
        "i64extend16s" = I64Extend16S([Id; 1]),
        "i64extend32s" = I64Extend32S([Id; 1]),
        "i64extendi32s" = I64ExtendI32S([Id; 1]),
        "i64extendi32u" = I64ExtendI32U([Id; 1]),

        "i32truncf32s" = I32TruncF32S([Id; 1]),
        "i32truncf32u" = I32TruncF32U([Id; 1]),
        "i32truncf64s" = I32TruncF64S([Id; 1]),
        "i32truncf64u" = I32TruncF64U([Id; 1]),

        "i64truncf32s" = I64TruncF32S([Id; 1]),
        "i64truncf32u" = I64TruncF32U([Id; 1]),
        "i64truncf64s" = I64TruncF64S([Id; 1]),
        "i64truncf64u" = I64TruncF64U([Id; 1]),

        "f32converti32s" = F32ConvertI32S([Id; 1]),
        "f32converti32u" = F32ConvertI32U([Id; 1]),
        "f32converti64s" = F32ConvertI64S([Id; 1]),
        "f32converti64u" = F32ConvertI64U([Id; 1]),
        "f32demotef64" = F32DemoteF64([Id; 1]),
        "f64converti32s" = F64ConvertI32S([Id; 1]),
        "f64converti32u" = F64ConvertI32U([Id; 1]),
        "f64converti64s" = F64ConvertI64S([Id; 1]),
        "f64converti64u" = F64ConvertI64U([Id; 1]),
        "f64promotef32" = F64PromoteF32([Id; 1]),
        "i32reinterpretf32" = I32ReinterpretF32([Id; 1]),
        "i64reinterpretf64" = I64ReinterpretF64([Id; 1]),
        "f32reinterpreti32" = F32ReinterpretI32([Id; 1]),
        "f64reinterpreti64" = F64ReinterpretI64([Id; 1]),
        "i32truncsatf32s" = I32TruncSatF32S([Id; 1]),
        "i32truncsatf32u" = I32TruncSatF32U([Id; 1]),
        "i32truncsatf64s" = I32TruncSatF64S([Id; 1]),
        "i32truncsatf64u" = I32TruncSatF64U([Id; 1]),
        "i64truncsatf32s" = I64TruncSatF32S([Id; 1]),
        "i64truncsatf32u" = I64TruncSatF32U([Id; 1]),
        "i64truncsatf64s" = I64TruncSatF64S([Id; 1]),
        "i64truncsatf64u" = I64TruncSatF64U([Id; 1]),

        // The firsts Id should be the function index
        "call" = Call(Vec<Id>),
        "drop" = Drop([Id; 1]),
        // Memory operations
        "i32load" = I32Load([Id;4]),
        "i64load" = I64Load([Id;4]),
        "f32load" = F32Load([Id;4]),
        "f64load" = F64Load([Id;4]),
        "i32load8s" = I32Load8S([Id;4]),
        "i32load8u" = I32Load8U([Id;4]),
        "i32load16s" = I32Load16S([Id;4]),
        "i32load16u" = I32Load16U([Id;4]),
        "i64load8s" = I64Load8S([Id;4]),
        "i64load8u" = I64Load8U([Id;4]),
        "i64load16s" = I64Load16S([Id;4]),
        "i64load16u" = I64Load16U([Id;4]),
        "i64load32s" = I64Load32S([Id;4]),
        "i64load32u" = I64Load32U([Id;4]),

        "i32store" = I32Store([Id;5]),
        "i64store" = I64Store([Id;5]),

        "f32store" = F32Store([Id;5]),
        "f64store" = F64Store([Id;5]),

        "i32store8" = I32Store8([Id;5]),
        "i32store16" = I32Store16([Id;5]),

        "i64store8" = I64Store8([Id;5]),
        "i64store16" = I64Store16([Id;5]),

        "i64store32" = I64Store32([Id;5]),

        "memory_size"= MemorySize([Id; 1]),
        "memory_grow" = MemoryGrow([Id; 1]),

        // TODO add the others

        // Custom mutation operations and instructions
        //
        /*
            This operation represent a random number, if its used, every time is should represent the same random number
        */
        "i32rand" = RandI32,
        "i64rand" = RandI64,
        /*
            This instructions is used to define unknown operands, for example when the value can come from the join of several basic blocks in a dfg
        */
        "undef" = Undef,
        /*
            Takes one constant operand and turn it into a sum of two random numbers whihch sum is the operand `i32.const x = i32.const r + i32.const (x - r) `
        */
        "i32unfold" = UnfoldI32(Id),
        "i64unfold" = UnfoldI64(Id),
        // End of custom mutation operations and instructions

        Const(i64),
        I32(i32),
        I64(i64),
        // Save the bits of the constant
        F32(u32),
        F64(u64),

        // Use the following to internally pass arguments that dont need to be
        // parsed as number constants. Since variants will be parsed in order,
        // this wont be created directly from `parse`
        Arg(u64), // TODO, create this as a children-having instruction
    }
}

impl Default for Lang {
    fn default() -> Self {
        Lang::Undef
    }
}
