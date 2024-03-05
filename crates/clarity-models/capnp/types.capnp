@0xec9da6286238cfc7;

using Std = import "std.capnp";

struct TypeSignature {
    union {
        noType @0 :Void;
        intType @1 :Void;
        uintType @2 :Void;
        boolType @3 :Void;
        sequenceType @4 :SequenceSubtype;
        principalType @5 :Void;
        tupleType @6 :TupleTypeSignature;
        optionalType @7 :TypeSignature;
        responseType @8 :Std.Tuple(TypeSignature, TypeSignature);
        callableType @9 :CallableSubtype;
        listUnionType @10 :List(CallableSubtype);
        traitReferenceType @11 :TraitIdentifier;
    }
}

struct StringSubtype {
    union {
        ascii @0 :UInt32;
        utf8 @1 :UInt32;
    }
}

struct StandardPrincipalData {
    version @0 :UInt8;
    address @1 :Data;
}

struct QualifiedContractIdentifier {
    issuer @0 :StandardPrincipalData;
    name @1 :Text;
}

struct PrincipalData {
    standard @0 :Std.Optional(StandardPrincipalData);
    contract @1 :Std.Optional(QualifiedContractIdentifier);
}

struct ContractIdentifier {
    relative @0 :Text;
    qualified @1 :QualifiedContractIdentifier;
}

struct CallableSubtype {
    principal @0 :QualifiedContractIdentifier;
    trait @1 :TraitIdentifier;
}

struct TraitIdentifier {
    name @0 :Text;
    contractIdentifier @1 :QualifiedContractIdentifier;
}

struct TupleTypeSignature {
    typeMap @0 :Std.Map(Text, TypeSignature);
}

struct SequenceSubtype {
    union {
        bufferType @0 :UInt32;
        listType @1 :ListTypeData;
        stringType @2 :StringSubtype;
    }
}

struct ListTypeData {
    maxLength @0 :UInt32;
    entryType @1 :TypeSignature;
}

struct DataMapMetadata {
    keyType @0 :TypeSignature;
    valueType @1 :TypeSignature;
}

struct DataVariableMetadata {
    valueType @0 :TypeSignature;
}

struct TraitDefinition {
    union {
        defined @0 :TraitIdentifier;
        imported @1 :TraitIdentifier;
    }
}

struct NonFungibleTokenMetadata {
    keyType @0 :TypeSignature;
}

struct FungibleTokenMetadata {
    totalSupply @0 :Std.Optional(Std.UInt128);
}