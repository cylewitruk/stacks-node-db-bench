@0xb5beee55937a1fe4;

using Tuple = import "std.capnp".Tuple;
using Value = import "clarity.capnp".Value;

using Types = import "types.capnp";

struct SymbolicExpression {
    expr @0 :SymbolicExpressionType;
    id @1 :UInt64;
}

struct SymbolicExpressionType {
    union {
        atomValue @0 :Value;
        atom @1 :Text;
        list @2 :List(List(SymbolicExpression));
        literalValue @3 :Value;
        field @4 :Types.TraitIdentifier;
        traitReference @5 :Tuple(Text, Types.TraitDefinition);
    }
}