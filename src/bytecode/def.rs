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
            PushScope {
                args: {
                    src: VirtualRegister
                },
                str: "push_scope"
            },
            PopScope {
                args: {
                    dst: VirtualRegister
                },
                str: "pop_scope"
            },
            ResolveScope {
                args: {
                    #[doc="Register where to put scope"]
                    dst: VirtualRegister,
                    #[doc="Variable name"]
                    src: VirtualRegister,
                }
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
            },
            PutById {
                args: {
                    value: VirtualRegister,
                    base: VirtualRegister,
                    id: VirtualRegister,
                    fdbk: u32
                },
                str: "put_by_id"
            },
            GetById {
                args: {
                    dst: VirtualRegister,
                    base: VirtualRegister
                    id: VirtualRegister,
                    fdbk: u32
                },
                str: "get_by_id"
            },
            PutByVal {
                args: {
                    value: VirtualRegister,
                    base: VirtualRegister,
                    key: VirtualRegister
                },
                str: "put_by_val"
            },
            GetByVal {
                args: {
                    dst: VirtualRegister,
                    base: VirtualRegister,
                    key: VirtualRegister
                },
                str: "get_by_val"
            },
            DelById {
                args: {
                    dst: VirtualRegister,
                    base: VirtualRegister,
                    property: VirtualRegister
                },
                str: "del_by_id",
            },
            DelByVal {
                args: {
                    dst: VirtualRegister,
                    base: VirtualRegister,
                    val: VirtualRegister
                },
                str: "del_by_val"
            },
            PutSetterByVal {
                args: {
                    base: VirtualRegister,
                    property: VirtualRegister,
                    accessor: VirtualRegister
                },
                str: "put_setter_by_val"
            }
            PutGetterByVal {
                args: {
                    base: VirtualRegister,
                    property: VirtualRegister,
                    accessor: VirtualRegister
                },
                str: "put_getter_by_val"
            },
            Jmp {
                args: {
                    label: i32
                },
                str: "jmp"
            },
            JTrue {
                args: {
                    condition: VirtualRegister,
                    label: i32
                },
                str: "jtrue"
            },
            JFalse {
                args: {
                    condition: VirtualRegister,
                    label: i32
                },
                str: "jfalse"
            },
            JEQNull {
                args: {
                    value: VirtualRegister,
                    label: i32
                },
                str: "jeqnull"
            },
            JNEQNull {
                args: {
                    value: VirtualRegister,
                    label: i32
                },
                str: "jneqnull"
            },
            JUndefinedOrNull {
                args: {
                    value: VirtualRegister,
                    label: i32
                },
                str: "jundefornull"
            },
            JNUndefinedOrNull {
                args: {
                    value: VirtualRegister,
                    label: i32
                },
                str: "jnundefornull"
            },
            op_group => BinaryJmp {
                [
                    Jeq("jeq"),
                    JStrictEq("jstricteq"),
                    Jneq("jneq"),
                    JNStrictEq("jnstricteq"),
                    JLess("jless"),
                    JLessEq("jlesseq"),
                    JGreaterEq("jgreatereq"),
                    JNLess("jnless"),
                    JNLessEq("jnlesseq"),
                    JNGreater("jngreater"),
                    JNGreaterEq("jngreatereq"),
                    JBelow("jbelow"),
                    JBelowEq("jbeloweq")
                ],
                args: {
                    lhs: VirtualRegister,
                    rhs: VirtualRegister,
                    label: i32
                }
            }


        }
    };
}
