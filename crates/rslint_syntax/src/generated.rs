//! Generated file, do not edit by hand, see `xtask/src/codegen`

#![allow(clippy::all)]
#![allow(bad_style, missing_docs, unreachable_pub)]
#[doc = r" The kind of syntax node, e.g. `IDENT`, `FUNCTION_KW`, or `FOR_STMT`."]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum JsSyntaxKind {
	#[doc(hidden)]
	TOMBSTONE,
	#[doc = r" Marks the end of the file.May have trivia attached"]
	EOF,
	SEMICOLON,
	COMMA,
	L_PAREN,
	R_PAREN,
	L_CURLY,
	R_CURLY,
	L_BRACK,
	R_BRACK,
	L_ANGLE,
	R_ANGLE,
	TILDE,
	QUESTION,
	QUESTION2,
	QUESTIONDOT,
	AMP,
	PIPE,
	PLUS,
	PLUS2,
	STAR,
	STAR2,
	SLASH,
	CARET,
	PERCENT,
	DOT,
	DOT2,
	COLON,
	EQ,
	EQ2,
	EQ3,
	FAT_ARROW,
	BANG,
	NEQ,
	NEQ2,
	MINUS,
	MINUS2,
	LTEQ,
	GTEQ,
	PLUSEQ,
	MINUSEQ,
	PIPEEQ,
	AMPEQ,
	CARETEQ,
	SLASHEQ,
	STAREQ,
	PERCENTEQ,
	AMP2,
	PIPE2,
	SHL,
	SHR,
	USHR,
	SHLEQ,
	SHREQ,
	USHREQ,
	AMP2EQ,
	PIPE2EQ,
	STAR2EQ,
	QUESTION2EQ,
	AT,
	BACKTICK,
	AWAIT_KW,
	BREAK_KW,
	CASE_KW,
	CATCH_KW,
	CLASS_KW,
	CONST_KW,
	CONTINUE_KW,
	DEBUGGER_KW,
	DEFAULT_KW,
	DELETE_KW,
	DO_KW,
	ELSE_KW,
	ENUM_KW,
	EXPORT_KW,
	EXTENDS_KW,
	FALSE_KW,
	FINALLY_KW,
	FOR_KW,
	FUNCTION_KW,
	IF_KW,
	IN_KW,
	INSTANCEOF_KW,
	INTERFACE_KW,
	IMPORT_KW,
	IMPLEMENTS_KW,
	NEW_KW,
	NULL_KW,
	PACKAGE_KW,
	PRIVATE_KW,
	PROTECTED_KW,
	PUBLIC_KW,
	RETURN_KW,
	SUPER_KW,
	SWITCH_KW,
	THIS_KW,
	THROW_KW,
	TRY_KW,
	TRUE_KW,
	TYPEOF_KW,
	VAR_KW,
	VOID_KW,
	WHILE_KW,
	WITH_KW,
	YIELD_KW,
	READONLY_KW,
	KEYOF_KW,
	UNIQUE_KW,
	DECLARE_KW,
	ABSTRACT_KW,
	STATIC_KW,
	ASYNC_KW,
	TYPE_KW,
	FROM_KW,
	AS_KW,
	REQUIRE_KW,
	NAMESPACE_KW,
	ASSERT_KW,
	MODULE_KW,
	GLOBAL_KW,
	INFER_KW,
	GET_KW,
	SET_KW,
	OF_KW,
	TARGET_KW,
	NEVER_KW,
	UNKNOWN_KW,
	ANY_KW,
	UNDEFINED_KW,
	LET_KW,
	FLOAT_KW,
	NUMBER_KW,
	META_KW,
	JS_NUMBER_LITERAL,
	JS_BIG_INT_LITERAL,
	JS_STRING_LITERAL,
	JS_REGEX_LITERAL,
	HASH,
	TEMPLATE_CHUNK,
	DOLLAR_CURLY,
	ERROR_TOKEN,
	IDENT,
	WHITESPACE,
	COMMENT,
	JS_SHEBANG,
	JS_MODULE,
	JS_MODULE_ITEM_LIST,
	JS_SCRIPT,
	JS_EXPRESSION_SNIPPED,
	JS_DIRECTIVE,
	JS_DIRECTIVE_LIST,
	JS_STATEMENT_LIST,
	JS_BLOCK_STATEMENT,
	JS_FUNCTION_BODY,
	JS_VARIABLE_STATEMENT,
	JS_VARIABLE_DECLARATIONS,
	JS_VARIABLE_DECLARATION_LIST,
	JS_VARIABLE_DECLARATION,
	JS_INITIALIZER_CLAUSE,
	JS_EMPTY_STATEMENT,
	JS_EXPRESSION_STATEMENT,
	JS_IF_STATEMENT,
	JS_ELSE_CLAUSE,
	JS_DO_WHILE_STATEMENT,
	JS_WHILE_STATEMENT,
	FOR_STMT,
	JS_FOR_IN_STATEMENT,
	JS_FOR_OF_STATEMENT,
	JS_FOR_VARIABLE_DECLARATION,
	JS_CONTINUE_STATEMENT,
	JS_BREAK_STATEMENT,
	JS_RETURN_STATEMENT,
	JS_WITH_STATEMENT,
	JS_SWITCH_STATEMENT,
	JS_SWITCH_CASE_LIST,
	JS_CASE_CLAUSE,
	JS_DEFAULT_CLAUSE,
	JS_LABELED_STATEMENT,
	JS_THROW_STATEMENT,
	JS_TRY_STATEMENT,
	JS_TRY_FINALLY_STATEMENT,
	JS_CATCH_CLAUSE,
	JS_CATCH_DECLARATION,
	JS_FINALLY_CLAUSE,
	JS_DEBUGGER_STATEMENT,
	JS_FUNCTION_DECLARATION,
	JS_PARAMETERS,
	JS_PARAMETER_LIST,
	JS_REST_PARAMETER,
	TS_TYPE_ANNOTATION,
	JS_IDENTIFIER_BINDING,
	JS_IDENTIFIER_EXPRESSION,
	JS_REFERENCE_IDENTIFIER,
	JS_NAME,
	JS_PRIVATE_NAME,
	JS_THIS_EXPRESSION,
	JS_ARRAY_EXPRESSION,
	JS_ARRAY_ELEMENT_LIST,
	JS_ARRAY_HOLE,
	JS_COMPUTED_MEMBER_NAME,
	JS_LITERAL_MEMBER_NAME,
	JS_OBJECT_EXPRESSION,
	JS_OBJECT_MEMBER_LIST,
	JS_PROPERTY_OBJECT_MEMBER,
	JS_GETTER_OBJECT_MEMBER,
	JS_SETTER_OBJECT_MEMBER,
	JS_METHOD_OBJECT_MEMBER,
	JS_SUPER_EXPRESSION,
	JS_PARENTHESIZED_EXPRESSION,
	NEW_EXPR,
	JS_FUNCTION_EXPRESSION,
	JS_STATIC_MEMBER_EXPRESSION,
	JS_COMPUTED_MEMBER_EXPRESSION,
	CALL_EXPR,
	JS_UNARY_EXPRESSION,
	JS_PRE_UPDATE_EXPRESSION,
	JS_POST_UPDATE_EXPRESSION,
	JS_BINARY_EXPRESSION,
	JS_LOGICAL_EXPRESSION,
	JS_CONDITIONAL_EXPRESSION,
	JS_ASSIGNMENT_EXPRESSION,
	JS_SEQUENCE_EXPRESSION,
	JS_CALL_ARGUMENTS,
	JS_CALL_ARGUMENT_LIST,
	JS_STRING_LITERAL_EXPRESSION,
	JS_NUMBER_LITERAL_EXPRESSION,
	JS_BIG_INT_LITERAL_EXPRESSION,
	JS_BOOLEAN_LITERAL_EXPRESSION,
	JS_NULL_LITERAL_EXPRESSION,
	JS_REGEX_LITERAL_EXPRESSION,
	TEMPLATE,
	TEMPLATE_ELEMENT,
	TEMPLATE_CHUNK_ELEMENT,
	TEMPLATE_ELEMENT_LIST,
	JS_IMPORT_CALL_EXPRESSION,
	NEW_TARGET,
	IMPORT_META,
	JS_SHORTHAND_PROPERTY_OBJECT_MEMBER,
	JS_SPREAD,
	JS_OBJECT_BINDING_PATTERN,
	JS_ARRAY_BINDING_PATTERN,
	JS_ARRAY_BINDING_PATTERN_ELEMENT_LIST,
	JS_BINDING_PATTERN_WITH_DEFAULT,
	JS_ARRAY_BINDING_PATTERN_REST_ELEMENT,
	JS_OBJECT_BINDING_PATTERN_PROPERTY_LIST,
	JS_OBJECT_BINDING_PATTERN_REST,
	JS_OBJECT_BINDING_PATTERN_PROPERTY,
	JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY,
	JS_ARROW_FUNCTION_EXPRESSION,
	JS_YIELD_EXPRESSION,
	JS_CLASS_DECLARATION,
	JS_CLASS_EXPRESSION,
	JS_CLASS_MEMBER_LIST,
	JS_EXTENDS_CLAUSE,
	JS_PRIVATE_CLASS_MEMBER_NAME,
	JS_CONSTRUCTOR_CLASS_MEMBER,
	JS_CONSTRUCTOR_PARAMETER_LIST,
	JS_CONSTRUCTOR_PARAMETERS,
	JS_CONSTRUCTOR_PARAMETER,
	JS_PROPERTY_CLASS_MEMBER,
	JS_METHOD_CLASS_MEMBER,
	JS_GETTER_CLASS_MEMBER,
	JS_SETTER_CLASS_MEMBER,
	JS_EMPTY_CLASS_MEMBER,
	JS_ASSIGNMENT_WITH_DEFAULT,
	JS_PARENTHESIZED_ASSIGNMENT,
	JS_IDENTIFIER_ASSIGNMENT,
	JS_STATIC_MEMBER_ASSIGNMENT,
	JS_COMPUTED_MEMBER_ASSIGNMENT,
	JS_ARRAY_ASSIGNMENT_PATTERN,
	JS_ARRAY_ASSIGNMENT_PATTERN_ELEMENT_LIST,
	JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT,
	JS_OBJECT_ASSIGNMENT_PATTERN,
	JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY_LIST,
	JS_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY,
	JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY,
	JS_OBJECT_ASSIGNMENT_PATTERN_REST,
	JS_IMPORT,
	JS_IMPORT_BARE_CLAUSE,
	JS_IMPORT_DEFAULT_CLAUSE,
	JS_IMPORT_NAMESPACE_CLAUSE,
	JS_IMPORT_NAMED_CLAUSE,
	JS_NAMED_IMPORT_SPECIFIERS,
	JS_NAMED_IMPORT_SPECIFIER_LIST,
	JS_NAMESPACE_IMPORT_SPECIFIER,
	JS_DEFAULT_IMPORT_SPECIFIER,
	JS_NAMED_IMPORT_SPECIFIER,
	JS_SHORTHAND_NAMED_IMPORT_SPECIFIER,
	JS_IMPORT_ASSERTION,
	JS_IMPORT_ASSERTION_ENTRY_LIST,
	JS_IMPORT_ASSERTION_ENTRY,
	JS_MODULE_SOURCE,
	JS_LITERAL_EXPORT_NAME,
	EXPORT_DECL,
	EXPORT_NAMED,
	EXPORT_NAMED_SPECIFIER_LIST,
	SPECIFIER,
	EXPORT_DEFAULT_DECL,
	EXPORT_DEFAULT_EXPR,
	EXPORT_WILDCARD,
	JS_AWAIT_EXPRESSION,
	FOR_STMT_TEST,
	FOR_STMT_UPDATE,
	FOR_STMT_INIT,
	TS_ANY,
	TS_UNKNOWN,
	TS_NUMBER,
	TS_OBJECT,
	TS_BOOLEAN,
	TS_BIGINT,
	TS_STRING,
	TS_SYMBOL,
	TS_VOID,
	TS_UNDEFINED,
	TS_NULL,
	TS_NEVER,
	TS_THIS,
	TS_LITERAL,
	TS_PREDICATE,
	TS_TUPLE,
	TS_TUPLE_ELEMENT,
	TS_PAREN,
	TS_TYPE_REF,
	TS_QUALIFIED_PATH,
	TS_TYPE_NAME,
	TS_TEMPLATE,
	TS_TEMPLATE_ELEMENT,
	TS_MAPPED_TYPE,
	TS_MAPPED_TYPE_PARAM,
	TS_MAPPED_TYPE_READONLY,
	TS_TYPE_QUERY,
	TS_TYPE_QUERY_EXPR,
	TS_IMPORT,
	TS_TYPE_ARGS,
	TS_TYPE_ARG_LIST,
	TS_ARRAY,
	TS_INDEXED_ARRAY,
	TS_TYPE_OPERATOR,
	TS_INTERSECTION,
	TS_UNION,
	TS_TYPE_PARAM_LIST,
	TS_TYPE_PARAMS,
	TS_FN_TYPE,
	TS_CONSTRUCTOR_TYPE,
	TS_IMPLEMENTS_CLAUSE,
	TS_TYPE_LIST,
	TS_EXTENDS,
	TS_CONDITIONAL_TYPE,
	TS_CONSTRAINT,
	TS_DEFAULT,
	TS_TYPE_PARAM,
	TS_NON_NULL,
	TS_ASSERTION,
	TS_CONST_ASSERTION,
	TS_ENUM,
	TS_ENUM_MEMBER_LIST,
	TS_ENUM_MEMBER,
	TS_TYPE_ALIAS_DECL,
	TS_NAMESPACE_DECL,
	TS_MODULE_BLOCK,
	TS_MODULE_DECL,
	TS_CONSTRUCTOR_PARAM,
	TS_CALL_SIGNATURE_DECL,
	TS_CONSTRUCT_SIGNATURE_DECL,
	TS_INDEX_SIGNATURE,
	TS_METHOD_SIGNATURE,
	TS_PROPERTY_SIGNATURE,
	TS_INTERFACE_DECL,
	TS_OBJECT_TYPE,
	TS_OBJECT_MEMBER_LIST,
	TS_EXPR_WITH_TYPE_ARGS,
	TS_IMPORT_EQUALS_DECL,
	TS_MODULE_REF,
	TS_EXTERNAL_MODULE_REF,
	TS_EXPORT_ASSIGNMENT,
	TS_NAMESPACE_EXPORT_DECL,
	TS_DECORATOR,
	TS_INFER,
	NULL,
	UNDEFINED,
	TS_ENTITY_NAME,
	BOOLEAN,
	BIG_INT_VALUE,
	JS_UNKNOWN,
	JS_UNKNOWN_EXPRESSION,
	JS_UNKNOWN_STATEMENT,
	JS_UNKNOWN_MEMBER,
	JS_UNKNOWN_BINDING,
	JS_UNKNOWN_MODIFIER,
	JS_UNKNOWN_IMPORT_ASSERTION_ENTRY,
	JS_UNKNOWN_NAMED_IMPORT_SPECIFIER,
	JS_UNKNOWN_ASSIGNMENT,
	#[doc(hidden)]
	__LAST,
}
use self::JsSyntaxKind::*;
impl JsSyntaxKind {
	pub fn is_keyword(self) -> bool {
		match self {
			AWAIT_KW | BREAK_KW | CASE_KW | CATCH_KW | CLASS_KW | CONST_KW | CONTINUE_KW
			| DEBUGGER_KW | DEFAULT_KW | DELETE_KW | DO_KW | ELSE_KW | ENUM_KW | EXPORT_KW
			| EXTENDS_KW | FALSE_KW | FINALLY_KW | FOR_KW | FUNCTION_KW | IF_KW | IN_KW
			| INSTANCEOF_KW | INTERFACE_KW | IMPORT_KW | IMPLEMENTS_KW | NEW_KW | NULL_KW
			| PACKAGE_KW | PRIVATE_KW | PROTECTED_KW | PUBLIC_KW | RETURN_KW | SUPER_KW
			| SWITCH_KW | THIS_KW | THROW_KW | TRY_KW | TRUE_KW | TYPEOF_KW | VAR_KW | VOID_KW
			| WHILE_KW | WITH_KW | YIELD_KW | READONLY_KW | KEYOF_KW | UNIQUE_KW | DECLARE_KW
			| ABSTRACT_KW | STATIC_KW | ASYNC_KW | TYPE_KW | FROM_KW | AS_KW | REQUIRE_KW
			| NAMESPACE_KW | ASSERT_KW | MODULE_KW | GLOBAL_KW | INFER_KW | GET_KW | SET_KW
			| OF_KW | TARGET_KW | NEVER_KW | UNKNOWN_KW | ANY_KW | UNDEFINED_KW | LET_KW
			| FLOAT_KW | NUMBER_KW | META_KW => true,
			_ => false,
		}
	}
	pub fn is_punct(self) -> bool {
		match self {
			SEMICOLON | COMMA | L_PAREN | R_PAREN | L_CURLY | R_CURLY | L_BRACK | R_BRACK
			| L_ANGLE | R_ANGLE | TILDE | QUESTION | QUESTION2 | QUESTIONDOT | AMP | PIPE
			| PLUS | PLUS2 | STAR | STAR2 | SLASH | CARET | PERCENT | DOT | DOT2 | COLON | EQ
			| EQ2 | EQ3 | FAT_ARROW | BANG | NEQ | NEQ2 | MINUS | MINUS2 | LTEQ | GTEQ | PLUSEQ
			| MINUSEQ | PIPEEQ | AMPEQ | CARETEQ | SLASHEQ | STAREQ | PERCENTEQ | AMP2 | PIPE2
			| SHL | SHR | USHR | SHLEQ | SHREQ | USHREQ | AMP2EQ | PIPE2EQ | STAR2EQ
			| QUESTION2EQ | AT | BACKTICK => true,
			_ => false,
		}
	}
	pub fn is_literal(self) -> bool {
		match self {
			JS_NUMBER_LITERAL | JS_BIG_INT_LITERAL | JS_STRING_LITERAL | JS_REGEX_LITERAL => true,
			_ => false,
		}
	}
	pub fn is_before_expr(self) -> bool {
		match self {
			BANG | L_PAREN | L_BRACK | L_CURLY | SEMICOLON | COMMA | COLON | QUESTION | PLUS2
			| MINUS2 | TILDE | CASE_KW | DEFAULT_KW | DO_KW | ELSE_KW | RETURN_KW | THROW_KW
			| NEW_KW | EXTENDS_KW | YIELD_KW | IN_KW | TYPEOF_KW | VOID_KW | DELETE_KW | PLUSEQ
			| MINUSEQ | PIPEEQ | AMPEQ | CARETEQ | SLASHEQ | STAREQ | PERCENTEQ | AMP2 | PIPE2
			| SHLEQ | SHREQ | USHREQ | EQ | FAT_ARROW | MINUS | PLUS => true,
			_ => false,
		}
	}
	pub fn from_keyword(ident: &str) -> Option<JsSyntaxKind> {
		let kw = match ident {
			"await" => AWAIT_KW,
			"break" => BREAK_KW,
			"case" => CASE_KW,
			"catch" => CATCH_KW,
			"class" => CLASS_KW,
			"const" => CONST_KW,
			"continue" => CONTINUE_KW,
			"debugger" => DEBUGGER_KW,
			"default" => DEFAULT_KW,
			"delete" => DELETE_KW,
			"do" => DO_KW,
			"else" => ELSE_KW,
			"enum" => ENUM_KW,
			"export" => EXPORT_KW,
			"extends" => EXTENDS_KW,
			"false" => FALSE_KW,
			"finally" => FINALLY_KW,
			"for" => FOR_KW,
			"function" => FUNCTION_KW,
			"if" => IF_KW,
			"in" => IN_KW,
			"instanceof" => INSTANCEOF_KW,
			"interface" => INTERFACE_KW,
			"import" => IMPORT_KW,
			"implements" => IMPLEMENTS_KW,
			"new" => NEW_KW,
			"null" => NULL_KW,
			"package" => PACKAGE_KW,
			"private" => PRIVATE_KW,
			"protected" => PROTECTED_KW,
			"public" => PUBLIC_KW,
			"return" => RETURN_KW,
			"super" => SUPER_KW,
			"switch" => SWITCH_KW,
			"this" => THIS_KW,
			"throw" => THROW_KW,
			"try" => TRY_KW,
			"true" => TRUE_KW,
			"typeof" => TYPEOF_KW,
			"var" => VAR_KW,
			"void" => VOID_KW,
			"while" => WHILE_KW,
			"with" => WITH_KW,
			"yield" => YIELD_KW,
			"readonly" => READONLY_KW,
			"keyof" => KEYOF_KW,
			"unique" => UNIQUE_KW,
			"declare" => DECLARE_KW,
			"abstract" => ABSTRACT_KW,
			"static" => STATIC_KW,
			"async" => ASYNC_KW,
			"type" => TYPE_KW,
			"from" => FROM_KW,
			"as" => AS_KW,
			"require" => REQUIRE_KW,
			"namespace" => NAMESPACE_KW,
			"assert" => ASSERT_KW,
			"module" => MODULE_KW,
			"global" => GLOBAL_KW,
			"infer" => INFER_KW,
			"get" => GET_KW,
			"set" => SET_KW,
			"of" => OF_KW,
			"target" => TARGET_KW,
			"never" => NEVER_KW,
			"unknown" => UNKNOWN_KW,
			"any" => ANY_KW,
			"undefined" => UNDEFINED_KW,
			"let" => LET_KW,
			"float" => FLOAT_KW,
			"number" => NUMBER_KW,
			"meta" => META_KW,
			_ => return None,
		};
		Some(kw)
	}
	pub fn from_char(c: char) -> Option<JsSyntaxKind> {
		let tok = match c {
			';' => SEMICOLON,
			',' => COMMA,
			'(' => L_PAREN,
			')' => R_PAREN,
			'{' => L_CURLY,
			'}' => R_CURLY,
			'[' => L_BRACK,
			']' => R_BRACK,
			'<' => L_ANGLE,
			'>' => R_ANGLE,
			'~' => TILDE,
			'?' => QUESTION,
			'&' => AMP,
			'|' => PIPE,
			'+' => PLUS,
			'*' => STAR,
			'/' => SLASH,
			'^' => CARET,
			'%' => PERCENT,
			'.' => DOT,
			':' => COLON,
			'=' => EQ,
			'!' => BANG,
			'-' => MINUS,
			'@' => AT,
			'`' => BACKTICK,
			_ => return None,
		};
		Some(tok)
	}
	pub fn to_string(&self) -> Option<&str> {
		let tok = match self {
			SEMICOLON => ";",
			COMMA => ",",
			L_PAREN => "'('",
			R_PAREN => "')'",
			L_CURLY => "'{'",
			R_CURLY => "'}'",
			L_BRACK => "'['",
			R_BRACK => "']'",
			L_ANGLE => "<",
			R_ANGLE => ">",
			TILDE => "~",
			QUESTION => "?",
			QUESTION2 => "??",
			QUESTIONDOT => "?.",
			AMP => "&",
			PIPE => "|",
			PLUS => "+",
			PLUS2 => "++",
			STAR => "*",
			STAR2 => "**",
			SLASH => "/",
			CARET => "^",
			PERCENT => "%",
			DOT => ".",
			DOT2 => "...",
			COLON => ":",
			EQ => "=",
			EQ2 => "==",
			EQ3 => "===",
			FAT_ARROW => "=>",
			BANG => "!",
			NEQ => "!=",
			NEQ2 => "!==",
			MINUS => "-",
			MINUS2 => "--",
			LTEQ => "<=",
			GTEQ => ">=",
			PLUSEQ => "+=",
			MINUSEQ => "-=",
			PIPEEQ => "|=",
			AMPEQ => "&=",
			CARETEQ => "^=",
			SLASHEQ => "/=",
			STAREQ => "*=",
			PERCENTEQ => "%=",
			AMP2 => "&&",
			PIPE2 => "||",
			SHL => "<<",
			SHR => ">>",
			USHR => ">>>",
			SHLEQ => "<<=",
			SHREQ => ">>=",
			USHREQ => ">>>=",
			AMP2EQ => "&&=",
			PIPE2EQ => "||=",
			STAR2EQ => "**=",
			QUESTION2EQ => "??=",
			AT => "@",
			BACKTICK => "'`'",
			JS_STRING_LITERAL => "string literal",
			_ => return None,
		};
		Some(tok)
	}
}
#[doc = r" Utility macro for creating a SyntaxKind through simple macro syntax"]
#[macro_export]
macro_rules ! T { [;] => { $ crate :: JsSyntaxKind :: SEMICOLON } ; [,] => { $ crate :: JsSyntaxKind :: COMMA } ; ['('] => { $ crate :: JsSyntaxKind :: L_PAREN } ; [')'] => { $ crate :: JsSyntaxKind :: R_PAREN } ; ['{'] => { $ crate :: JsSyntaxKind :: L_CURLY } ; ['}'] => { $ crate :: JsSyntaxKind :: R_CURLY } ; ['['] => { $ crate :: JsSyntaxKind :: L_BRACK } ; [']'] => { $ crate :: JsSyntaxKind :: R_BRACK } ; [<] => { $ crate :: JsSyntaxKind :: L_ANGLE } ; [>] => { $ crate :: JsSyntaxKind :: R_ANGLE } ; [~] => { $ crate :: JsSyntaxKind :: TILDE } ; [?] => { $ crate :: JsSyntaxKind :: QUESTION } ; [??] => { $ crate :: JsSyntaxKind :: QUESTION2 } ; [?.] => { $ crate :: JsSyntaxKind :: QUESTIONDOT } ; [&] => { $ crate :: JsSyntaxKind :: AMP } ; [|] => { $ crate :: JsSyntaxKind :: PIPE } ; [+] => { $ crate :: JsSyntaxKind :: PLUS } ; [++] => { $ crate :: JsSyntaxKind :: PLUS2 } ; [*] => { $ crate :: JsSyntaxKind :: STAR } ; [**] => { $ crate :: JsSyntaxKind :: STAR2 } ; [/] => { $ crate :: JsSyntaxKind :: SLASH } ; [^] => { $ crate :: JsSyntaxKind :: CARET } ; [%] => { $ crate :: JsSyntaxKind :: PERCENT } ; [.] => { $ crate :: JsSyntaxKind :: DOT } ; [...] => { $ crate :: JsSyntaxKind :: DOT2 } ; [:] => { $ crate :: JsSyntaxKind :: COLON } ; [=] => { $ crate :: JsSyntaxKind :: EQ } ; [==] => { $ crate :: JsSyntaxKind :: EQ2 } ; [===] => { $ crate :: JsSyntaxKind :: EQ3 } ; [=>] => { $ crate :: JsSyntaxKind :: FAT_ARROW } ; [!] => { $ crate :: JsSyntaxKind :: BANG } ; [!=] => { $ crate :: JsSyntaxKind :: NEQ } ; [!==] => { $ crate :: JsSyntaxKind :: NEQ2 } ; [-] => { $ crate :: JsSyntaxKind :: MINUS } ; [--] => { $ crate :: JsSyntaxKind :: MINUS2 } ; [<=] => { $ crate :: JsSyntaxKind :: LTEQ } ; [>=] => { $ crate :: JsSyntaxKind :: GTEQ } ; [+=] => { $ crate :: JsSyntaxKind :: PLUSEQ } ; [-=] => { $ crate :: JsSyntaxKind :: MINUSEQ } ; [|=] => { $ crate :: JsSyntaxKind :: PIPEEQ } ; [&=] => { $ crate :: JsSyntaxKind :: AMPEQ } ; [^=] => { $ crate :: JsSyntaxKind :: CARETEQ } ; [/=] => { $ crate :: JsSyntaxKind :: SLASHEQ } ; [*=] => { $ crate :: JsSyntaxKind :: STAREQ } ; [%=] => { $ crate :: JsSyntaxKind :: PERCENTEQ } ; [&&] => { $ crate :: JsSyntaxKind :: AMP2 } ; [||] => { $ crate :: JsSyntaxKind :: PIPE2 } ; [<<] => { $ crate :: JsSyntaxKind :: SHL } ; [>>] => { $ crate :: JsSyntaxKind :: SHR } ; [>>>] => { $ crate :: JsSyntaxKind :: USHR } ; [<<=] => { $ crate :: JsSyntaxKind :: SHLEQ } ; [>>=] => { $ crate :: JsSyntaxKind :: SHREQ } ; [>>>=] => { $ crate :: JsSyntaxKind :: USHREQ } ; [&&=] => { $ crate :: JsSyntaxKind :: AMP2EQ } ; [||=] => { $ crate :: JsSyntaxKind :: PIPE2EQ } ; [**=] => { $ crate :: JsSyntaxKind :: STAR2EQ } ; [??=] => { $ crate :: JsSyntaxKind :: QUESTION2EQ } ; [@] => { $ crate :: JsSyntaxKind :: AT } ; ['`'] => { $ crate :: JsSyntaxKind :: BACKTICK } ; [await] => { $ crate :: JsSyntaxKind :: AWAIT_KW } ; [break] => { $ crate :: JsSyntaxKind :: BREAK_KW } ; [case] => { $ crate :: JsSyntaxKind :: CASE_KW } ; [catch] => { $ crate :: JsSyntaxKind :: CATCH_KW } ; [class] => { $ crate :: JsSyntaxKind :: CLASS_KW } ; [const] => { $ crate :: JsSyntaxKind :: CONST_KW } ; [continue] => { $ crate :: JsSyntaxKind :: CONTINUE_KW } ; [debugger] => { $ crate :: JsSyntaxKind :: DEBUGGER_KW } ; [default] => { $ crate :: JsSyntaxKind :: DEFAULT_KW } ; [delete] => { $ crate :: JsSyntaxKind :: DELETE_KW } ; [do] => { $ crate :: JsSyntaxKind :: DO_KW } ; [else] => { $ crate :: JsSyntaxKind :: ELSE_KW } ; [enum] => { $ crate :: JsSyntaxKind :: ENUM_KW } ; [export] => { $ crate :: JsSyntaxKind :: EXPORT_KW } ; [extends] => { $ crate :: JsSyntaxKind :: EXTENDS_KW } ; [false] => { $ crate :: JsSyntaxKind :: FALSE_KW } ; [finally] => { $ crate :: JsSyntaxKind :: FINALLY_KW } ; [for] => { $ crate :: JsSyntaxKind :: FOR_KW } ; [function] => { $ crate :: JsSyntaxKind :: FUNCTION_KW } ; [if] => { $ crate :: JsSyntaxKind :: IF_KW } ; [in] => { $ crate :: JsSyntaxKind :: IN_KW } ; [instanceof] => { $ crate :: JsSyntaxKind :: INSTANCEOF_KW } ; [interface] => { $ crate :: JsSyntaxKind :: INTERFACE_KW } ; [import] => { $ crate :: JsSyntaxKind :: IMPORT_KW } ; [implements] => { $ crate :: JsSyntaxKind :: IMPLEMENTS_KW } ; [new] => { $ crate :: JsSyntaxKind :: NEW_KW } ; [null] => { $ crate :: JsSyntaxKind :: NULL_KW } ; [package] => { $ crate :: JsSyntaxKind :: PACKAGE_KW } ; [private] => { $ crate :: JsSyntaxKind :: PRIVATE_KW } ; [protected] => { $ crate :: JsSyntaxKind :: PROTECTED_KW } ; [public] => { $ crate :: JsSyntaxKind :: PUBLIC_KW } ; [return] => { $ crate :: JsSyntaxKind :: RETURN_KW } ; [super] => { $ crate :: JsSyntaxKind :: SUPER_KW } ; [switch] => { $ crate :: JsSyntaxKind :: SWITCH_KW } ; [this] => { $ crate :: JsSyntaxKind :: THIS_KW } ; [throw] => { $ crate :: JsSyntaxKind :: THROW_KW } ; [try] => { $ crate :: JsSyntaxKind :: TRY_KW } ; [true] => { $ crate :: JsSyntaxKind :: TRUE_KW } ; [typeof] => { $ crate :: JsSyntaxKind :: TYPEOF_KW } ; [var] => { $ crate :: JsSyntaxKind :: VAR_KW } ; [void] => { $ crate :: JsSyntaxKind :: VOID_KW } ; [while] => { $ crate :: JsSyntaxKind :: WHILE_KW } ; [with] => { $ crate :: JsSyntaxKind :: WITH_KW } ; [yield] => { $ crate :: JsSyntaxKind :: YIELD_KW } ; [readonly] => { $ crate :: JsSyntaxKind :: READONLY_KW } ; [keyof] => { $ crate :: JsSyntaxKind :: KEYOF_KW } ; [unique] => { $ crate :: JsSyntaxKind :: UNIQUE_KW } ; [declare] => { $ crate :: JsSyntaxKind :: DECLARE_KW } ; [abstract] => { $ crate :: JsSyntaxKind :: ABSTRACT_KW } ; [static] => { $ crate :: JsSyntaxKind :: STATIC_KW } ; [async] => { $ crate :: JsSyntaxKind :: ASYNC_KW } ; [type] => { $ crate :: JsSyntaxKind :: TYPE_KW } ; [from] => { $ crate :: JsSyntaxKind :: FROM_KW } ; [as] => { $ crate :: JsSyntaxKind :: AS_KW } ; [require] => { $ crate :: JsSyntaxKind :: REQUIRE_KW } ; [namespace] => { $ crate :: JsSyntaxKind :: NAMESPACE_KW } ; [assert] => { $ crate :: JsSyntaxKind :: ASSERT_KW } ; [module] => { $ crate :: JsSyntaxKind :: MODULE_KW } ; [global] => { $ crate :: JsSyntaxKind :: GLOBAL_KW } ; [infer] => { $ crate :: JsSyntaxKind :: INFER_KW } ; [get] => { $ crate :: JsSyntaxKind :: GET_KW } ; [set] => { $ crate :: JsSyntaxKind :: SET_KW } ; [of] => { $ crate :: JsSyntaxKind :: OF_KW } ; [target] => { $ crate :: JsSyntaxKind :: TARGET_KW } ; [never] => { $ crate :: JsSyntaxKind :: NEVER_KW } ; [unknown] => { $ crate :: JsSyntaxKind :: UNKNOWN_KW } ; [any] => { $ crate :: JsSyntaxKind :: ANY_KW } ; [undefined] => { $ crate :: JsSyntaxKind :: UNDEFINED_KW } ; [let] => { $ crate :: JsSyntaxKind :: LET_KW } ; [float] => { $ crate :: JsSyntaxKind :: FLOAT_KW } ; [number] => { $ crate :: JsSyntaxKind :: NUMBER_KW } ; [meta] => { $ crate :: JsSyntaxKind :: META_KW } ; [ident] => { $ crate :: JsSyntaxKind :: IDENT } ; [EOF] => { $ crate :: JsSyntaxKind :: EOF } ; [#] => { $ crate :: JsSyntaxKind :: HASH } ; }
