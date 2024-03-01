@0xf7167749bfdae2bb;

using UInt128 = import "std.capnp".UInt128;
using Int128 = import "std.capnp".Int128;
using Optional = import "std.capnp".Optional;
using Map = import "std.capnp".Map;
using Tuple = import "std.capnp".Tuple;
using Thruple = import "std.capnp".Thruple;

using Types = import "types.capnp";
using SymbolicExpression = import "expressions.capnp".SymbolicExpression;

struct DefinedFunction {
    identifier @0 :Text;
    name @1 :Text;
    argTypes @2 :List(Types.TypeSignature);
    defineType @3 :DefineType;
    arguments @4 :List(Text);
    body @5 :SymbolicExpression;
}

enum DefineType {
    readOnly @0;
    public @1;
    private @2;
}

struct FunctionSignature {
    args @0 :List(Types.TypeSignature);
    returns @1 :Types.TypeSignature;
}

struct FixedFunction {
    args @0 :List(FunctionArg);
    returns @1 :Types.TypeSignature;
}

struct FunctionArg {
    single @0 :Types.TypeSignature;
    union @1 :List(Types.TypeSignature);
}

struct FunctionReturnsSignature {
    typeOfArgAtPosition @0 :UInt8;
    fixed @1 :Types.TypeSignature;
}

struct FunctionType {
    union {
        variadic @0 :Tuple(Types.TypeSignature, Types.TypeSignature);
        fixed @1 :FixedFunction;
        unionArgs @2 :Tuple(List(Types.TypeSignature), Types.TypeSignature);
        arithmeticVariadic @3 :Void;
        arithmeticUnary @4 :Void;
        arithmeticBinary @5 :Void;
        arithmeticComparison @6 :Void;
        binary @7 :Thruple(FunctionArgSignature, FunctionArgSignature, FunctionReturnsSignature);
    }
}

struct FunctionArgSignature {
    union @0 :List(Types.TypeSignature);
    single @1 :Types.TypeSignature;
}