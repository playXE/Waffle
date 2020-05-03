use super::*;
use virtualregister::*;
macro_rules! op_list {
    ($m: ident) => {
        $m! {
            GetScope: {
                args: {
                    dst: VirtualRegister
                },
                str: "get_scope"
            },
            GetArgument: {
                args: {
                    dst: VirtualRegister,
                    index: i32
                },
                str: "get_argument"
            },
            ArgumentCount {
                args: {
                    dst: VirtualRegister,
                },
                str: "argument_count"
            },
            Mov {
                args: {
                    dst: VirtualRegister,
                    src: VirtualRegister
                },
                str: "mov"
            },

            op_group => Binary {
                [ Eq("eq"),
                  Neq("neq"),
                  StrictEq("stricteq"),
                  StrictNeq("nstricteq"),
                  Less("less"),
                  LessEq("lesseq"),
                  Greater("greater"),
                  GreaterEq("greatereq"),
                  Below("below"),
                  BelowEq("beloweq"),
                  Mod("mod"),
                  Pow("pow"),
                  LShift("lshift"),
                  RShift("rshift"),
                  URshift("urshift")
                ],
                args: {
                    dst: VirtualRegister,
                    lhs: VirtualRegister,
                    rhs: VirtualRegister
                },
            }
            op_group => ProfiledBinary {
                [
                    Add("add"),
                    Sub("sub"),
                    Div("div"),
                    Mul("mul"),
                    BitAnd("bitand"),
                    BitOr("bitor"),
                    BitXor("bitxor")
                ],
                args: {
                    dst: VirtualRegister,
                    lhs: VirtualRegister,
                    rhs: VirtualRegister,
                    fdbk: u32
                }
            },
            BitNot {
                args: {
                    dst: VirtualRegister,
                    operand: VirtualRegister
                },
                str: "bitnot"
            },
            op_group => UnaryOp {
                [
                    EqNull("eq_null"),
                    NeqNull("neq_null"),
                    ToString("to_string"),
                    Unsigned("unsigned"),
                    IsEmpty("is_empty"),
                    IsUndefined("is_undefined"),
                    IsUndefOrNull("is_undefiner_or_null"),
                    IsBoolean("is_boolean"),
                    IsNumber("is_number"),
                    IsObject("is_object"),
                    IsFunction("is_function")
                ],
                args: {
                    dst: VirtualRegister,
                    src: VirtualRegister
                }
            }
            Inc {
                args: {
                    srcdst: VirtualRegister
                },
                str: "inc"
            },
            Dec {
                args: {
                    srcdst: VirtualRegister
                },
                str: "dec"
            }

        }
    };
}
