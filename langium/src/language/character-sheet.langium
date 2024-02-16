grammar CharacterSheet

entry Collection:
    baseDefinitions+=BaseDefinition*
    extensions+=ExtensionDefinition*;

BaseDefinition:
    'base' name=ID '{'
        attributes+=AttributeDefinition*
    '}';

AttributeDefinition:
    type=ID name=ID ':' ;

ExtensionDefinition:
    'extension' name=ID 'for' base=[BaseDefinition:ID]
    '{'
        attributes+=AttributeDefinition*
    '}';


hidden terminal WS: /\s+/;
terminal ID: /[_a-zA-Z][\w_]*/;
terminal INT returns number: /[0-9]+/;
terminal STRING: /'(\\.|[^'\\])*'/;

hidden terminal ML_COMMENT: /\/\*[\s\S]*?\*\//;
hidden terminal SL_COMMENT: /\/\/[^\n\r]*/;

