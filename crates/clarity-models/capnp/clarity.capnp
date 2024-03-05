@0xd8399e829a59887f;

using UInt128 = import "std.capnp".UInt128;
using Int128 = import "std.capnp".Int128;
using Optional = import "std.capnp".Optional;
using Map = import "std.capnp".Map;
using Set = import "std.capnp".Set;

using Types = import "types.capnp";
using Funcs = import "functions.capnp";

enum ClarityVersion {
    clarityV1 @0;
    clarityV2 @1;
}

struct OptionalData {
    data @0 :Optional(Value);
}

struct ResponseData {
    committed @0 :Bool;
    data @1 :Value;
}

struct Value {
    union { 
        int @0 :Int128;
        uint @1 :UInt128;
        bool @2 :Bool;
        sequence @3 :SequenceData;
        principal @4 :Types.PrincipalData;
        tuple @5 :TupleData;
        optional @6 :Optional(Value);
        response @7 :ResponseData;
        callableContract @8 :CallableData;
    }
}

struct CallableData {
    contractIdentifier @0 :Types.QualifiedContractIdentifier;
    traitIdentifier @1 :Optional(Types.TraitIdentifier);
}

struct SequenceData {
    union {
        buffer @0 :Data;
        list @1 :ListData;
        string @2 :CharType;
    }
}

struct ListData {
    data @0 :List(Value);
    typeSignature @1 :Types.ListTypeData;
}

struct CharType {
    union {
        utf8 @0 :Data;
        ascii @1 :Data;
    }
}

struct TupleData {
    typeSignature @0 :Types.TupleTypeSignature;
    dataMap @1 :Map(Text, Value);
}

struct ContractContext {
    contractIdentifier @0 :Types.QualifiedContractIdentifier;
    variables @1 :Map(Text, Value);
    functions @2 :Map(Text, Funcs.DefinedFunction);
    definedTraits @3 :Map(Text, Map(Text, Funcs.FunctionSignature));
    implementedTraits @4 :Set(Types.TraitIdentifier);
    persistedNames @5 :Set(Text);
    metaDataMap @6 :Map(Text, Types.DataMapMetadata);
    metaDataVar @7 :Map(Text, Types.DataVariableMetadata);
    metaNft @8 :Map(Text, Types.NonFungibleTokenMetadata);
    metaFt @9 :Map(Text, Types.FungibleTokenMetadata);
    dataSize @10 :UInt64;
    clarityVersion @11 :ClarityVersion;
}