// SPDX-License-Identifier: Apache-2.0
#![allow(clippy::all)]
#![allow(unused_parens)]
#![cfg_attr(rustfmt, rustfmt_skip)]
// Generated from FuncTestCaseParser.g4 by ANTLR 4.13.2
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use antlr4rust::PredictionContextCache;
use antlr4rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr4rust::token_stream::TokenStream;
use antlr4rust::TokenSource;
use antlr4rust::parser_atn_simulator::ParserATNSimulator;
use antlr4rust::errors::*;
use antlr4rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr4rust::recognizer::{Recognizer,Actions};
use antlr4rust::atn_deserializer::ATNDeserializer;
use antlr4rust::dfa::DFA;
use antlr4rust::atn::{ATN, INVALID_ALT};
use antlr4rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr4rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr4rust::tree::*;
use antlr4rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr4rust::int_stream::EOF;
use antlr4rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr4rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::functestcaseparserlistener::*;
use antlr4rust::lazy_static;
use antlr4rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const FuncTestCaseParser_Whitespace:i32=1; 
		pub const FuncTestCaseParser_TripleHash:i32=2; 
		pub const FuncTestCaseParser_SubstraitScalarTest:i32=3; 
		pub const FuncTestCaseParser_SubstraitAggregateTest:i32=4; 
		pub const FuncTestCaseParser_SubstraitInclude:i32=5; 
		pub const FuncTestCaseParser_SubstraitDependency:i32=6; 
		pub const FuncTestCaseParser_ExtensionUrn:i32=7; 
		pub const FuncTestCaseParser_FormatVersion:i32=8; 
		pub const FuncTestCaseParser_DescriptionLine:i32=9; 
		pub const FuncTestCaseParser_Define:i32=10; 
		pub const FuncTestCaseParser_ErrorResult:i32=11; 
		pub const FuncTestCaseParser_UndefineResult:i32=12; 
		pub const FuncTestCaseParser_Overflow:i32=13; 
		pub const FuncTestCaseParser_Rounding:i32=14; 
		pub const FuncTestCaseParser_Error:i32=15; 
		pub const FuncTestCaseParser_Saturate:i32=16; 
		pub const FuncTestCaseParser_Silent:i32=17; 
		pub const FuncTestCaseParser_TieToEven:i32=18; 
		pub const FuncTestCaseParser_NaN:i32=19; 
		pub const FuncTestCaseParser_AcceptNulls:i32=20; 
		pub const FuncTestCaseParser_IgnoreNulls:i32=21; 
		pub const FuncTestCaseParser_NullHandling:i32=22; 
		pub const FuncTestCaseParser_SpacesOnly:i32=23; 
		pub const FuncTestCaseParser_Truncate:i32=24; 
		pub const FuncTestCaseParser_IntegerLiteral:i32=25; 
		pub const FuncTestCaseParser_DecimalLiteral:i32=26; 
		pub const FuncTestCaseParser_FloatLiteral:i32=27; 
		pub const FuncTestCaseParser_BooleanLiteral:i32=28; 
		pub const FuncTestCaseParser_TimestampTzLiteral:i32=29; 
		pub const FuncTestCaseParser_TimestampLiteral:i32=30; 
		pub const FuncTestCaseParser_TimeLiteral:i32=31; 
		pub const FuncTestCaseParser_DateLiteral:i32=32; 
		pub const FuncTestCaseParser_PeriodPrefix:i32=33; 
		pub const FuncTestCaseParser_TimePrefix:i32=34; 
		pub const FuncTestCaseParser_YearSuffix:i32=35; 
		pub const FuncTestCaseParser_MSuffix:i32=36; 
		pub const FuncTestCaseParser_DaySuffix:i32=37; 
		pub const FuncTestCaseParser_HourSuffix:i32=38; 
		pub const FuncTestCaseParser_SecondSuffix:i32=39; 
		pub const FuncTestCaseParser_FractionalSecondSuffix:i32=40; 
		pub const FuncTestCaseParser_OAngleBracket:i32=41; 
		pub const FuncTestCaseParser_CAngleBracket:i32=42; 
		pub const FuncTestCaseParser_IntervalYearLiteral:i32=43; 
		pub const FuncTestCaseParser_IntervalDayLiteral:i32=44; 
		pub const FuncTestCaseParser_IntervalCompoundLiteral:i32=45; 
		pub const FuncTestCaseParser_NullLiteral:i32=46; 
		pub const FuncTestCaseParser_StringLiteral:i32=47; 
		pub const FuncTestCaseParser_EnumType:i32=48; 
		pub const FuncTestCaseParser_OBrace:i32=49; 
		pub const FuncTestCaseParser_CBrace:i32=50; 
		pub const FuncTestCaseParser_ColumnName:i32=51; 
		pub const FuncTestCaseParser_LineComment:i32=52; 
		pub const FuncTestCaseParser_BlockComment:i32=53; 
		pub const FuncTestCaseParser_If:i32=54; 
		pub const FuncTestCaseParser_Then:i32=55; 
		pub const FuncTestCaseParser_Else:i32=56; 
		pub const FuncTestCaseParser_Func:i32=57; 
		pub const FuncTestCaseParser_Boolean:i32=58; 
		pub const FuncTestCaseParser_I8:i32=59; 
		pub const FuncTestCaseParser_I16:i32=60; 
		pub const FuncTestCaseParser_I32:i32=61; 
		pub const FuncTestCaseParser_I64:i32=62; 
		pub const FuncTestCaseParser_FP32:i32=63; 
		pub const FuncTestCaseParser_FP64:i32=64; 
		pub const FuncTestCaseParser_String:i32=65; 
		pub const FuncTestCaseParser_Binary:i32=66; 
		pub const FuncTestCaseParser_Date:i32=67; 
		pub const FuncTestCaseParser_Interval_Year:i32=68; 
		pub const FuncTestCaseParser_Interval_Day:i32=69; 
		pub const FuncTestCaseParser_Interval_Compound:i32=70; 
		pub const FuncTestCaseParser_UUID:i32=71; 
		pub const FuncTestCaseParser_Decimal:i32=72; 
		pub const FuncTestCaseParser_Precision_Time:i32=73; 
		pub const FuncTestCaseParser_Precision_Timestamp:i32=74; 
		pub const FuncTestCaseParser_Precision_Timestamp_TZ:i32=75; 
		pub const FuncTestCaseParser_FixedChar:i32=76; 
		pub const FuncTestCaseParser_VarChar:i32=77; 
		pub const FuncTestCaseParser_FixedBinary:i32=78; 
		pub const FuncTestCaseParser_Struct:i32=79; 
		pub const FuncTestCaseParser_NStruct:i32=80; 
		pub const FuncTestCaseParser_List:i32=81; 
		pub const FuncTestCaseParser_Map:i32=82; 
		pub const FuncTestCaseParser_UserDefined:i32=83; 
		pub const FuncTestCaseParser_Bool:i32=84; 
		pub const FuncTestCaseParser_Str:i32=85; 
		pub const FuncTestCaseParser_VBin:i32=86; 
		pub const FuncTestCaseParser_IYear:i32=87; 
		pub const FuncTestCaseParser_IDay:i32=88; 
		pub const FuncTestCaseParser_ICompound:i32=89; 
		pub const FuncTestCaseParser_Dec:i32=90; 
		pub const FuncTestCaseParser_PT:i32=91; 
		pub const FuncTestCaseParser_PTs:i32=92; 
		pub const FuncTestCaseParser_PTsTZ:i32=93; 
		pub const FuncTestCaseParser_FChar:i32=94; 
		pub const FuncTestCaseParser_VChar:i32=95; 
		pub const FuncTestCaseParser_FBin:i32=96; 
		pub const FuncTestCaseParser_Any:i32=97; 
		pub const FuncTestCaseParser_AnyVar:i32=98; 
		pub const FuncTestCaseParser_DoubleColon:i32=99; 
		pub const FuncTestCaseParser_Plus:i32=100; 
		pub const FuncTestCaseParser_Minus:i32=101; 
		pub const FuncTestCaseParser_Asterisk:i32=102; 
		pub const FuncTestCaseParser_ForwardSlash:i32=103; 
		pub const FuncTestCaseParser_Percent:i32=104; 
		pub const FuncTestCaseParser_Eq:i32=105; 
		pub const FuncTestCaseParser_Ne:i32=106; 
		pub const FuncTestCaseParser_Gte:i32=107; 
		pub const FuncTestCaseParser_Lte:i32=108; 
		pub const FuncTestCaseParser_Gt:i32=109; 
		pub const FuncTestCaseParser_Lt:i32=110; 
		pub const FuncTestCaseParser_Bang:i32=111; 
		pub const FuncTestCaseParser_OParen:i32=112; 
		pub const FuncTestCaseParser_CParen:i32=113; 
		pub const FuncTestCaseParser_OBracket:i32=114; 
		pub const FuncTestCaseParser_CBracket:i32=115; 
		pub const FuncTestCaseParser_Comma:i32=116; 
		pub const FuncTestCaseParser_Colon:i32=117; 
		pub const FuncTestCaseParser_QMark:i32=118; 
		pub const FuncTestCaseParser_Hash:i32=119; 
		pub const FuncTestCaseParser_Dot:i32=120; 
		pub const FuncTestCaseParser_And:i32=121; 
		pub const FuncTestCaseParser_Or:i32=122; 
		pub const FuncTestCaseParser_Assign:i32=123; 
		pub const FuncTestCaseParser_Arrow:i32=124; 
		pub const FuncTestCaseParser_Number:i32=125; 
		pub const FuncTestCaseParser_Identifier:i32=126; 
		pub const FuncTestCaseParser_Newline:i32=127;
	pub const FuncTestCaseParser_EOF:i32=EOF;
	pub const RULE_doc:usize = 0; 
	pub const RULE_header:usize = 1; 
	pub const RULE_version:usize = 2; 
	pub const RULE_include:usize = 3; 
	pub const RULE_dependency:usize = 4; 
	pub const RULE_testGroupDescription:usize = 5; 
	pub const RULE_testCase:usize = 6; 
	pub const RULE_testGroup:usize = 7; 
	pub const RULE_arguments:usize = 8; 
	pub const RULE_result:usize = 9; 
	pub const RULE_argument:usize = 10; 
	pub const RULE_aggFuncTestCase:usize = 11; 
	pub const RULE_aggFuncCall:usize = 12; 
	pub const RULE_tableData:usize = 13; 
	pub const RULE_tableRows:usize = 14; 
	pub const RULE_dataColumn:usize = 15; 
	pub const RULE_columnValues:usize = 16; 
	pub const RULE_literal:usize = 17; 
	pub const RULE_qualifiedAggregateFuncArgs:usize = 18; 
	pub const RULE_aggregateFuncArgs:usize = 19; 
	pub const RULE_qualifiedAggregateFuncArg:usize = 20; 
	pub const RULE_aggregateFuncArg:usize = 21; 
	pub const RULE_numericLiteral:usize = 22; 
	pub const RULE_floatLiteral:usize = 23; 
	pub const RULE_nullArg:usize = 24; 
	pub const RULE_intArg:usize = 25; 
	pub const RULE_floatArg:usize = 26; 
	pub const RULE_decimalArg:usize = 27; 
	pub const RULE_booleanArg:usize = 28; 
	pub const RULE_stringArg:usize = 29; 
	pub const RULE_dateArg:usize = 30; 
	pub const RULE_intervalYearArg:usize = 31; 
	pub const RULE_intervalDayArg:usize = 32; 
	pub const RULE_intervalCompoundArg:usize = 33; 
	pub const RULE_fixedCharArg:usize = 34; 
	pub const RULE_varCharArg:usize = 35; 
	pub const RULE_fixedBinaryArg:usize = 36; 
	pub const RULE_precisionTimeArg:usize = 37; 
	pub const RULE_precisionTimestampArg:usize = 38; 
	pub const RULE_precisionTimestampTZArg:usize = 39; 
	pub const RULE_listArg:usize = 40; 
	pub const RULE_structArg:usize = 41; 
	pub const RULE_mapArg:usize = 42; 
	pub const RULE_userDefinedArg:usize = 43; 
	pub const RULE_lambdaArg:usize = 44; 
	pub const RULE_funcCallArg:usize = 45; 
	pub const RULE_enumArg:usize = 46; 
	pub const RULE_literalList:usize = 47; 
	pub const RULE_literalStruct:usize = 48; 
	pub const RULE_literalMap:usize = 49; 
	pub const RULE_mapEntry:usize = 50; 
	pub const RULE_compoundLiteral:usize = 51; 
	pub const RULE_literalLambda:usize = 52; 
	pub const RULE_lambdaParameters:usize = 53; 
	pub const RULE_lambdaBody:usize = 54; 
	pub const RULE_dataType:usize = 55; 
	pub const RULE_scalarType:usize = 56; 
	pub const RULE_userDefinedType:usize = 57; 
	pub const RULE_booleanType:usize = 58; 
	pub const RULE_stringType:usize = 59; 
	pub const RULE_binaryType:usize = 60; 
	pub const RULE_intType:usize = 61; 
	pub const RULE_floatType:usize = 62; 
	pub const RULE_dateType:usize = 63; 
	pub const RULE_intervalYearType:usize = 64; 
	pub const RULE_intervalDayType:usize = 65; 
	pub const RULE_intervalCompoundType:usize = 66; 
	pub const RULE_fixedCharType:usize = 67; 
	pub const RULE_varCharType:usize = 68; 
	pub const RULE_fixedBinaryType:usize = 69; 
	pub const RULE_decimalType:usize = 70; 
	pub const RULE_precisionTimeType:usize = 71; 
	pub const RULE_precisionTimestampType:usize = 72; 
	pub const RULE_precisionTimestampTZType:usize = 73; 
	pub const RULE_listType:usize = 74; 
	pub const RULE_structType:usize = 75; 
	pub const RULE_mapType:usize = 76; 
	pub const RULE_funcType:usize = 77; 
	pub const RULE_funcParameters:usize = 78; 
	pub const RULE_parameterizedType:usize = 79; 
	pub const RULE_numericParameter:usize = 80; 
	pub const RULE_substraitError:usize = 81; 
	pub const RULE_funcOption:usize = 82; 
	pub const RULE_optionName:usize = 83; 
	pub const RULE_optionValue:usize = 84; 
	pub const RULE_funcOptions:usize = 85; 
	pub const RULE_nonReserved:usize = 86; 
	pub const RULE_identifier:usize = 87;
	pub const ruleNames: [&'static str; 88] =  [
		"doc", "header", "version", "include", "dependency", "testGroupDescription", 
		"testCase", "testGroup", "arguments", "result", "argument", "aggFuncTestCase", 
		"aggFuncCall", "tableData", "tableRows", "dataColumn", "columnValues", 
		"literal", "qualifiedAggregateFuncArgs", "aggregateFuncArgs", "qualifiedAggregateFuncArg", 
		"aggregateFuncArg", "numericLiteral", "floatLiteral", "nullArg", "intArg", 
		"floatArg", "decimalArg", "booleanArg", "stringArg", "dateArg", "intervalYearArg", 
		"intervalDayArg", "intervalCompoundArg", "fixedCharArg", "varCharArg", 
		"fixedBinaryArg", "precisionTimeArg", "precisionTimestampArg", "precisionTimestampTZArg", 
		"listArg", "structArg", "mapArg", "userDefinedArg", "lambdaArg", "funcCallArg", 
		"enumArg", "literalList", "literalStruct", "literalMap", "mapEntry", "compoundLiteral", 
		"literalLambda", "lambdaParameters", "lambdaBody", "dataType", "scalarType", 
		"userDefinedType", "booleanType", "stringType", "binaryType", "intType", 
		"floatType", "dateType", "intervalYearType", "intervalDayType", "intervalCompoundType", 
		"fixedCharType", "varCharType", "fixedBinaryType", "decimalType", "precisionTimeType", 
		"precisionTimestampType", "precisionTimestampTZType", "listType", "structType", 
		"mapType", "funcType", "funcParameters", "parameterizedType", "numericParameter", 
		"substraitError", "funcOption", "optionName", "optionValue", "funcOptions", 
		"nonReserved", "identifier"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;125] = [
		None, None, Some("'###'"), Some("'SUBSTRAIT_SCALAR_TEST'"), Some("'SUBSTRAIT_AGGREGATE_TEST'"), 
		Some("'SUBSTRAIT_INCLUDE'"), Some("'SUBSTRAIT_DEPENDENCY'"), None, None, 
		None, Some("'DEFINE'"), Some("'<!ERROR>'"), Some("'<!UNDEFINED>'"), Some("'OVERFLOW'"), 
		Some("'ROUNDING'"), Some("'ERROR'"), Some("'SATURATE'"), Some("'SILENT'"), 
		Some("'TIE_TO_EVEN'"), Some("'NAN'"), Some("'ACCEPT_NULLS'"), Some("'IGNORE_NULLS'"), 
		Some("'NULL_HANDLING'"), Some("'SPACES_ONLY'"), Some("'TRUNCATE'"), None, 
		None, None, None, None, None, None, None, Some("'P'"), Some("'T'"), Some("'Y'"), 
		Some("'M'"), Some("'D'"), Some("'H'"), Some("'S'"), Some("'F'"), None, 
		None, None, None, None, Some("'null'"), None, Some("'enum'"), Some("'{'"), 
		Some("'}'"), None, None, None, Some("'IF'"), Some("'THEN'"), Some("'ELSE'"), 
		Some("'FUNC'"), Some("'BOOLEAN'"), Some("'I8'"), Some("'I16'"), Some("'I32'"), 
		Some("'I64'"), Some("'FP32'"), Some("'FP64'"), Some("'STRING'"), Some("'BINARY'"), 
		Some("'DATE'"), Some("'INTERVAL_YEAR'"), Some("'INTERVAL_DAY'"), Some("'INTERVAL_COMPOUND'"), 
		Some("'UUID'"), Some("'DECIMAL'"), Some("'PRECISION_TIME'"), Some("'PRECISION_TIMESTAMP'"), 
		Some("'PRECISION_TIMESTAMP_TZ'"), Some("'FIXEDCHAR'"), Some("'VARCHAR'"), 
		Some("'FIXEDBINARY'"), Some("'STRUCT'"), Some("'NSTRUCT'"), Some("'LIST'"), 
		Some("'MAP'"), Some("'U!'"), Some("'BOOL'"), Some("'STR'"), Some("'VBIN'"), 
		Some("'IYEAR'"), Some("'IDAY'"), Some("'ICOMPOUND'"), Some("'DEC'"), Some("'PT'"), 
		Some("'PTS'"), Some("'PTSTZ'"), Some("'FCHAR'"), Some("'VCHAR'"), Some("'FBIN'"), 
		Some("'ANY'"), None, Some("'::'"), Some("'+'"), Some("'-'"), Some("'*'"), 
		Some("'/'"), Some("'%'"), Some("'='"), Some("'!='"), Some("'>='"), Some("'<='"), 
		Some("'>'"), Some("'<'"), Some("'!'"), Some("'('"), Some("')'"), Some("'['"), 
		Some("']'"), Some("','"), Some("':'"), Some("'?'"), Some("'#'"), Some("'.'"), 
		Some("'AND'"), Some("'OR'"), Some("':='"), Some("'->'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;128]  = [
		None, Some("Whitespace"), Some("TripleHash"), Some("SubstraitScalarTest"), 
		Some("SubstraitAggregateTest"), Some("SubstraitInclude"), Some("SubstraitDependency"), 
		Some("ExtensionUrn"), Some("FormatVersion"), Some("DescriptionLine"), 
		Some("Define"), Some("ErrorResult"), Some("UndefineResult"), Some("Overflow"), 
		Some("Rounding"), Some("Error"), Some("Saturate"), Some("Silent"), Some("TieToEven"), 
		Some("NaN"), Some("AcceptNulls"), Some("IgnoreNulls"), Some("NullHandling"), 
		Some("SpacesOnly"), Some("Truncate"), Some("IntegerLiteral"), Some("DecimalLiteral"), 
		Some("FloatLiteral"), Some("BooleanLiteral"), Some("TimestampTzLiteral"), 
		Some("TimestampLiteral"), Some("TimeLiteral"), Some("DateLiteral"), Some("PeriodPrefix"), 
		Some("TimePrefix"), Some("YearSuffix"), Some("MSuffix"), Some("DaySuffix"), 
		Some("HourSuffix"), Some("SecondSuffix"), Some("FractionalSecondSuffix"), 
		Some("OAngleBracket"), Some("CAngleBracket"), Some("IntervalYearLiteral"), 
		Some("IntervalDayLiteral"), Some("IntervalCompoundLiteral"), Some("NullLiteral"), 
		Some("StringLiteral"), Some("EnumType"), Some("OBrace"), Some("CBrace"), 
		Some("ColumnName"), Some("LineComment"), Some("BlockComment"), Some("If"), 
		Some("Then"), Some("Else"), Some("Func"), Some("Boolean"), Some("I8"), 
		Some("I16"), Some("I32"), Some("I64"), Some("FP32"), Some("FP64"), Some("String"), 
		Some("Binary"), Some("Date"), Some("Interval_Year"), Some("Interval_Day"), 
		Some("Interval_Compound"), Some("UUID"), Some("Decimal"), Some("Precision_Time"), 
		Some("Precision_Timestamp"), Some("Precision_Timestamp_TZ"), Some("FixedChar"), 
		Some("VarChar"), Some("FixedBinary"), Some("Struct"), Some("NStruct"), 
		Some("List"), Some("Map"), Some("UserDefined"), Some("Bool"), Some("Str"), 
		Some("VBin"), Some("IYear"), Some("IDay"), Some("ICompound"), Some("Dec"), 
		Some("PT"), Some("PTs"), Some("PTsTZ"), Some("FChar"), Some("VChar"), 
		Some("FBin"), Some("Any"), Some("AnyVar"), Some("DoubleColon"), Some("Plus"), 
		Some("Minus"), Some("Asterisk"), Some("ForwardSlash"), Some("Percent"), 
		Some("Eq"), Some("Ne"), Some("Gte"), Some("Lte"), Some("Gt"), Some("Lt"), 
		Some("Bang"), Some("OParen"), Some("CParen"), Some("OBracket"), Some("CBracket"), 
		Some("Comma"), Some("Colon"), Some("QMark"), Some("Hash"), Some("Dot"), 
		Some("And"), Some("Or"), Some("Assign"), Some("Arrow"), Some("Number"), 
		Some("Identifier"), Some("Newline")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,FuncTestCaseParserExt<'input>, I, FuncTestCaseParserContextType , dyn FuncTestCaseParserListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type FuncTestCaseParserTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, FuncTestCaseParserContextType , dyn FuncTestCaseParserListener<'input> + 'a>;

/// Parser for FuncTestCaseParser grammar
pub struct FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: Box<dyn ErrorStrategy<'input,BaseParserType<'input,I> > >,
}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn set_error_strategy(&mut self, strategy: Box<dyn ErrorStrategy<'input,BaseParserType<'input,I> > >) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: Box<dyn ErrorStrategy<'input,BaseParserType<'input,I> > >) -> Self {
		antlr4rust::recognizer::check_version("0","5");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				FuncTestCaseParserExt{
					_pd: Default::default(),
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for FuncTestCaseParser
pub trait FuncTestCaseParserContext<'input>:
	for<'x> Listenable<dyn FuncTestCaseParserListener<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=FuncTestCaseParserContextType>
{}

antlr4rust::coerce_from!{ 'input : FuncTestCaseParserContext<'input> }

impl<'input> FuncTestCaseParserContext<'input> for TerminalNode<'input,FuncTestCaseParserContextType> {}
impl<'input> FuncTestCaseParserContext<'input> for ErrorNode<'input,FuncTestCaseParserContextType> {}

antlr4rust::tid! { impl<'input> TidAble<'input> for dyn FuncTestCaseParserContext<'input> + 'input }

antlr4rust::tid! { impl<'input> TidAble<'input> for dyn FuncTestCaseParserListener<'input> + 'input }

pub struct FuncTestCaseParserContextType;
antlr4rust::tid!{FuncTestCaseParserContextType}

impl<'input> ParserNodeType<'input> for FuncTestCaseParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn FuncTestCaseParserContext<'input> + 'input;
}

impl<'input, I> Deref for FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I> DerefMut for FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct FuncTestCaseParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> FuncTestCaseParserExt<'input>{
}
antlr4rust::tid! { FuncTestCaseParserExt<'a> }

impl<'input> TokenAware<'input> for FuncTestCaseParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for FuncTestCaseParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for FuncTestCaseParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "FuncTestCaseParser.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
}
//------------------- doc ----------------
pub type DocContextAll<'input> = DocContext<'input>;


pub type DocContext<'input> = BaseParserRuleContext<'input,DocContextExt<'input>>;

#[derive(Clone)]
pub struct DocContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for DocContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for DocContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_doc(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_doc(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for DocContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_doc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_doc }
}
antlr4rust::tid!{DocContextExt<'a>}

impl<'input> DocContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<DocContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DocContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait DocContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<DocContextExt<'input>>{

fn header(&self) -> Option<Rc<HeaderContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_EOF, 0)
}
fn testGroup_all(&self) ->  Vec<Rc<TestGroupContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn testGroup(&self, i: usize) -> Option<Rc<TestGroupContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> DocContextAttrs<'input> for DocContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn doc(&mut self,)
	-> Result<Rc<DocContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DocContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_doc);
        let mut _localctx: Rc<DocContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule header*/
			recog.base.set_state(176);
			recog.header()?;

			recog.base.set_state(178); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule testGroup*/
				recog.base.set_state(177);
				recog.testGroup()?;

				}
				}
				recog.base.set_state(180); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !((((_la) & !0x3f) == 0 && ((1usize << _la) & 16778752) != 0) || ((((_la - 112)) & !0x3f) == 0 && ((1usize << (_la - 112)) & 17921) != 0)) {break}
			}
			recog.base.set_state(182);
			recog.base.match_token(FuncTestCaseParser_EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- header ----------------
pub type HeaderContextAll<'input> = HeaderContext<'input>;


pub type HeaderContext<'input> = BaseParserRuleContext<'input,HeaderContextExt<'input>>;

#[derive(Clone)]
pub struct HeaderContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for HeaderContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for HeaderContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_header(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_header(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for HeaderContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_header }
	//fn type_rule_index() -> usize where Self: Sized { RULE_header }
}
antlr4rust::tid!{HeaderContextExt<'a>}

impl<'input> HeaderContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<HeaderContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,HeaderContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait HeaderContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<HeaderContextExt<'input>>{

fn version(&self) -> Option<Rc<VersionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn include(&self) -> Option<Rc<IncludeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn dependency_all(&self) ->  Vec<Rc<DependencyContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn dependency(&self, i: usize) -> Option<Rc<DependencyContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> HeaderContextAttrs<'input> for HeaderContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn header(&mut self,)
	-> Result<Rc<HeaderContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = HeaderContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_header);
        let mut _localctx: Rc<HeaderContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule version*/
			recog.base.set_state(184);
			recog.version()?;

			/*InvokeRule include*/
			recog.base.set_state(185);
			recog.include()?;

			recog.base.set_state(189);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==FuncTestCaseParser_TripleHash {
				{
				{
				/*InvokeRule dependency*/
				recog.base.set_state(186);
				recog.dependency()?;

				}
				}
				recog.base.set_state(191);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- version ----------------
pub type VersionContextAll<'input> = VersionContext<'input>;


pub type VersionContext<'input> = BaseParserRuleContext<'input,VersionContextExt<'input>>;

#[derive(Clone)]
pub struct VersionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for VersionContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for VersionContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_version(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_version(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for VersionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_version }
	//fn type_rule_index() -> usize where Self: Sized { RULE_version }
}
antlr4rust::tid!{VersionContextExt<'a>}

impl<'input> VersionContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<VersionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,VersionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait VersionContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<VersionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token TripleHash
/// Returns `None` if there is no child corresponding to token TripleHash
fn TripleHash(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_TripleHash, 0)
}
/// Retrieves first TerminalNode corresponding to token Colon
/// Returns `None` if there is no child corresponding to token Colon
fn Colon(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Colon, 0)
}
/// Retrieves first TerminalNode corresponding to token FormatVersion
/// Returns `None` if there is no child corresponding to token FormatVersion
fn FormatVersion(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_FormatVersion, 0)
}
/// Retrieves first TerminalNode corresponding to token SubstraitScalarTest
/// Returns `None` if there is no child corresponding to token SubstraitScalarTest
fn SubstraitScalarTest(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_SubstraitScalarTest, 0)
}
/// Retrieves first TerminalNode corresponding to token SubstraitAggregateTest
/// Returns `None` if there is no child corresponding to token SubstraitAggregateTest
fn SubstraitAggregateTest(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_SubstraitAggregateTest, 0)
}

}

impl<'input> VersionContextAttrs<'input> for VersionContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn version(&mut self,)
	-> Result<Rc<VersionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = VersionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_version);
        let mut _localctx: Rc<VersionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(192);
			recog.base.match_token(FuncTestCaseParser_TripleHash,&mut recog.err_handler)?;

			recog.base.set_state(193);
			_la = recog.base.input.la(1);
			if { !(_la==FuncTestCaseParser_SubstraitScalarTest || _la==FuncTestCaseParser_SubstraitAggregateTest) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			recog.base.set_state(194);
			recog.base.match_token(FuncTestCaseParser_Colon,&mut recog.err_handler)?;

			recog.base.set_state(195);
			recog.base.match_token(FuncTestCaseParser_FormatVersion,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- include ----------------
pub type IncludeContextAll<'input> = IncludeContext<'input>;


pub type IncludeContext<'input> = BaseParserRuleContext<'input,IncludeContextExt<'input>>;

#[derive(Clone)]
pub struct IncludeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for IncludeContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for IncludeContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_include(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_include(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for IncludeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_include }
	//fn type_rule_index() -> usize where Self: Sized { RULE_include }
}
antlr4rust::tid!{IncludeContextExt<'a>}

impl<'input> IncludeContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<IncludeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IncludeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait IncludeContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<IncludeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token TripleHash
/// Returns `None` if there is no child corresponding to token TripleHash
fn TripleHash(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_TripleHash, 0)
}
/// Retrieves first TerminalNode corresponding to token SubstraitInclude
/// Returns `None` if there is no child corresponding to token SubstraitInclude
fn SubstraitInclude(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_SubstraitInclude, 0)
}
/// Retrieves first TerminalNode corresponding to token Colon
/// Returns `None` if there is no child corresponding to token Colon
fn Colon(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Colon, 0)
}
/// Retrieves first TerminalNode corresponding to token ExtensionUrn
/// Returns `None` if there is no child corresponding to token ExtensionUrn
fn ExtensionUrn(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_ExtensionUrn, 0)
}

}

impl<'input> IncludeContextAttrs<'input> for IncludeContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn include(&mut self,)
	-> Result<Rc<IncludeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IncludeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_include);
        let mut _localctx: Rc<IncludeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(197);
			recog.base.match_token(FuncTestCaseParser_TripleHash,&mut recog.err_handler)?;

			recog.base.set_state(198);
			recog.base.match_token(FuncTestCaseParser_SubstraitInclude,&mut recog.err_handler)?;

			recog.base.set_state(199);
			recog.base.match_token(FuncTestCaseParser_Colon,&mut recog.err_handler)?;

			recog.base.set_state(200);
			recog.base.match_token(FuncTestCaseParser_ExtensionUrn,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- dependency ----------------
pub type DependencyContextAll<'input> = DependencyContext<'input>;


pub type DependencyContext<'input> = BaseParserRuleContext<'input,DependencyContextExt<'input>>;

#[derive(Clone)]
pub struct DependencyContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for DependencyContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for DependencyContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_dependency(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_dependency(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for DependencyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_dependency }
	//fn type_rule_index() -> usize where Self: Sized { RULE_dependency }
}
antlr4rust::tid!{DependencyContextExt<'a>}

impl<'input> DependencyContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<DependencyContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DependencyContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait DependencyContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<DependencyContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token TripleHash
/// Returns `None` if there is no child corresponding to token TripleHash
fn TripleHash(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_TripleHash, 0)
}
/// Retrieves first TerminalNode corresponding to token SubstraitDependency
/// Returns `None` if there is no child corresponding to token SubstraitDependency
fn SubstraitDependency(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_SubstraitDependency, 0)
}
/// Retrieves first TerminalNode corresponding to token Colon
/// Returns `None` if there is no child corresponding to token Colon
fn Colon(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Colon, 0)
}
/// Retrieves first TerminalNode corresponding to token ExtensionUrn
/// Returns `None` if there is no child corresponding to token ExtensionUrn
fn ExtensionUrn(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_ExtensionUrn, 0)
}

}

impl<'input> DependencyContextAttrs<'input> for DependencyContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn dependency(&mut self,)
	-> Result<Rc<DependencyContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DependencyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_dependency);
        let mut _localctx: Rc<DependencyContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(202);
			recog.base.match_token(FuncTestCaseParser_TripleHash,&mut recog.err_handler)?;

			recog.base.set_state(203);
			recog.base.match_token(FuncTestCaseParser_SubstraitDependency,&mut recog.err_handler)?;

			recog.base.set_state(204);
			recog.base.match_token(FuncTestCaseParser_Colon,&mut recog.err_handler)?;

			recog.base.set_state(205);
			recog.base.match_token(FuncTestCaseParser_ExtensionUrn,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- testGroupDescription ----------------
pub type TestGroupDescriptionContextAll<'input> = TestGroupDescriptionContext<'input>;


pub type TestGroupDescriptionContext<'input> = BaseParserRuleContext<'input,TestGroupDescriptionContextExt<'input>>;

#[derive(Clone)]
pub struct TestGroupDescriptionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for TestGroupDescriptionContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for TestGroupDescriptionContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_testGroupDescription(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_testGroupDescription(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for TestGroupDescriptionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_testGroupDescription }
	//fn type_rule_index() -> usize where Self: Sized { RULE_testGroupDescription }
}
antlr4rust::tid!{TestGroupDescriptionContextExt<'a>}

impl<'input> TestGroupDescriptionContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<TestGroupDescriptionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TestGroupDescriptionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait TestGroupDescriptionContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<TestGroupDescriptionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token DescriptionLine
/// Returns `None` if there is no child corresponding to token DescriptionLine
fn DescriptionLine(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_DescriptionLine, 0)
}

}

impl<'input> TestGroupDescriptionContextAttrs<'input> for TestGroupDescriptionContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn testGroupDescription(&mut self,)
	-> Result<Rc<TestGroupDescriptionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TestGroupDescriptionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_testGroupDescription);
        let mut _localctx: Rc<TestGroupDescriptionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(207);
			recog.base.match_token(FuncTestCaseParser_DescriptionLine,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- testCase ----------------
pub type TestCaseContextAll<'input> = TestCaseContext<'input>;


pub type TestCaseContext<'input> = BaseParserRuleContext<'input,TestCaseContextExt<'input>>;

#[derive(Clone)]
pub struct TestCaseContextExt<'input>{
	pub functionName: Option<Rc<IdentifierContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for TestCaseContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for TestCaseContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_testCase(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_testCase(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for TestCaseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_testCase }
	//fn type_rule_index() -> usize where Self: Sized { RULE_testCase }
}
antlr4rust::tid!{TestCaseContextExt<'a>}

impl<'input> TestCaseContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<TestCaseContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TestCaseContextExt{
				functionName: None, 

				ph:PhantomData
			}),
		)
	}
}

pub trait TestCaseContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<TestCaseContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token OParen
/// Returns `None` if there is no child corresponding to token OParen
fn OParen(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_OParen, 0)
}
fn arguments(&self) -> Option<Rc<ArgumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token CParen
/// Returns `None` if there is no child corresponding to token CParen
fn CParen(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_CParen, 0)
}
/// Retrieves first TerminalNode corresponding to token Eq
/// Returns `None` if there is no child corresponding to token Eq
fn Eq(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Eq, 0)
}
fn result(&self) -> Option<Rc<ResultContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token OBracket
/// Returns `None` if there is no child corresponding to token OBracket
fn OBracket(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_OBracket, 0)
}
fn funcOptions(&self) -> Option<Rc<FuncOptionsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token CBracket
/// Returns `None` if there is no child corresponding to token CBracket
fn CBracket(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_CBracket, 0)
}

}

impl<'input> TestCaseContextAttrs<'input> for TestCaseContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn testCase(&mut self,)
	-> Result<Rc<TestCaseContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TestCaseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_testCase);
        let mut _localctx: Rc<TestCaseContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule identifier*/
			recog.base.set_state(209);
			let tmp = recog.identifier()?;
			 cast_mut::<_,TestCaseContext >(&mut _localctx).functionName = Some(tmp.clone());
			  

			recog.base.set_state(210);
			recog.base.match_token(FuncTestCaseParser_OParen,&mut recog.err_handler)?;

			/*InvokeRule arguments*/
			recog.base.set_state(211);
			recog.arguments()?;

			recog.base.set_state(212);
			recog.base.match_token(FuncTestCaseParser_CParen,&mut recog.err_handler)?;

			recog.base.set_state(217);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==FuncTestCaseParser_OBracket {
				{
				recog.base.set_state(213);
				recog.base.match_token(FuncTestCaseParser_OBracket,&mut recog.err_handler)?;

				/*InvokeRule funcOptions*/
				recog.base.set_state(214);
				recog.funcOptions()?;

				recog.base.set_state(215);
				recog.base.match_token(FuncTestCaseParser_CBracket,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(219);
			recog.base.match_token(FuncTestCaseParser_Eq,&mut recog.err_handler)?;

			/*InvokeRule result*/
			recog.base.set_state(220);
			recog.result()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- testGroup ----------------
#[derive(Debug)]
pub enum TestGroupContextAll<'input>{
	ScalarFuncTestGroupContext(ScalarFuncTestGroupContext<'input>),
	AggregateFuncTestGroupContext(AggregateFuncTestGroupContext<'input>),
Error(TestGroupContext<'input>)
}
antlr4rust::tid!{TestGroupContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for TestGroupContextAll<'input>{}

impl<'input> FuncTestCaseParserContext<'input> for TestGroupContextAll<'input>{}

impl<'input> Deref for TestGroupContextAll<'input>{
	type Target = dyn TestGroupContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use TestGroupContextAll::*;
		match self{
			ScalarFuncTestGroupContext(inner) => inner,
			AggregateFuncTestGroupContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for TestGroupContextAll<'input>{
    fn enter(&self, listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
}



pub type TestGroupContext<'input> = BaseParserRuleContext<'input,TestGroupContextExt<'input>>;

#[derive(Clone)]
pub struct TestGroupContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for TestGroupContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for TestGroupContext<'input>{
}

impl<'input> CustomRuleContext<'input> for TestGroupContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_testGroup }
	//fn type_rule_index() -> usize where Self: Sized { RULE_testGroup }
}
antlr4rust::tid!{TestGroupContextExt<'a>}

impl<'input> TestGroupContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<TestGroupContextAll<'input>> {
		Rc::new(
		TestGroupContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TestGroupContextExt{

				ph:PhantomData
			}),
		)
		)
	}
}

pub trait TestGroupContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<TestGroupContextExt<'input>>{


}

impl<'input> TestGroupContextAttrs<'input> for TestGroupContext<'input>{}

pub type ScalarFuncTestGroupContext<'input> = BaseParserRuleContext<'input,ScalarFuncTestGroupContextExt<'input>>;

pub trait ScalarFuncTestGroupContextAttrs<'input>: FuncTestCaseParserContext<'input>{
	fn testGroupDescription(&self) -> Option<Rc<TestGroupDescriptionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn testCase_all(&self) ->  Vec<Rc<TestCaseContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn testCase(&self, i: usize) -> Option<Rc<TestCaseContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> ScalarFuncTestGroupContextAttrs<'input> for ScalarFuncTestGroupContext<'input>{}

pub struct ScalarFuncTestGroupContextExt<'input>{
	base:TestGroupContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{ScalarFuncTestGroupContextExt<'a>}

impl<'input> FuncTestCaseParserContext<'input> for ScalarFuncTestGroupContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for ScalarFuncTestGroupContext<'input>{
	fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_scalarFuncTestGroup(self);
		Ok(())
	}
}

impl<'input> CustomRuleContext<'input> for ScalarFuncTestGroupContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_testGroup }
	//fn type_rule_index() -> usize where Self: Sized { RULE_testGroup }
}

impl<'input> Borrow<TestGroupContextExt<'input>> for ScalarFuncTestGroupContext<'input>{
	fn borrow(&self) -> &TestGroupContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<TestGroupContextExt<'input>> for ScalarFuncTestGroupContext<'input>{
	fn borrow_mut(&mut self) -> &mut TestGroupContextExt<'input> { &mut self.base }
}

impl<'input> TestGroupContextAttrs<'input> for ScalarFuncTestGroupContext<'input> {}

impl<'input> ScalarFuncTestGroupContextExt<'input>{
	fn new(ctx: &dyn TestGroupContextAttrs<'input>) -> Rc<TestGroupContextAll<'input>>  {
		Rc::new(
			TestGroupContextAll::ScalarFuncTestGroupContext(
				BaseParserRuleContext::copy_from(ctx,ScalarFuncTestGroupContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type AggregateFuncTestGroupContext<'input> = BaseParserRuleContext<'input,AggregateFuncTestGroupContextExt<'input>>;

pub trait AggregateFuncTestGroupContextAttrs<'input>: FuncTestCaseParserContext<'input>{
	fn testGroupDescription(&self) -> Option<Rc<TestGroupDescriptionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn aggFuncTestCase_all(&self) ->  Vec<Rc<AggFuncTestCaseContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn aggFuncTestCase(&self, i: usize) -> Option<Rc<AggFuncTestCaseContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> AggregateFuncTestGroupContextAttrs<'input> for AggregateFuncTestGroupContext<'input>{}

pub struct AggregateFuncTestGroupContextExt<'input>{
	base:TestGroupContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{AggregateFuncTestGroupContextExt<'a>}

impl<'input> FuncTestCaseParserContext<'input> for AggregateFuncTestGroupContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for AggregateFuncTestGroupContext<'input>{
	fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_aggregateFuncTestGroup(self);
		Ok(())
	}
}

impl<'input> CustomRuleContext<'input> for AggregateFuncTestGroupContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_testGroup }
	//fn type_rule_index() -> usize where Self: Sized { RULE_testGroup }
}

impl<'input> Borrow<TestGroupContextExt<'input>> for AggregateFuncTestGroupContext<'input>{
	fn borrow(&self) -> &TestGroupContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<TestGroupContextExt<'input>> for AggregateFuncTestGroupContext<'input>{
	fn borrow_mut(&mut self) -> &mut TestGroupContextExt<'input> { &mut self.base }
}

impl<'input> TestGroupContextAttrs<'input> for AggregateFuncTestGroupContext<'input> {}

impl<'input> AggregateFuncTestGroupContextExt<'input>{
	fn new(ctx: &dyn TestGroupContextAttrs<'input>) -> Rc<TestGroupContextAll<'input>>  {
		Rc::new(
			TestGroupContextAll::AggregateFuncTestGroupContext(
				BaseParserRuleContext::copy_from(ctx,AggregateFuncTestGroupContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn testGroup(&mut self,)
	-> Result<Rc<TestGroupContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TestGroupContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_testGroup);
        let mut _localctx: Rc<TestGroupContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			recog.base.set_state(238);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(7,&mut recog.base)? {
				1 =>{
					let tmp = ScalarFuncTestGroupContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
					_localctx = tmp;
					{
					recog.base.set_state(223);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==FuncTestCaseParser_DescriptionLine {
						{
						/*InvokeRule testGroupDescription*/
						recog.base.set_state(222);
						recog.testGroupDescription()?;

						}
					}

					recog.base.set_state(226); 
					recog.err_handler.sync(&mut recog.base)?;
					_alt = 1;
					loop {
						match _alt {
						    x if x == 1=>
							{
							{
							/*InvokeRule testCase*/
							recog.base.set_state(225);
							recog.testCase()?;

							}
							}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
						}
						recog.base.set_state(228); 
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(4,&mut recog.base)?;
						if _alt==2 || _alt==INVALID_ALT { break }
					}
					}
				}
			,
				2 =>{
					let tmp = AggregateFuncTestGroupContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2)?;
					_localctx = tmp;
					{
					recog.base.set_state(231);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==FuncTestCaseParser_DescriptionLine {
						{
						/*InvokeRule testGroupDescription*/
						recog.base.set_state(230);
						recog.testGroupDescription()?;

						}
					}

					recog.base.set_state(234); 
					recog.err_handler.sync(&mut recog.base)?;
					_alt = 1;
					loop {
						match _alt {
						    x if x == 1=>
							{
							{
							/*InvokeRule aggFuncTestCase*/
							recog.base.set_state(233);
							recog.aggFuncTestCase()?;

							}
							}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
						}
						recog.base.set_state(236); 
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(6,&mut recog.base)?;
						if _alt==2 || _alt==INVALID_ALT { break }
					}
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- arguments ----------------
pub type ArgumentsContextAll<'input> = ArgumentsContext<'input>;


pub type ArgumentsContext<'input> = BaseParserRuleContext<'input,ArgumentsContextExt<'input>>;

#[derive(Clone)]
pub struct ArgumentsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for ArgumentsContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for ArgumentsContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_arguments(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_arguments(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for ArgumentsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_arguments }
	//fn type_rule_index() -> usize where Self: Sized { RULE_arguments }
}
antlr4rust::tid!{ArgumentsContextExt<'a>}

impl<'input> ArgumentsContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ArgumentsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ArgumentsContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ArgumentsContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<ArgumentsContextExt<'input>>{

fn argument_all(&self) ->  Vec<Rc<ArgumentContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn argument(&self, i: usize) -> Option<Rc<ArgumentContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Comma, i)
}

}

impl<'input> ArgumentsContextAttrs<'input> for ArgumentsContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn arguments(&mut self,)
	-> Result<Rc<ArgumentsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ArgumentsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_arguments);
        let mut _localctx: Rc<ArgumentsContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule argument*/
			recog.base.set_state(240);
			recog.argument()?;

			recog.base.set_state(245);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==FuncTestCaseParser_Comma {
				{
				{
				recog.base.set_state(241);
				recog.base.match_token(FuncTestCaseParser_Comma,&mut recog.err_handler)?;

				/*InvokeRule argument*/
				recog.base.set_state(242);
				recog.argument()?;

				}
				}
				recog.base.set_state(247);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- result ----------------
pub type ResultContextAll<'input> = ResultContext<'input>;


pub type ResultContext<'input> = BaseParserRuleContext<'input,ResultContextExt<'input>>;

#[derive(Clone)]
pub struct ResultContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for ResultContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for ResultContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_result(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_result(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for ResultContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_result }
	//fn type_rule_index() -> usize where Self: Sized { RULE_result }
}
antlr4rust::tid!{ResultContextExt<'a>}

impl<'input> ResultContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ResultContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ResultContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ResultContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<ResultContextExt<'input>>{

fn argument(&self) -> Option<Rc<ArgumentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn substraitError(&self) -> Option<Rc<SubstraitErrorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ResultContextAttrs<'input> for ResultContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn result(&mut self,)
	-> Result<Rc<ResultContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ResultContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_result);
        let mut _localctx: Rc<ResultContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(250);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			FuncTestCaseParser_NaN |FuncTestCaseParser_Truncate |FuncTestCaseParser_IntegerLiteral |
			FuncTestCaseParser_DecimalLiteral |FuncTestCaseParser_FloatLiteral |FuncTestCaseParser_BooleanLiteral |
			FuncTestCaseParser_TimestampTzLiteral |FuncTestCaseParser_TimestampLiteral |
			FuncTestCaseParser_TimeLiteral |FuncTestCaseParser_DateLiteral |FuncTestCaseParser_IntervalYearLiteral |
			FuncTestCaseParser_IntervalDayLiteral |FuncTestCaseParser_IntervalCompoundLiteral |
			FuncTestCaseParser_NullLiteral |FuncTestCaseParser_StringLiteral |FuncTestCaseParser_OBrace |
			FuncTestCaseParser_OParen |FuncTestCaseParser_OBracket |FuncTestCaseParser_And |
			FuncTestCaseParser_Or |FuncTestCaseParser_Identifier 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule argument*/
					recog.base.set_state(248);
					recog.argument()?;

					}
				}

			FuncTestCaseParser_ErrorResult |FuncTestCaseParser_UndefineResult 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule substraitError*/
					recog.base.set_state(249);
					recog.substraitError()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- argument ----------------
pub type ArgumentContextAll<'input> = ArgumentContext<'input>;


pub type ArgumentContext<'input> = BaseParserRuleContext<'input,ArgumentContextExt<'input>>;

#[derive(Clone)]
pub struct ArgumentContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for ArgumentContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for ArgumentContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_argument(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_argument(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for ArgumentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_argument }
	//fn type_rule_index() -> usize where Self: Sized { RULE_argument }
}
antlr4rust::tid!{ArgumentContextExt<'a>}

impl<'input> ArgumentContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ArgumentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ArgumentContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ArgumentContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<ArgumentContextExt<'input>>{

fn nullArg(&self) -> Option<Rc<NullArgContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn enumArg(&self) -> Option<Rc<EnumArgContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn intArg(&self) -> Option<Rc<IntArgContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn floatArg(&self) -> Option<Rc<FloatArgContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn booleanArg(&self) -> Option<Rc<BooleanArgContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn stringArg(&self) -> Option<Rc<StringArgContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn decimalArg(&self) -> Option<Rc<DecimalArgContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn dateArg(&self) -> Option<Rc<DateArgContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn intervalYearArg(&self) -> Option<Rc<IntervalYearArgContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn intervalDayArg(&self) -> Option<Rc<IntervalDayArgContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn intervalCompoundArg(&self) -> Option<Rc<IntervalCompoundArgContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fixedCharArg(&self) -> Option<Rc<FixedCharArgContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn varCharArg(&self) -> Option<Rc<VarCharArgContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fixedBinaryArg(&self) -> Option<Rc<FixedBinaryArgContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn precisionTimeArg(&self) -> Option<Rc<PrecisionTimeArgContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn precisionTimestampArg(&self) -> Option<Rc<PrecisionTimestampArgContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn precisionTimestampTZArg(&self) -> Option<Rc<PrecisionTimestampTZArgContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn listArg(&self) -> Option<Rc<ListArgContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn structArg(&self) -> Option<Rc<StructArgContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn mapArg(&self) -> Option<Rc<MapArgContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn userDefinedArg(&self) -> Option<Rc<UserDefinedArgContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn lambdaArg(&self) -> Option<Rc<LambdaArgContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn funcCallArg(&self) -> Option<Rc<FuncCallArgContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Identifier, 0)
}

}

impl<'input> ArgumentContextAttrs<'input> for ArgumentContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn argument(&mut self,)
	-> Result<Rc<ArgumentContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ArgumentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_argument);
        let mut _localctx: Rc<ArgumentContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(276);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(10,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule nullArg*/
					recog.base.set_state(252);
					recog.nullArg()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule enumArg*/
					recog.base.set_state(253);
					recog.enumArg()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule intArg*/
					recog.base.set_state(254);
					recog.intArg()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					/*InvokeRule floatArg*/
					recog.base.set_state(255);
					recog.floatArg()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					/*InvokeRule booleanArg*/
					recog.base.set_state(256);
					recog.booleanArg()?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6)?;
					recog.base.enter_outer_alt(None, 6)?;
					{
					/*InvokeRule stringArg*/
					recog.base.set_state(257);
					recog.stringArg()?;

					}
				}
			,
				7 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 7)?;
					recog.base.enter_outer_alt(None, 7)?;
					{
					/*InvokeRule decimalArg*/
					recog.base.set_state(258);
					recog.decimalArg()?;

					}
				}
			,
				8 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 8)?;
					recog.base.enter_outer_alt(None, 8)?;
					{
					/*InvokeRule dateArg*/
					recog.base.set_state(259);
					recog.dateArg()?;

					}
				}
			,
				9 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 9)?;
					recog.base.enter_outer_alt(None, 9)?;
					{
					/*InvokeRule intervalYearArg*/
					recog.base.set_state(260);
					recog.intervalYearArg()?;

					}
				}
			,
				10 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 10)?;
					recog.base.enter_outer_alt(None, 10)?;
					{
					/*InvokeRule intervalDayArg*/
					recog.base.set_state(261);
					recog.intervalDayArg()?;

					}
				}
			,
				11 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 11)?;
					recog.base.enter_outer_alt(None, 11)?;
					{
					/*InvokeRule intervalCompoundArg*/
					recog.base.set_state(262);
					recog.intervalCompoundArg()?;

					}
				}
			,
				12 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 12)?;
					recog.base.enter_outer_alt(None, 12)?;
					{
					/*InvokeRule fixedCharArg*/
					recog.base.set_state(263);
					recog.fixedCharArg()?;

					}
				}
			,
				13 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 13)?;
					recog.base.enter_outer_alt(None, 13)?;
					{
					/*InvokeRule varCharArg*/
					recog.base.set_state(264);
					recog.varCharArg()?;

					}
				}
			,
				14 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 14)?;
					recog.base.enter_outer_alt(None, 14)?;
					{
					/*InvokeRule fixedBinaryArg*/
					recog.base.set_state(265);
					recog.fixedBinaryArg()?;

					}
				}
			,
				15 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 15)?;
					recog.base.enter_outer_alt(None, 15)?;
					{
					/*InvokeRule precisionTimeArg*/
					recog.base.set_state(266);
					recog.precisionTimeArg()?;

					}
				}
			,
				16 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 16)?;
					recog.base.enter_outer_alt(None, 16)?;
					{
					/*InvokeRule precisionTimestampArg*/
					recog.base.set_state(267);
					recog.precisionTimestampArg()?;

					}
				}
			,
				17 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 17)?;
					recog.base.enter_outer_alt(None, 17)?;
					{
					/*InvokeRule precisionTimestampTZArg*/
					recog.base.set_state(268);
					recog.precisionTimestampTZArg()?;

					}
				}
			,
				18 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 18)?;
					recog.base.enter_outer_alt(None, 18)?;
					{
					/*InvokeRule listArg*/
					recog.base.set_state(269);
					recog.listArg()?;

					}
				}
			,
				19 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 19)?;
					recog.base.enter_outer_alt(None, 19)?;
					{
					/*InvokeRule structArg*/
					recog.base.set_state(270);
					recog.structArg()?;

					}
				}
			,
				20 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 20)?;
					recog.base.enter_outer_alt(None, 20)?;
					{
					/*InvokeRule mapArg*/
					recog.base.set_state(271);
					recog.mapArg()?;

					}
				}
			,
				21 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 21)?;
					recog.base.enter_outer_alt(None, 21)?;
					{
					/*InvokeRule userDefinedArg*/
					recog.base.set_state(272);
					recog.userDefinedArg()?;

					}
				}
			,
				22 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 22)?;
					recog.base.enter_outer_alt(None, 22)?;
					{
					/*InvokeRule lambdaArg*/
					recog.base.set_state(273);
					recog.lambdaArg()?;

					}
				}
			,
				23 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 23)?;
					recog.base.enter_outer_alt(None, 23)?;
					{
					/*InvokeRule funcCallArg*/
					recog.base.set_state(274);
					recog.funcCallArg()?;

					}
				}
			,
				24 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 24)?;
					recog.base.enter_outer_alt(None, 24)?;
					{
					recog.base.set_state(275);
					recog.base.match_token(FuncTestCaseParser_Identifier,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- aggFuncTestCase ----------------
pub type AggFuncTestCaseContextAll<'input> = AggFuncTestCaseContext<'input>;


pub type AggFuncTestCaseContext<'input> = BaseParserRuleContext<'input,AggFuncTestCaseContextExt<'input>>;

#[derive(Clone)]
pub struct AggFuncTestCaseContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for AggFuncTestCaseContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for AggFuncTestCaseContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_aggFuncTestCase(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_aggFuncTestCase(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for AggFuncTestCaseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_aggFuncTestCase }
	//fn type_rule_index() -> usize where Self: Sized { RULE_aggFuncTestCase }
}
antlr4rust::tid!{AggFuncTestCaseContextExt<'a>}

impl<'input> AggFuncTestCaseContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<AggFuncTestCaseContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AggFuncTestCaseContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait AggFuncTestCaseContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<AggFuncTestCaseContextExt<'input>>{

fn aggFuncCall(&self) -> Option<Rc<AggFuncCallContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Eq
/// Returns `None` if there is no child corresponding to token Eq
fn Eq(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Eq, 0)
}
fn result(&self) -> Option<Rc<ResultContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token OBracket
/// Returns `None` if there is no child corresponding to token OBracket
fn OBracket(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_OBracket, 0)
}
fn funcOptions(&self) -> Option<Rc<FuncOptionsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token CBracket
/// Returns `None` if there is no child corresponding to token CBracket
fn CBracket(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_CBracket, 0)
}

}

impl<'input> AggFuncTestCaseContextAttrs<'input> for AggFuncTestCaseContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn aggFuncTestCase(&mut self,)
	-> Result<Rc<AggFuncTestCaseContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AggFuncTestCaseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_aggFuncTestCase);
        let mut _localctx: Rc<AggFuncTestCaseContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule aggFuncCall*/
			recog.base.set_state(278);
			recog.aggFuncCall()?;

			recog.base.set_state(283);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==FuncTestCaseParser_OBracket {
				{
				recog.base.set_state(279);
				recog.base.match_token(FuncTestCaseParser_OBracket,&mut recog.err_handler)?;

				/*InvokeRule funcOptions*/
				recog.base.set_state(280);
				recog.funcOptions()?;

				recog.base.set_state(281);
				recog.base.match_token(FuncTestCaseParser_CBracket,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(285);
			recog.base.match_token(FuncTestCaseParser_Eq,&mut recog.err_handler)?;

			/*InvokeRule result*/
			recog.base.set_state(286);
			recog.result()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- aggFuncCall ----------------
#[derive(Debug)]
pub enum AggFuncCallContextAll<'input>{
	SingleArgAggregateFuncCallContext(SingleArgAggregateFuncCallContext<'input>),
	MultiArgAggregateFuncCallContext(MultiArgAggregateFuncCallContext<'input>),
	CompactAggregateFuncCallContext(CompactAggregateFuncCallContext<'input>),
Error(AggFuncCallContext<'input>)
}
antlr4rust::tid!{AggFuncCallContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for AggFuncCallContextAll<'input>{}

impl<'input> FuncTestCaseParserContext<'input> for AggFuncCallContextAll<'input>{}

impl<'input> Deref for AggFuncCallContextAll<'input>{
	type Target = dyn AggFuncCallContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use AggFuncCallContextAll::*;
		match self{
			SingleArgAggregateFuncCallContext(inner) => inner,
			MultiArgAggregateFuncCallContext(inner) => inner,
			CompactAggregateFuncCallContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for AggFuncCallContextAll<'input>{
    fn enter(&self, listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
}



pub type AggFuncCallContext<'input> = BaseParserRuleContext<'input,AggFuncCallContextExt<'input>>;

#[derive(Clone)]
pub struct AggFuncCallContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for AggFuncCallContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for AggFuncCallContext<'input>{
}

impl<'input> CustomRuleContext<'input> for AggFuncCallContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_aggFuncCall }
	//fn type_rule_index() -> usize where Self: Sized { RULE_aggFuncCall }
}
antlr4rust::tid!{AggFuncCallContextExt<'a>}

impl<'input> AggFuncCallContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<AggFuncCallContextAll<'input>> {
		Rc::new(
		AggFuncCallContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AggFuncCallContextExt{

				ph:PhantomData
			}),
		)
		)
	}
}

pub trait AggFuncCallContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<AggFuncCallContextExt<'input>>{


}

impl<'input> AggFuncCallContextAttrs<'input> for AggFuncCallContext<'input>{}

pub type SingleArgAggregateFuncCallContext<'input> = BaseParserRuleContext<'input,SingleArgAggregateFuncCallContextExt<'input>>;

pub trait SingleArgAggregateFuncCallContextAttrs<'input>: FuncTestCaseParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token OParen
	/// Returns `None` if there is no child corresponding to token OParen
	fn OParen(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
		self.get_token(FuncTestCaseParser_OParen, 0)
	}
	fn dataColumn(&self) -> Option<Rc<DataColumnContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token CParen
	/// Returns `None` if there is no child corresponding to token CParen
	fn CParen(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
		self.get_token(FuncTestCaseParser_CParen, 0)
	}
	fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> SingleArgAggregateFuncCallContextAttrs<'input> for SingleArgAggregateFuncCallContext<'input>{}

pub struct SingleArgAggregateFuncCallContextExt<'input>{
	base:AggFuncCallContextExt<'input>,
	pub functName: Option<Rc<IdentifierContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{SingleArgAggregateFuncCallContextExt<'a>}

impl<'input> FuncTestCaseParserContext<'input> for SingleArgAggregateFuncCallContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for SingleArgAggregateFuncCallContext<'input>{
	fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_singleArgAggregateFuncCall(self);
		Ok(())
	}
}

impl<'input> CustomRuleContext<'input> for SingleArgAggregateFuncCallContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_aggFuncCall }
	//fn type_rule_index() -> usize where Self: Sized { RULE_aggFuncCall }
}

impl<'input> Borrow<AggFuncCallContextExt<'input>> for SingleArgAggregateFuncCallContext<'input>{
	fn borrow(&self) -> &AggFuncCallContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<AggFuncCallContextExt<'input>> for SingleArgAggregateFuncCallContext<'input>{
	fn borrow_mut(&mut self) -> &mut AggFuncCallContextExt<'input> { &mut self.base }
}

impl<'input> AggFuncCallContextAttrs<'input> for SingleArgAggregateFuncCallContext<'input> {}

impl<'input> SingleArgAggregateFuncCallContextExt<'input>{
	fn new(ctx: &dyn AggFuncCallContextAttrs<'input>) -> Rc<AggFuncCallContextAll<'input>>  {
		Rc::new(
			AggFuncCallContextAll::SingleArgAggregateFuncCallContext(
				BaseParserRuleContext::copy_from(ctx,SingleArgAggregateFuncCallContextExt{
        			functName:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type MultiArgAggregateFuncCallContext<'input> = BaseParserRuleContext<'input,MultiArgAggregateFuncCallContextExt<'input>>;

pub trait MultiArgAggregateFuncCallContextAttrs<'input>: FuncTestCaseParserContext<'input>{
	fn tableData(&self) -> Option<Rc<TableDataContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token OParen
	/// Returns `None` if there is no child corresponding to token OParen
	fn OParen(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
		self.get_token(FuncTestCaseParser_OParen, 0)
	}
	/// Retrieves first TerminalNode corresponding to token CParen
	/// Returns `None` if there is no child corresponding to token CParen
	fn CParen(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
		self.get_token(FuncTestCaseParser_CParen, 0)
	}
	fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn qualifiedAggregateFuncArgs(&self) -> Option<Rc<QualifiedAggregateFuncArgsContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> MultiArgAggregateFuncCallContextAttrs<'input> for MultiArgAggregateFuncCallContext<'input>{}

pub struct MultiArgAggregateFuncCallContextExt<'input>{
	base:AggFuncCallContextExt<'input>,
	pub funcName: Option<Rc<IdentifierContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{MultiArgAggregateFuncCallContextExt<'a>}

impl<'input> FuncTestCaseParserContext<'input> for MultiArgAggregateFuncCallContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for MultiArgAggregateFuncCallContext<'input>{
	fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_multiArgAggregateFuncCall(self);
		Ok(())
	}
}

impl<'input> CustomRuleContext<'input> for MultiArgAggregateFuncCallContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_aggFuncCall }
	//fn type_rule_index() -> usize where Self: Sized { RULE_aggFuncCall }
}

impl<'input> Borrow<AggFuncCallContextExt<'input>> for MultiArgAggregateFuncCallContext<'input>{
	fn borrow(&self) -> &AggFuncCallContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<AggFuncCallContextExt<'input>> for MultiArgAggregateFuncCallContext<'input>{
	fn borrow_mut(&mut self) -> &mut AggFuncCallContextExt<'input> { &mut self.base }
}

impl<'input> AggFuncCallContextAttrs<'input> for MultiArgAggregateFuncCallContext<'input> {}

impl<'input> MultiArgAggregateFuncCallContextExt<'input>{
	fn new(ctx: &dyn AggFuncCallContextAttrs<'input>) -> Rc<AggFuncCallContextAll<'input>>  {
		Rc::new(
			AggFuncCallContextAll::MultiArgAggregateFuncCallContext(
				BaseParserRuleContext::copy_from(ctx,MultiArgAggregateFuncCallContextExt{
        			funcName:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type CompactAggregateFuncCallContext<'input> = BaseParserRuleContext<'input,CompactAggregateFuncCallContextExt<'input>>;

pub trait CompactAggregateFuncCallContextAttrs<'input>: FuncTestCaseParserContext<'input>{
	fn tableRows(&self) -> Option<Rc<TableRowsContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token OParen
	/// Returns `None` if there is no child corresponding to token OParen
	fn OParen(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
		self.get_token(FuncTestCaseParser_OParen, 0)
	}
	/// Retrieves first TerminalNode corresponding to token CParen
	/// Returns `None` if there is no child corresponding to token CParen
	fn CParen(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
		self.get_token(FuncTestCaseParser_CParen, 0)
	}
	fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn aggregateFuncArgs(&self) -> Option<Rc<AggregateFuncArgsContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> CompactAggregateFuncCallContextAttrs<'input> for CompactAggregateFuncCallContext<'input>{}

pub struct CompactAggregateFuncCallContextExt<'input>{
	base:AggFuncCallContextExt<'input>,
	pub functName: Option<Rc<IdentifierContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{CompactAggregateFuncCallContextExt<'a>}

impl<'input> FuncTestCaseParserContext<'input> for CompactAggregateFuncCallContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for CompactAggregateFuncCallContext<'input>{
	fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_compactAggregateFuncCall(self);
		Ok(())
	}
}

impl<'input> CustomRuleContext<'input> for CompactAggregateFuncCallContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_aggFuncCall }
	//fn type_rule_index() -> usize where Self: Sized { RULE_aggFuncCall }
}

impl<'input> Borrow<AggFuncCallContextExt<'input>> for CompactAggregateFuncCallContext<'input>{
	fn borrow(&self) -> &AggFuncCallContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<AggFuncCallContextExt<'input>> for CompactAggregateFuncCallContext<'input>{
	fn borrow_mut(&mut self) -> &mut AggFuncCallContextExt<'input> { &mut self.base }
}

impl<'input> AggFuncCallContextAttrs<'input> for CompactAggregateFuncCallContext<'input> {}

impl<'input> CompactAggregateFuncCallContextExt<'input>{
	fn new(ctx: &dyn AggFuncCallContextAttrs<'input>) -> Rc<AggFuncCallContextAll<'input>>  {
		Rc::new(
			AggFuncCallContextAll::CompactAggregateFuncCallContext(
				BaseParserRuleContext::copy_from(ctx,CompactAggregateFuncCallContextExt{
        			functName:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn aggFuncCall(&mut self,)
	-> Result<Rc<AggFuncCallContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AggFuncCallContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_aggFuncCall);
        let mut _localctx: Rc<AggFuncCallContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(309);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			FuncTestCaseParser_Define 
				=> {
					let tmp = MultiArgAggregateFuncCallContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
					_localctx = tmp;
					{
					/*InvokeRule tableData*/
					recog.base.set_state(288);
					recog.tableData()?;

					/*InvokeRule identifier*/
					recog.base.set_state(289);
					let tmp = recog.identifier()?;
					if let AggFuncCallContextAll::MultiArgAggregateFuncCallContext(ctx) = cast_mut::<_,AggFuncCallContextAll >(&mut _localctx){
					ctx.funcName = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(290);
					recog.base.match_token(FuncTestCaseParser_OParen,&mut recog.err_handler)?;

					recog.base.set_state(292);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if ((((_la - 19)) & !0x3f) == 0 && ((1usize << (_la - 19)) & 1593851873) != 0) || ((((_la - 112)) & !0x3f) == 0 && ((1usize << (_la - 112)) & 17925) != 0) {
						{
						/*InvokeRule qualifiedAggregateFuncArgs*/
						recog.base.set_state(291);
						recog.qualifiedAggregateFuncArgs()?;

						}
					}

					recog.base.set_state(294);
					recog.base.match_token(FuncTestCaseParser_CParen,&mut recog.err_handler)?;

					}
				}

			FuncTestCaseParser_OParen 
				=> {
					let tmp = CompactAggregateFuncCallContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2)?;
					_localctx = tmp;
					{
					/*InvokeRule tableRows*/
					recog.base.set_state(296);
					recog.tableRows()?;

					/*InvokeRule identifier*/
					recog.base.set_state(297);
					let tmp = recog.identifier()?;
					if let AggFuncCallContextAll::CompactAggregateFuncCallContext(ctx) = cast_mut::<_,AggFuncCallContextAll >(&mut _localctx){
					ctx.functName = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(298);
					recog.base.match_token(FuncTestCaseParser_OParen,&mut recog.err_handler)?;

					recog.base.set_state(300);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & 4278714368) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 718849) != 0) || ((((_la - 112)) & !0x3f) == 0 && ((1usize << (_la - 112)) & 17925) != 0) {
						{
						/*InvokeRule aggregateFuncArgs*/
						recog.base.set_state(299);
						recog.aggregateFuncArgs()?;

						}
					}

					recog.base.set_state(302);
					recog.base.match_token(FuncTestCaseParser_CParen,&mut recog.err_handler)?;

					}
				}

			FuncTestCaseParser_Truncate |FuncTestCaseParser_And |FuncTestCaseParser_Or |
			FuncTestCaseParser_Identifier 
				=> {
					let tmp = SingleArgAggregateFuncCallContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3)?;
					_localctx = tmp;
					{
					/*InvokeRule identifier*/
					recog.base.set_state(304);
					let tmp = recog.identifier()?;
					if let AggFuncCallContextAll::SingleArgAggregateFuncCallContext(ctx) = cast_mut::<_,AggFuncCallContextAll >(&mut _localctx){
					ctx.functName = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(305);
					recog.base.match_token(FuncTestCaseParser_OParen,&mut recog.err_handler)?;

					/*InvokeRule dataColumn*/
					recog.base.set_state(306);
					recog.dataColumn()?;

					recog.base.set_state(307);
					recog.base.match_token(FuncTestCaseParser_CParen,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- tableData ----------------
pub type TableDataContextAll<'input> = TableDataContext<'input>;


pub type TableDataContext<'input> = BaseParserRuleContext<'input,TableDataContextExt<'input>>;

#[derive(Clone)]
pub struct TableDataContextExt<'input>{
	pub tableName: Option<TokenType<'input>>,
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for TableDataContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for TableDataContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tableData(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tableData(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for TableDataContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tableData }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tableData }
}
antlr4rust::tid!{TableDataContextExt<'a>}

impl<'input> TableDataContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<TableDataContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TableDataContextExt{
				tableName: None, 

				ph:PhantomData
			}),
		)
	}
}

pub trait TableDataContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<TableDataContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Define
/// Returns `None` if there is no child corresponding to token Define
fn Define(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Define, 0)
}
/// Retrieves first TerminalNode corresponding to token OParen
/// Returns `None` if there is no child corresponding to token OParen
fn OParen(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_OParen, 0)
}
fn dataType_all(&self) ->  Vec<Rc<DataTypeContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn dataType(&self, i: usize) -> Option<Rc<DataTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token CParen
/// Returns `None` if there is no child corresponding to token CParen
fn CParen(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_CParen, 0)
}
/// Retrieves first TerminalNode corresponding to token Eq
/// Returns `None` if there is no child corresponding to token Eq
fn Eq(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Eq, 0)
}
fn tableRows(&self) -> Option<Rc<TableRowsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Identifier, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Comma, i)
}

}

impl<'input> TableDataContextAttrs<'input> for TableDataContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tableData(&mut self,)
	-> Result<Rc<TableDataContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TableDataContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_tableData);
        let mut _localctx: Rc<TableDataContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(311);
			recog.base.match_token(FuncTestCaseParser_Define,&mut recog.err_handler)?;

			recog.base.set_state(312);
			let tmp = recog.base.match_token(FuncTestCaseParser_Identifier,&mut recog.err_handler)?;
			 cast_mut::<_,TableDataContext >(&mut _localctx).tableName = Some(tmp.clone());
			  

			recog.base.set_state(313);
			recog.base.match_token(FuncTestCaseParser_OParen,&mut recog.err_handler)?;

			/*InvokeRule dataType*/
			recog.base.set_state(314);
			recog.dataType()?;

			recog.base.set_state(319);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==FuncTestCaseParser_Comma {
				{
				{
				recog.base.set_state(315);
				recog.base.match_token(FuncTestCaseParser_Comma,&mut recog.err_handler)?;

				/*InvokeRule dataType*/
				recog.base.set_state(316);
				recog.dataType()?;

				}
				}
				recog.base.set_state(321);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(322);
			recog.base.match_token(FuncTestCaseParser_CParen,&mut recog.err_handler)?;

			recog.base.set_state(323);
			recog.base.match_token(FuncTestCaseParser_Eq,&mut recog.err_handler)?;

			/*InvokeRule tableRows*/
			recog.base.set_state(324);
			recog.tableRows()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- tableRows ----------------
pub type TableRowsContextAll<'input> = TableRowsContext<'input>;


pub type TableRowsContext<'input> = BaseParserRuleContext<'input,TableRowsContextExt<'input>>;

#[derive(Clone)]
pub struct TableRowsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for TableRowsContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for TableRowsContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_tableRows(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_tableRows(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for TableRowsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tableRows }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tableRows }
}
antlr4rust::tid!{TableRowsContextExt<'a>}

impl<'input> TableRowsContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<TableRowsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TableRowsContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait TableRowsContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<TableRowsContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token OParen
/// Returns `None` if there is no child corresponding to token OParen
fn OParen(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_OParen, 0)
}
/// Retrieves first TerminalNode corresponding to token CParen
/// Returns `None` if there is no child corresponding to token CParen
fn CParen(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_CParen, 0)
}
fn columnValues_all(&self) ->  Vec<Rc<ColumnValuesContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn columnValues(&self, i: usize) -> Option<Rc<ColumnValuesContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Comma, i)
}

}

impl<'input> TableRowsContextAttrs<'input> for TableRowsContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn tableRows(&mut self,)
	-> Result<Rc<TableRowsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TableRowsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_tableRows);
        let mut _localctx: Rc<TableRowsContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(326);
			recog.base.match_token(FuncTestCaseParser_OParen,&mut recog.err_handler)?;

			recog.base.set_state(335);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==FuncTestCaseParser_OParen {
				{
				/*InvokeRule columnValues*/
				recog.base.set_state(327);
				recog.columnValues()?;

				recog.base.set_state(332);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==FuncTestCaseParser_Comma {
					{
					{
					recog.base.set_state(328);
					recog.base.match_token(FuncTestCaseParser_Comma,&mut recog.err_handler)?;

					/*InvokeRule columnValues*/
					recog.base.set_state(329);
					recog.columnValues()?;

					}
					}
					recog.base.set_state(334);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			recog.base.set_state(337);
			recog.base.match_token(FuncTestCaseParser_CParen,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- dataColumn ----------------
pub type DataColumnContextAll<'input> = DataColumnContext<'input>;


pub type DataColumnContext<'input> = BaseParserRuleContext<'input,DataColumnContextExt<'input>>;

#[derive(Clone)]
pub struct DataColumnContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for DataColumnContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for DataColumnContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_dataColumn(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_dataColumn(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for DataColumnContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_dataColumn }
	//fn type_rule_index() -> usize where Self: Sized { RULE_dataColumn }
}
antlr4rust::tid!{DataColumnContextExt<'a>}

impl<'input> DataColumnContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<DataColumnContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DataColumnContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait DataColumnContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<DataColumnContextExt<'input>>{

fn columnValues(&self) -> Option<Rc<ColumnValuesContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token DoubleColon
/// Returns `None` if there is no child corresponding to token DoubleColon
fn DoubleColon(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_DoubleColon, 0)
}
fn dataType(&self) -> Option<Rc<DataTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> DataColumnContextAttrs<'input> for DataColumnContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn dataColumn(&mut self,)
	-> Result<Rc<DataColumnContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DataColumnContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_dataColumn);
        let mut _localctx: Rc<DataColumnContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule columnValues*/
			recog.base.set_state(339);
			recog.columnValues()?;

			recog.base.set_state(340);
			recog.base.match_token(FuncTestCaseParser_DoubleColon,&mut recog.err_handler)?;

			/*InvokeRule dataType*/
			recog.base.set_state(341);
			recog.dataType()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- columnValues ----------------
pub type ColumnValuesContextAll<'input> = ColumnValuesContext<'input>;


pub type ColumnValuesContext<'input> = BaseParserRuleContext<'input,ColumnValuesContextExt<'input>>;

#[derive(Clone)]
pub struct ColumnValuesContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for ColumnValuesContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for ColumnValuesContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_columnValues(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_columnValues(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for ColumnValuesContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_columnValues }
	//fn type_rule_index() -> usize where Self: Sized { RULE_columnValues }
}
antlr4rust::tid!{ColumnValuesContextExt<'a>}

impl<'input> ColumnValuesContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ColumnValuesContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ColumnValuesContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ColumnValuesContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<ColumnValuesContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token OParen
/// Returns `None` if there is no child corresponding to token OParen
fn OParen(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_OParen, 0)
}
/// Retrieves first TerminalNode corresponding to token CParen
/// Returns `None` if there is no child corresponding to token CParen
fn CParen(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_CParen, 0)
}
fn literal_all(&self) ->  Vec<Rc<LiteralContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn literal(&self, i: usize) -> Option<Rc<LiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Comma, i)
}

}

impl<'input> ColumnValuesContextAttrs<'input> for ColumnValuesContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn columnValues(&mut self,)
	-> Result<Rc<ColumnValuesContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ColumnValuesContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_columnValues);
        let mut _localctx: Rc<ColumnValuesContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(343);
			recog.base.match_token(FuncTestCaseParser_OParen,&mut recog.err_handler)?;

			recog.base.set_state(352);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 19)) & !0x3f) == 0 && ((1usize << (_la - 19)) & 520110017) != 0) {
				{
				/*InvokeRule literal*/
				recog.base.set_state(344);
				recog.literal()?;

				recog.base.set_state(349);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==FuncTestCaseParser_Comma {
					{
					{
					recog.base.set_state(345);
					recog.base.match_token(FuncTestCaseParser_Comma,&mut recog.err_handler)?;

					/*InvokeRule literal*/
					recog.base.set_state(346);
					recog.literal()?;

					}
					}
					recog.base.set_state(351);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			recog.base.set_state(354);
			recog.base.match_token(FuncTestCaseParser_CParen,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- literal ----------------
pub type LiteralContextAll<'input> = LiteralContext<'input>;


pub type LiteralContext<'input> = BaseParserRuleContext<'input,LiteralContextExt<'input>>;

#[derive(Clone)]
pub struct LiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for LiteralContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for LiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_literal(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_literal(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for LiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_literal }
	//fn type_rule_index() -> usize where Self: Sized { RULE_literal }
}
antlr4rust::tid!{LiteralContextExt<'a>}

impl<'input> LiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<LiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LiteralContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait LiteralContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<LiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token NullLiteral
/// Returns `None` if there is no child corresponding to token NullLiteral
fn NullLiteral(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_NullLiteral, 0)
}
fn numericLiteral(&self) -> Option<Rc<NumericLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token BooleanLiteral
/// Returns `None` if there is no child corresponding to token BooleanLiteral
fn BooleanLiteral(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_BooleanLiteral, 0)
}
/// Retrieves first TerminalNode corresponding to token StringLiteral
/// Returns `None` if there is no child corresponding to token StringLiteral
fn StringLiteral(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_StringLiteral, 0)
}
/// Retrieves first TerminalNode corresponding to token DateLiteral
/// Returns `None` if there is no child corresponding to token DateLiteral
fn DateLiteral(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_DateLiteral, 0)
}
/// Retrieves first TerminalNode corresponding to token TimeLiteral
/// Returns `None` if there is no child corresponding to token TimeLiteral
fn TimeLiteral(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_TimeLiteral, 0)
}
/// Retrieves first TerminalNode corresponding to token TimestampLiteral
/// Returns `None` if there is no child corresponding to token TimestampLiteral
fn TimestampLiteral(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_TimestampLiteral, 0)
}
/// Retrieves first TerminalNode corresponding to token TimestampTzLiteral
/// Returns `None` if there is no child corresponding to token TimestampTzLiteral
fn TimestampTzLiteral(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_TimestampTzLiteral, 0)
}
/// Retrieves first TerminalNode corresponding to token IntervalYearLiteral
/// Returns `None` if there is no child corresponding to token IntervalYearLiteral
fn IntervalYearLiteral(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_IntervalYearLiteral, 0)
}
/// Retrieves first TerminalNode corresponding to token IntervalDayLiteral
/// Returns `None` if there is no child corresponding to token IntervalDayLiteral
fn IntervalDayLiteral(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_IntervalDayLiteral, 0)
}
/// Retrieves first TerminalNode corresponding to token IntervalCompoundLiteral
/// Returns `None` if there is no child corresponding to token IntervalCompoundLiteral
fn IntervalCompoundLiteral(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_IntervalCompoundLiteral, 0)
}

}

impl<'input> LiteralContextAttrs<'input> for LiteralContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn literal(&mut self,)
	-> Result<Rc<LiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_literal);
        let mut _localctx: Rc<LiteralContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(367);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			FuncTestCaseParser_NullLiteral 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(356);
					recog.base.match_token(FuncTestCaseParser_NullLiteral,&mut recog.err_handler)?;

					}
				}

			FuncTestCaseParser_NaN |FuncTestCaseParser_IntegerLiteral |FuncTestCaseParser_DecimalLiteral |
			FuncTestCaseParser_FloatLiteral 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule numericLiteral*/
					recog.base.set_state(357);
					recog.numericLiteral()?;

					}
				}

			FuncTestCaseParser_BooleanLiteral 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					recog.base.set_state(358);
					recog.base.match_token(FuncTestCaseParser_BooleanLiteral,&mut recog.err_handler)?;

					}
				}

			FuncTestCaseParser_StringLiteral 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					recog.base.set_state(359);
					recog.base.match_token(FuncTestCaseParser_StringLiteral,&mut recog.err_handler)?;

					}
				}

			FuncTestCaseParser_DateLiteral 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					recog.base.set_state(360);
					recog.base.match_token(FuncTestCaseParser_DateLiteral,&mut recog.err_handler)?;

					}
				}

			FuncTestCaseParser_TimeLiteral 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 6)?;
					recog.base.enter_outer_alt(None, 6)?;
					{
					recog.base.set_state(361);
					recog.base.match_token(FuncTestCaseParser_TimeLiteral,&mut recog.err_handler)?;

					}
				}

			FuncTestCaseParser_TimestampLiteral 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 7)?;
					recog.base.enter_outer_alt(None, 7)?;
					{
					recog.base.set_state(362);
					recog.base.match_token(FuncTestCaseParser_TimestampLiteral,&mut recog.err_handler)?;

					}
				}

			FuncTestCaseParser_TimestampTzLiteral 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 8)?;
					recog.base.enter_outer_alt(None, 8)?;
					{
					recog.base.set_state(363);
					recog.base.match_token(FuncTestCaseParser_TimestampTzLiteral,&mut recog.err_handler)?;

					}
				}

			FuncTestCaseParser_IntervalYearLiteral 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 9)?;
					recog.base.enter_outer_alt(None, 9)?;
					{
					recog.base.set_state(364);
					recog.base.match_token(FuncTestCaseParser_IntervalYearLiteral,&mut recog.err_handler)?;

					}
				}

			FuncTestCaseParser_IntervalDayLiteral 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 10)?;
					recog.base.enter_outer_alt(None, 10)?;
					{
					recog.base.set_state(365);
					recog.base.match_token(FuncTestCaseParser_IntervalDayLiteral,&mut recog.err_handler)?;

					}
				}

			FuncTestCaseParser_IntervalCompoundLiteral 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 11)?;
					recog.base.enter_outer_alt(None, 11)?;
					{
					recog.base.set_state(366);
					recog.base.match_token(FuncTestCaseParser_IntervalCompoundLiteral,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- qualifiedAggregateFuncArgs ----------------
pub type QualifiedAggregateFuncArgsContextAll<'input> = QualifiedAggregateFuncArgsContext<'input>;


pub type QualifiedAggregateFuncArgsContext<'input> = BaseParserRuleContext<'input,QualifiedAggregateFuncArgsContextExt<'input>>;

#[derive(Clone)]
pub struct QualifiedAggregateFuncArgsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for QualifiedAggregateFuncArgsContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for QualifiedAggregateFuncArgsContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_qualifiedAggregateFuncArgs(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_qualifiedAggregateFuncArgs(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for QualifiedAggregateFuncArgsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_qualifiedAggregateFuncArgs }
	//fn type_rule_index() -> usize where Self: Sized { RULE_qualifiedAggregateFuncArgs }
}
antlr4rust::tid!{QualifiedAggregateFuncArgsContextExt<'a>}

impl<'input> QualifiedAggregateFuncArgsContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<QualifiedAggregateFuncArgsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,QualifiedAggregateFuncArgsContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait QualifiedAggregateFuncArgsContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<QualifiedAggregateFuncArgsContextExt<'input>>{

fn qualifiedAggregateFuncArg_all(&self) ->  Vec<Rc<QualifiedAggregateFuncArgContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn qualifiedAggregateFuncArg(&self, i: usize) -> Option<Rc<QualifiedAggregateFuncArgContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Comma, i)
}

}

impl<'input> QualifiedAggregateFuncArgsContextAttrs<'input> for QualifiedAggregateFuncArgsContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn qualifiedAggregateFuncArgs(&mut self,)
	-> Result<Rc<QualifiedAggregateFuncArgsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = QualifiedAggregateFuncArgsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_qualifiedAggregateFuncArgs);
        let mut _localctx: Rc<QualifiedAggregateFuncArgsContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule qualifiedAggregateFuncArg*/
			recog.base.set_state(369);
			recog.qualifiedAggregateFuncArg()?;

			recog.base.set_state(374);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==FuncTestCaseParser_Comma {
				{
				{
				recog.base.set_state(370);
				recog.base.match_token(FuncTestCaseParser_Comma,&mut recog.err_handler)?;

				/*InvokeRule qualifiedAggregateFuncArg*/
				recog.base.set_state(371);
				recog.qualifiedAggregateFuncArg()?;

				}
				}
				recog.base.set_state(376);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- aggregateFuncArgs ----------------
pub type AggregateFuncArgsContextAll<'input> = AggregateFuncArgsContext<'input>;


pub type AggregateFuncArgsContext<'input> = BaseParserRuleContext<'input,AggregateFuncArgsContextExt<'input>>;

#[derive(Clone)]
pub struct AggregateFuncArgsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for AggregateFuncArgsContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for AggregateFuncArgsContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_aggregateFuncArgs(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_aggregateFuncArgs(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for AggregateFuncArgsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_aggregateFuncArgs }
	//fn type_rule_index() -> usize where Self: Sized { RULE_aggregateFuncArgs }
}
antlr4rust::tid!{AggregateFuncArgsContextExt<'a>}

impl<'input> AggregateFuncArgsContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<AggregateFuncArgsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AggregateFuncArgsContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait AggregateFuncArgsContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<AggregateFuncArgsContextExt<'input>>{

fn aggregateFuncArg_all(&self) ->  Vec<Rc<AggregateFuncArgContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn aggregateFuncArg(&self, i: usize) -> Option<Rc<AggregateFuncArgContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Comma, i)
}

}

impl<'input> AggregateFuncArgsContextAttrs<'input> for AggregateFuncArgsContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn aggregateFuncArgs(&mut self,)
	-> Result<Rc<AggregateFuncArgsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AggregateFuncArgsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_aggregateFuncArgs);
        let mut _localctx: Rc<AggregateFuncArgsContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule aggregateFuncArg*/
			recog.base.set_state(377);
			recog.aggregateFuncArg()?;

			recog.base.set_state(382);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==FuncTestCaseParser_Comma {
				{
				{
				recog.base.set_state(378);
				recog.base.match_token(FuncTestCaseParser_Comma,&mut recog.err_handler)?;

				/*InvokeRule aggregateFuncArg*/
				recog.base.set_state(379);
				recog.aggregateFuncArg()?;

				}
				}
				recog.base.set_state(384);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- qualifiedAggregateFuncArg ----------------
pub type QualifiedAggregateFuncArgContextAll<'input> = QualifiedAggregateFuncArgContext<'input>;


pub type QualifiedAggregateFuncArgContext<'input> = BaseParserRuleContext<'input,QualifiedAggregateFuncArgContextExt<'input>>;

#[derive(Clone)]
pub struct QualifiedAggregateFuncArgContextExt<'input>{
	pub tableName: Option<TokenType<'input>>,
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for QualifiedAggregateFuncArgContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for QualifiedAggregateFuncArgContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_qualifiedAggregateFuncArg(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_qualifiedAggregateFuncArg(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for QualifiedAggregateFuncArgContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_qualifiedAggregateFuncArg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_qualifiedAggregateFuncArg }
}
antlr4rust::tid!{QualifiedAggregateFuncArgContextExt<'a>}

impl<'input> QualifiedAggregateFuncArgContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<QualifiedAggregateFuncArgContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,QualifiedAggregateFuncArgContextExt{
				tableName: None, 

				ph:PhantomData
			}),
		)
	}
}

pub trait QualifiedAggregateFuncArgContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<QualifiedAggregateFuncArgContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Dot
/// Returns `None` if there is no child corresponding to token Dot
fn Dot(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Dot, 0)
}
/// Retrieves first TerminalNode corresponding to token ColumnName
/// Returns `None` if there is no child corresponding to token ColumnName
fn ColumnName(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_ColumnName, 0)
}
/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Identifier, 0)
}
fn argument(&self) -> Option<Rc<ArgumentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> QualifiedAggregateFuncArgContextAttrs<'input> for QualifiedAggregateFuncArgContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn qualifiedAggregateFuncArg(&mut self,)
	-> Result<Rc<QualifiedAggregateFuncArgContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = QualifiedAggregateFuncArgContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 40, RULE_qualifiedAggregateFuncArg);
        let mut _localctx: Rc<QualifiedAggregateFuncArgContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(389);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(23,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(385);
					let tmp = recog.base.match_token(FuncTestCaseParser_Identifier,&mut recog.err_handler)?;
					 cast_mut::<_,QualifiedAggregateFuncArgContext >(&mut _localctx).tableName = Some(tmp.clone());
					  

					recog.base.set_state(386);
					recog.base.match_token(FuncTestCaseParser_Dot,&mut recog.err_handler)?;

					recog.base.set_state(387);
					recog.base.match_token(FuncTestCaseParser_ColumnName,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule argument*/
					recog.base.set_state(388);
					recog.argument()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- aggregateFuncArg ----------------
pub type AggregateFuncArgContextAll<'input> = AggregateFuncArgContext<'input>;


pub type AggregateFuncArgContext<'input> = BaseParserRuleContext<'input,AggregateFuncArgContextExt<'input>>;

#[derive(Clone)]
pub struct AggregateFuncArgContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for AggregateFuncArgContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for AggregateFuncArgContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_aggregateFuncArg(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_aggregateFuncArg(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for AggregateFuncArgContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_aggregateFuncArg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_aggregateFuncArg }
}
antlr4rust::tid!{AggregateFuncArgContextExt<'a>}

impl<'input> AggregateFuncArgContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<AggregateFuncArgContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AggregateFuncArgContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait AggregateFuncArgContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<AggregateFuncArgContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ColumnName
/// Returns `None` if there is no child corresponding to token ColumnName
fn ColumnName(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_ColumnName, 0)
}
/// Retrieves first TerminalNode corresponding to token DoubleColon
/// Returns `None` if there is no child corresponding to token DoubleColon
fn DoubleColon(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_DoubleColon, 0)
}
fn dataType(&self) -> Option<Rc<DataTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn argument(&self) -> Option<Rc<ArgumentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> AggregateFuncArgContextAttrs<'input> for AggregateFuncArgContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn aggregateFuncArg(&mut self,)
	-> Result<Rc<AggregateFuncArgContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AggregateFuncArgContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 42, RULE_aggregateFuncArg);
        let mut _localctx: Rc<AggregateFuncArgContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(395);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			FuncTestCaseParser_ColumnName 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(391);
					recog.base.match_token(FuncTestCaseParser_ColumnName,&mut recog.err_handler)?;

					recog.base.set_state(392);
					recog.base.match_token(FuncTestCaseParser_DoubleColon,&mut recog.err_handler)?;

					/*InvokeRule dataType*/
					recog.base.set_state(393);
					recog.dataType()?;

					}
				}

			FuncTestCaseParser_NaN |FuncTestCaseParser_Truncate |FuncTestCaseParser_IntegerLiteral |
			FuncTestCaseParser_DecimalLiteral |FuncTestCaseParser_FloatLiteral |FuncTestCaseParser_BooleanLiteral |
			FuncTestCaseParser_TimestampTzLiteral |FuncTestCaseParser_TimestampLiteral |
			FuncTestCaseParser_TimeLiteral |FuncTestCaseParser_DateLiteral |FuncTestCaseParser_IntervalYearLiteral |
			FuncTestCaseParser_IntervalDayLiteral |FuncTestCaseParser_IntervalCompoundLiteral |
			FuncTestCaseParser_NullLiteral |FuncTestCaseParser_StringLiteral |FuncTestCaseParser_OBrace |
			FuncTestCaseParser_OParen |FuncTestCaseParser_OBracket |FuncTestCaseParser_And |
			FuncTestCaseParser_Or |FuncTestCaseParser_Identifier 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule argument*/
					recog.base.set_state(394);
					recog.argument()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- numericLiteral ----------------
pub type NumericLiteralContextAll<'input> = NumericLiteralContext<'input>;


pub type NumericLiteralContext<'input> = BaseParserRuleContext<'input,NumericLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct NumericLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for NumericLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for NumericLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_numericLiteral(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_numericLiteral(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for NumericLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_numericLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_numericLiteral }
}
antlr4rust::tid!{NumericLiteralContextExt<'a>}

impl<'input> NumericLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<NumericLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NumericLiteralContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait NumericLiteralContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<NumericLiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token DecimalLiteral
/// Returns `None` if there is no child corresponding to token DecimalLiteral
fn DecimalLiteral(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_DecimalLiteral, 0)
}
/// Retrieves first TerminalNode corresponding to token IntegerLiteral
/// Returns `None` if there is no child corresponding to token IntegerLiteral
fn IntegerLiteral(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_IntegerLiteral, 0)
}
fn floatLiteral(&self) -> Option<Rc<FloatLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> NumericLiteralContextAttrs<'input> for NumericLiteralContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn numericLiteral(&mut self,)
	-> Result<Rc<NumericLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NumericLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 44, RULE_numericLiteral);
        let mut _localctx: Rc<NumericLiteralContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(400);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			FuncTestCaseParser_DecimalLiteral 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(397);
					recog.base.match_token(FuncTestCaseParser_DecimalLiteral,&mut recog.err_handler)?;

					}
				}

			FuncTestCaseParser_IntegerLiteral 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(398);
					recog.base.match_token(FuncTestCaseParser_IntegerLiteral,&mut recog.err_handler)?;

					}
				}

			FuncTestCaseParser_NaN |FuncTestCaseParser_FloatLiteral 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule floatLiteral*/
					recog.base.set_state(399);
					recog.floatLiteral()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- floatLiteral ----------------
pub type FloatLiteralContextAll<'input> = FloatLiteralContext<'input>;


pub type FloatLiteralContext<'input> = BaseParserRuleContext<'input,FloatLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct FloatLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for FloatLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for FloatLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_floatLiteral(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_floatLiteral(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for FloatLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_floatLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_floatLiteral }
}
antlr4rust::tid!{FloatLiteralContextExt<'a>}

impl<'input> FloatLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<FloatLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FloatLiteralContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait FloatLiteralContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<FloatLiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token FloatLiteral
/// Returns `None` if there is no child corresponding to token FloatLiteral
fn FloatLiteral(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_FloatLiteral, 0)
}
/// Retrieves first TerminalNode corresponding to token NaN
/// Returns `None` if there is no child corresponding to token NaN
fn NaN(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_NaN, 0)
}

}

impl<'input> FloatLiteralContextAttrs<'input> for FloatLiteralContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn floatLiteral(&mut self,)
	-> Result<Rc<FloatLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FloatLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 46, RULE_floatLiteral);
        let mut _localctx: Rc<FloatLiteralContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(402);
			_la = recog.base.input.la(1);
			if { !(_la==FuncTestCaseParser_NaN || _la==FuncTestCaseParser_FloatLiteral) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- nullArg ----------------
pub type NullArgContextAll<'input> = NullArgContext<'input>;


pub type NullArgContext<'input> = BaseParserRuleContext<'input,NullArgContextExt<'input>>;

#[derive(Clone)]
pub struct NullArgContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for NullArgContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for NullArgContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_nullArg(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_nullArg(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for NullArgContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_nullArg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_nullArg }
}
antlr4rust::tid!{NullArgContextExt<'a>}

impl<'input> NullArgContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<NullArgContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NullArgContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait NullArgContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<NullArgContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token NullLiteral
/// Returns `None` if there is no child corresponding to token NullLiteral
fn NullLiteral(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_NullLiteral, 0)
}
/// Retrieves first TerminalNode corresponding to token DoubleColon
/// Returns `None` if there is no child corresponding to token DoubleColon
fn DoubleColon(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_DoubleColon, 0)
}
fn dataType(&self) -> Option<Rc<DataTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> NullArgContextAttrs<'input> for NullArgContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn nullArg(&mut self,)
	-> Result<Rc<NullArgContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NullArgContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 48, RULE_nullArg);
        let mut _localctx: Rc<NullArgContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(404);
			recog.base.match_token(FuncTestCaseParser_NullLiteral,&mut recog.err_handler)?;

			recog.base.set_state(405);
			recog.base.match_token(FuncTestCaseParser_DoubleColon,&mut recog.err_handler)?;

			/*InvokeRule dataType*/
			recog.base.set_state(406);
			recog.dataType()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- intArg ----------------
pub type IntArgContextAll<'input> = IntArgContext<'input>;


pub type IntArgContext<'input> = BaseParserRuleContext<'input,IntArgContextExt<'input>>;

#[derive(Clone)]
pub struct IntArgContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for IntArgContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for IntArgContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_intArg(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_intArg(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for IntArgContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_intArg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_intArg }
}
antlr4rust::tid!{IntArgContextExt<'a>}

impl<'input> IntArgContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<IntArgContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IntArgContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait IntArgContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<IntArgContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IntegerLiteral
/// Returns `None` if there is no child corresponding to token IntegerLiteral
fn IntegerLiteral(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_IntegerLiteral, 0)
}
/// Retrieves first TerminalNode corresponding to token DoubleColon
/// Returns `None` if there is no child corresponding to token DoubleColon
fn DoubleColon(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_DoubleColon, 0)
}
fn intType(&self) -> Option<Rc<IntTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> IntArgContextAttrs<'input> for IntArgContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn intArg(&mut self,)
	-> Result<Rc<IntArgContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IntArgContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 50, RULE_intArg);
        let mut _localctx: Rc<IntArgContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(408);
			recog.base.match_token(FuncTestCaseParser_IntegerLiteral,&mut recog.err_handler)?;

			recog.base.set_state(409);
			recog.base.match_token(FuncTestCaseParser_DoubleColon,&mut recog.err_handler)?;

			/*InvokeRule intType*/
			recog.base.set_state(410);
			recog.intType()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- floatArg ----------------
pub type FloatArgContextAll<'input> = FloatArgContext<'input>;


pub type FloatArgContext<'input> = BaseParserRuleContext<'input,FloatArgContextExt<'input>>;

#[derive(Clone)]
pub struct FloatArgContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for FloatArgContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for FloatArgContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_floatArg(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_floatArg(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for FloatArgContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_floatArg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_floatArg }
}
antlr4rust::tid!{FloatArgContextExt<'a>}

impl<'input> FloatArgContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<FloatArgContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FloatArgContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait FloatArgContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<FloatArgContextExt<'input>>{

fn numericLiteral(&self) -> Option<Rc<NumericLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token DoubleColon
/// Returns `None` if there is no child corresponding to token DoubleColon
fn DoubleColon(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_DoubleColon, 0)
}
fn floatType(&self) -> Option<Rc<FloatTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> FloatArgContextAttrs<'input> for FloatArgContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn floatArg(&mut self,)
	-> Result<Rc<FloatArgContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FloatArgContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 52, RULE_floatArg);
        let mut _localctx: Rc<FloatArgContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule numericLiteral*/
			recog.base.set_state(412);
			recog.numericLiteral()?;

			recog.base.set_state(413);
			recog.base.match_token(FuncTestCaseParser_DoubleColon,&mut recog.err_handler)?;

			/*InvokeRule floatType*/
			recog.base.set_state(414);
			recog.floatType()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- decimalArg ----------------
pub type DecimalArgContextAll<'input> = DecimalArgContext<'input>;


pub type DecimalArgContext<'input> = BaseParserRuleContext<'input,DecimalArgContextExt<'input>>;

#[derive(Clone)]
pub struct DecimalArgContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for DecimalArgContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for DecimalArgContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_decimalArg(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_decimalArg(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for DecimalArgContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_decimalArg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_decimalArg }
}
antlr4rust::tid!{DecimalArgContextExt<'a>}

impl<'input> DecimalArgContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<DecimalArgContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DecimalArgContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait DecimalArgContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<DecimalArgContextExt<'input>>{

fn numericLiteral(&self) -> Option<Rc<NumericLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token DoubleColon
/// Returns `None` if there is no child corresponding to token DoubleColon
fn DoubleColon(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_DoubleColon, 0)
}
fn decimalType(&self) -> Option<Rc<DecimalTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> DecimalArgContextAttrs<'input> for DecimalArgContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn decimalArg(&mut self,)
	-> Result<Rc<DecimalArgContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DecimalArgContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 54, RULE_decimalArg);
        let mut _localctx: Rc<DecimalArgContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule numericLiteral*/
			recog.base.set_state(416);
			recog.numericLiteral()?;

			recog.base.set_state(417);
			recog.base.match_token(FuncTestCaseParser_DoubleColon,&mut recog.err_handler)?;

			/*InvokeRule decimalType*/
			recog.base.set_state(418);
			recog.decimalType()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- booleanArg ----------------
pub type BooleanArgContextAll<'input> = BooleanArgContext<'input>;


pub type BooleanArgContext<'input> = BaseParserRuleContext<'input,BooleanArgContextExt<'input>>;

#[derive(Clone)]
pub struct BooleanArgContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for BooleanArgContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for BooleanArgContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_booleanArg(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_booleanArg(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for BooleanArgContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_booleanArg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_booleanArg }
}
antlr4rust::tid!{BooleanArgContextExt<'a>}

impl<'input> BooleanArgContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<BooleanArgContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BooleanArgContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait BooleanArgContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<BooleanArgContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token BooleanLiteral
/// Returns `None` if there is no child corresponding to token BooleanLiteral
fn BooleanLiteral(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_BooleanLiteral, 0)
}
/// Retrieves first TerminalNode corresponding to token DoubleColon
/// Returns `None` if there is no child corresponding to token DoubleColon
fn DoubleColon(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_DoubleColon, 0)
}
fn booleanType(&self) -> Option<Rc<BooleanTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> BooleanArgContextAttrs<'input> for BooleanArgContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn booleanArg(&mut self,)
	-> Result<Rc<BooleanArgContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BooleanArgContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 56, RULE_booleanArg);
        let mut _localctx: Rc<BooleanArgContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(420);
			recog.base.match_token(FuncTestCaseParser_BooleanLiteral,&mut recog.err_handler)?;

			recog.base.set_state(421);
			recog.base.match_token(FuncTestCaseParser_DoubleColon,&mut recog.err_handler)?;

			/*InvokeRule booleanType*/
			recog.base.set_state(422);
			recog.booleanType()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- stringArg ----------------
pub type StringArgContextAll<'input> = StringArgContext<'input>;


pub type StringArgContext<'input> = BaseParserRuleContext<'input,StringArgContextExt<'input>>;

#[derive(Clone)]
pub struct StringArgContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for StringArgContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for StringArgContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_stringArg(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_stringArg(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for StringArgContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stringArg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stringArg }
}
antlr4rust::tid!{StringArgContextExt<'a>}

impl<'input> StringArgContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<StringArgContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StringArgContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait StringArgContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<StringArgContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token StringLiteral
/// Returns `None` if there is no child corresponding to token StringLiteral
fn StringLiteral(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_StringLiteral, 0)
}
/// Retrieves first TerminalNode corresponding to token DoubleColon
/// Returns `None` if there is no child corresponding to token DoubleColon
fn DoubleColon(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_DoubleColon, 0)
}
fn stringType(&self) -> Option<Rc<StringTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> StringArgContextAttrs<'input> for StringArgContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn stringArg(&mut self,)
	-> Result<Rc<StringArgContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StringArgContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 58, RULE_stringArg);
        let mut _localctx: Rc<StringArgContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(424);
			recog.base.match_token(FuncTestCaseParser_StringLiteral,&mut recog.err_handler)?;

			recog.base.set_state(425);
			recog.base.match_token(FuncTestCaseParser_DoubleColon,&mut recog.err_handler)?;

			/*InvokeRule stringType*/
			recog.base.set_state(426);
			recog.stringType()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- dateArg ----------------
pub type DateArgContextAll<'input> = DateArgContext<'input>;


pub type DateArgContext<'input> = BaseParserRuleContext<'input,DateArgContextExt<'input>>;

#[derive(Clone)]
pub struct DateArgContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for DateArgContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for DateArgContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_dateArg(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_dateArg(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for DateArgContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_dateArg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_dateArg }
}
antlr4rust::tid!{DateArgContextExt<'a>}

impl<'input> DateArgContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<DateArgContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DateArgContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait DateArgContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<DateArgContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token DateLiteral
/// Returns `None` if there is no child corresponding to token DateLiteral
fn DateLiteral(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_DateLiteral, 0)
}
/// Retrieves first TerminalNode corresponding to token DoubleColon
/// Returns `None` if there is no child corresponding to token DoubleColon
fn DoubleColon(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_DoubleColon, 0)
}
fn dateType(&self) -> Option<Rc<DateTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> DateArgContextAttrs<'input> for DateArgContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn dateArg(&mut self,)
	-> Result<Rc<DateArgContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DateArgContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 60, RULE_dateArg);
        let mut _localctx: Rc<DateArgContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(428);
			recog.base.match_token(FuncTestCaseParser_DateLiteral,&mut recog.err_handler)?;

			recog.base.set_state(429);
			recog.base.match_token(FuncTestCaseParser_DoubleColon,&mut recog.err_handler)?;

			/*InvokeRule dateType*/
			recog.base.set_state(430);
			recog.dateType()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- intervalYearArg ----------------
pub type IntervalYearArgContextAll<'input> = IntervalYearArgContext<'input>;


pub type IntervalYearArgContext<'input> = BaseParserRuleContext<'input,IntervalYearArgContextExt<'input>>;

#[derive(Clone)]
pub struct IntervalYearArgContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for IntervalYearArgContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for IntervalYearArgContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_intervalYearArg(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_intervalYearArg(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for IntervalYearArgContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_intervalYearArg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_intervalYearArg }
}
antlr4rust::tid!{IntervalYearArgContextExt<'a>}

impl<'input> IntervalYearArgContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<IntervalYearArgContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IntervalYearArgContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait IntervalYearArgContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<IntervalYearArgContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IntervalYearLiteral
/// Returns `None` if there is no child corresponding to token IntervalYearLiteral
fn IntervalYearLiteral(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_IntervalYearLiteral, 0)
}
/// Retrieves first TerminalNode corresponding to token DoubleColon
/// Returns `None` if there is no child corresponding to token DoubleColon
fn DoubleColon(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_DoubleColon, 0)
}
fn intervalYearType(&self) -> Option<Rc<IntervalYearTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> IntervalYearArgContextAttrs<'input> for IntervalYearArgContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn intervalYearArg(&mut self,)
	-> Result<Rc<IntervalYearArgContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IntervalYearArgContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 62, RULE_intervalYearArg);
        let mut _localctx: Rc<IntervalYearArgContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(432);
			recog.base.match_token(FuncTestCaseParser_IntervalYearLiteral,&mut recog.err_handler)?;

			recog.base.set_state(433);
			recog.base.match_token(FuncTestCaseParser_DoubleColon,&mut recog.err_handler)?;

			/*InvokeRule intervalYearType*/
			recog.base.set_state(434);
			recog.intervalYearType()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- intervalDayArg ----------------
pub type IntervalDayArgContextAll<'input> = IntervalDayArgContext<'input>;


pub type IntervalDayArgContext<'input> = BaseParserRuleContext<'input,IntervalDayArgContextExt<'input>>;

#[derive(Clone)]
pub struct IntervalDayArgContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for IntervalDayArgContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for IntervalDayArgContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_intervalDayArg(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_intervalDayArg(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for IntervalDayArgContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_intervalDayArg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_intervalDayArg }
}
antlr4rust::tid!{IntervalDayArgContextExt<'a>}

impl<'input> IntervalDayArgContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<IntervalDayArgContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IntervalDayArgContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait IntervalDayArgContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<IntervalDayArgContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IntervalDayLiteral
/// Returns `None` if there is no child corresponding to token IntervalDayLiteral
fn IntervalDayLiteral(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_IntervalDayLiteral, 0)
}
/// Retrieves first TerminalNode corresponding to token DoubleColon
/// Returns `None` if there is no child corresponding to token DoubleColon
fn DoubleColon(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_DoubleColon, 0)
}
fn intervalDayType(&self) -> Option<Rc<IntervalDayTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> IntervalDayArgContextAttrs<'input> for IntervalDayArgContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn intervalDayArg(&mut self,)
	-> Result<Rc<IntervalDayArgContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IntervalDayArgContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 64, RULE_intervalDayArg);
        let mut _localctx: Rc<IntervalDayArgContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(436);
			recog.base.match_token(FuncTestCaseParser_IntervalDayLiteral,&mut recog.err_handler)?;

			recog.base.set_state(437);
			recog.base.match_token(FuncTestCaseParser_DoubleColon,&mut recog.err_handler)?;

			/*InvokeRule intervalDayType*/
			recog.base.set_state(438);
			recog.intervalDayType()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- intervalCompoundArg ----------------
pub type IntervalCompoundArgContextAll<'input> = IntervalCompoundArgContext<'input>;


pub type IntervalCompoundArgContext<'input> = BaseParserRuleContext<'input,IntervalCompoundArgContextExt<'input>>;

#[derive(Clone)]
pub struct IntervalCompoundArgContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for IntervalCompoundArgContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for IntervalCompoundArgContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_intervalCompoundArg(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_intervalCompoundArg(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for IntervalCompoundArgContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_intervalCompoundArg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_intervalCompoundArg }
}
antlr4rust::tid!{IntervalCompoundArgContextExt<'a>}

impl<'input> IntervalCompoundArgContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<IntervalCompoundArgContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IntervalCompoundArgContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait IntervalCompoundArgContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<IntervalCompoundArgContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IntervalCompoundLiteral
/// Returns `None` if there is no child corresponding to token IntervalCompoundLiteral
fn IntervalCompoundLiteral(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_IntervalCompoundLiteral, 0)
}
/// Retrieves first TerminalNode corresponding to token DoubleColon
/// Returns `None` if there is no child corresponding to token DoubleColon
fn DoubleColon(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_DoubleColon, 0)
}
fn intervalCompoundType(&self) -> Option<Rc<IntervalCompoundTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> IntervalCompoundArgContextAttrs<'input> for IntervalCompoundArgContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn intervalCompoundArg(&mut self,)
	-> Result<Rc<IntervalCompoundArgContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IntervalCompoundArgContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 66, RULE_intervalCompoundArg);
        let mut _localctx: Rc<IntervalCompoundArgContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(440);
			recog.base.match_token(FuncTestCaseParser_IntervalCompoundLiteral,&mut recog.err_handler)?;

			recog.base.set_state(441);
			recog.base.match_token(FuncTestCaseParser_DoubleColon,&mut recog.err_handler)?;

			/*InvokeRule intervalCompoundType*/
			recog.base.set_state(442);
			recog.intervalCompoundType()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- fixedCharArg ----------------
pub type FixedCharArgContextAll<'input> = FixedCharArgContext<'input>;


pub type FixedCharArgContext<'input> = BaseParserRuleContext<'input,FixedCharArgContextExt<'input>>;

#[derive(Clone)]
pub struct FixedCharArgContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for FixedCharArgContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for FixedCharArgContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_fixedCharArg(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_fixedCharArg(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for FixedCharArgContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fixedCharArg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fixedCharArg }
}
antlr4rust::tid!{FixedCharArgContextExt<'a>}

impl<'input> FixedCharArgContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<FixedCharArgContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FixedCharArgContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait FixedCharArgContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<FixedCharArgContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token StringLiteral
/// Returns `None` if there is no child corresponding to token StringLiteral
fn StringLiteral(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_StringLiteral, 0)
}
/// Retrieves first TerminalNode corresponding to token DoubleColon
/// Returns `None` if there is no child corresponding to token DoubleColon
fn DoubleColon(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_DoubleColon, 0)
}
fn fixedCharType(&self) -> Option<Rc<FixedCharTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> FixedCharArgContextAttrs<'input> for FixedCharArgContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn fixedCharArg(&mut self,)
	-> Result<Rc<FixedCharArgContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FixedCharArgContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 68, RULE_fixedCharArg);
        let mut _localctx: Rc<FixedCharArgContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(444);
			recog.base.match_token(FuncTestCaseParser_StringLiteral,&mut recog.err_handler)?;

			recog.base.set_state(445);
			recog.base.match_token(FuncTestCaseParser_DoubleColon,&mut recog.err_handler)?;

			/*InvokeRule fixedCharType*/
			recog.base.set_state(446);
			recog.fixedCharType()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- varCharArg ----------------
pub type VarCharArgContextAll<'input> = VarCharArgContext<'input>;


pub type VarCharArgContext<'input> = BaseParserRuleContext<'input,VarCharArgContextExt<'input>>;

#[derive(Clone)]
pub struct VarCharArgContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for VarCharArgContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for VarCharArgContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_varCharArg(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_varCharArg(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for VarCharArgContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_varCharArg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_varCharArg }
}
antlr4rust::tid!{VarCharArgContextExt<'a>}

impl<'input> VarCharArgContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<VarCharArgContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,VarCharArgContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait VarCharArgContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<VarCharArgContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token StringLiteral
/// Returns `None` if there is no child corresponding to token StringLiteral
fn StringLiteral(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_StringLiteral, 0)
}
/// Retrieves first TerminalNode corresponding to token DoubleColon
/// Returns `None` if there is no child corresponding to token DoubleColon
fn DoubleColon(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_DoubleColon, 0)
}
fn varCharType(&self) -> Option<Rc<VarCharTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> VarCharArgContextAttrs<'input> for VarCharArgContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn varCharArg(&mut self,)
	-> Result<Rc<VarCharArgContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = VarCharArgContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 70, RULE_varCharArg);
        let mut _localctx: Rc<VarCharArgContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(448);
			recog.base.match_token(FuncTestCaseParser_StringLiteral,&mut recog.err_handler)?;

			recog.base.set_state(449);
			recog.base.match_token(FuncTestCaseParser_DoubleColon,&mut recog.err_handler)?;

			/*InvokeRule varCharType*/
			recog.base.set_state(450);
			recog.varCharType()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- fixedBinaryArg ----------------
pub type FixedBinaryArgContextAll<'input> = FixedBinaryArgContext<'input>;


pub type FixedBinaryArgContext<'input> = BaseParserRuleContext<'input,FixedBinaryArgContextExt<'input>>;

#[derive(Clone)]
pub struct FixedBinaryArgContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for FixedBinaryArgContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for FixedBinaryArgContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_fixedBinaryArg(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_fixedBinaryArg(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for FixedBinaryArgContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fixedBinaryArg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fixedBinaryArg }
}
antlr4rust::tid!{FixedBinaryArgContextExt<'a>}

impl<'input> FixedBinaryArgContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<FixedBinaryArgContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FixedBinaryArgContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait FixedBinaryArgContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<FixedBinaryArgContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token StringLiteral
/// Returns `None` if there is no child corresponding to token StringLiteral
fn StringLiteral(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_StringLiteral, 0)
}
/// Retrieves first TerminalNode corresponding to token DoubleColon
/// Returns `None` if there is no child corresponding to token DoubleColon
fn DoubleColon(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_DoubleColon, 0)
}
fn fixedBinaryType(&self) -> Option<Rc<FixedBinaryTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> FixedBinaryArgContextAttrs<'input> for FixedBinaryArgContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn fixedBinaryArg(&mut self,)
	-> Result<Rc<FixedBinaryArgContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FixedBinaryArgContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 72, RULE_fixedBinaryArg);
        let mut _localctx: Rc<FixedBinaryArgContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(452);
			recog.base.match_token(FuncTestCaseParser_StringLiteral,&mut recog.err_handler)?;

			recog.base.set_state(453);
			recog.base.match_token(FuncTestCaseParser_DoubleColon,&mut recog.err_handler)?;

			/*InvokeRule fixedBinaryType*/
			recog.base.set_state(454);
			recog.fixedBinaryType()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- precisionTimeArg ----------------
pub type PrecisionTimeArgContextAll<'input> = PrecisionTimeArgContext<'input>;


pub type PrecisionTimeArgContext<'input> = BaseParserRuleContext<'input,PrecisionTimeArgContextExt<'input>>;

#[derive(Clone)]
pub struct PrecisionTimeArgContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for PrecisionTimeArgContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for PrecisionTimeArgContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_precisionTimeArg(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_precisionTimeArg(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for PrecisionTimeArgContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_precisionTimeArg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_precisionTimeArg }
}
antlr4rust::tid!{PrecisionTimeArgContextExt<'a>}

impl<'input> PrecisionTimeArgContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<PrecisionTimeArgContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PrecisionTimeArgContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait PrecisionTimeArgContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<PrecisionTimeArgContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token TimeLiteral
/// Returns `None` if there is no child corresponding to token TimeLiteral
fn TimeLiteral(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_TimeLiteral, 0)
}
/// Retrieves first TerminalNode corresponding to token DoubleColon
/// Returns `None` if there is no child corresponding to token DoubleColon
fn DoubleColon(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_DoubleColon, 0)
}
fn precisionTimeType(&self) -> Option<Rc<PrecisionTimeTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> PrecisionTimeArgContextAttrs<'input> for PrecisionTimeArgContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn precisionTimeArg(&mut self,)
	-> Result<Rc<PrecisionTimeArgContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PrecisionTimeArgContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 74, RULE_precisionTimeArg);
        let mut _localctx: Rc<PrecisionTimeArgContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(456);
			recog.base.match_token(FuncTestCaseParser_TimeLiteral,&mut recog.err_handler)?;

			recog.base.set_state(457);
			recog.base.match_token(FuncTestCaseParser_DoubleColon,&mut recog.err_handler)?;

			/*InvokeRule precisionTimeType*/
			recog.base.set_state(458);
			recog.precisionTimeType()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- precisionTimestampArg ----------------
pub type PrecisionTimestampArgContextAll<'input> = PrecisionTimestampArgContext<'input>;


pub type PrecisionTimestampArgContext<'input> = BaseParserRuleContext<'input,PrecisionTimestampArgContextExt<'input>>;

#[derive(Clone)]
pub struct PrecisionTimestampArgContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for PrecisionTimestampArgContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for PrecisionTimestampArgContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_precisionTimestampArg(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_precisionTimestampArg(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for PrecisionTimestampArgContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_precisionTimestampArg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_precisionTimestampArg }
}
antlr4rust::tid!{PrecisionTimestampArgContextExt<'a>}

impl<'input> PrecisionTimestampArgContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<PrecisionTimestampArgContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PrecisionTimestampArgContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait PrecisionTimestampArgContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<PrecisionTimestampArgContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token TimestampLiteral
/// Returns `None` if there is no child corresponding to token TimestampLiteral
fn TimestampLiteral(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_TimestampLiteral, 0)
}
/// Retrieves first TerminalNode corresponding to token DoubleColon
/// Returns `None` if there is no child corresponding to token DoubleColon
fn DoubleColon(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_DoubleColon, 0)
}
fn precisionTimestampType(&self) -> Option<Rc<PrecisionTimestampTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> PrecisionTimestampArgContextAttrs<'input> for PrecisionTimestampArgContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn precisionTimestampArg(&mut self,)
	-> Result<Rc<PrecisionTimestampArgContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PrecisionTimestampArgContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 76, RULE_precisionTimestampArg);
        let mut _localctx: Rc<PrecisionTimestampArgContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(460);
			recog.base.match_token(FuncTestCaseParser_TimestampLiteral,&mut recog.err_handler)?;

			recog.base.set_state(461);
			recog.base.match_token(FuncTestCaseParser_DoubleColon,&mut recog.err_handler)?;

			/*InvokeRule precisionTimestampType*/
			recog.base.set_state(462);
			recog.precisionTimestampType()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- precisionTimestampTZArg ----------------
pub type PrecisionTimestampTZArgContextAll<'input> = PrecisionTimestampTZArgContext<'input>;


pub type PrecisionTimestampTZArgContext<'input> = BaseParserRuleContext<'input,PrecisionTimestampTZArgContextExt<'input>>;

#[derive(Clone)]
pub struct PrecisionTimestampTZArgContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for PrecisionTimestampTZArgContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for PrecisionTimestampTZArgContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_precisionTimestampTZArg(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_precisionTimestampTZArg(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for PrecisionTimestampTZArgContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_precisionTimestampTZArg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_precisionTimestampTZArg }
}
antlr4rust::tid!{PrecisionTimestampTZArgContextExt<'a>}

impl<'input> PrecisionTimestampTZArgContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<PrecisionTimestampTZArgContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PrecisionTimestampTZArgContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait PrecisionTimestampTZArgContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<PrecisionTimestampTZArgContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token TimestampTzLiteral
/// Returns `None` if there is no child corresponding to token TimestampTzLiteral
fn TimestampTzLiteral(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_TimestampTzLiteral, 0)
}
/// Retrieves first TerminalNode corresponding to token DoubleColon
/// Returns `None` if there is no child corresponding to token DoubleColon
fn DoubleColon(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_DoubleColon, 0)
}
fn precisionTimestampTZType(&self) -> Option<Rc<PrecisionTimestampTZTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> PrecisionTimestampTZArgContextAttrs<'input> for PrecisionTimestampTZArgContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn precisionTimestampTZArg(&mut self,)
	-> Result<Rc<PrecisionTimestampTZArgContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PrecisionTimestampTZArgContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 78, RULE_precisionTimestampTZArg);
        let mut _localctx: Rc<PrecisionTimestampTZArgContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(464);
			recog.base.match_token(FuncTestCaseParser_TimestampTzLiteral,&mut recog.err_handler)?;

			recog.base.set_state(465);
			recog.base.match_token(FuncTestCaseParser_DoubleColon,&mut recog.err_handler)?;

			/*InvokeRule precisionTimestampTZType*/
			recog.base.set_state(466);
			recog.precisionTimestampTZType()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- listArg ----------------
pub type ListArgContextAll<'input> = ListArgContext<'input>;


pub type ListArgContext<'input> = BaseParserRuleContext<'input,ListArgContextExt<'input>>;

#[derive(Clone)]
pub struct ListArgContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for ListArgContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for ListArgContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_listArg(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_listArg(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for ListArgContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_listArg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_listArg }
}
antlr4rust::tid!{ListArgContextExt<'a>}

impl<'input> ListArgContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ListArgContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ListArgContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ListArgContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<ListArgContextExt<'input>>{

fn literalList(&self) -> Option<Rc<LiteralListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token DoubleColon
/// Returns `None` if there is no child corresponding to token DoubleColon
fn DoubleColon(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_DoubleColon, 0)
}
fn listType(&self) -> Option<Rc<ListTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ListArgContextAttrs<'input> for ListArgContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn listArg(&mut self,)
	-> Result<Rc<ListArgContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ListArgContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 80, RULE_listArg);
        let mut _localctx: Rc<ListArgContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule literalList*/
			recog.base.set_state(468);
			recog.literalList()?;

			recog.base.set_state(469);
			recog.base.match_token(FuncTestCaseParser_DoubleColon,&mut recog.err_handler)?;

			/*InvokeRule listType*/
			recog.base.set_state(470);
			recog.listType()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- structArg ----------------
pub type StructArgContextAll<'input> = StructArgContext<'input>;


pub type StructArgContext<'input> = BaseParserRuleContext<'input,StructArgContextExt<'input>>;

#[derive(Clone)]
pub struct StructArgContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for StructArgContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for StructArgContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_structArg(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_structArg(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for StructArgContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_structArg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_structArg }
}
antlr4rust::tid!{StructArgContextExt<'a>}

impl<'input> StructArgContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<StructArgContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StructArgContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait StructArgContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<StructArgContextExt<'input>>{

fn literalStruct(&self) -> Option<Rc<LiteralStructContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token DoubleColon
/// Returns `None` if there is no child corresponding to token DoubleColon
fn DoubleColon(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_DoubleColon, 0)
}
fn structType(&self) -> Option<Rc<StructTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> StructArgContextAttrs<'input> for StructArgContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn structArg(&mut self,)
	-> Result<Rc<StructArgContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StructArgContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 82, RULE_structArg);
        let mut _localctx: Rc<StructArgContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule literalStruct*/
			recog.base.set_state(472);
			recog.literalStruct()?;

			recog.base.set_state(473);
			recog.base.match_token(FuncTestCaseParser_DoubleColon,&mut recog.err_handler)?;

			/*InvokeRule structType*/
			recog.base.set_state(474);
			recog.structType()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- mapArg ----------------
pub type MapArgContextAll<'input> = MapArgContext<'input>;


pub type MapArgContext<'input> = BaseParserRuleContext<'input,MapArgContextExt<'input>>;

#[derive(Clone)]
pub struct MapArgContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for MapArgContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for MapArgContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_mapArg(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_mapArg(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for MapArgContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_mapArg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mapArg }
}
antlr4rust::tid!{MapArgContextExt<'a>}

impl<'input> MapArgContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<MapArgContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MapArgContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait MapArgContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<MapArgContextExt<'input>>{

fn literalMap(&self) -> Option<Rc<LiteralMapContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token DoubleColon
/// Returns `None` if there is no child corresponding to token DoubleColon
fn DoubleColon(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_DoubleColon, 0)
}
fn mapType(&self) -> Option<Rc<MapTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> MapArgContextAttrs<'input> for MapArgContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn mapArg(&mut self,)
	-> Result<Rc<MapArgContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MapArgContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 84, RULE_mapArg);
        let mut _localctx: Rc<MapArgContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule literalMap*/
			recog.base.set_state(476);
			recog.literalMap()?;

			recog.base.set_state(477);
			recog.base.match_token(FuncTestCaseParser_DoubleColon,&mut recog.err_handler)?;

			/*InvokeRule mapType*/
			recog.base.set_state(478);
			recog.mapType()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- userDefinedArg ----------------
pub type UserDefinedArgContextAll<'input> = UserDefinedArgContext<'input>;


pub type UserDefinedArgContext<'input> = BaseParserRuleContext<'input,UserDefinedArgContextExt<'input>>;

#[derive(Clone)]
pub struct UserDefinedArgContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for UserDefinedArgContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for UserDefinedArgContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_userDefinedArg(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_userDefinedArg(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for UserDefinedArgContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_userDefinedArg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_userDefinedArg }
}
antlr4rust::tid!{UserDefinedArgContextExt<'a>}

impl<'input> UserDefinedArgContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<UserDefinedArgContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,UserDefinedArgContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait UserDefinedArgContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<UserDefinedArgContextExt<'input>>{

fn literalStruct(&self) -> Option<Rc<LiteralStructContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token DoubleColon
/// Returns `None` if there is no child corresponding to token DoubleColon
fn DoubleColon(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_DoubleColon, 0)
}
fn userDefinedType(&self) -> Option<Rc<UserDefinedTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> UserDefinedArgContextAttrs<'input> for UserDefinedArgContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn userDefinedArg(&mut self,)
	-> Result<Rc<UserDefinedArgContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = UserDefinedArgContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 86, RULE_userDefinedArg);
        let mut _localctx: Rc<UserDefinedArgContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule literalStruct*/
			recog.base.set_state(480);
			recog.literalStruct()?;

			recog.base.set_state(481);
			recog.base.match_token(FuncTestCaseParser_DoubleColon,&mut recog.err_handler)?;

			/*InvokeRule userDefinedType*/
			recog.base.set_state(482);
			recog.userDefinedType()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- lambdaArg ----------------
pub type LambdaArgContextAll<'input> = LambdaArgContext<'input>;


pub type LambdaArgContext<'input> = BaseParserRuleContext<'input,LambdaArgContextExt<'input>>;

#[derive(Clone)]
pub struct LambdaArgContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for LambdaArgContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for LambdaArgContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_lambdaArg(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_lambdaArg(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for LambdaArgContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_lambdaArg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_lambdaArg }
}
antlr4rust::tid!{LambdaArgContextExt<'a>}

impl<'input> LambdaArgContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<LambdaArgContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LambdaArgContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait LambdaArgContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<LambdaArgContextExt<'input>>{

fn literalLambda(&self) -> Option<Rc<LiteralLambdaContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token DoubleColon
/// Returns `None` if there is no child corresponding to token DoubleColon
fn DoubleColon(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_DoubleColon, 0)
}
fn funcType(&self) -> Option<Rc<FuncTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> LambdaArgContextAttrs<'input> for LambdaArgContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn lambdaArg(&mut self,)
	-> Result<Rc<LambdaArgContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LambdaArgContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 88, RULE_lambdaArg);
        let mut _localctx: Rc<LambdaArgContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule literalLambda*/
			recog.base.set_state(484);
			recog.literalLambda()?;

			recog.base.set_state(485);
			recog.base.match_token(FuncTestCaseParser_DoubleColon,&mut recog.err_handler)?;

			/*InvokeRule funcType*/
			recog.base.set_state(486);
			recog.funcType()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- funcCallArg ----------------
pub type FuncCallArgContextAll<'input> = FuncCallArgContext<'input>;


pub type FuncCallArgContext<'input> = BaseParserRuleContext<'input,FuncCallArgContextExt<'input>>;

#[derive(Clone)]
pub struct FuncCallArgContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for FuncCallArgContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for FuncCallArgContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_funcCallArg(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_funcCallArg(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for FuncCallArgContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_funcCallArg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_funcCallArg }
}
antlr4rust::tid!{FuncCallArgContextExt<'a>}

impl<'input> FuncCallArgContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<FuncCallArgContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FuncCallArgContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait FuncCallArgContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<FuncCallArgContextExt<'input>>{

fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token OParen
/// Returns `None` if there is no child corresponding to token OParen
fn OParen(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_OParen, 0)
}
fn arguments(&self) -> Option<Rc<ArgumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token CParen
/// Returns `None` if there is no child corresponding to token CParen
fn CParen(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_CParen, 0)
}

}

impl<'input> FuncCallArgContextAttrs<'input> for FuncCallArgContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn funcCallArg(&mut self,)
	-> Result<Rc<FuncCallArgContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FuncCallArgContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 90, RULE_funcCallArg);
        let mut _localctx: Rc<FuncCallArgContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule identifier*/
			recog.base.set_state(488);
			recog.identifier()?;

			recog.base.set_state(489);
			recog.base.match_token(FuncTestCaseParser_OParen,&mut recog.err_handler)?;

			/*InvokeRule arguments*/
			recog.base.set_state(490);
			recog.arguments()?;

			recog.base.set_state(491);
			recog.base.match_token(FuncTestCaseParser_CParen,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- enumArg ----------------
pub type EnumArgContextAll<'input> = EnumArgContext<'input>;


pub type EnumArgContext<'input> = BaseParserRuleContext<'input,EnumArgContextExt<'input>>;

#[derive(Clone)]
pub struct EnumArgContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for EnumArgContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for EnumArgContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_enumArg(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_enumArg(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for EnumArgContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_enumArg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_enumArg }
}
antlr4rust::tid!{EnumArgContextExt<'a>}

impl<'input> EnumArgContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<EnumArgContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EnumArgContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait EnumArgContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<EnumArgContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Identifier, 0)
}
/// Retrieves first TerminalNode corresponding to token DoubleColon
/// Returns `None` if there is no child corresponding to token DoubleColon
fn DoubleColon(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_DoubleColon, 0)
}
/// Retrieves first TerminalNode corresponding to token EnumType
/// Returns `None` if there is no child corresponding to token EnumType
fn EnumType(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_EnumType, 0)
}

}

impl<'input> EnumArgContextAttrs<'input> for EnumArgContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn enumArg(&mut self,)
	-> Result<Rc<EnumArgContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EnumArgContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 92, RULE_enumArg);
        let mut _localctx: Rc<EnumArgContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(493);
			recog.base.match_token(FuncTestCaseParser_Identifier,&mut recog.err_handler)?;

			recog.base.set_state(494);
			recog.base.match_token(FuncTestCaseParser_DoubleColon,&mut recog.err_handler)?;

			recog.base.set_state(495);
			recog.base.match_token(FuncTestCaseParser_EnumType,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- literalList ----------------
pub type LiteralListContextAll<'input> = LiteralListContext<'input>;


pub type LiteralListContext<'input> = BaseParserRuleContext<'input,LiteralListContextExt<'input>>;

#[derive(Clone)]
pub struct LiteralListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for LiteralListContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for LiteralListContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_literalList(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_literalList(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for LiteralListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_literalList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_literalList }
}
antlr4rust::tid!{LiteralListContextExt<'a>}

impl<'input> LiteralListContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<LiteralListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LiteralListContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait LiteralListContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<LiteralListContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token OBracket
/// Returns `None` if there is no child corresponding to token OBracket
fn OBracket(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_OBracket, 0)
}
/// Retrieves first TerminalNode corresponding to token CBracket
/// Returns `None` if there is no child corresponding to token CBracket
fn CBracket(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_CBracket, 0)
}
fn compoundLiteral_all(&self) ->  Vec<Rc<CompoundLiteralContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn compoundLiteral(&self, i: usize) -> Option<Rc<CompoundLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Comma, i)
}

}

impl<'input> LiteralListContextAttrs<'input> for LiteralListContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn literalList(&mut self,)
	-> Result<Rc<LiteralListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LiteralListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 94, RULE_literalList);
        let mut _localctx: Rc<LiteralListContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(497);
			recog.base.match_token(FuncTestCaseParser_OBracket,&mut recog.err_handler)?;

			recog.base.set_state(506);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 19)) & !0x3f) == 0 && ((1usize << (_la - 19)) & 1593851841) != 0) || _la==FuncTestCaseParser_OParen || _la==FuncTestCaseParser_OBracket {
				{
				/*InvokeRule compoundLiteral*/
				recog.base.set_state(498);
				recog.compoundLiteral()?;

				recog.base.set_state(503);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==FuncTestCaseParser_Comma {
					{
					{
					recog.base.set_state(499);
					recog.base.match_token(FuncTestCaseParser_Comma,&mut recog.err_handler)?;

					/*InvokeRule compoundLiteral*/
					recog.base.set_state(500);
					recog.compoundLiteral()?;

					}
					}
					recog.base.set_state(505);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			recog.base.set_state(508);
			recog.base.match_token(FuncTestCaseParser_CBracket,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- literalStruct ----------------
pub type LiteralStructContextAll<'input> = LiteralStructContext<'input>;


pub type LiteralStructContext<'input> = BaseParserRuleContext<'input,LiteralStructContextExt<'input>>;

#[derive(Clone)]
pub struct LiteralStructContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for LiteralStructContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for LiteralStructContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_literalStruct(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_literalStruct(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for LiteralStructContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_literalStruct }
	//fn type_rule_index() -> usize where Self: Sized { RULE_literalStruct }
}
antlr4rust::tid!{LiteralStructContextExt<'a>}

impl<'input> LiteralStructContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<LiteralStructContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LiteralStructContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait LiteralStructContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<LiteralStructContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token OParen
/// Returns `None` if there is no child corresponding to token OParen
fn OParen(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_OParen, 0)
}
/// Retrieves first TerminalNode corresponding to token CParen
/// Returns `None` if there is no child corresponding to token CParen
fn CParen(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_CParen, 0)
}
fn compoundLiteral_all(&self) ->  Vec<Rc<CompoundLiteralContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn compoundLiteral(&self, i: usize) -> Option<Rc<CompoundLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Comma, i)
}

}

impl<'input> LiteralStructContextAttrs<'input> for LiteralStructContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn literalStruct(&mut self,)
	-> Result<Rc<LiteralStructContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LiteralStructContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 96, RULE_literalStruct);
        let mut _localctx: Rc<LiteralStructContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(510);
			recog.base.match_token(FuncTestCaseParser_OParen,&mut recog.err_handler)?;

			recog.base.set_state(519);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 19)) & !0x3f) == 0 && ((1usize << (_la - 19)) & 1593851841) != 0) || _la==FuncTestCaseParser_OParen || _la==FuncTestCaseParser_OBracket {
				{
				/*InvokeRule compoundLiteral*/
				recog.base.set_state(511);
				recog.compoundLiteral()?;

				recog.base.set_state(516);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==FuncTestCaseParser_Comma {
					{
					{
					recog.base.set_state(512);
					recog.base.match_token(FuncTestCaseParser_Comma,&mut recog.err_handler)?;

					/*InvokeRule compoundLiteral*/
					recog.base.set_state(513);
					recog.compoundLiteral()?;

					}
					}
					recog.base.set_state(518);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			recog.base.set_state(521);
			recog.base.match_token(FuncTestCaseParser_CParen,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- literalMap ----------------
pub type LiteralMapContextAll<'input> = LiteralMapContext<'input>;


pub type LiteralMapContext<'input> = BaseParserRuleContext<'input,LiteralMapContextExt<'input>>;

#[derive(Clone)]
pub struct LiteralMapContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for LiteralMapContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for LiteralMapContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_literalMap(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_literalMap(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for LiteralMapContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_literalMap }
	//fn type_rule_index() -> usize where Self: Sized { RULE_literalMap }
}
antlr4rust::tid!{LiteralMapContextExt<'a>}

impl<'input> LiteralMapContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<LiteralMapContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LiteralMapContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait LiteralMapContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<LiteralMapContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token OBrace
/// Returns `None` if there is no child corresponding to token OBrace
fn OBrace(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_OBrace, 0)
}
/// Retrieves first TerminalNode corresponding to token CBrace
/// Returns `None` if there is no child corresponding to token CBrace
fn CBrace(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_CBrace, 0)
}
fn mapEntry_all(&self) ->  Vec<Rc<MapEntryContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn mapEntry(&self, i: usize) -> Option<Rc<MapEntryContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Comma, i)
}

}

impl<'input> LiteralMapContextAttrs<'input> for LiteralMapContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn literalMap(&mut self,)
	-> Result<Rc<LiteralMapContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LiteralMapContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 98, RULE_literalMap);
        let mut _localctx: Rc<LiteralMapContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(523);
			recog.base.match_token(FuncTestCaseParser_OBrace,&mut recog.err_handler)?;

			recog.base.set_state(532);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 19)) & !0x3f) == 0 && ((1usize << (_la - 19)) & 1593851841) != 0) || _la==FuncTestCaseParser_OParen || _la==FuncTestCaseParser_OBracket {
				{
				/*InvokeRule mapEntry*/
				recog.base.set_state(524);
				recog.mapEntry()?;

				recog.base.set_state(529);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==FuncTestCaseParser_Comma {
					{
					{
					recog.base.set_state(525);
					recog.base.match_token(FuncTestCaseParser_Comma,&mut recog.err_handler)?;

					/*InvokeRule mapEntry*/
					recog.base.set_state(526);
					recog.mapEntry()?;

					}
					}
					recog.base.set_state(531);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			recog.base.set_state(534);
			recog.base.match_token(FuncTestCaseParser_CBrace,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- mapEntry ----------------
pub type MapEntryContextAll<'input> = MapEntryContext<'input>;


pub type MapEntryContext<'input> = BaseParserRuleContext<'input,MapEntryContextExt<'input>>;

#[derive(Clone)]
pub struct MapEntryContextExt<'input>{
	pub key: Option<Rc<CompoundLiteralContextAll<'input>>>,
	pub value: Option<Rc<CompoundLiteralContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for MapEntryContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for MapEntryContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_mapEntry(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_mapEntry(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for MapEntryContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_mapEntry }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mapEntry }
}
antlr4rust::tid!{MapEntryContextExt<'a>}

impl<'input> MapEntryContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<MapEntryContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MapEntryContextExt{
				key: None, value: None, 

				ph:PhantomData
			}),
		)
	}
}

pub trait MapEntryContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<MapEntryContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Colon
/// Returns `None` if there is no child corresponding to token Colon
fn Colon(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Colon, 0)
}
fn compoundLiteral_all(&self) ->  Vec<Rc<CompoundLiteralContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn compoundLiteral(&self, i: usize) -> Option<Rc<CompoundLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> MapEntryContextAttrs<'input> for MapEntryContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn mapEntry(&mut self,)
	-> Result<Rc<MapEntryContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MapEntryContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 100, RULE_mapEntry);
        let mut _localctx: Rc<MapEntryContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule compoundLiteral*/
			recog.base.set_state(536);
			let tmp = recog.compoundLiteral()?;
			 cast_mut::<_,MapEntryContext >(&mut _localctx).key = Some(tmp.clone());
			  

			recog.base.set_state(537);
			recog.base.match_token(FuncTestCaseParser_Colon,&mut recog.err_handler)?;

			/*InvokeRule compoundLiteral*/
			recog.base.set_state(538);
			let tmp = recog.compoundLiteral()?;
			 cast_mut::<_,MapEntryContext >(&mut _localctx).value = Some(tmp.clone());
			  

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- compoundLiteral ----------------
pub type CompoundLiteralContextAll<'input> = CompoundLiteralContext<'input>;


pub type CompoundLiteralContext<'input> = BaseParserRuleContext<'input,CompoundLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct CompoundLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for CompoundLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for CompoundLiteralContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_compoundLiteral(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_compoundLiteral(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for CompoundLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_compoundLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_compoundLiteral }
}
antlr4rust::tid!{CompoundLiteralContextExt<'a>}

impl<'input> CompoundLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<CompoundLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CompoundLiteralContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait CompoundLiteralContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<CompoundLiteralContextExt<'input>>{

fn literal(&self) -> Option<Rc<LiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn literalList(&self) -> Option<Rc<LiteralListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn literalStruct(&self) -> Option<Rc<LiteralStructContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn literalMap(&self) -> Option<Rc<LiteralMapContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> CompoundLiteralContextAttrs<'input> for CompoundLiteralContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn compoundLiteral(&mut self,)
	-> Result<Rc<CompoundLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CompoundLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 102, RULE_compoundLiteral);
        let mut _localctx: Rc<CompoundLiteralContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(544);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			FuncTestCaseParser_NaN |FuncTestCaseParser_IntegerLiteral |FuncTestCaseParser_DecimalLiteral |
			FuncTestCaseParser_FloatLiteral |FuncTestCaseParser_BooleanLiteral |FuncTestCaseParser_TimestampTzLiteral |
			FuncTestCaseParser_TimestampLiteral |FuncTestCaseParser_TimeLiteral |
			FuncTestCaseParser_DateLiteral |FuncTestCaseParser_IntervalYearLiteral |
			FuncTestCaseParser_IntervalDayLiteral |FuncTestCaseParser_IntervalCompoundLiteral |
			FuncTestCaseParser_NullLiteral |FuncTestCaseParser_StringLiteral 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule literal*/
					recog.base.set_state(540);
					recog.literal()?;

					}
				}

			FuncTestCaseParser_OBracket 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule literalList*/
					recog.base.set_state(541);
					recog.literalList()?;

					}
				}

			FuncTestCaseParser_OParen 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule literalStruct*/
					recog.base.set_state(542);
					recog.literalStruct()?;

					}
				}

			FuncTestCaseParser_OBrace 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					/*InvokeRule literalMap*/
					recog.base.set_state(543);
					recog.literalMap()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- literalLambda ----------------
pub type LiteralLambdaContextAll<'input> = LiteralLambdaContext<'input>;


pub type LiteralLambdaContext<'input> = BaseParserRuleContext<'input,LiteralLambdaContextExt<'input>>;

#[derive(Clone)]
pub struct LiteralLambdaContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for LiteralLambdaContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for LiteralLambdaContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_literalLambda(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_literalLambda(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for LiteralLambdaContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_literalLambda }
	//fn type_rule_index() -> usize where Self: Sized { RULE_literalLambda }
}
antlr4rust::tid!{LiteralLambdaContextExt<'a>}

impl<'input> LiteralLambdaContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<LiteralLambdaContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LiteralLambdaContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait LiteralLambdaContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<LiteralLambdaContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token OParen
/// Returns `None` if there is no child corresponding to token OParen
fn OParen(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_OParen, 0)
}
fn lambdaParameters(&self) -> Option<Rc<LambdaParametersContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Arrow
/// Returns `None` if there is no child corresponding to token Arrow
fn Arrow(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Arrow, 0)
}
fn lambdaBody(&self) -> Option<Rc<LambdaBodyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token CParen
/// Returns `None` if there is no child corresponding to token CParen
fn CParen(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_CParen, 0)
}

}

impl<'input> LiteralLambdaContextAttrs<'input> for LiteralLambdaContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn literalLambda(&mut self,)
	-> Result<Rc<LiteralLambdaContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LiteralLambdaContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 104, RULE_literalLambda);
        let mut _localctx: Rc<LiteralLambdaContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(546);
			recog.base.match_token(FuncTestCaseParser_OParen,&mut recog.err_handler)?;

			/*InvokeRule lambdaParameters*/
			recog.base.set_state(547);
			recog.lambdaParameters()?;

			recog.base.set_state(548);
			recog.base.match_token(FuncTestCaseParser_Arrow,&mut recog.err_handler)?;

			/*InvokeRule lambdaBody*/
			recog.base.set_state(549);
			recog.lambdaBody()?;

			recog.base.set_state(550);
			recog.base.match_token(FuncTestCaseParser_CParen,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- lambdaParameters ----------------
#[derive(Debug)]
pub enum LambdaParametersContextAll<'input>{
	TupleParamsContext(TupleParamsContext<'input>),
	SingleParamContext(SingleParamContext<'input>),
Error(LambdaParametersContext<'input>)
}
antlr4rust::tid!{LambdaParametersContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for LambdaParametersContextAll<'input>{}

impl<'input> FuncTestCaseParserContext<'input> for LambdaParametersContextAll<'input>{}

impl<'input> Deref for LambdaParametersContextAll<'input>{
	type Target = dyn LambdaParametersContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use LambdaParametersContextAll::*;
		match self{
			TupleParamsContext(inner) => inner,
			SingleParamContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for LambdaParametersContextAll<'input>{
    fn enter(&self, listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
}



pub type LambdaParametersContext<'input> = BaseParserRuleContext<'input,LambdaParametersContextExt<'input>>;

#[derive(Clone)]
pub struct LambdaParametersContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for LambdaParametersContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for LambdaParametersContext<'input>{
}

impl<'input> CustomRuleContext<'input> for LambdaParametersContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_lambdaParameters }
	//fn type_rule_index() -> usize where Self: Sized { RULE_lambdaParameters }
}
antlr4rust::tid!{LambdaParametersContextExt<'a>}

impl<'input> LambdaParametersContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<LambdaParametersContextAll<'input>> {
		Rc::new(
		LambdaParametersContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LambdaParametersContextExt{

				ph:PhantomData
			}),
		)
		)
	}
}

pub trait LambdaParametersContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<LambdaParametersContextExt<'input>>{


}

impl<'input> LambdaParametersContextAttrs<'input> for LambdaParametersContext<'input>{}

pub type TupleParamsContext<'input> = BaseParserRuleContext<'input,TupleParamsContextExt<'input>>;

pub trait TupleParamsContextAttrs<'input>: FuncTestCaseParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token OParen
	/// Returns `None` if there is no child corresponding to token OParen
	fn OParen(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
		self.get_token(FuncTestCaseParser_OParen, 0)
	}
	/// Retrieves all `TerminalNode`s corresponding to token Identifier in current rule
	fn Identifier_all(&self) -> Vec<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token Identifier, starting from 0.
	/// Returns `None` if number of children corresponding to token Identifier is less or equal than `i`.
	fn Identifier(&self, i: usize) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
		self.get_token(FuncTestCaseParser_Identifier, i)
	}
	/// Retrieves first TerminalNode corresponding to token CParen
	/// Returns `None` if there is no child corresponding to token CParen
	fn CParen(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
		self.get_token(FuncTestCaseParser_CParen, 0)
	}
	/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
	fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
	/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
	fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
		self.get_token(FuncTestCaseParser_Comma, i)
	}
}

impl<'input> TupleParamsContextAttrs<'input> for TupleParamsContext<'input>{}

pub struct TupleParamsContextExt<'input>{
	base:LambdaParametersContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{TupleParamsContextExt<'a>}

impl<'input> FuncTestCaseParserContext<'input> for TupleParamsContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for TupleParamsContext<'input>{
	fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_tupleParams(self);
		Ok(())
	}
}

impl<'input> CustomRuleContext<'input> for TupleParamsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_lambdaParameters }
	//fn type_rule_index() -> usize where Self: Sized { RULE_lambdaParameters }
}

impl<'input> Borrow<LambdaParametersContextExt<'input>> for TupleParamsContext<'input>{
	fn borrow(&self) -> &LambdaParametersContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<LambdaParametersContextExt<'input>> for TupleParamsContext<'input>{
	fn borrow_mut(&mut self) -> &mut LambdaParametersContextExt<'input> { &mut self.base }
}

impl<'input> LambdaParametersContextAttrs<'input> for TupleParamsContext<'input> {}

impl<'input> TupleParamsContextExt<'input>{
	fn new(ctx: &dyn LambdaParametersContextAttrs<'input>) -> Rc<LambdaParametersContextAll<'input>>  {
		Rc::new(
			LambdaParametersContextAll::TupleParamsContext(
				BaseParserRuleContext::copy_from(ctx,TupleParamsContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type SingleParamContext<'input> = BaseParserRuleContext<'input,SingleParamContextExt<'input>>;

pub trait SingleParamContextAttrs<'input>: FuncTestCaseParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token Identifier
	/// Returns `None` if there is no child corresponding to token Identifier
	fn Identifier(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
		self.get_token(FuncTestCaseParser_Identifier, 0)
	}
}

impl<'input> SingleParamContextAttrs<'input> for SingleParamContext<'input>{}

pub struct SingleParamContextExt<'input>{
	base:LambdaParametersContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{SingleParamContextExt<'a>}

impl<'input> FuncTestCaseParserContext<'input> for SingleParamContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for SingleParamContext<'input>{
	fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_singleParam(self);
		Ok(())
	}
}

impl<'input> CustomRuleContext<'input> for SingleParamContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_lambdaParameters }
	//fn type_rule_index() -> usize where Self: Sized { RULE_lambdaParameters }
}

impl<'input> Borrow<LambdaParametersContextExt<'input>> for SingleParamContext<'input>{
	fn borrow(&self) -> &LambdaParametersContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<LambdaParametersContextExt<'input>> for SingleParamContext<'input>{
	fn borrow_mut(&mut self) -> &mut LambdaParametersContextExt<'input> { &mut self.base }
}

impl<'input> LambdaParametersContextAttrs<'input> for SingleParamContext<'input> {}

impl<'input> SingleParamContextExt<'input>{
	fn new(ctx: &dyn LambdaParametersContextAttrs<'input>) -> Rc<LambdaParametersContextAll<'input>>  {
		Rc::new(
			LambdaParametersContextAll::SingleParamContext(
				BaseParserRuleContext::copy_from(ctx,SingleParamContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn lambdaParameters(&mut self,)
	-> Result<Rc<LambdaParametersContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LambdaParametersContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 106, RULE_lambdaParameters);
        let mut _localctx: Rc<LambdaParametersContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(562);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			FuncTestCaseParser_Identifier 
				=> {
					let tmp = SingleParamContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
					_localctx = tmp;
					{
					recog.base.set_state(552);
					recog.base.match_token(FuncTestCaseParser_Identifier,&mut recog.err_handler)?;

					}
				}

			FuncTestCaseParser_OParen 
				=> {
					let tmp = TupleParamsContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2)?;
					_localctx = tmp;
					{
					recog.base.set_state(553);
					recog.base.match_token(FuncTestCaseParser_OParen,&mut recog.err_handler)?;

					recog.base.set_state(554);
					recog.base.match_token(FuncTestCaseParser_Identifier,&mut recog.err_handler)?;

					recog.base.set_state(557); 
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					loop {
						{
						{
						recog.base.set_state(555);
						recog.base.match_token(FuncTestCaseParser_Comma,&mut recog.err_handler)?;

						recog.base.set_state(556);
						recog.base.match_token(FuncTestCaseParser_Identifier,&mut recog.err_handler)?;

						}
						}
						recog.base.set_state(559); 
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						if !(_la==FuncTestCaseParser_Comma) {break}
					}
					recog.base.set_state(561);
					recog.base.match_token(FuncTestCaseParser_CParen,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- lambdaBody ----------------
pub type LambdaBodyContextAll<'input> = LambdaBodyContext<'input>;


pub type LambdaBodyContext<'input> = BaseParserRuleContext<'input,LambdaBodyContextExt<'input>>;

#[derive(Clone)]
pub struct LambdaBodyContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for LambdaBodyContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for LambdaBodyContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_lambdaBody(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_lambdaBody(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for LambdaBodyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_lambdaBody }
	//fn type_rule_index() -> usize where Self: Sized { RULE_lambdaBody }
}
antlr4rust::tid!{LambdaBodyContextExt<'a>}

impl<'input> LambdaBodyContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<LambdaBodyContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LambdaBodyContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait LambdaBodyContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<LambdaBodyContextExt<'input>>{

fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token OParen
/// Returns `None` if there is no child corresponding to token OParen
fn OParen(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_OParen, 0)
}
fn arguments(&self) -> Option<Rc<ArgumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token CParen
/// Returns `None` if there is no child corresponding to token CParen
fn CParen(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_CParen, 0)
}

}

impl<'input> LambdaBodyContextAttrs<'input> for LambdaBodyContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn lambdaBody(&mut self,)
	-> Result<Rc<LambdaBodyContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LambdaBodyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 108, RULE_lambdaBody);
        let mut _localctx: Rc<LambdaBodyContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule identifier*/
			recog.base.set_state(564);
			recog.identifier()?;

			recog.base.set_state(565);
			recog.base.match_token(FuncTestCaseParser_OParen,&mut recog.err_handler)?;

			/*InvokeRule arguments*/
			recog.base.set_state(566);
			recog.arguments()?;

			recog.base.set_state(567);
			recog.base.match_token(FuncTestCaseParser_CParen,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- dataType ----------------
pub type DataTypeContextAll<'input> = DataTypeContext<'input>;


pub type DataTypeContext<'input> = BaseParserRuleContext<'input,DataTypeContextExt<'input>>;

#[derive(Clone)]
pub struct DataTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for DataTypeContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for DataTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_dataType(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_dataType(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for DataTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_dataType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_dataType }
}
antlr4rust::tid!{DataTypeContextExt<'a>}

impl<'input> DataTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<DataTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DataTypeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait DataTypeContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<DataTypeContextExt<'input>>{

fn scalarType(&self) -> Option<Rc<ScalarTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn parameterizedType(&self) -> Option<Rc<ParameterizedTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> DataTypeContextAttrs<'input> for DataTypeContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn dataType(&mut self,)
	-> Result<Rc<DataTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DataTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 110, RULE_dataType);
        let mut _localctx: Rc<DataTypeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(571);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			FuncTestCaseParser_Boolean |FuncTestCaseParser_I8 |FuncTestCaseParser_I16 |
			FuncTestCaseParser_I32 |FuncTestCaseParser_I64 |FuncTestCaseParser_FP32 |
			FuncTestCaseParser_FP64 |FuncTestCaseParser_String |FuncTestCaseParser_Binary |
			FuncTestCaseParser_Date |FuncTestCaseParser_Interval_Year |FuncTestCaseParser_UUID |
			FuncTestCaseParser_UserDefined |FuncTestCaseParser_Bool |FuncTestCaseParser_Str |
			FuncTestCaseParser_VBin |FuncTestCaseParser_IYear 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule scalarType*/
					recog.base.set_state(569);
					recog.scalarType()?;

					}
				}

			FuncTestCaseParser_Func |FuncTestCaseParser_Interval_Day |FuncTestCaseParser_Interval_Compound |
			FuncTestCaseParser_Decimal |FuncTestCaseParser_Precision_Time |FuncTestCaseParser_Precision_Timestamp |
			FuncTestCaseParser_Precision_Timestamp_TZ |FuncTestCaseParser_FixedChar |
			FuncTestCaseParser_VarChar |FuncTestCaseParser_FixedBinary |FuncTestCaseParser_Struct |
			FuncTestCaseParser_List |FuncTestCaseParser_Map |FuncTestCaseParser_IDay |
			FuncTestCaseParser_ICompound |FuncTestCaseParser_Dec |FuncTestCaseParser_PT |
			FuncTestCaseParser_PTs |FuncTestCaseParser_PTsTZ |FuncTestCaseParser_FChar |
			FuncTestCaseParser_VChar |FuncTestCaseParser_FBin 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule parameterizedType*/
					recog.base.set_state(570);
					recog.parameterizedType()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- scalarType ----------------
#[derive(Debug)]
pub enum ScalarTypeContextAll<'input>{
	DateContext(DateContext<'input>),
	BooleanContext(BooleanContext<'input>),
	StringContext(StringContext<'input>),
	BinaryContext(BinaryContext<'input>),
	UserDefinedContext(UserDefinedContext<'input>),
	FloatContext(FloatContext<'input>),
	IntervalYearContext(IntervalYearContext<'input>),
	UuidContext(UuidContext<'input>),
	IntContext(IntContext<'input>),
Error(ScalarTypeContext<'input>)
}
antlr4rust::tid!{ScalarTypeContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for ScalarTypeContextAll<'input>{}

impl<'input> FuncTestCaseParserContext<'input> for ScalarTypeContextAll<'input>{}

impl<'input> Deref for ScalarTypeContextAll<'input>{
	type Target = dyn ScalarTypeContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use ScalarTypeContextAll::*;
		match self{
			DateContext(inner) => inner,
			BooleanContext(inner) => inner,
			StringContext(inner) => inner,
			BinaryContext(inner) => inner,
			UserDefinedContext(inner) => inner,
			FloatContext(inner) => inner,
			IntervalYearContext(inner) => inner,
			UuidContext(inner) => inner,
			IntContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for ScalarTypeContextAll<'input>{
    fn enter(&self, listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
}



pub type ScalarTypeContext<'input> = BaseParserRuleContext<'input,ScalarTypeContextExt<'input>>;

#[derive(Clone)]
pub struct ScalarTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for ScalarTypeContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for ScalarTypeContext<'input>{
}

impl<'input> CustomRuleContext<'input> for ScalarTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_scalarType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_scalarType }
}
antlr4rust::tid!{ScalarTypeContextExt<'a>}

impl<'input> ScalarTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ScalarTypeContextAll<'input>> {
		Rc::new(
		ScalarTypeContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ScalarTypeContextExt{

				ph:PhantomData
			}),
		)
		)
	}
}

pub trait ScalarTypeContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<ScalarTypeContextExt<'input>>{


}

impl<'input> ScalarTypeContextAttrs<'input> for ScalarTypeContext<'input>{}

pub type DateContext<'input> = BaseParserRuleContext<'input,DateContextExt<'input>>;

pub trait DateContextAttrs<'input>: FuncTestCaseParserContext<'input>{
	fn dateType(&self) -> Option<Rc<DateTypeContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> DateContextAttrs<'input> for DateContext<'input>{}

pub struct DateContextExt<'input>{
	base:ScalarTypeContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{DateContextExt<'a>}

impl<'input> FuncTestCaseParserContext<'input> for DateContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for DateContext<'input>{
	fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_date(self);
		Ok(())
	}
}

impl<'input> CustomRuleContext<'input> for DateContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_scalarType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_scalarType }
}

impl<'input> Borrow<ScalarTypeContextExt<'input>> for DateContext<'input>{
	fn borrow(&self) -> &ScalarTypeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ScalarTypeContextExt<'input>> for DateContext<'input>{
	fn borrow_mut(&mut self) -> &mut ScalarTypeContextExt<'input> { &mut self.base }
}

impl<'input> ScalarTypeContextAttrs<'input> for DateContext<'input> {}

impl<'input> DateContextExt<'input>{
	fn new(ctx: &dyn ScalarTypeContextAttrs<'input>) -> Rc<ScalarTypeContextAll<'input>>  {
		Rc::new(
			ScalarTypeContextAll::DateContext(
				BaseParserRuleContext::copy_from(ctx,DateContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type BooleanContext<'input> = BaseParserRuleContext<'input,BooleanContextExt<'input>>;

pub trait BooleanContextAttrs<'input>: FuncTestCaseParserContext<'input>{
	fn booleanType(&self) -> Option<Rc<BooleanTypeContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> BooleanContextAttrs<'input> for BooleanContext<'input>{}

pub struct BooleanContextExt<'input>{
	base:ScalarTypeContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{BooleanContextExt<'a>}

impl<'input> FuncTestCaseParserContext<'input> for BooleanContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for BooleanContext<'input>{
	fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_boolean(self);
		Ok(())
	}
}

impl<'input> CustomRuleContext<'input> for BooleanContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_scalarType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_scalarType }
}

impl<'input> Borrow<ScalarTypeContextExt<'input>> for BooleanContext<'input>{
	fn borrow(&self) -> &ScalarTypeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ScalarTypeContextExt<'input>> for BooleanContext<'input>{
	fn borrow_mut(&mut self) -> &mut ScalarTypeContextExt<'input> { &mut self.base }
}

impl<'input> ScalarTypeContextAttrs<'input> for BooleanContext<'input> {}

impl<'input> BooleanContextExt<'input>{
	fn new(ctx: &dyn ScalarTypeContextAttrs<'input>) -> Rc<ScalarTypeContextAll<'input>>  {
		Rc::new(
			ScalarTypeContextAll::BooleanContext(
				BaseParserRuleContext::copy_from(ctx,BooleanContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type StringContext<'input> = BaseParserRuleContext<'input,StringContextExt<'input>>;

pub trait StringContextAttrs<'input>: FuncTestCaseParserContext<'input>{
	fn stringType(&self) -> Option<Rc<StringTypeContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> StringContextAttrs<'input> for StringContext<'input>{}

pub struct StringContextExt<'input>{
	base:ScalarTypeContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{StringContextExt<'a>}

impl<'input> FuncTestCaseParserContext<'input> for StringContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for StringContext<'input>{
	fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_string(self);
		Ok(())
	}
}

impl<'input> CustomRuleContext<'input> for StringContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_scalarType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_scalarType }
}

impl<'input> Borrow<ScalarTypeContextExt<'input>> for StringContext<'input>{
	fn borrow(&self) -> &ScalarTypeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ScalarTypeContextExt<'input>> for StringContext<'input>{
	fn borrow_mut(&mut self) -> &mut ScalarTypeContextExt<'input> { &mut self.base }
}

impl<'input> ScalarTypeContextAttrs<'input> for StringContext<'input> {}

impl<'input> StringContextExt<'input>{
	fn new(ctx: &dyn ScalarTypeContextAttrs<'input>) -> Rc<ScalarTypeContextAll<'input>>  {
		Rc::new(
			ScalarTypeContextAll::StringContext(
				BaseParserRuleContext::copy_from(ctx,StringContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type BinaryContext<'input> = BaseParserRuleContext<'input,BinaryContextExt<'input>>;

pub trait BinaryContextAttrs<'input>: FuncTestCaseParserContext<'input>{
	fn binaryType(&self) -> Option<Rc<BinaryTypeContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> BinaryContextAttrs<'input> for BinaryContext<'input>{}

pub struct BinaryContextExt<'input>{
	base:ScalarTypeContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{BinaryContextExt<'a>}

impl<'input> FuncTestCaseParserContext<'input> for BinaryContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for BinaryContext<'input>{
	fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_binary(self);
		Ok(())
	}
}

impl<'input> CustomRuleContext<'input> for BinaryContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_scalarType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_scalarType }
}

impl<'input> Borrow<ScalarTypeContextExt<'input>> for BinaryContext<'input>{
	fn borrow(&self) -> &ScalarTypeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ScalarTypeContextExt<'input>> for BinaryContext<'input>{
	fn borrow_mut(&mut self) -> &mut ScalarTypeContextExt<'input> { &mut self.base }
}

impl<'input> ScalarTypeContextAttrs<'input> for BinaryContext<'input> {}

impl<'input> BinaryContextExt<'input>{
	fn new(ctx: &dyn ScalarTypeContextAttrs<'input>) -> Rc<ScalarTypeContextAll<'input>>  {
		Rc::new(
			ScalarTypeContextAll::BinaryContext(
				BaseParserRuleContext::copy_from(ctx,BinaryContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type UserDefinedContext<'input> = BaseParserRuleContext<'input,UserDefinedContextExt<'input>>;

pub trait UserDefinedContextAttrs<'input>: FuncTestCaseParserContext<'input>{
	fn userDefinedType(&self) -> Option<Rc<UserDefinedTypeContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> UserDefinedContextAttrs<'input> for UserDefinedContext<'input>{}

pub struct UserDefinedContextExt<'input>{
	base:ScalarTypeContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{UserDefinedContextExt<'a>}

impl<'input> FuncTestCaseParserContext<'input> for UserDefinedContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for UserDefinedContext<'input>{
	fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_userDefined(self);
		Ok(())
	}
}

impl<'input> CustomRuleContext<'input> for UserDefinedContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_scalarType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_scalarType }
}

impl<'input> Borrow<ScalarTypeContextExt<'input>> for UserDefinedContext<'input>{
	fn borrow(&self) -> &ScalarTypeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ScalarTypeContextExt<'input>> for UserDefinedContext<'input>{
	fn borrow_mut(&mut self) -> &mut ScalarTypeContextExt<'input> { &mut self.base }
}

impl<'input> ScalarTypeContextAttrs<'input> for UserDefinedContext<'input> {}

impl<'input> UserDefinedContextExt<'input>{
	fn new(ctx: &dyn ScalarTypeContextAttrs<'input>) -> Rc<ScalarTypeContextAll<'input>>  {
		Rc::new(
			ScalarTypeContextAll::UserDefinedContext(
				BaseParserRuleContext::copy_from(ctx,UserDefinedContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type FloatContext<'input> = BaseParserRuleContext<'input,FloatContextExt<'input>>;

pub trait FloatContextAttrs<'input>: FuncTestCaseParserContext<'input>{
	fn floatType(&self) -> Option<Rc<FloatTypeContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> FloatContextAttrs<'input> for FloatContext<'input>{}

pub struct FloatContextExt<'input>{
	base:ScalarTypeContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{FloatContextExt<'a>}

impl<'input> FuncTestCaseParserContext<'input> for FloatContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for FloatContext<'input>{
	fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_float(self);
		Ok(())
	}
}

impl<'input> CustomRuleContext<'input> for FloatContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_scalarType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_scalarType }
}

impl<'input> Borrow<ScalarTypeContextExt<'input>> for FloatContext<'input>{
	fn borrow(&self) -> &ScalarTypeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ScalarTypeContextExt<'input>> for FloatContext<'input>{
	fn borrow_mut(&mut self) -> &mut ScalarTypeContextExt<'input> { &mut self.base }
}

impl<'input> ScalarTypeContextAttrs<'input> for FloatContext<'input> {}

impl<'input> FloatContextExt<'input>{
	fn new(ctx: &dyn ScalarTypeContextAttrs<'input>) -> Rc<ScalarTypeContextAll<'input>>  {
		Rc::new(
			ScalarTypeContextAll::FloatContext(
				BaseParserRuleContext::copy_from(ctx,FloatContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type IntervalYearContext<'input> = BaseParserRuleContext<'input,IntervalYearContextExt<'input>>;

pub trait IntervalYearContextAttrs<'input>: FuncTestCaseParserContext<'input>{
	fn intervalYearType(&self) -> Option<Rc<IntervalYearTypeContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> IntervalYearContextAttrs<'input> for IntervalYearContext<'input>{}

pub struct IntervalYearContextExt<'input>{
	base:ScalarTypeContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{IntervalYearContextExt<'a>}

impl<'input> FuncTestCaseParserContext<'input> for IntervalYearContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for IntervalYearContext<'input>{
	fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_intervalYear(self);
		Ok(())
	}
}

impl<'input> CustomRuleContext<'input> for IntervalYearContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_scalarType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_scalarType }
}

impl<'input> Borrow<ScalarTypeContextExt<'input>> for IntervalYearContext<'input>{
	fn borrow(&self) -> &ScalarTypeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ScalarTypeContextExt<'input>> for IntervalYearContext<'input>{
	fn borrow_mut(&mut self) -> &mut ScalarTypeContextExt<'input> { &mut self.base }
}

impl<'input> ScalarTypeContextAttrs<'input> for IntervalYearContext<'input> {}

impl<'input> IntervalYearContextExt<'input>{
	fn new(ctx: &dyn ScalarTypeContextAttrs<'input>) -> Rc<ScalarTypeContextAll<'input>>  {
		Rc::new(
			ScalarTypeContextAll::IntervalYearContext(
				BaseParserRuleContext::copy_from(ctx,IntervalYearContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type UuidContext<'input> = BaseParserRuleContext<'input,UuidContextExt<'input>>;

pub trait UuidContextAttrs<'input>: FuncTestCaseParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token UUID
	/// Returns `None` if there is no child corresponding to token UUID
	fn UUID(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
		self.get_token(FuncTestCaseParser_UUID, 0)
	}
	/// Retrieves first TerminalNode corresponding to token QMark
	/// Returns `None` if there is no child corresponding to token QMark
	fn QMark(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
		self.get_token(FuncTestCaseParser_QMark, 0)
	}
}

impl<'input> UuidContextAttrs<'input> for UuidContext<'input>{}

pub struct UuidContextExt<'input>{
	base:ScalarTypeContextExt<'input>,
	pub isnull: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{UuidContextExt<'a>}

impl<'input> FuncTestCaseParserContext<'input> for UuidContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for UuidContext<'input>{
	fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_uuid(self);
		Ok(())
	}
}

impl<'input> CustomRuleContext<'input> for UuidContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_scalarType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_scalarType }
}

impl<'input> Borrow<ScalarTypeContextExt<'input>> for UuidContext<'input>{
	fn borrow(&self) -> &ScalarTypeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ScalarTypeContextExt<'input>> for UuidContext<'input>{
	fn borrow_mut(&mut self) -> &mut ScalarTypeContextExt<'input> { &mut self.base }
}

impl<'input> ScalarTypeContextAttrs<'input> for UuidContext<'input> {}

impl<'input> UuidContextExt<'input>{
	fn new(ctx: &dyn ScalarTypeContextAttrs<'input>) -> Rc<ScalarTypeContextAll<'input>>  {
		Rc::new(
			ScalarTypeContextAll::UuidContext(
				BaseParserRuleContext::copy_from(ctx,UuidContextExt{
					isnull:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type IntContext<'input> = BaseParserRuleContext<'input,IntContextExt<'input>>;

pub trait IntContextAttrs<'input>: FuncTestCaseParserContext<'input>{
	fn intType(&self) -> Option<Rc<IntTypeContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> IntContextAttrs<'input> for IntContext<'input>{}

pub struct IntContextExt<'input>{
	base:ScalarTypeContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{IntContextExt<'a>}

impl<'input> FuncTestCaseParserContext<'input> for IntContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for IntContext<'input>{
	fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_int(self);
		Ok(())
	}
}

impl<'input> CustomRuleContext<'input> for IntContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_scalarType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_scalarType }
}

impl<'input> Borrow<ScalarTypeContextExt<'input>> for IntContext<'input>{
	fn borrow(&self) -> &ScalarTypeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ScalarTypeContextExt<'input>> for IntContext<'input>{
	fn borrow_mut(&mut self) -> &mut ScalarTypeContextExt<'input> { &mut self.base }
}

impl<'input> ScalarTypeContextAttrs<'input> for IntContext<'input> {}

impl<'input> IntContextExt<'input>{
	fn new(ctx: &dyn ScalarTypeContextAttrs<'input>) -> Rc<ScalarTypeContextAll<'input>>  {
		Rc::new(
			ScalarTypeContextAll::IntContext(
				BaseParserRuleContext::copy_from(ctx,IntContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn scalarType(&mut self,)
	-> Result<Rc<ScalarTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ScalarTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 112, RULE_scalarType);
        let mut _localctx: Rc<ScalarTypeContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(585);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			FuncTestCaseParser_Boolean |FuncTestCaseParser_Bool 
				=> {
					let tmp = BooleanContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
					_localctx = tmp;
					{
					/*InvokeRule booleanType*/
					recog.base.set_state(573);
					recog.booleanType()?;

					}
				}

			FuncTestCaseParser_I8 |FuncTestCaseParser_I16 |FuncTestCaseParser_I32 |
			FuncTestCaseParser_I64 
				=> {
					let tmp = IntContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2)?;
					_localctx = tmp;
					{
					/*InvokeRule intType*/
					recog.base.set_state(574);
					recog.intType()?;

					}
				}

			FuncTestCaseParser_FP32 |FuncTestCaseParser_FP64 
				=> {
					let tmp = FloatContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3)?;
					_localctx = tmp;
					{
					/*InvokeRule floatType*/
					recog.base.set_state(575);
					recog.floatType()?;

					}
				}

			FuncTestCaseParser_String |FuncTestCaseParser_Str 
				=> {
					let tmp = StringContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 4)?;
					_localctx = tmp;
					{
					/*InvokeRule stringType*/
					recog.base.set_state(576);
					recog.stringType()?;

					}
				}

			FuncTestCaseParser_Binary |FuncTestCaseParser_VBin 
				=> {
					let tmp = BinaryContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 5)?;
					_localctx = tmp;
					{
					/*InvokeRule binaryType*/
					recog.base.set_state(577);
					recog.binaryType()?;

					}
				}

			FuncTestCaseParser_Date 
				=> {
					let tmp = DateContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 6)?;
					_localctx = tmp;
					{
					/*InvokeRule dateType*/
					recog.base.set_state(578);
					recog.dateType()?;

					}
				}

			FuncTestCaseParser_Interval_Year |FuncTestCaseParser_IYear 
				=> {
					let tmp = IntervalYearContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 7)?;
					_localctx = tmp;
					{
					/*InvokeRule intervalYearType*/
					recog.base.set_state(579);
					recog.intervalYearType()?;

					}
				}

			FuncTestCaseParser_UUID 
				=> {
					let tmp = UuidContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 8)?;
					_localctx = tmp;
					{
					recog.base.set_state(580);
					recog.base.match_token(FuncTestCaseParser_UUID,&mut recog.err_handler)?;

					recog.base.set_state(582);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==FuncTestCaseParser_QMark {
						{
						recog.base.set_state(581);
						let tmp = recog.base.match_token(FuncTestCaseParser_QMark,&mut recog.err_handler)?;
						if let ScalarTypeContextAll::UuidContext(ctx) = cast_mut::<_,ScalarTypeContextAll >(&mut _localctx){
						ctx.isnull = Some(tmp.clone()); } else {unreachable!("cant cast");}  

						}
					}

					}
				}

			FuncTestCaseParser_UserDefined 
				=> {
					let tmp = UserDefinedContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 9)?;
					_localctx = tmp;
					{
					/*InvokeRule userDefinedType*/
					recog.base.set_state(584);
					recog.userDefinedType()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- userDefinedType ----------------
pub type UserDefinedTypeContextAll<'input> = UserDefinedTypeContext<'input>;


pub type UserDefinedTypeContext<'input> = BaseParserRuleContext<'input,UserDefinedTypeContextExt<'input>>;

#[derive(Clone)]
pub struct UserDefinedTypeContextExt<'input>{
	pub isnull: Option<TokenType<'input>>,
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for UserDefinedTypeContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for UserDefinedTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_userDefinedType(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_userDefinedType(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for UserDefinedTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_userDefinedType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_userDefinedType }
}
antlr4rust::tid!{UserDefinedTypeContextExt<'a>}

impl<'input> UserDefinedTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<UserDefinedTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,UserDefinedTypeContextExt{
				isnull: None, 

				ph:PhantomData
			}),
		)
	}
}

pub trait UserDefinedTypeContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<UserDefinedTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token UserDefined
/// Returns `None` if there is no child corresponding to token UserDefined
fn UserDefined(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_UserDefined, 0)
}
/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Identifier, 0)
}
/// Retrieves first TerminalNode corresponding to token QMark
/// Returns `None` if there is no child corresponding to token QMark
fn QMark(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_QMark, 0)
}

}

impl<'input> UserDefinedTypeContextAttrs<'input> for UserDefinedTypeContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn userDefinedType(&mut self,)
	-> Result<Rc<UserDefinedTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = UserDefinedTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 114, RULE_userDefinedType);
        let mut _localctx: Rc<UserDefinedTypeContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(587);
			recog.base.match_token(FuncTestCaseParser_UserDefined,&mut recog.err_handler)?;

			recog.base.set_state(588);
			recog.base.match_token(FuncTestCaseParser_Identifier,&mut recog.err_handler)?;

			recog.base.set_state(590);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==FuncTestCaseParser_QMark {
				{
				recog.base.set_state(589);
				let tmp = recog.base.match_token(FuncTestCaseParser_QMark,&mut recog.err_handler)?;
				 cast_mut::<_,UserDefinedTypeContext >(&mut _localctx).isnull = Some(tmp.clone());
				  

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- booleanType ----------------
pub type BooleanTypeContextAll<'input> = BooleanTypeContext<'input>;


pub type BooleanTypeContext<'input> = BaseParserRuleContext<'input,BooleanTypeContextExt<'input>>;

#[derive(Clone)]
pub struct BooleanTypeContextExt<'input>{
	pub isnull: Option<TokenType<'input>>,
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for BooleanTypeContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for BooleanTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_booleanType(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_booleanType(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for BooleanTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_booleanType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_booleanType }
}
antlr4rust::tid!{BooleanTypeContextExt<'a>}

impl<'input> BooleanTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<BooleanTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BooleanTypeContextExt{
				isnull: None, 

				ph:PhantomData
			}),
		)
	}
}

pub trait BooleanTypeContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<BooleanTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Bool
/// Returns `None` if there is no child corresponding to token Bool
fn Bool(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Bool, 0)
}
/// Retrieves first TerminalNode corresponding to token Boolean
/// Returns `None` if there is no child corresponding to token Boolean
fn Boolean(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Boolean, 0)
}
/// Retrieves first TerminalNode corresponding to token QMark
/// Returns `None` if there is no child corresponding to token QMark
fn QMark(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_QMark, 0)
}

}

impl<'input> BooleanTypeContextAttrs<'input> for BooleanTypeContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn booleanType(&mut self,)
	-> Result<Rc<BooleanTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BooleanTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 116, RULE_booleanType);
        let mut _localctx: Rc<BooleanTypeContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(592);
			_la = recog.base.input.la(1);
			if { !(_la==FuncTestCaseParser_Boolean || _la==FuncTestCaseParser_Bool) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			recog.base.set_state(594);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==FuncTestCaseParser_QMark {
				{
				recog.base.set_state(593);
				let tmp = recog.base.match_token(FuncTestCaseParser_QMark,&mut recog.err_handler)?;
				 cast_mut::<_,BooleanTypeContext >(&mut _localctx).isnull = Some(tmp.clone());
				  

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- stringType ----------------
pub type StringTypeContextAll<'input> = StringTypeContext<'input>;


pub type StringTypeContext<'input> = BaseParserRuleContext<'input,StringTypeContextExt<'input>>;

#[derive(Clone)]
pub struct StringTypeContextExt<'input>{
	pub isnull: Option<TokenType<'input>>,
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for StringTypeContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for StringTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_stringType(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_stringType(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for StringTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stringType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stringType }
}
antlr4rust::tid!{StringTypeContextExt<'a>}

impl<'input> StringTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<StringTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StringTypeContextExt{
				isnull: None, 

				ph:PhantomData
			}),
		)
	}
}

pub trait StringTypeContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<StringTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Str
/// Returns `None` if there is no child corresponding to token Str
fn Str(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Str, 0)
}
/// Retrieves first TerminalNode corresponding to token String
/// Returns `None` if there is no child corresponding to token String
fn String(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_String, 0)
}
/// Retrieves first TerminalNode corresponding to token QMark
/// Returns `None` if there is no child corresponding to token QMark
fn QMark(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_QMark, 0)
}

}

impl<'input> StringTypeContextAttrs<'input> for StringTypeContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn stringType(&mut self,)
	-> Result<Rc<StringTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StringTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 118, RULE_stringType);
        let mut _localctx: Rc<StringTypeContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(596);
			_la = recog.base.input.la(1);
			if { !(_la==FuncTestCaseParser_String || _la==FuncTestCaseParser_Str) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			recog.base.set_state(598);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==FuncTestCaseParser_QMark {
				{
				recog.base.set_state(597);
				let tmp = recog.base.match_token(FuncTestCaseParser_QMark,&mut recog.err_handler)?;
				 cast_mut::<_,StringTypeContext >(&mut _localctx).isnull = Some(tmp.clone());
				  

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- binaryType ----------------
pub type BinaryTypeContextAll<'input> = BinaryTypeContext<'input>;


pub type BinaryTypeContext<'input> = BaseParserRuleContext<'input,BinaryTypeContextExt<'input>>;

#[derive(Clone)]
pub struct BinaryTypeContextExt<'input>{
	pub isnull: Option<TokenType<'input>>,
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for BinaryTypeContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for BinaryTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_binaryType(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_binaryType(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for BinaryTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_binaryType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_binaryType }
}
antlr4rust::tid!{BinaryTypeContextExt<'a>}

impl<'input> BinaryTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<BinaryTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BinaryTypeContextExt{
				isnull: None, 

				ph:PhantomData
			}),
		)
	}
}

pub trait BinaryTypeContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<BinaryTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Binary
/// Returns `None` if there is no child corresponding to token Binary
fn Binary(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Binary, 0)
}
/// Retrieves first TerminalNode corresponding to token VBin
/// Returns `None` if there is no child corresponding to token VBin
fn VBin(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_VBin, 0)
}
/// Retrieves first TerminalNode corresponding to token QMark
/// Returns `None` if there is no child corresponding to token QMark
fn QMark(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_QMark, 0)
}

}

impl<'input> BinaryTypeContextAttrs<'input> for BinaryTypeContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn binaryType(&mut self,)
	-> Result<Rc<BinaryTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BinaryTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 120, RULE_binaryType);
        let mut _localctx: Rc<BinaryTypeContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(600);
			_la = recog.base.input.la(1);
			if { !(_la==FuncTestCaseParser_Binary || _la==FuncTestCaseParser_VBin) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			recog.base.set_state(602);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==FuncTestCaseParser_QMark {
				{
				recog.base.set_state(601);
				let tmp = recog.base.match_token(FuncTestCaseParser_QMark,&mut recog.err_handler)?;
				 cast_mut::<_,BinaryTypeContext >(&mut _localctx).isnull = Some(tmp.clone());
				  

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- intType ----------------
pub type IntTypeContextAll<'input> = IntTypeContext<'input>;


pub type IntTypeContext<'input> = BaseParserRuleContext<'input,IntTypeContextExt<'input>>;

#[derive(Clone)]
pub struct IntTypeContextExt<'input>{
	pub isnull: Option<TokenType<'input>>,
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for IntTypeContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for IntTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_intType(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_intType(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for IntTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_intType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_intType }
}
antlr4rust::tid!{IntTypeContextExt<'a>}

impl<'input> IntTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<IntTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IntTypeContextExt{
				isnull: None, 

				ph:PhantomData
			}),
		)
	}
}

pub trait IntTypeContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<IntTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token I8
/// Returns `None` if there is no child corresponding to token I8
fn I8(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_I8, 0)
}
/// Retrieves first TerminalNode corresponding to token I16
/// Returns `None` if there is no child corresponding to token I16
fn I16(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_I16, 0)
}
/// Retrieves first TerminalNode corresponding to token I32
/// Returns `None` if there is no child corresponding to token I32
fn I32(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_I32, 0)
}
/// Retrieves first TerminalNode corresponding to token I64
/// Returns `None` if there is no child corresponding to token I64
fn I64(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_I64, 0)
}
/// Retrieves first TerminalNode corresponding to token QMark
/// Returns `None` if there is no child corresponding to token QMark
fn QMark(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_QMark, 0)
}

}

impl<'input> IntTypeContextAttrs<'input> for IntTypeContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn intType(&mut self,)
	-> Result<Rc<IntTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IntTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 122, RULE_intType);
        let mut _localctx: Rc<IntTypeContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(604);
			_la = recog.base.input.la(1);
			if { !(((((_la - 59)) & !0x3f) == 0 && ((1usize << (_la - 59)) & 15) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			recog.base.set_state(606);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==FuncTestCaseParser_QMark {
				{
				recog.base.set_state(605);
				let tmp = recog.base.match_token(FuncTestCaseParser_QMark,&mut recog.err_handler)?;
				 cast_mut::<_,IntTypeContext >(&mut _localctx).isnull = Some(tmp.clone());
				  

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- floatType ----------------
pub type FloatTypeContextAll<'input> = FloatTypeContext<'input>;


pub type FloatTypeContext<'input> = BaseParserRuleContext<'input,FloatTypeContextExt<'input>>;

#[derive(Clone)]
pub struct FloatTypeContextExt<'input>{
	pub isnull: Option<TokenType<'input>>,
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for FloatTypeContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for FloatTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_floatType(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_floatType(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for FloatTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_floatType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_floatType }
}
antlr4rust::tid!{FloatTypeContextExt<'a>}

impl<'input> FloatTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<FloatTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FloatTypeContextExt{
				isnull: None, 

				ph:PhantomData
			}),
		)
	}
}

pub trait FloatTypeContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<FloatTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token FP32
/// Returns `None` if there is no child corresponding to token FP32
fn FP32(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_FP32, 0)
}
/// Retrieves first TerminalNode corresponding to token FP64
/// Returns `None` if there is no child corresponding to token FP64
fn FP64(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_FP64, 0)
}
/// Retrieves first TerminalNode corresponding to token QMark
/// Returns `None` if there is no child corresponding to token QMark
fn QMark(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_QMark, 0)
}

}

impl<'input> FloatTypeContextAttrs<'input> for FloatTypeContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn floatType(&mut self,)
	-> Result<Rc<FloatTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FloatTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 124, RULE_floatType);
        let mut _localctx: Rc<FloatTypeContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(608);
			_la = recog.base.input.la(1);
			if { !(_la==FuncTestCaseParser_FP32 || _la==FuncTestCaseParser_FP64) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			recog.base.set_state(610);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==FuncTestCaseParser_QMark {
				{
				recog.base.set_state(609);
				let tmp = recog.base.match_token(FuncTestCaseParser_QMark,&mut recog.err_handler)?;
				 cast_mut::<_,FloatTypeContext >(&mut _localctx).isnull = Some(tmp.clone());
				  

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- dateType ----------------
pub type DateTypeContextAll<'input> = DateTypeContext<'input>;


pub type DateTypeContext<'input> = BaseParserRuleContext<'input,DateTypeContextExt<'input>>;

#[derive(Clone)]
pub struct DateTypeContextExt<'input>{
	pub isnull: Option<TokenType<'input>>,
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for DateTypeContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for DateTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_dateType(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_dateType(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for DateTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_dateType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_dateType }
}
antlr4rust::tid!{DateTypeContextExt<'a>}

impl<'input> DateTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<DateTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DateTypeContextExt{
				isnull: None, 

				ph:PhantomData
			}),
		)
	}
}

pub trait DateTypeContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<DateTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Date
/// Returns `None` if there is no child corresponding to token Date
fn Date(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Date, 0)
}
/// Retrieves first TerminalNode corresponding to token QMark
/// Returns `None` if there is no child corresponding to token QMark
fn QMark(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_QMark, 0)
}

}

impl<'input> DateTypeContextAttrs<'input> for DateTypeContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn dateType(&mut self,)
	-> Result<Rc<DateTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DateTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 126, RULE_dateType);
        let mut _localctx: Rc<DateTypeContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(612);
			recog.base.match_token(FuncTestCaseParser_Date,&mut recog.err_handler)?;

			recog.base.set_state(614);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==FuncTestCaseParser_QMark {
				{
				recog.base.set_state(613);
				let tmp = recog.base.match_token(FuncTestCaseParser_QMark,&mut recog.err_handler)?;
				 cast_mut::<_,DateTypeContext >(&mut _localctx).isnull = Some(tmp.clone());
				  

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- intervalYearType ----------------
pub type IntervalYearTypeContextAll<'input> = IntervalYearTypeContext<'input>;


pub type IntervalYearTypeContext<'input> = BaseParserRuleContext<'input,IntervalYearTypeContextExt<'input>>;

#[derive(Clone)]
pub struct IntervalYearTypeContextExt<'input>{
	pub isnull: Option<TokenType<'input>>,
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for IntervalYearTypeContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for IntervalYearTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_intervalYearType(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_intervalYearType(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for IntervalYearTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_intervalYearType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_intervalYearType }
}
antlr4rust::tid!{IntervalYearTypeContextExt<'a>}

impl<'input> IntervalYearTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<IntervalYearTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IntervalYearTypeContextExt{
				isnull: None, 

				ph:PhantomData
			}),
		)
	}
}

pub trait IntervalYearTypeContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<IntervalYearTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IYear
/// Returns `None` if there is no child corresponding to token IYear
fn IYear(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_IYear, 0)
}
/// Retrieves first TerminalNode corresponding to token Interval_Year
/// Returns `None` if there is no child corresponding to token Interval_Year
fn Interval_Year(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Interval_Year, 0)
}
/// Retrieves first TerminalNode corresponding to token QMark
/// Returns `None` if there is no child corresponding to token QMark
fn QMark(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_QMark, 0)
}

}

impl<'input> IntervalYearTypeContextAttrs<'input> for IntervalYearTypeContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn intervalYearType(&mut self,)
	-> Result<Rc<IntervalYearTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IntervalYearTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 128, RULE_intervalYearType);
        let mut _localctx: Rc<IntervalYearTypeContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(616);
			_la = recog.base.input.la(1);
			if { !(_la==FuncTestCaseParser_Interval_Year || _la==FuncTestCaseParser_IYear) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			recog.base.set_state(618);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==FuncTestCaseParser_QMark {
				{
				recog.base.set_state(617);
				let tmp = recog.base.match_token(FuncTestCaseParser_QMark,&mut recog.err_handler)?;
				 cast_mut::<_,IntervalYearTypeContext >(&mut _localctx).isnull = Some(tmp.clone());
				  

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- intervalDayType ----------------
pub type IntervalDayTypeContextAll<'input> = IntervalDayTypeContext<'input>;


pub type IntervalDayTypeContext<'input> = BaseParserRuleContext<'input,IntervalDayTypeContextExt<'input>>;

#[derive(Clone)]
pub struct IntervalDayTypeContextExt<'input>{
	pub isnull: Option<TokenType<'input>>,
	pub len: Option<Rc<NumericParameterContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for IntervalDayTypeContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for IntervalDayTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_intervalDayType(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_intervalDayType(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for IntervalDayTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_intervalDayType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_intervalDayType }
}
antlr4rust::tid!{IntervalDayTypeContextExt<'a>}

impl<'input> IntervalDayTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<IntervalDayTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IntervalDayTypeContextExt{
				isnull: None, 
				len: None, 

				ph:PhantomData
			}),
		)
	}
}

pub trait IntervalDayTypeContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<IntervalDayTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDay
/// Returns `None` if there is no child corresponding to token IDay
fn IDay(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_IDay, 0)
}
/// Retrieves first TerminalNode corresponding to token Interval_Day
/// Returns `None` if there is no child corresponding to token Interval_Day
fn Interval_Day(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Interval_Day, 0)
}
/// Retrieves first TerminalNode corresponding to token OAngleBracket
/// Returns `None` if there is no child corresponding to token OAngleBracket
fn OAngleBracket(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_OAngleBracket, 0)
}
/// Retrieves first TerminalNode corresponding to token CAngleBracket
/// Returns `None` if there is no child corresponding to token CAngleBracket
fn CAngleBracket(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_CAngleBracket, 0)
}
/// Retrieves first TerminalNode corresponding to token QMark
/// Returns `None` if there is no child corresponding to token QMark
fn QMark(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_QMark, 0)
}
fn numericParameter(&self) -> Option<Rc<NumericParameterContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> IntervalDayTypeContextAttrs<'input> for IntervalDayTypeContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn intervalDayType(&mut self,)
	-> Result<Rc<IntervalDayTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IntervalDayTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 130, RULE_intervalDayType);
        let mut _localctx: Rc<IntervalDayTypeContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(620);
			_la = recog.base.input.la(1);
			if { !(_la==FuncTestCaseParser_Interval_Day || _la==FuncTestCaseParser_IDay) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			recog.base.set_state(622);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==FuncTestCaseParser_QMark {
				{
				recog.base.set_state(621);
				let tmp = recog.base.match_token(FuncTestCaseParser_QMark,&mut recog.err_handler)?;
				 cast_mut::<_,IntervalDayTypeContext >(&mut _localctx).isnull = Some(tmp.clone());
				  

				}
			}

			recog.base.set_state(628);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==FuncTestCaseParser_OAngleBracket {
				{
				recog.base.set_state(624);
				recog.base.match_token(FuncTestCaseParser_OAngleBracket,&mut recog.err_handler)?;

				/*InvokeRule numericParameter*/
				recog.base.set_state(625);
				let tmp = recog.numericParameter()?;
				 cast_mut::<_,IntervalDayTypeContext >(&mut _localctx).len = Some(tmp.clone());
				  

				recog.base.set_state(626);
				recog.base.match_token(FuncTestCaseParser_CAngleBracket,&mut recog.err_handler)?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- intervalCompoundType ----------------
pub type IntervalCompoundTypeContextAll<'input> = IntervalCompoundTypeContext<'input>;


pub type IntervalCompoundTypeContext<'input> = BaseParserRuleContext<'input,IntervalCompoundTypeContextExt<'input>>;

#[derive(Clone)]
pub struct IntervalCompoundTypeContextExt<'input>{
	pub isnull: Option<TokenType<'input>>,
	pub len: Option<Rc<NumericParameterContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for IntervalCompoundTypeContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for IntervalCompoundTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_intervalCompoundType(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_intervalCompoundType(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for IntervalCompoundTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_intervalCompoundType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_intervalCompoundType }
}
antlr4rust::tid!{IntervalCompoundTypeContextExt<'a>}

impl<'input> IntervalCompoundTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<IntervalCompoundTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IntervalCompoundTypeContextExt{
				isnull: None, 
				len: None, 

				ph:PhantomData
			}),
		)
	}
}

pub trait IntervalCompoundTypeContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<IntervalCompoundTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ICompound
/// Returns `None` if there is no child corresponding to token ICompound
fn ICompound(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_ICompound, 0)
}
/// Retrieves first TerminalNode corresponding to token Interval_Compound
/// Returns `None` if there is no child corresponding to token Interval_Compound
fn Interval_Compound(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Interval_Compound, 0)
}
/// Retrieves first TerminalNode corresponding to token OAngleBracket
/// Returns `None` if there is no child corresponding to token OAngleBracket
fn OAngleBracket(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_OAngleBracket, 0)
}
/// Retrieves first TerminalNode corresponding to token CAngleBracket
/// Returns `None` if there is no child corresponding to token CAngleBracket
fn CAngleBracket(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_CAngleBracket, 0)
}
/// Retrieves first TerminalNode corresponding to token QMark
/// Returns `None` if there is no child corresponding to token QMark
fn QMark(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_QMark, 0)
}
fn numericParameter(&self) -> Option<Rc<NumericParameterContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> IntervalCompoundTypeContextAttrs<'input> for IntervalCompoundTypeContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn intervalCompoundType(&mut self,)
	-> Result<Rc<IntervalCompoundTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IntervalCompoundTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 132, RULE_intervalCompoundType);
        let mut _localctx: Rc<IntervalCompoundTypeContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(630);
			_la = recog.base.input.la(1);
			if { !(_la==FuncTestCaseParser_Interval_Compound || _la==FuncTestCaseParser_ICompound) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			recog.base.set_state(632);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==FuncTestCaseParser_QMark {
				{
				recog.base.set_state(631);
				let tmp = recog.base.match_token(FuncTestCaseParser_QMark,&mut recog.err_handler)?;
				 cast_mut::<_,IntervalCompoundTypeContext >(&mut _localctx).isnull = Some(tmp.clone());
				  

				}
			}

			recog.base.set_state(638);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==FuncTestCaseParser_OAngleBracket {
				{
				recog.base.set_state(634);
				recog.base.match_token(FuncTestCaseParser_OAngleBracket,&mut recog.err_handler)?;

				/*InvokeRule numericParameter*/
				recog.base.set_state(635);
				let tmp = recog.numericParameter()?;
				 cast_mut::<_,IntervalCompoundTypeContext >(&mut _localctx).len = Some(tmp.clone());
				  

				recog.base.set_state(636);
				recog.base.match_token(FuncTestCaseParser_CAngleBracket,&mut recog.err_handler)?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- fixedCharType ----------------
pub type FixedCharTypeContextAll<'input> = FixedCharTypeContext<'input>;


pub type FixedCharTypeContext<'input> = BaseParserRuleContext<'input,FixedCharTypeContextExt<'input>>;

#[derive(Clone)]
pub struct FixedCharTypeContextExt<'input>{
	pub isnull: Option<TokenType<'input>>,
	pub len: Option<Rc<NumericParameterContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for FixedCharTypeContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for FixedCharTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_fixedCharType(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_fixedCharType(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for FixedCharTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fixedCharType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fixedCharType }
}
antlr4rust::tid!{FixedCharTypeContextExt<'a>}

impl<'input> FixedCharTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<FixedCharTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FixedCharTypeContextExt{
				isnull: None, 
				len: None, 

				ph:PhantomData
			}),
		)
	}
}

pub trait FixedCharTypeContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<FixedCharTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token OAngleBracket
/// Returns `None` if there is no child corresponding to token OAngleBracket
fn OAngleBracket(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_OAngleBracket, 0)
}
/// Retrieves first TerminalNode corresponding to token CAngleBracket
/// Returns `None` if there is no child corresponding to token CAngleBracket
fn CAngleBracket(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_CAngleBracket, 0)
}
/// Retrieves first TerminalNode corresponding to token FChar
/// Returns `None` if there is no child corresponding to token FChar
fn FChar(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_FChar, 0)
}
/// Retrieves first TerminalNode corresponding to token FixedChar
/// Returns `None` if there is no child corresponding to token FixedChar
fn FixedChar(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_FixedChar, 0)
}
fn numericParameter(&self) -> Option<Rc<NumericParameterContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token QMark
/// Returns `None` if there is no child corresponding to token QMark
fn QMark(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_QMark, 0)
}

}

impl<'input> FixedCharTypeContextAttrs<'input> for FixedCharTypeContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn fixedCharType(&mut self,)
	-> Result<Rc<FixedCharTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FixedCharTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 134, RULE_fixedCharType);
        let mut _localctx: Rc<FixedCharTypeContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(640);
			_la = recog.base.input.la(1);
			if { !(_la==FuncTestCaseParser_FixedChar || _la==FuncTestCaseParser_FChar) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			recog.base.set_state(642);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==FuncTestCaseParser_QMark {
				{
				recog.base.set_state(641);
				let tmp = recog.base.match_token(FuncTestCaseParser_QMark,&mut recog.err_handler)?;
				 cast_mut::<_,FixedCharTypeContext >(&mut _localctx).isnull = Some(tmp.clone());
				  

				}
			}

			recog.base.set_state(644);
			recog.base.match_token(FuncTestCaseParser_OAngleBracket,&mut recog.err_handler)?;

			/*InvokeRule numericParameter*/
			recog.base.set_state(645);
			let tmp = recog.numericParameter()?;
			 cast_mut::<_,FixedCharTypeContext >(&mut _localctx).len = Some(tmp.clone());
			  

			recog.base.set_state(646);
			recog.base.match_token(FuncTestCaseParser_CAngleBracket,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- varCharType ----------------
pub type VarCharTypeContextAll<'input> = VarCharTypeContext<'input>;


pub type VarCharTypeContext<'input> = BaseParserRuleContext<'input,VarCharTypeContextExt<'input>>;

#[derive(Clone)]
pub struct VarCharTypeContextExt<'input>{
	pub isnull: Option<TokenType<'input>>,
	pub len: Option<Rc<NumericParameterContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for VarCharTypeContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for VarCharTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_varCharType(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_varCharType(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for VarCharTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_varCharType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_varCharType }
}
antlr4rust::tid!{VarCharTypeContextExt<'a>}

impl<'input> VarCharTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<VarCharTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,VarCharTypeContextExt{
				isnull: None, 
				len: None, 

				ph:PhantomData
			}),
		)
	}
}

pub trait VarCharTypeContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<VarCharTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token OAngleBracket
/// Returns `None` if there is no child corresponding to token OAngleBracket
fn OAngleBracket(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_OAngleBracket, 0)
}
/// Retrieves first TerminalNode corresponding to token CAngleBracket
/// Returns `None` if there is no child corresponding to token CAngleBracket
fn CAngleBracket(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_CAngleBracket, 0)
}
/// Retrieves first TerminalNode corresponding to token VChar
/// Returns `None` if there is no child corresponding to token VChar
fn VChar(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_VChar, 0)
}
/// Retrieves first TerminalNode corresponding to token VarChar
/// Returns `None` if there is no child corresponding to token VarChar
fn VarChar(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_VarChar, 0)
}
fn numericParameter(&self) -> Option<Rc<NumericParameterContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token QMark
/// Returns `None` if there is no child corresponding to token QMark
fn QMark(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_QMark, 0)
}

}

impl<'input> VarCharTypeContextAttrs<'input> for VarCharTypeContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn varCharType(&mut self,)
	-> Result<Rc<VarCharTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = VarCharTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 136, RULE_varCharType);
        let mut _localctx: Rc<VarCharTypeContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(648);
			_la = recog.base.input.la(1);
			if { !(_la==FuncTestCaseParser_VarChar || _la==FuncTestCaseParser_VChar) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			recog.base.set_state(650);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==FuncTestCaseParser_QMark {
				{
				recog.base.set_state(649);
				let tmp = recog.base.match_token(FuncTestCaseParser_QMark,&mut recog.err_handler)?;
				 cast_mut::<_,VarCharTypeContext >(&mut _localctx).isnull = Some(tmp.clone());
				  

				}
			}

			recog.base.set_state(652);
			recog.base.match_token(FuncTestCaseParser_OAngleBracket,&mut recog.err_handler)?;

			/*InvokeRule numericParameter*/
			recog.base.set_state(653);
			let tmp = recog.numericParameter()?;
			 cast_mut::<_,VarCharTypeContext >(&mut _localctx).len = Some(tmp.clone());
			  

			recog.base.set_state(654);
			recog.base.match_token(FuncTestCaseParser_CAngleBracket,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- fixedBinaryType ----------------
pub type FixedBinaryTypeContextAll<'input> = FixedBinaryTypeContext<'input>;


pub type FixedBinaryTypeContext<'input> = BaseParserRuleContext<'input,FixedBinaryTypeContextExt<'input>>;

#[derive(Clone)]
pub struct FixedBinaryTypeContextExt<'input>{
	pub isnull: Option<TokenType<'input>>,
	pub len: Option<Rc<NumericParameterContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for FixedBinaryTypeContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for FixedBinaryTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_fixedBinaryType(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_fixedBinaryType(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for FixedBinaryTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fixedBinaryType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fixedBinaryType }
}
antlr4rust::tid!{FixedBinaryTypeContextExt<'a>}

impl<'input> FixedBinaryTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<FixedBinaryTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FixedBinaryTypeContextExt{
				isnull: None, 
				len: None, 

				ph:PhantomData
			}),
		)
	}
}

pub trait FixedBinaryTypeContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<FixedBinaryTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token OAngleBracket
/// Returns `None` if there is no child corresponding to token OAngleBracket
fn OAngleBracket(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_OAngleBracket, 0)
}
/// Retrieves first TerminalNode corresponding to token CAngleBracket
/// Returns `None` if there is no child corresponding to token CAngleBracket
fn CAngleBracket(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_CAngleBracket, 0)
}
/// Retrieves first TerminalNode corresponding to token FBin
/// Returns `None` if there is no child corresponding to token FBin
fn FBin(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_FBin, 0)
}
/// Retrieves first TerminalNode corresponding to token FixedBinary
/// Returns `None` if there is no child corresponding to token FixedBinary
fn FixedBinary(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_FixedBinary, 0)
}
fn numericParameter(&self) -> Option<Rc<NumericParameterContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token QMark
/// Returns `None` if there is no child corresponding to token QMark
fn QMark(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_QMark, 0)
}

}

impl<'input> FixedBinaryTypeContextAttrs<'input> for FixedBinaryTypeContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn fixedBinaryType(&mut self,)
	-> Result<Rc<FixedBinaryTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FixedBinaryTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 138, RULE_fixedBinaryType);
        let mut _localctx: Rc<FixedBinaryTypeContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(656);
			_la = recog.base.input.la(1);
			if { !(_la==FuncTestCaseParser_FixedBinary || _la==FuncTestCaseParser_FBin) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			recog.base.set_state(658);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==FuncTestCaseParser_QMark {
				{
				recog.base.set_state(657);
				let tmp = recog.base.match_token(FuncTestCaseParser_QMark,&mut recog.err_handler)?;
				 cast_mut::<_,FixedBinaryTypeContext >(&mut _localctx).isnull = Some(tmp.clone());
				  

				}
			}

			recog.base.set_state(660);
			recog.base.match_token(FuncTestCaseParser_OAngleBracket,&mut recog.err_handler)?;

			/*InvokeRule numericParameter*/
			recog.base.set_state(661);
			let tmp = recog.numericParameter()?;
			 cast_mut::<_,FixedBinaryTypeContext >(&mut _localctx).len = Some(tmp.clone());
			  

			recog.base.set_state(662);
			recog.base.match_token(FuncTestCaseParser_CAngleBracket,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- decimalType ----------------
pub type DecimalTypeContextAll<'input> = DecimalTypeContext<'input>;


pub type DecimalTypeContext<'input> = BaseParserRuleContext<'input,DecimalTypeContextExt<'input>>;

#[derive(Clone)]
pub struct DecimalTypeContextExt<'input>{
	pub isnull: Option<TokenType<'input>>,
	pub precision: Option<Rc<NumericParameterContextAll<'input>>>,
	pub scale: Option<Rc<NumericParameterContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for DecimalTypeContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for DecimalTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_decimalType(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_decimalType(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for DecimalTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_decimalType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_decimalType }
}
antlr4rust::tid!{DecimalTypeContextExt<'a>}

impl<'input> DecimalTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<DecimalTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DecimalTypeContextExt{
				isnull: None, 
				precision: None, scale: None, 

				ph:PhantomData
			}),
		)
	}
}

pub trait DecimalTypeContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<DecimalTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Dec
/// Returns `None` if there is no child corresponding to token Dec
fn Dec(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Dec, 0)
}
/// Retrieves first TerminalNode corresponding to token Decimal
/// Returns `None` if there is no child corresponding to token Decimal
fn Decimal(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Decimal, 0)
}
/// Retrieves first TerminalNode corresponding to token OAngleBracket
/// Returns `None` if there is no child corresponding to token OAngleBracket
fn OAngleBracket(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_OAngleBracket, 0)
}
/// Retrieves first TerminalNode corresponding to token Comma
/// Returns `None` if there is no child corresponding to token Comma
fn Comma(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Comma, 0)
}
/// Retrieves first TerminalNode corresponding to token CAngleBracket
/// Returns `None` if there is no child corresponding to token CAngleBracket
fn CAngleBracket(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_CAngleBracket, 0)
}
/// Retrieves first TerminalNode corresponding to token QMark
/// Returns `None` if there is no child corresponding to token QMark
fn QMark(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_QMark, 0)
}
fn numericParameter_all(&self) ->  Vec<Rc<NumericParameterContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn numericParameter(&self, i: usize) -> Option<Rc<NumericParameterContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> DecimalTypeContextAttrs<'input> for DecimalTypeContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn decimalType(&mut self,)
	-> Result<Rc<DecimalTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DecimalTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 140, RULE_decimalType);
        let mut _localctx: Rc<DecimalTypeContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(664);
			_la = recog.base.input.la(1);
			if { !(_la==FuncTestCaseParser_Decimal || _la==FuncTestCaseParser_Dec) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			recog.base.set_state(666);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==FuncTestCaseParser_QMark {
				{
				recog.base.set_state(665);
				let tmp = recog.base.match_token(FuncTestCaseParser_QMark,&mut recog.err_handler)?;
				 cast_mut::<_,DecimalTypeContext >(&mut _localctx).isnull = Some(tmp.clone());
				  

				}
			}

			recog.base.set_state(674);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==FuncTestCaseParser_OAngleBracket {
				{
				recog.base.set_state(668);
				recog.base.match_token(FuncTestCaseParser_OAngleBracket,&mut recog.err_handler)?;

				/*InvokeRule numericParameter*/
				recog.base.set_state(669);
				let tmp = recog.numericParameter()?;
				 cast_mut::<_,DecimalTypeContext >(&mut _localctx).precision = Some(tmp.clone());
				  

				recog.base.set_state(670);
				recog.base.match_token(FuncTestCaseParser_Comma,&mut recog.err_handler)?;

				/*InvokeRule numericParameter*/
				recog.base.set_state(671);
				let tmp = recog.numericParameter()?;
				 cast_mut::<_,DecimalTypeContext >(&mut _localctx).scale = Some(tmp.clone());
				  

				recog.base.set_state(672);
				recog.base.match_token(FuncTestCaseParser_CAngleBracket,&mut recog.err_handler)?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- precisionTimeType ----------------
pub type PrecisionTimeTypeContextAll<'input> = PrecisionTimeTypeContext<'input>;


pub type PrecisionTimeTypeContext<'input> = BaseParserRuleContext<'input,PrecisionTimeTypeContextExt<'input>>;

#[derive(Clone)]
pub struct PrecisionTimeTypeContextExt<'input>{
	pub isnull: Option<TokenType<'input>>,
	pub precision: Option<Rc<NumericParameterContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for PrecisionTimeTypeContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for PrecisionTimeTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_precisionTimeType(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_precisionTimeType(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for PrecisionTimeTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_precisionTimeType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_precisionTimeType }
}
antlr4rust::tid!{PrecisionTimeTypeContextExt<'a>}

impl<'input> PrecisionTimeTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<PrecisionTimeTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PrecisionTimeTypeContextExt{
				isnull: None, 
				precision: None, 

				ph:PhantomData
			}),
		)
	}
}

pub trait PrecisionTimeTypeContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<PrecisionTimeTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token OAngleBracket
/// Returns `None` if there is no child corresponding to token OAngleBracket
fn OAngleBracket(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_OAngleBracket, 0)
}
/// Retrieves first TerminalNode corresponding to token CAngleBracket
/// Returns `None` if there is no child corresponding to token CAngleBracket
fn CAngleBracket(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_CAngleBracket, 0)
}
/// Retrieves first TerminalNode corresponding to token PT
/// Returns `None` if there is no child corresponding to token PT
fn PT(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_PT, 0)
}
/// Retrieves first TerminalNode corresponding to token Precision_Time
/// Returns `None` if there is no child corresponding to token Precision_Time
fn Precision_Time(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Precision_Time, 0)
}
fn numericParameter(&self) -> Option<Rc<NumericParameterContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token QMark
/// Returns `None` if there is no child corresponding to token QMark
fn QMark(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_QMark, 0)
}

}

impl<'input> PrecisionTimeTypeContextAttrs<'input> for PrecisionTimeTypeContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn precisionTimeType(&mut self,)
	-> Result<Rc<PrecisionTimeTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PrecisionTimeTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 142, RULE_precisionTimeType);
        let mut _localctx: Rc<PrecisionTimeTypeContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(676);
			_la = recog.base.input.la(1);
			if { !(_la==FuncTestCaseParser_Precision_Time || _la==FuncTestCaseParser_PT) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			recog.base.set_state(678);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==FuncTestCaseParser_QMark {
				{
				recog.base.set_state(677);
				let tmp = recog.base.match_token(FuncTestCaseParser_QMark,&mut recog.err_handler)?;
				 cast_mut::<_,PrecisionTimeTypeContext >(&mut _localctx).isnull = Some(tmp.clone());
				  

				}
			}

			recog.base.set_state(680);
			recog.base.match_token(FuncTestCaseParser_OAngleBracket,&mut recog.err_handler)?;

			/*InvokeRule numericParameter*/
			recog.base.set_state(681);
			let tmp = recog.numericParameter()?;
			 cast_mut::<_,PrecisionTimeTypeContext >(&mut _localctx).precision = Some(tmp.clone());
			  

			recog.base.set_state(682);
			recog.base.match_token(FuncTestCaseParser_CAngleBracket,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- precisionTimestampType ----------------
pub type PrecisionTimestampTypeContextAll<'input> = PrecisionTimestampTypeContext<'input>;


pub type PrecisionTimestampTypeContext<'input> = BaseParserRuleContext<'input,PrecisionTimestampTypeContextExt<'input>>;

#[derive(Clone)]
pub struct PrecisionTimestampTypeContextExt<'input>{
	pub isnull: Option<TokenType<'input>>,
	pub precision: Option<Rc<NumericParameterContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for PrecisionTimestampTypeContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for PrecisionTimestampTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_precisionTimestampType(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_precisionTimestampType(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for PrecisionTimestampTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_precisionTimestampType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_precisionTimestampType }
}
antlr4rust::tid!{PrecisionTimestampTypeContextExt<'a>}

impl<'input> PrecisionTimestampTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<PrecisionTimestampTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PrecisionTimestampTypeContextExt{
				isnull: None, 
				precision: None, 

				ph:PhantomData
			}),
		)
	}
}

pub trait PrecisionTimestampTypeContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<PrecisionTimestampTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token OAngleBracket
/// Returns `None` if there is no child corresponding to token OAngleBracket
fn OAngleBracket(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_OAngleBracket, 0)
}
/// Retrieves first TerminalNode corresponding to token CAngleBracket
/// Returns `None` if there is no child corresponding to token CAngleBracket
fn CAngleBracket(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_CAngleBracket, 0)
}
/// Retrieves first TerminalNode corresponding to token PTs
/// Returns `None` if there is no child corresponding to token PTs
fn PTs(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_PTs, 0)
}
/// Retrieves first TerminalNode corresponding to token Precision_Timestamp
/// Returns `None` if there is no child corresponding to token Precision_Timestamp
fn Precision_Timestamp(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Precision_Timestamp, 0)
}
fn numericParameter(&self) -> Option<Rc<NumericParameterContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token QMark
/// Returns `None` if there is no child corresponding to token QMark
fn QMark(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_QMark, 0)
}

}

impl<'input> PrecisionTimestampTypeContextAttrs<'input> for PrecisionTimestampTypeContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn precisionTimestampType(&mut self,)
	-> Result<Rc<PrecisionTimestampTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PrecisionTimestampTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 144, RULE_precisionTimestampType);
        let mut _localctx: Rc<PrecisionTimestampTypeContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(684);
			_la = recog.base.input.la(1);
			if { !(_la==FuncTestCaseParser_Precision_Timestamp || _la==FuncTestCaseParser_PTs) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			recog.base.set_state(686);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==FuncTestCaseParser_QMark {
				{
				recog.base.set_state(685);
				let tmp = recog.base.match_token(FuncTestCaseParser_QMark,&mut recog.err_handler)?;
				 cast_mut::<_,PrecisionTimestampTypeContext >(&mut _localctx).isnull = Some(tmp.clone());
				  

				}
			}

			recog.base.set_state(688);
			recog.base.match_token(FuncTestCaseParser_OAngleBracket,&mut recog.err_handler)?;

			/*InvokeRule numericParameter*/
			recog.base.set_state(689);
			let tmp = recog.numericParameter()?;
			 cast_mut::<_,PrecisionTimestampTypeContext >(&mut _localctx).precision = Some(tmp.clone());
			  

			recog.base.set_state(690);
			recog.base.match_token(FuncTestCaseParser_CAngleBracket,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- precisionTimestampTZType ----------------
pub type PrecisionTimestampTZTypeContextAll<'input> = PrecisionTimestampTZTypeContext<'input>;


pub type PrecisionTimestampTZTypeContext<'input> = BaseParserRuleContext<'input,PrecisionTimestampTZTypeContextExt<'input>>;

#[derive(Clone)]
pub struct PrecisionTimestampTZTypeContextExt<'input>{
	pub isnull: Option<TokenType<'input>>,
	pub precision: Option<Rc<NumericParameterContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for PrecisionTimestampTZTypeContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for PrecisionTimestampTZTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_precisionTimestampTZType(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_precisionTimestampTZType(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for PrecisionTimestampTZTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_precisionTimestampTZType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_precisionTimestampTZType }
}
antlr4rust::tid!{PrecisionTimestampTZTypeContextExt<'a>}

impl<'input> PrecisionTimestampTZTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<PrecisionTimestampTZTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PrecisionTimestampTZTypeContextExt{
				isnull: None, 
				precision: None, 

				ph:PhantomData
			}),
		)
	}
}

pub trait PrecisionTimestampTZTypeContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<PrecisionTimestampTZTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token OAngleBracket
/// Returns `None` if there is no child corresponding to token OAngleBracket
fn OAngleBracket(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_OAngleBracket, 0)
}
/// Retrieves first TerminalNode corresponding to token CAngleBracket
/// Returns `None` if there is no child corresponding to token CAngleBracket
fn CAngleBracket(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_CAngleBracket, 0)
}
/// Retrieves first TerminalNode corresponding to token PTsTZ
/// Returns `None` if there is no child corresponding to token PTsTZ
fn PTsTZ(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_PTsTZ, 0)
}
/// Retrieves first TerminalNode corresponding to token Precision_Timestamp_TZ
/// Returns `None` if there is no child corresponding to token Precision_Timestamp_TZ
fn Precision_Timestamp_TZ(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Precision_Timestamp_TZ, 0)
}
fn numericParameter(&self) -> Option<Rc<NumericParameterContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token QMark
/// Returns `None` if there is no child corresponding to token QMark
fn QMark(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_QMark, 0)
}

}

impl<'input> PrecisionTimestampTZTypeContextAttrs<'input> for PrecisionTimestampTZTypeContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn precisionTimestampTZType(&mut self,)
	-> Result<Rc<PrecisionTimestampTZTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PrecisionTimestampTZTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 146, RULE_precisionTimestampTZType);
        let mut _localctx: Rc<PrecisionTimestampTZTypeContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(692);
			_la = recog.base.input.la(1);
			if { !(_la==FuncTestCaseParser_Precision_Timestamp_TZ || _la==FuncTestCaseParser_PTsTZ) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			recog.base.set_state(694);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==FuncTestCaseParser_QMark {
				{
				recog.base.set_state(693);
				let tmp = recog.base.match_token(FuncTestCaseParser_QMark,&mut recog.err_handler)?;
				 cast_mut::<_,PrecisionTimestampTZTypeContext >(&mut _localctx).isnull = Some(tmp.clone());
				  

				}
			}

			recog.base.set_state(696);
			recog.base.match_token(FuncTestCaseParser_OAngleBracket,&mut recog.err_handler)?;

			/*InvokeRule numericParameter*/
			recog.base.set_state(697);
			let tmp = recog.numericParameter()?;
			 cast_mut::<_,PrecisionTimestampTZTypeContext >(&mut _localctx).precision = Some(tmp.clone());
			  

			recog.base.set_state(698);
			recog.base.match_token(FuncTestCaseParser_CAngleBracket,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- listType ----------------
#[derive(Debug)]
pub enum ListTypeContextAll<'input>{
	ListContext(ListContext<'input>),
Error(ListTypeContext<'input>)
}
antlr4rust::tid!{ListTypeContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for ListTypeContextAll<'input>{}

impl<'input> FuncTestCaseParserContext<'input> for ListTypeContextAll<'input>{}

impl<'input> Deref for ListTypeContextAll<'input>{
	type Target = dyn ListTypeContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use ListTypeContextAll::*;
		match self{
			ListContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for ListTypeContextAll<'input>{
    fn enter(&self, listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
}



pub type ListTypeContext<'input> = BaseParserRuleContext<'input,ListTypeContextExt<'input>>;

#[derive(Clone)]
pub struct ListTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for ListTypeContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for ListTypeContext<'input>{
}

impl<'input> CustomRuleContext<'input> for ListTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_listType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_listType }
}
antlr4rust::tid!{ListTypeContextExt<'a>}

impl<'input> ListTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ListTypeContextAll<'input>> {
		Rc::new(
		ListTypeContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ListTypeContextExt{

				ph:PhantomData
			}),
		)
		)
	}
}

pub trait ListTypeContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<ListTypeContextExt<'input>>{


}

impl<'input> ListTypeContextAttrs<'input> for ListTypeContext<'input>{}

pub type ListContext<'input> = BaseParserRuleContext<'input,ListContextExt<'input>>;

pub trait ListContextAttrs<'input>: FuncTestCaseParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token List
	/// Returns `None` if there is no child corresponding to token List
	fn List(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
		self.get_token(FuncTestCaseParser_List, 0)
	}
	/// Retrieves first TerminalNode corresponding to token OAngleBracket
	/// Returns `None` if there is no child corresponding to token OAngleBracket
	fn OAngleBracket(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
		self.get_token(FuncTestCaseParser_OAngleBracket, 0)
	}
	/// Retrieves first TerminalNode corresponding to token CAngleBracket
	/// Returns `None` if there is no child corresponding to token CAngleBracket
	fn CAngleBracket(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
		self.get_token(FuncTestCaseParser_CAngleBracket, 0)
	}
	fn dataType(&self) -> Option<Rc<DataTypeContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token QMark
	/// Returns `None` if there is no child corresponding to token QMark
	fn QMark(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
		self.get_token(FuncTestCaseParser_QMark, 0)
	}
}

impl<'input> ListContextAttrs<'input> for ListContext<'input>{}

pub struct ListContextExt<'input>{
	base:ListTypeContextExt<'input>,
	pub isnull: Option<TokenType<'input>>,
	pub elemType: Option<Rc<DataTypeContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{ListContextExt<'a>}

impl<'input> FuncTestCaseParserContext<'input> for ListContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for ListContext<'input>{
	fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_list(self);
		Ok(())
	}
}

impl<'input> CustomRuleContext<'input> for ListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_listType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_listType }
}

impl<'input> Borrow<ListTypeContextExt<'input>> for ListContext<'input>{
	fn borrow(&self) -> &ListTypeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ListTypeContextExt<'input>> for ListContext<'input>{
	fn borrow_mut(&mut self) -> &mut ListTypeContextExt<'input> { &mut self.base }
}

impl<'input> ListTypeContextAttrs<'input> for ListContext<'input> {}

impl<'input> ListContextExt<'input>{
	fn new(ctx: &dyn ListTypeContextAttrs<'input>) -> Rc<ListTypeContextAll<'input>>  {
		Rc::new(
			ListTypeContextAll::ListContext(
				BaseParserRuleContext::copy_from(ctx,ListContextExt{
					isnull:None, 
        			elemType:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn listType(&mut self,)
	-> Result<Rc<ListTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ListTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 148, RULE_listType);
        let mut _localctx: Rc<ListTypeContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let tmp = ListContextExt::new(&**_localctx);
			recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
			_localctx = tmp;
			{
			recog.base.set_state(700);
			recog.base.match_token(FuncTestCaseParser_List,&mut recog.err_handler)?;

			recog.base.set_state(702);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==FuncTestCaseParser_QMark {
				{
				recog.base.set_state(701);
				let tmp = recog.base.match_token(FuncTestCaseParser_QMark,&mut recog.err_handler)?;
				if let ListTypeContextAll::ListContext(ctx) = cast_mut::<_,ListTypeContextAll >(&mut _localctx){
				ctx.isnull = Some(tmp.clone()); } else {unreachable!("cant cast");}  

				}
			}

			recog.base.set_state(704);
			recog.base.match_token(FuncTestCaseParser_OAngleBracket,&mut recog.err_handler)?;

			/*InvokeRule dataType*/
			recog.base.set_state(705);
			let tmp = recog.dataType()?;
			if let ListTypeContextAll::ListContext(ctx) = cast_mut::<_,ListTypeContextAll >(&mut _localctx){
			ctx.elemType = Some(tmp.clone()); } else {unreachable!("cant cast");}  

			recog.base.set_state(706);
			recog.base.match_token(FuncTestCaseParser_CAngleBracket,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- structType ----------------
pub type StructTypeContextAll<'input> = StructTypeContext<'input>;


pub type StructTypeContext<'input> = BaseParserRuleContext<'input,StructTypeContextExt<'input>>;

#[derive(Clone)]
pub struct StructTypeContextExt<'input>{
	pub isnull: Option<TokenType<'input>>,
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for StructTypeContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for StructTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_structType(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_structType(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for StructTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_structType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_structType }
}
antlr4rust::tid!{StructTypeContextExt<'a>}

impl<'input> StructTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<StructTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StructTypeContextExt{
				isnull: None, 

				ph:PhantomData
			}),
		)
	}
}

pub trait StructTypeContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<StructTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Struct
/// Returns `None` if there is no child corresponding to token Struct
fn Struct(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Struct, 0)
}
/// Retrieves first TerminalNode corresponding to token OAngleBracket
/// Returns `None` if there is no child corresponding to token OAngleBracket
fn OAngleBracket(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_OAngleBracket, 0)
}
/// Retrieves first TerminalNode corresponding to token CAngleBracket
/// Returns `None` if there is no child corresponding to token CAngleBracket
fn CAngleBracket(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_CAngleBracket, 0)
}
fn dataType_all(&self) ->  Vec<Rc<DataTypeContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn dataType(&self, i: usize) -> Option<Rc<DataTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token QMark
/// Returns `None` if there is no child corresponding to token QMark
fn QMark(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_QMark, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Comma, i)
}

}

impl<'input> StructTypeContextAttrs<'input> for StructTypeContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn structType(&mut self,)
	-> Result<Rc<StructTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StructTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 150, RULE_structType);
        let mut _localctx: Rc<StructTypeContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(708);
			recog.base.match_token(FuncTestCaseParser_Struct,&mut recog.err_handler)?;

			recog.base.set_state(710);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==FuncTestCaseParser_QMark {
				{
				recog.base.set_state(709);
				let tmp = recog.base.match_token(FuncTestCaseParser_QMark,&mut recog.err_handler)?;
				 cast_mut::<_,StructTypeContext >(&mut _localctx).isnull = Some(tmp.clone());
				  

				}
			}

			recog.base.set_state(712);
			recog.base.match_token(FuncTestCaseParser_OAngleBracket,&mut recog.err_handler)?;

			recog.base.set_state(721);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 57)) & !0x3f) == 0 && ((1usize << (_la - 57)) & 4286578687) != 0) || ((((_la - 89)) & !0x3f) == 0 && ((1usize << (_la - 89)) & 255) != 0) {
				{
				/*InvokeRule dataType*/
				recog.base.set_state(713);
				recog.dataType()?;

				recog.base.set_state(718);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==FuncTestCaseParser_Comma {
					{
					{
					recog.base.set_state(714);
					recog.base.match_token(FuncTestCaseParser_Comma,&mut recog.err_handler)?;

					/*InvokeRule dataType*/
					recog.base.set_state(715);
					recog.dataType()?;

					}
					}
					recog.base.set_state(720);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			recog.base.set_state(723);
			recog.base.match_token(FuncTestCaseParser_CAngleBracket,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- mapType ----------------
pub type MapTypeContextAll<'input> = MapTypeContext<'input>;


pub type MapTypeContext<'input> = BaseParserRuleContext<'input,MapTypeContextExt<'input>>;

#[derive(Clone)]
pub struct MapTypeContextExt<'input>{
	pub isnull: Option<TokenType<'input>>,
	pub keyType: Option<Rc<DataTypeContextAll<'input>>>,
	pub valueType: Option<Rc<DataTypeContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for MapTypeContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for MapTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_mapType(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_mapType(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for MapTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_mapType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mapType }
}
antlr4rust::tid!{MapTypeContextExt<'a>}

impl<'input> MapTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<MapTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MapTypeContextExt{
				isnull: None, 
				keyType: None, valueType: None, 

				ph:PhantomData
			}),
		)
	}
}

pub trait MapTypeContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<MapTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Map
/// Returns `None` if there is no child corresponding to token Map
fn Map(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Map, 0)
}
/// Retrieves first TerminalNode corresponding to token OAngleBracket
/// Returns `None` if there is no child corresponding to token OAngleBracket
fn OAngleBracket(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_OAngleBracket, 0)
}
/// Retrieves first TerminalNode corresponding to token Comma
/// Returns `None` if there is no child corresponding to token Comma
fn Comma(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Comma, 0)
}
/// Retrieves first TerminalNode corresponding to token CAngleBracket
/// Returns `None` if there is no child corresponding to token CAngleBracket
fn CAngleBracket(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_CAngleBracket, 0)
}
fn dataType_all(&self) ->  Vec<Rc<DataTypeContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn dataType(&self, i: usize) -> Option<Rc<DataTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token QMark
/// Returns `None` if there is no child corresponding to token QMark
fn QMark(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_QMark, 0)
}

}

impl<'input> MapTypeContextAttrs<'input> for MapTypeContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn mapType(&mut self,)
	-> Result<Rc<MapTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MapTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 152, RULE_mapType);
        let mut _localctx: Rc<MapTypeContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(725);
			recog.base.match_token(FuncTestCaseParser_Map,&mut recog.err_handler)?;

			recog.base.set_state(727);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==FuncTestCaseParser_QMark {
				{
				recog.base.set_state(726);
				let tmp = recog.base.match_token(FuncTestCaseParser_QMark,&mut recog.err_handler)?;
				 cast_mut::<_,MapTypeContext >(&mut _localctx).isnull = Some(tmp.clone());
				  

				}
			}

			recog.base.set_state(729);
			recog.base.match_token(FuncTestCaseParser_OAngleBracket,&mut recog.err_handler)?;

			/*InvokeRule dataType*/
			recog.base.set_state(730);
			let tmp = recog.dataType()?;
			 cast_mut::<_,MapTypeContext >(&mut _localctx).keyType = Some(tmp.clone());
			  

			recog.base.set_state(731);
			recog.base.match_token(FuncTestCaseParser_Comma,&mut recog.err_handler)?;

			/*InvokeRule dataType*/
			recog.base.set_state(732);
			let tmp = recog.dataType()?;
			 cast_mut::<_,MapTypeContext >(&mut _localctx).valueType = Some(tmp.clone());
			  

			recog.base.set_state(733);
			recog.base.match_token(FuncTestCaseParser_CAngleBracket,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- funcType ----------------
pub type FuncTypeContextAll<'input> = FuncTypeContext<'input>;


pub type FuncTypeContext<'input> = BaseParserRuleContext<'input,FuncTypeContextExt<'input>>;

#[derive(Clone)]
pub struct FuncTypeContextExt<'input>{
	pub isnull: Option<TokenType<'input>>,
	pub params: Option<Rc<FuncParametersContextAll<'input>>>,
	pub returnType: Option<Rc<DataTypeContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for FuncTypeContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for FuncTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_funcType(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_funcType(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for FuncTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_funcType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_funcType }
}
antlr4rust::tid!{FuncTypeContextExt<'a>}

impl<'input> FuncTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<FuncTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FuncTypeContextExt{
				isnull: None, 
				params: None, returnType: None, 

				ph:PhantomData
			}),
		)
	}
}

pub trait FuncTypeContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<FuncTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Func
/// Returns `None` if there is no child corresponding to token Func
fn Func(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Func, 0)
}
/// Retrieves first TerminalNode corresponding to token OAngleBracket
/// Returns `None` if there is no child corresponding to token OAngleBracket
fn OAngleBracket(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_OAngleBracket, 0)
}
/// Retrieves first TerminalNode corresponding to token Arrow
/// Returns `None` if there is no child corresponding to token Arrow
fn Arrow(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Arrow, 0)
}
/// Retrieves first TerminalNode corresponding to token CAngleBracket
/// Returns `None` if there is no child corresponding to token CAngleBracket
fn CAngleBracket(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_CAngleBracket, 0)
}
fn funcParameters(&self) -> Option<Rc<FuncParametersContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn dataType(&self) -> Option<Rc<DataTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token QMark
/// Returns `None` if there is no child corresponding to token QMark
fn QMark(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_QMark, 0)
}

}

impl<'input> FuncTypeContextAttrs<'input> for FuncTypeContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn funcType(&mut self,)
	-> Result<Rc<FuncTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FuncTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 154, RULE_funcType);
        let mut _localctx: Rc<FuncTypeContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(735);
			recog.base.match_token(FuncTestCaseParser_Func,&mut recog.err_handler)?;

			recog.base.set_state(737);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==FuncTestCaseParser_QMark {
				{
				recog.base.set_state(736);
				let tmp = recog.base.match_token(FuncTestCaseParser_QMark,&mut recog.err_handler)?;
				 cast_mut::<_,FuncTypeContext >(&mut _localctx).isnull = Some(tmp.clone());
				  

				}
			}

			recog.base.set_state(739);
			recog.base.match_token(FuncTestCaseParser_OAngleBracket,&mut recog.err_handler)?;

			/*InvokeRule funcParameters*/
			recog.base.set_state(740);
			let tmp = recog.funcParameters()?;
			 cast_mut::<_,FuncTypeContext >(&mut _localctx).params = Some(tmp.clone());
			  

			recog.base.set_state(741);
			recog.base.match_token(FuncTestCaseParser_Arrow,&mut recog.err_handler)?;

			/*InvokeRule dataType*/
			recog.base.set_state(742);
			let tmp = recog.dataType()?;
			 cast_mut::<_,FuncTypeContext >(&mut _localctx).returnType = Some(tmp.clone());
			  

			recog.base.set_state(743);
			recog.base.match_token(FuncTestCaseParser_CAngleBracket,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- funcParameters ----------------
#[derive(Debug)]
pub enum FuncParametersContextAll<'input>{
	SingleFuncParamContext(SingleFuncParamContext<'input>),
	FuncParamsWithParensContext(FuncParamsWithParensContext<'input>),
Error(FuncParametersContext<'input>)
}
antlr4rust::tid!{FuncParametersContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for FuncParametersContextAll<'input>{}

impl<'input> FuncTestCaseParserContext<'input> for FuncParametersContextAll<'input>{}

impl<'input> Deref for FuncParametersContextAll<'input>{
	type Target = dyn FuncParametersContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use FuncParametersContextAll::*;
		match self{
			SingleFuncParamContext(inner) => inner,
			FuncParamsWithParensContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for FuncParametersContextAll<'input>{
    fn enter(&self, listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
}



pub type FuncParametersContext<'input> = BaseParserRuleContext<'input,FuncParametersContextExt<'input>>;

#[derive(Clone)]
pub struct FuncParametersContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for FuncParametersContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for FuncParametersContext<'input>{
}

impl<'input> CustomRuleContext<'input> for FuncParametersContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_funcParameters }
	//fn type_rule_index() -> usize where Self: Sized { RULE_funcParameters }
}
antlr4rust::tid!{FuncParametersContextExt<'a>}

impl<'input> FuncParametersContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<FuncParametersContextAll<'input>> {
		Rc::new(
		FuncParametersContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FuncParametersContextExt{

				ph:PhantomData
			}),
		)
		)
	}
}

pub trait FuncParametersContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<FuncParametersContextExt<'input>>{


}

impl<'input> FuncParametersContextAttrs<'input> for FuncParametersContext<'input>{}

pub type SingleFuncParamContext<'input> = BaseParserRuleContext<'input,SingleFuncParamContextExt<'input>>;

pub trait SingleFuncParamContextAttrs<'input>: FuncTestCaseParserContext<'input>{
	fn dataType(&self) -> Option<Rc<DataTypeContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> SingleFuncParamContextAttrs<'input> for SingleFuncParamContext<'input>{}

pub struct SingleFuncParamContextExt<'input>{
	base:FuncParametersContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{SingleFuncParamContextExt<'a>}

impl<'input> FuncTestCaseParserContext<'input> for SingleFuncParamContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for SingleFuncParamContext<'input>{
	fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_singleFuncParam(self);
		Ok(())
	}
}

impl<'input> CustomRuleContext<'input> for SingleFuncParamContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_funcParameters }
	//fn type_rule_index() -> usize where Self: Sized { RULE_funcParameters }
}

impl<'input> Borrow<FuncParametersContextExt<'input>> for SingleFuncParamContext<'input>{
	fn borrow(&self) -> &FuncParametersContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<FuncParametersContextExt<'input>> for SingleFuncParamContext<'input>{
	fn borrow_mut(&mut self) -> &mut FuncParametersContextExt<'input> { &mut self.base }
}

impl<'input> FuncParametersContextAttrs<'input> for SingleFuncParamContext<'input> {}

impl<'input> SingleFuncParamContextExt<'input>{
	fn new(ctx: &dyn FuncParametersContextAttrs<'input>) -> Rc<FuncParametersContextAll<'input>>  {
		Rc::new(
			FuncParametersContextAll::SingleFuncParamContext(
				BaseParserRuleContext::copy_from(ctx,SingleFuncParamContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type FuncParamsWithParensContext<'input> = BaseParserRuleContext<'input,FuncParamsWithParensContextExt<'input>>;

pub trait FuncParamsWithParensContextAttrs<'input>: FuncTestCaseParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token OParen
	/// Returns `None` if there is no child corresponding to token OParen
	fn OParen(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
		self.get_token(FuncTestCaseParser_OParen, 0)
	}
	fn dataType_all(&self) ->  Vec<Rc<DataTypeContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn dataType(&self, i: usize) -> Option<Rc<DataTypeContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token CParen
	/// Returns `None` if there is no child corresponding to token CParen
	fn CParen(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
		self.get_token(FuncTestCaseParser_CParen, 0)
	}
	/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
	fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
	/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
	fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
		self.get_token(FuncTestCaseParser_Comma, i)
	}
}

impl<'input> FuncParamsWithParensContextAttrs<'input> for FuncParamsWithParensContext<'input>{}

pub struct FuncParamsWithParensContextExt<'input>{
	base:FuncParametersContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{FuncParamsWithParensContextExt<'a>}

impl<'input> FuncTestCaseParserContext<'input> for FuncParamsWithParensContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for FuncParamsWithParensContext<'input>{
	fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_funcParamsWithParens(self);
		Ok(())
	}
}

impl<'input> CustomRuleContext<'input> for FuncParamsWithParensContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_funcParameters }
	//fn type_rule_index() -> usize where Self: Sized { RULE_funcParameters }
}

impl<'input> Borrow<FuncParametersContextExt<'input>> for FuncParamsWithParensContext<'input>{
	fn borrow(&self) -> &FuncParametersContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<FuncParametersContextExt<'input>> for FuncParamsWithParensContext<'input>{
	fn borrow_mut(&mut self) -> &mut FuncParametersContextExt<'input> { &mut self.base }
}

impl<'input> FuncParametersContextAttrs<'input> for FuncParamsWithParensContext<'input> {}

impl<'input> FuncParamsWithParensContextExt<'input>{
	fn new(ctx: &dyn FuncParametersContextAttrs<'input>) -> Rc<FuncParametersContextAll<'input>>  {
		Rc::new(
			FuncParametersContextAll::FuncParamsWithParensContext(
				BaseParserRuleContext::copy_from(ctx,FuncParamsWithParensContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn funcParameters(&mut self,)
	-> Result<Rc<FuncParametersContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FuncParametersContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 156, RULE_funcParameters);
        let mut _localctx: Rc<FuncParametersContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(757);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			FuncTestCaseParser_Func |FuncTestCaseParser_Boolean |FuncTestCaseParser_I8 |
			FuncTestCaseParser_I16 |FuncTestCaseParser_I32 |FuncTestCaseParser_I64 |
			FuncTestCaseParser_FP32 |FuncTestCaseParser_FP64 |FuncTestCaseParser_String |
			FuncTestCaseParser_Binary |FuncTestCaseParser_Date |FuncTestCaseParser_Interval_Year |
			FuncTestCaseParser_Interval_Day |FuncTestCaseParser_Interval_Compound |
			FuncTestCaseParser_UUID |FuncTestCaseParser_Decimal |FuncTestCaseParser_Precision_Time |
			FuncTestCaseParser_Precision_Timestamp |FuncTestCaseParser_Precision_Timestamp_TZ |
			FuncTestCaseParser_FixedChar |FuncTestCaseParser_VarChar |FuncTestCaseParser_FixedBinary |
			FuncTestCaseParser_Struct |FuncTestCaseParser_List |FuncTestCaseParser_Map |
			FuncTestCaseParser_UserDefined |FuncTestCaseParser_Bool |FuncTestCaseParser_Str |
			FuncTestCaseParser_VBin |FuncTestCaseParser_IYear |FuncTestCaseParser_IDay |
			FuncTestCaseParser_ICompound |FuncTestCaseParser_Dec |FuncTestCaseParser_PT |
			FuncTestCaseParser_PTs |FuncTestCaseParser_PTsTZ |FuncTestCaseParser_FChar |
			FuncTestCaseParser_VChar |FuncTestCaseParser_FBin 
				=> {
					let tmp = SingleFuncParamContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
					_localctx = tmp;
					{
					/*InvokeRule dataType*/
					recog.base.set_state(745);
					recog.dataType()?;

					}
				}

			FuncTestCaseParser_OParen 
				=> {
					let tmp = FuncParamsWithParensContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2)?;
					_localctx = tmp;
					{
					recog.base.set_state(746);
					recog.base.match_token(FuncTestCaseParser_OParen,&mut recog.err_handler)?;

					/*InvokeRule dataType*/
					recog.base.set_state(747);
					recog.dataType()?;

					recog.base.set_state(752);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==FuncTestCaseParser_Comma {
						{
						{
						recog.base.set_state(748);
						recog.base.match_token(FuncTestCaseParser_Comma,&mut recog.err_handler)?;

						/*InvokeRule dataType*/
						recog.base.set_state(749);
						recog.dataType()?;

						}
						}
						recog.base.set_state(754);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(755);
					recog.base.match_token(FuncTestCaseParser_CParen,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- parameterizedType ----------------
pub type ParameterizedTypeContextAll<'input> = ParameterizedTypeContext<'input>;


pub type ParameterizedTypeContext<'input> = BaseParserRuleContext<'input,ParameterizedTypeContextExt<'input>>;

#[derive(Clone)]
pub struct ParameterizedTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for ParameterizedTypeContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for ParameterizedTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_parameterizedType(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_parameterizedType(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for ParameterizedTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parameterizedType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parameterizedType }
}
antlr4rust::tid!{ParameterizedTypeContextExt<'a>}

impl<'input> ParameterizedTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ParameterizedTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ParameterizedTypeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait ParameterizedTypeContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<ParameterizedTypeContextExt<'input>>{

fn fixedCharType(&self) -> Option<Rc<FixedCharTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn varCharType(&self) -> Option<Rc<VarCharTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn fixedBinaryType(&self) -> Option<Rc<FixedBinaryTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn decimalType(&self) -> Option<Rc<DecimalTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn intervalDayType(&self) -> Option<Rc<IntervalDayTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn intervalCompoundType(&self) -> Option<Rc<IntervalCompoundTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn precisionTimeType(&self) -> Option<Rc<PrecisionTimeTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn precisionTimestampType(&self) -> Option<Rc<PrecisionTimestampTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn precisionTimestampTZType(&self) -> Option<Rc<PrecisionTimestampTZTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn listType(&self) -> Option<Rc<ListTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn structType(&self) -> Option<Rc<StructTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn mapType(&self) -> Option<Rc<MapTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn funcType(&self) -> Option<Rc<FuncTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ParameterizedTypeContextAttrs<'input> for ParameterizedTypeContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn parameterizedType(&mut self,)
	-> Result<Rc<ParameterizedTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ParameterizedTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 158, RULE_parameterizedType);
        let mut _localctx: Rc<ParameterizedTypeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(772);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			FuncTestCaseParser_FixedChar |FuncTestCaseParser_FChar 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule fixedCharType*/
					recog.base.set_state(759);
					recog.fixedCharType()?;

					}
				}

			FuncTestCaseParser_VarChar |FuncTestCaseParser_VChar 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule varCharType*/
					recog.base.set_state(760);
					recog.varCharType()?;

					}
				}

			FuncTestCaseParser_FixedBinary |FuncTestCaseParser_FBin 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule fixedBinaryType*/
					recog.base.set_state(761);
					recog.fixedBinaryType()?;

					}
				}

			FuncTestCaseParser_Decimal |FuncTestCaseParser_Dec 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					/*InvokeRule decimalType*/
					recog.base.set_state(762);
					recog.decimalType()?;

					}
				}

			FuncTestCaseParser_Interval_Day |FuncTestCaseParser_IDay 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					/*InvokeRule intervalDayType*/
					recog.base.set_state(763);
					recog.intervalDayType()?;

					}
				}

			FuncTestCaseParser_Interval_Compound |FuncTestCaseParser_ICompound 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 6)?;
					recog.base.enter_outer_alt(None, 6)?;
					{
					/*InvokeRule intervalCompoundType*/
					recog.base.set_state(764);
					recog.intervalCompoundType()?;

					}
				}

			FuncTestCaseParser_Precision_Time |FuncTestCaseParser_PT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 7)?;
					recog.base.enter_outer_alt(None, 7)?;
					{
					/*InvokeRule precisionTimeType*/
					recog.base.set_state(765);
					recog.precisionTimeType()?;

					}
				}

			FuncTestCaseParser_Precision_Timestamp |FuncTestCaseParser_PTs 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 8)?;
					recog.base.enter_outer_alt(None, 8)?;
					{
					/*InvokeRule precisionTimestampType*/
					recog.base.set_state(766);
					recog.precisionTimestampType()?;

					}
				}

			FuncTestCaseParser_Precision_Timestamp_TZ |FuncTestCaseParser_PTsTZ 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 9)?;
					recog.base.enter_outer_alt(None, 9)?;
					{
					/*InvokeRule precisionTimestampTZType*/
					recog.base.set_state(767);
					recog.precisionTimestampTZType()?;

					}
				}

			FuncTestCaseParser_List 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 10)?;
					recog.base.enter_outer_alt(None, 10)?;
					{
					/*InvokeRule listType*/
					recog.base.set_state(768);
					recog.listType()?;

					}
				}

			FuncTestCaseParser_Struct 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 11)?;
					recog.base.enter_outer_alt(None, 11)?;
					{
					/*InvokeRule structType*/
					recog.base.set_state(769);
					recog.structType()?;

					}
				}

			FuncTestCaseParser_Map 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 12)?;
					recog.base.enter_outer_alt(None, 12)?;
					{
					/*InvokeRule mapType*/
					recog.base.set_state(770);
					recog.mapType()?;

					}
				}

			FuncTestCaseParser_Func 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 13)?;
					recog.base.enter_outer_alt(None, 13)?;
					{
					/*InvokeRule funcType*/
					recog.base.set_state(771);
					recog.funcType()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- numericParameter ----------------
#[derive(Debug)]
pub enum NumericParameterContextAll<'input>{
	IntegerLiteralContext(IntegerLiteralContext<'input>),
Error(NumericParameterContext<'input>)
}
antlr4rust::tid!{NumericParameterContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for NumericParameterContextAll<'input>{}

impl<'input> FuncTestCaseParserContext<'input> for NumericParameterContextAll<'input>{}

impl<'input> Deref for NumericParameterContextAll<'input>{
	type Target = dyn NumericParameterContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use NumericParameterContextAll::*;
		match self{
			IntegerLiteralContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for NumericParameterContextAll<'input>{
    fn enter(&self, listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
}



pub type NumericParameterContext<'input> = BaseParserRuleContext<'input,NumericParameterContextExt<'input>>;

#[derive(Clone)]
pub struct NumericParameterContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for NumericParameterContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for NumericParameterContext<'input>{
}

impl<'input> CustomRuleContext<'input> for NumericParameterContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_numericParameter }
	//fn type_rule_index() -> usize where Self: Sized { RULE_numericParameter }
}
antlr4rust::tid!{NumericParameterContextExt<'a>}

impl<'input> NumericParameterContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<NumericParameterContextAll<'input>> {
		Rc::new(
		NumericParameterContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NumericParameterContextExt{

				ph:PhantomData
			}),
		)
		)
	}
}

pub trait NumericParameterContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<NumericParameterContextExt<'input>>{


}

impl<'input> NumericParameterContextAttrs<'input> for NumericParameterContext<'input>{}

pub type IntegerLiteralContext<'input> = BaseParserRuleContext<'input,IntegerLiteralContextExt<'input>>;

pub trait IntegerLiteralContextAttrs<'input>: FuncTestCaseParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token IntegerLiteral
	/// Returns `None` if there is no child corresponding to token IntegerLiteral
	fn IntegerLiteral(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
		self.get_token(FuncTestCaseParser_IntegerLiteral, 0)
	}
}

impl<'input> IntegerLiteralContextAttrs<'input> for IntegerLiteralContext<'input>{}

pub struct IntegerLiteralContextExt<'input>{
	base:NumericParameterContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{IntegerLiteralContextExt<'a>}

impl<'input> FuncTestCaseParserContext<'input> for IntegerLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for IntegerLiteralContext<'input>{
	fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_integerLiteral(self);
		Ok(())
	}
}

impl<'input> CustomRuleContext<'input> for IntegerLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_numericParameter }
	//fn type_rule_index() -> usize where Self: Sized { RULE_numericParameter }
}

impl<'input> Borrow<NumericParameterContextExt<'input>> for IntegerLiteralContext<'input>{
	fn borrow(&self) -> &NumericParameterContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<NumericParameterContextExt<'input>> for IntegerLiteralContext<'input>{
	fn borrow_mut(&mut self) -> &mut NumericParameterContextExt<'input> { &mut self.base }
}

impl<'input> NumericParameterContextAttrs<'input> for IntegerLiteralContext<'input> {}

impl<'input> IntegerLiteralContextExt<'input>{
	fn new(ctx: &dyn NumericParameterContextAttrs<'input>) -> Rc<NumericParameterContextAll<'input>>  {
		Rc::new(
			NumericParameterContextAll::IntegerLiteralContext(
				BaseParserRuleContext::copy_from(ctx,IntegerLiteralContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn numericParameter(&mut self,)
	-> Result<Rc<NumericParameterContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NumericParameterContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 160, RULE_numericParameter);
        let mut _localctx: Rc<NumericParameterContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let tmp = IntegerLiteralContextExt::new(&**_localctx);
			recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
			_localctx = tmp;
			{
			recog.base.set_state(774);
			recog.base.match_token(FuncTestCaseParser_IntegerLiteral,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- substraitError ----------------
pub type SubstraitErrorContextAll<'input> = SubstraitErrorContext<'input>;


pub type SubstraitErrorContext<'input> = BaseParserRuleContext<'input,SubstraitErrorContextExt<'input>>;

#[derive(Clone)]
pub struct SubstraitErrorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for SubstraitErrorContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for SubstraitErrorContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_substraitError(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_substraitError(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for SubstraitErrorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_substraitError }
	//fn type_rule_index() -> usize where Self: Sized { RULE_substraitError }
}
antlr4rust::tid!{SubstraitErrorContextExt<'a>}

impl<'input> SubstraitErrorContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<SubstraitErrorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SubstraitErrorContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait SubstraitErrorContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<SubstraitErrorContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ErrorResult
/// Returns `None` if there is no child corresponding to token ErrorResult
fn ErrorResult(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_ErrorResult, 0)
}
/// Retrieves first TerminalNode corresponding to token UndefineResult
/// Returns `None` if there is no child corresponding to token UndefineResult
fn UndefineResult(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_UndefineResult, 0)
}

}

impl<'input> SubstraitErrorContextAttrs<'input> for SubstraitErrorContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn substraitError(&mut self,)
	-> Result<Rc<SubstraitErrorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SubstraitErrorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 162, RULE_substraitError);
        let mut _localctx: Rc<SubstraitErrorContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(776);
			_la = recog.base.input.la(1);
			if { !(_la==FuncTestCaseParser_ErrorResult || _la==FuncTestCaseParser_UndefineResult) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- funcOption ----------------
pub type FuncOptionContextAll<'input> = FuncOptionContext<'input>;


pub type FuncOptionContext<'input> = BaseParserRuleContext<'input,FuncOptionContextExt<'input>>;

#[derive(Clone)]
pub struct FuncOptionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for FuncOptionContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for FuncOptionContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_funcOption(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_funcOption(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for FuncOptionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_funcOption }
	//fn type_rule_index() -> usize where Self: Sized { RULE_funcOption }
}
antlr4rust::tid!{FuncOptionContextExt<'a>}

impl<'input> FuncOptionContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<FuncOptionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FuncOptionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait FuncOptionContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<FuncOptionContextExt<'input>>{

fn optionName(&self) -> Option<Rc<OptionNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Colon
/// Returns `None` if there is no child corresponding to token Colon
fn Colon(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Colon, 0)
}
fn optionValue(&self) -> Option<Rc<OptionValueContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> FuncOptionContextAttrs<'input> for FuncOptionContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn funcOption(&mut self,)
	-> Result<Rc<FuncOptionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FuncOptionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 164, RULE_funcOption);
        let mut _localctx: Rc<FuncOptionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule optionName*/
			recog.base.set_state(778);
			recog.optionName()?;

			recog.base.set_state(779);
			recog.base.match_token(FuncTestCaseParser_Colon,&mut recog.err_handler)?;

			/*InvokeRule optionValue*/
			recog.base.set_state(780);
			recog.optionValue()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- optionName ----------------
pub type OptionNameContextAll<'input> = OptionNameContext<'input>;


pub type OptionNameContext<'input> = BaseParserRuleContext<'input,OptionNameContextExt<'input>>;

#[derive(Clone)]
pub struct OptionNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for OptionNameContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for OptionNameContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_optionName(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_optionName(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OptionNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_optionName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_optionName }
}
antlr4rust::tid!{OptionNameContextExt<'a>}

impl<'input> OptionNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OptionNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OptionNameContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OptionNameContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<OptionNameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Overflow
/// Returns `None` if there is no child corresponding to token Overflow
fn Overflow(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Overflow, 0)
}
/// Retrieves first TerminalNode corresponding to token Rounding
/// Returns `None` if there is no child corresponding to token Rounding
fn Rounding(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Rounding, 0)
}
/// Retrieves first TerminalNode corresponding to token NullHandling
/// Returns `None` if there is no child corresponding to token NullHandling
fn NullHandling(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_NullHandling, 0)
}
/// Retrieves first TerminalNode corresponding to token SpacesOnly
/// Returns `None` if there is no child corresponding to token SpacesOnly
fn SpacesOnly(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_SpacesOnly, 0)
}
/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Identifier, 0)
}

}

impl<'input> OptionNameContextAttrs<'input> for OptionNameContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn optionName(&mut self,)
	-> Result<Rc<OptionNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OptionNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 166, RULE_optionName);
        let mut _localctx: Rc<OptionNameContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(782);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & 12607488) != 0) || _la==FuncTestCaseParser_Identifier) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- optionValue ----------------
pub type OptionValueContextAll<'input> = OptionValueContext<'input>;


pub type OptionValueContext<'input> = BaseParserRuleContext<'input,OptionValueContextExt<'input>>;

#[derive(Clone)]
pub struct OptionValueContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for OptionValueContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for OptionValueContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_optionValue(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_optionValue(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for OptionValueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_optionValue }
	//fn type_rule_index() -> usize where Self: Sized { RULE_optionValue }
}
antlr4rust::tid!{OptionValueContextExt<'a>}

impl<'input> OptionValueContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<OptionValueContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OptionValueContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait OptionValueContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<OptionValueContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Error
/// Returns `None` if there is no child corresponding to token Error
fn Error(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Error, 0)
}
/// Retrieves first TerminalNode corresponding to token Saturate
/// Returns `None` if there is no child corresponding to token Saturate
fn Saturate(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Saturate, 0)
}
/// Retrieves first TerminalNode corresponding to token Silent
/// Returns `None` if there is no child corresponding to token Silent
fn Silent(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Silent, 0)
}
/// Retrieves first TerminalNode corresponding to token TieToEven
/// Returns `None` if there is no child corresponding to token TieToEven
fn TieToEven(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_TieToEven, 0)
}
/// Retrieves first TerminalNode corresponding to token NaN
/// Returns `None` if there is no child corresponding to token NaN
fn NaN(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_NaN, 0)
}
/// Retrieves first TerminalNode corresponding to token Truncate
/// Returns `None` if there is no child corresponding to token Truncate
fn Truncate(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Truncate, 0)
}
/// Retrieves first TerminalNode corresponding to token AcceptNulls
/// Returns `None` if there is no child corresponding to token AcceptNulls
fn AcceptNulls(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_AcceptNulls, 0)
}
/// Retrieves first TerminalNode corresponding to token IgnoreNulls
/// Returns `None` if there is no child corresponding to token IgnoreNulls
fn IgnoreNulls(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_IgnoreNulls, 0)
}
/// Retrieves first TerminalNode corresponding to token BooleanLiteral
/// Returns `None` if there is no child corresponding to token BooleanLiteral
fn BooleanLiteral(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_BooleanLiteral, 0)
}
/// Retrieves first TerminalNode corresponding to token NullLiteral
/// Returns `None` if there is no child corresponding to token NullLiteral
fn NullLiteral(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_NullLiteral, 0)
}
/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Identifier, 0)
}

}

impl<'input> OptionValueContextAttrs<'input> for OptionValueContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn optionValue(&mut self,)
	-> Result<Rc<OptionValueContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OptionValueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 168, RULE_optionValue);
        let mut _localctx: Rc<OptionValueContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(784);
			_la = recog.base.input.la(1);
			if { !(((((_la - 15)) & !0x3f) == 0 && ((1usize << (_la - 15)) & 2147492479) != 0) || _la==FuncTestCaseParser_Identifier) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- funcOptions ----------------
pub type FuncOptionsContextAll<'input> = FuncOptionsContext<'input>;


pub type FuncOptionsContext<'input> = BaseParserRuleContext<'input,FuncOptionsContextExt<'input>>;

#[derive(Clone)]
pub struct FuncOptionsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for FuncOptionsContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for FuncOptionsContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_funcOptions(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_funcOptions(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for FuncOptionsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_funcOptions }
	//fn type_rule_index() -> usize where Self: Sized { RULE_funcOptions }
}
antlr4rust::tid!{FuncOptionsContextExt<'a>}

impl<'input> FuncOptionsContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<FuncOptionsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FuncOptionsContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait FuncOptionsContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<FuncOptionsContextExt<'input>>{

fn funcOption_all(&self) ->  Vec<Rc<FuncOptionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn funcOption(&self, i: usize) -> Option<Rc<FuncOptionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Comma, i)
}

}

impl<'input> FuncOptionsContextAttrs<'input> for FuncOptionsContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn funcOptions(&mut self,)
	-> Result<Rc<FuncOptionsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FuncOptionsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 170, RULE_funcOptions);
        let mut _localctx: Rc<FuncOptionsContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule funcOption*/
			recog.base.set_state(786);
			recog.funcOption()?;

			recog.base.set_state(791);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==FuncTestCaseParser_Comma {
				{
				{
				recog.base.set_state(787);
				recog.base.match_token(FuncTestCaseParser_Comma,&mut recog.err_handler)?;

				/*InvokeRule funcOption*/
				recog.base.set_state(788);
				recog.funcOption()?;

				}
				}
				recog.base.set_state(793);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- nonReserved ----------------
pub type NonReservedContextAll<'input> = NonReservedContext<'input>;


pub type NonReservedContext<'input> = BaseParserRuleContext<'input,NonReservedContextExt<'input>>;

#[derive(Clone)]
pub struct NonReservedContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for NonReservedContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for NonReservedContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_nonReserved(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_nonReserved(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for NonReservedContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_nonReserved }
	//fn type_rule_index() -> usize where Self: Sized { RULE_nonReserved }
}
antlr4rust::tid!{NonReservedContextExt<'a>}

impl<'input> NonReservedContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<NonReservedContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NonReservedContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait NonReservedContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<NonReservedContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token And
/// Returns `None` if there is no child corresponding to token And
fn And(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_And, 0)
}
/// Retrieves first TerminalNode corresponding to token Or
/// Returns `None` if there is no child corresponding to token Or
fn Or(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Or, 0)
}
/// Retrieves first TerminalNode corresponding to token Truncate
/// Returns `None` if there is no child corresponding to token Truncate
fn Truncate(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Truncate, 0)
}

}

impl<'input> NonReservedContextAttrs<'input> for NonReservedContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn nonReserved(&mut self,)
	-> Result<Rc<NonReservedContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NonReservedContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 172, RULE_nonReserved);
        let mut _localctx: Rc<NonReservedContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(794);
			_la = recog.base.input.la(1);
			if { !(_la==FuncTestCaseParser_Truncate || _la==FuncTestCaseParser_And || _la==FuncTestCaseParser_Or) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- identifier ----------------
pub type IdentifierContextAll<'input> = IdentifierContext<'input>;


pub type IdentifierContext<'input> = BaseParserRuleContext<'input,IdentifierContextExt<'input>>;

#[derive(Clone)]
pub struct IdentifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> FuncTestCaseParserContext<'input> for IdentifierContext<'input>{}

impl<'input,'a> Listenable<dyn FuncTestCaseParserListener<'input> + 'a> for IdentifierContext<'input>{
		fn enter(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_identifier(self);
			Ok(())
		}fn exit(&self,listener: &mut (dyn FuncTestCaseParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_identifier(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input> CustomRuleContext<'input> for IdentifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = FuncTestCaseParserContextType;
	fn get_rule_index(&self) -> usize { RULE_identifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_identifier }
}
antlr4rust::tid!{IdentifierContextExt<'a>}

impl<'input> IdentifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn FuncTestCaseParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<IdentifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IdentifierContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait IdentifierContextAttrs<'input>: FuncTestCaseParserContext<'input> + BorrowMut<IdentifierContextExt<'input>>{

fn nonReserved(&self) -> Option<Rc<NonReservedContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,FuncTestCaseParserContextType>>> where Self:Sized{
	self.get_token(FuncTestCaseParser_Identifier, 0)
}

}

impl<'input> IdentifierContextAttrs<'input> for IdentifierContext<'input>{}

impl<'input, I> FuncTestCaseParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn identifier(&mut self,)
	-> Result<Rc<IdentifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IdentifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 174, RULE_identifier);
        let mut _localctx: Rc<IdentifierContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(798);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			FuncTestCaseParser_Truncate |FuncTestCaseParser_And |FuncTestCaseParser_Or 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule nonReserved*/
					recog.base.set_state(796);
					recog.nonReserved()?;

					}
				}

			FuncTestCaseParser_Identifier 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(797);
					recog.base.match_token(FuncTestCaseParser_Identifier,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
	lazy_static!{
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(&mut _serializedATN.iter()));
    static ref _decision_to_DFA: Arc<Vec<antlr4rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len() as i32;
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i,
            ).into())
        }
        Arc::new(dfa)
    };
	static ref _serializedATN: Vec<i32> = vec![
		4, 1, 127, 801, 2, 0, 7, 0, 2, 1, 7, 1, 2, 2, 7, 2, 2, 3, 7, 3, 2, 4, 
		7, 4, 2, 5, 7, 5, 2, 6, 7, 6, 2, 7, 7, 7, 2, 8, 7, 8, 2, 9, 7, 9, 2, 10, 
		7, 10, 2, 11, 7, 11, 2, 12, 7, 12, 2, 13, 7, 13, 2, 14, 7, 14, 2, 15, 
		7, 15, 2, 16, 7, 16, 2, 17, 7, 17, 2, 18, 7, 18, 2, 19, 7, 19, 2, 20, 
		7, 20, 2, 21, 7, 21, 2, 22, 7, 22, 2, 23, 7, 23, 2, 24, 7, 24, 2, 25, 
		7, 25, 2, 26, 7, 26, 2, 27, 7, 27, 2, 28, 7, 28, 2, 29, 7, 29, 2, 30, 
		7, 30, 2, 31, 7, 31, 2, 32, 7, 32, 2, 33, 7, 33, 2, 34, 7, 34, 2, 35, 
		7, 35, 2, 36, 7, 36, 2, 37, 7, 37, 2, 38, 7, 38, 2, 39, 7, 39, 2, 40, 
		7, 40, 2, 41, 7, 41, 2, 42, 7, 42, 2, 43, 7, 43, 2, 44, 7, 44, 2, 45, 
		7, 45, 2, 46, 7, 46, 2, 47, 7, 47, 2, 48, 7, 48, 2, 49, 7, 49, 2, 50, 
		7, 50, 2, 51, 7, 51, 2, 52, 7, 52, 2, 53, 7, 53, 2, 54, 7, 54, 2, 55, 
		7, 55, 2, 56, 7, 56, 2, 57, 7, 57, 2, 58, 7, 58, 2, 59, 7, 59, 2, 60, 
		7, 60, 2, 61, 7, 61, 2, 62, 7, 62, 2, 63, 7, 63, 2, 64, 7, 64, 2, 65, 
		7, 65, 2, 66, 7, 66, 2, 67, 7, 67, 2, 68, 7, 68, 2, 69, 7, 69, 2, 70, 
		7, 70, 2, 71, 7, 71, 2, 72, 7, 72, 2, 73, 7, 73, 2, 74, 7, 74, 2, 75, 
		7, 75, 2, 76, 7, 76, 2, 77, 7, 77, 2, 78, 7, 78, 2, 79, 7, 79, 2, 80, 
		7, 80, 2, 81, 7, 81, 2, 82, 7, 82, 2, 83, 7, 83, 2, 84, 7, 84, 2, 85, 
		7, 85, 2, 86, 7, 86, 2, 87, 7, 87, 1, 0, 1, 0, 4, 0, 179, 8, 0, 11, 0, 
		12, 0, 180, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 5, 1, 188, 8, 1, 10, 1, 12, 
		1, 191, 9, 1, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 3, 1, 3, 1, 3, 1, 3, 1, 
		3, 1, 4, 1, 4, 1, 4, 1, 4, 1, 4, 1, 5, 1, 5, 1, 6, 1, 6, 1, 6, 1, 6, 1, 
		6, 1, 6, 1, 6, 1, 6, 3, 6, 218, 8, 6, 1, 6, 1, 6, 1, 6, 1, 7, 3, 7, 224, 
		8, 7, 1, 7, 4, 7, 227, 8, 7, 11, 7, 12, 7, 228, 1, 7, 3, 7, 232, 8, 7, 
		1, 7, 4, 7, 235, 8, 7, 11, 7, 12, 7, 236, 3, 7, 239, 8, 7, 1, 8, 1, 8, 
		1, 8, 5, 8, 244, 8, 8, 10, 8, 12, 8, 247, 9, 8, 1, 9, 1, 9, 3, 9, 251, 
		8, 9, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 
		10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 
		10, 1, 10, 1, 10, 1, 10, 1, 10, 3, 10, 277, 8, 10, 1, 11, 1, 11, 1, 11, 
		1, 11, 1, 11, 3, 11, 284, 8, 11, 1, 11, 1, 11, 1, 11, 1, 12, 1, 12, 1, 
		12, 1, 12, 3, 12, 293, 8, 12, 1, 12, 1, 12, 1, 12, 1, 12, 1, 12, 1, 12, 
		3, 12, 301, 8, 12, 1, 12, 1, 12, 1, 12, 1, 12, 1, 12, 1, 12, 1, 12, 3, 
		12, 310, 8, 12, 1, 13, 1, 13, 1, 13, 1, 13, 1, 13, 1, 13, 5, 13, 318, 
		8, 13, 10, 13, 12, 13, 321, 9, 13, 1, 13, 1, 13, 1, 13, 1, 13, 1, 14, 
		1, 14, 1, 14, 1, 14, 5, 14, 331, 8, 14, 10, 14, 12, 14, 334, 9, 14, 3, 
		14, 336, 8, 14, 1, 14, 1, 14, 1, 15, 1, 15, 1, 15, 1, 15, 1, 16, 1, 16, 
		1, 16, 1, 16, 5, 16, 348, 8, 16, 10, 16, 12, 16, 351, 9, 16, 3, 16, 353, 
		8, 16, 1, 16, 1, 16, 1, 17, 1, 17, 1, 17, 1, 17, 1, 17, 1, 17, 1, 17, 
		1, 17, 1, 17, 1, 17, 1, 17, 3, 17, 368, 8, 17, 1, 18, 1, 18, 1, 18, 5, 
		18, 373, 8, 18, 10, 18, 12, 18, 376, 9, 18, 1, 19, 1, 19, 1, 19, 5, 19, 
		381, 8, 19, 10, 19, 12, 19, 384, 9, 19, 1, 20, 1, 20, 1, 20, 1, 20, 3, 
		20, 390, 8, 20, 1, 21, 1, 21, 1, 21, 1, 21, 3, 21, 396, 8, 21, 1, 22, 
		1, 22, 1, 22, 3, 22, 401, 8, 22, 1, 23, 1, 23, 1, 24, 1, 24, 1, 24, 1, 
		24, 1, 25, 1, 25, 1, 25, 1, 25, 1, 26, 1, 26, 1, 26, 1, 26, 1, 27, 1, 
		27, 1, 27, 1, 27, 1, 28, 1, 28, 1, 28, 1, 28, 1, 29, 1, 29, 1, 29, 1, 
		29, 1, 30, 1, 30, 1, 30, 1, 30, 1, 31, 1, 31, 1, 31, 1, 31, 1, 32, 1, 
		32, 1, 32, 1, 32, 1, 33, 1, 33, 1, 33, 1, 33, 1, 34, 1, 34, 1, 34, 1, 
		34, 1, 35, 1, 35, 1, 35, 1, 35, 1, 36, 1, 36, 1, 36, 1, 36, 1, 37, 1, 
		37, 1, 37, 1, 37, 1, 38, 1, 38, 1, 38, 1, 38, 1, 39, 1, 39, 1, 39, 1, 
		39, 1, 40, 1, 40, 1, 40, 1, 40, 1, 41, 1, 41, 1, 41, 1, 41, 1, 42, 1, 
		42, 1, 42, 1, 42, 1, 43, 1, 43, 1, 43, 1, 43, 1, 44, 1, 44, 1, 44, 1, 
		44, 1, 45, 1, 45, 1, 45, 1, 45, 1, 45, 1, 46, 1, 46, 1, 46, 1, 46, 1, 
		47, 1, 47, 1, 47, 1, 47, 5, 47, 502, 8, 47, 10, 47, 12, 47, 505, 9, 47, 
		3, 47, 507, 8, 47, 1, 47, 1, 47, 1, 48, 1, 48, 1, 48, 1, 48, 5, 48, 515, 
		8, 48, 10, 48, 12, 48, 518, 9, 48, 3, 48, 520, 8, 48, 1, 48, 1, 48, 1, 
		49, 1, 49, 1, 49, 1, 49, 5, 49, 528, 8, 49, 10, 49, 12, 49, 531, 9, 49, 
		3, 49, 533, 8, 49, 1, 49, 1, 49, 1, 50, 1, 50, 1, 50, 1, 50, 1, 51, 1, 
		51, 1, 51, 1, 51, 3, 51, 545, 8, 51, 1, 52, 1, 52, 1, 52, 1, 52, 1, 52, 
		1, 52, 1, 53, 1, 53, 1, 53, 1, 53, 1, 53, 4, 53, 558, 8, 53, 11, 53, 12, 
		53, 559, 1, 53, 3, 53, 563, 8, 53, 1, 54, 1, 54, 1, 54, 1, 54, 1, 54, 
		1, 55, 1, 55, 3, 55, 572, 8, 55, 1, 56, 1, 56, 1, 56, 1, 56, 1, 56, 1, 
		56, 1, 56, 1, 56, 1, 56, 3, 56, 583, 8, 56, 1, 56, 3, 56, 586, 8, 56, 
		1, 57, 1, 57, 1, 57, 3, 57, 591, 8, 57, 1, 58, 1, 58, 3, 58, 595, 8, 58, 
		1, 59, 1, 59, 3, 59, 599, 8, 59, 1, 60, 1, 60, 3, 60, 603, 8, 60, 1, 61, 
		1, 61, 3, 61, 607, 8, 61, 1, 62, 1, 62, 3, 62, 611, 8, 62, 1, 63, 1, 63, 
		3, 63, 615, 8, 63, 1, 64, 1, 64, 3, 64, 619, 8, 64, 1, 65, 1, 65, 3, 65, 
		623, 8, 65, 1, 65, 1, 65, 1, 65, 1, 65, 3, 65, 629, 8, 65, 1, 66, 1, 66, 
		3, 66, 633, 8, 66, 1, 66, 1, 66, 1, 66, 1, 66, 3, 66, 639, 8, 66, 1, 67, 
		1, 67, 3, 67, 643, 8, 67, 1, 67, 1, 67, 1, 67, 1, 67, 1, 68, 1, 68, 3, 
		68, 651, 8, 68, 1, 68, 1, 68, 1, 68, 1, 68, 1, 69, 1, 69, 3, 69, 659, 
		8, 69, 1, 69, 1, 69, 1, 69, 1, 69, 1, 70, 1, 70, 3, 70, 667, 8, 70, 1, 
		70, 1, 70, 1, 70, 1, 70, 1, 70, 1, 70, 3, 70, 675, 8, 70, 1, 71, 1, 71, 
		3, 71, 679, 8, 71, 1, 71, 1, 71, 1, 71, 1, 71, 1, 72, 1, 72, 3, 72, 687, 
		8, 72, 1, 72, 1, 72, 1, 72, 1, 72, 1, 73, 1, 73, 3, 73, 695, 8, 73, 1, 
		73, 1, 73, 1, 73, 1, 73, 1, 74, 1, 74, 3, 74, 703, 8, 74, 1, 74, 1, 74, 
		1, 74, 1, 74, 1, 75, 1, 75, 3, 75, 711, 8, 75, 1, 75, 1, 75, 1, 75, 1, 
		75, 5, 75, 717, 8, 75, 10, 75, 12, 75, 720, 9, 75, 3, 75, 722, 8, 75, 
		1, 75, 1, 75, 1, 76, 1, 76, 3, 76, 728, 8, 76, 1, 76, 1, 76, 1, 76, 1, 
		76, 1, 76, 1, 76, 1, 77, 1, 77, 3, 77, 738, 8, 77, 1, 77, 1, 77, 1, 77, 
		1, 77, 1, 77, 1, 77, 1, 78, 1, 78, 1, 78, 1, 78, 1, 78, 5, 78, 751, 8, 
		78, 10, 78, 12, 78, 754, 9, 78, 1, 78, 1, 78, 3, 78, 758, 8, 78, 1, 79, 
		1, 79, 1, 79, 1, 79, 1, 79, 1, 79, 1, 79, 1, 79, 1, 79, 1, 79, 1, 79, 
		1, 79, 1, 79, 3, 79, 773, 8, 79, 1, 80, 1, 80, 1, 81, 1, 81, 1, 82, 1, 
		82, 1, 82, 1, 82, 1, 83, 1, 83, 1, 84, 1, 84, 1, 85, 1, 85, 1, 85, 5, 
		85, 790, 8, 85, 10, 85, 12, 85, 793, 9, 85, 1, 86, 1, 86, 1, 87, 1, 87, 
		3, 87, 799, 8, 87, 1, 87, 0, 0, 88, 0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 
		20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40, 42, 44, 46, 48, 50, 52, 54, 
		56, 58, 60, 62, 64, 66, 68, 70, 72, 74, 76, 78, 80, 82, 84, 86, 88, 90, 
		92, 94, 96, 98, 100, 102, 104, 106, 108, 110, 112, 114, 116, 118, 120, 
		122, 124, 126, 128, 130, 132, 134, 136, 138, 140, 142, 144, 146, 148, 
		150, 152, 154, 156, 158, 160, 162, 164, 166, 168, 170, 172, 174, 0, 21, 
		1, 0, 3, 4, 2, 0, 19, 19, 27, 27, 2, 0, 58, 58, 84, 84, 2, 0, 65, 65, 
		85, 85, 2, 0, 66, 66, 86, 86, 1, 0, 59, 62, 1, 0, 63, 64, 2, 0, 68, 68, 
		87, 87, 2, 0, 69, 69, 88, 88, 2, 0, 70, 70, 89, 89, 2, 0, 76, 76, 94, 
		94, 2, 0, 77, 77, 95, 95, 2, 0, 78, 78, 96, 96, 2, 0, 72, 72, 90, 90, 
		2, 0, 73, 73, 91, 91, 2, 0, 74, 74, 92, 92, 2, 0, 75, 75, 93, 93, 1, 0, 
		11, 12, 3, 0, 13, 14, 22, 23, 126, 126, 5, 0, 15, 21, 24, 24, 28, 28, 
		46, 46, 126, 126, 2, 0, 24, 24, 121, 122, 834, 0, 176, 1, 0, 0, 0, 2, 
		184, 1, 0, 0, 0, 4, 192, 1, 0, 0, 0, 6, 197, 1, 0, 0, 0, 8, 202, 1, 0, 
		0, 0, 10, 207, 1, 0, 0, 0, 12, 209, 1, 0, 0, 0, 14, 238, 1, 0, 0, 0, 16, 
		240, 1, 0, 0, 0, 18, 250, 1, 0, 0, 0, 20, 276, 1, 0, 0, 0, 22, 278, 1, 
		0, 0, 0, 24, 309, 1, 0, 0, 0, 26, 311, 1, 0, 0, 0, 28, 326, 1, 0, 0, 0, 
		30, 339, 1, 0, 0, 0, 32, 343, 1, 0, 0, 0, 34, 367, 1, 0, 0, 0, 36, 369, 
		1, 0, 0, 0, 38, 377, 1, 0, 0, 0, 40, 389, 1, 0, 0, 0, 42, 395, 1, 0, 0, 
		0, 44, 400, 1, 0, 0, 0, 46, 402, 1, 0, 0, 0, 48, 404, 1, 0, 0, 0, 50, 
		408, 1, 0, 0, 0, 52, 412, 1, 0, 0, 0, 54, 416, 1, 0, 0, 0, 56, 420, 1, 
		0, 0, 0, 58, 424, 1, 0, 0, 0, 60, 428, 1, 0, 0, 0, 62, 432, 1, 0, 0, 0, 
		64, 436, 1, 0, 0, 0, 66, 440, 1, 0, 0, 0, 68, 444, 1, 0, 0, 0, 70, 448, 
		1, 0, 0, 0, 72, 452, 1, 0, 0, 0, 74, 456, 1, 0, 0, 0, 76, 460, 1, 0, 0, 
		0, 78, 464, 1, 0, 0, 0, 80, 468, 1, 0, 0, 0, 82, 472, 1, 0, 0, 0, 84, 
		476, 1, 0, 0, 0, 86, 480, 1, 0, 0, 0, 88, 484, 1, 0, 0, 0, 90, 488, 1, 
		0, 0, 0, 92, 493, 1, 0, 0, 0, 94, 497, 1, 0, 0, 0, 96, 510, 1, 0, 0, 0, 
		98, 523, 1, 0, 0, 0, 100, 536, 1, 0, 0, 0, 102, 544, 1, 0, 0, 0, 104, 
		546, 1, 0, 0, 0, 106, 562, 1, 0, 0, 0, 108, 564, 1, 0, 0, 0, 110, 571, 
		1, 0, 0, 0, 112, 585, 1, 0, 0, 0, 114, 587, 1, 0, 0, 0, 116, 592, 1, 0, 
		0, 0, 118, 596, 1, 0, 0, 0, 120, 600, 1, 0, 0, 0, 122, 604, 1, 0, 0, 0, 
		124, 608, 1, 0, 0, 0, 126, 612, 1, 0, 0, 0, 128, 616, 1, 0, 0, 0, 130, 
		620, 1, 0, 0, 0, 132, 630, 1, 0, 0, 0, 134, 640, 1, 0, 0, 0, 136, 648, 
		1, 0, 0, 0, 138, 656, 1, 0, 0, 0, 140, 664, 1, 0, 0, 0, 142, 676, 1, 0, 
		0, 0, 144, 684, 1, 0, 0, 0, 146, 692, 1, 0, 0, 0, 148, 700, 1, 0, 0, 0, 
		150, 708, 1, 0, 0, 0, 152, 725, 1, 0, 0, 0, 154, 735, 1, 0, 0, 0, 156, 
		757, 1, 0, 0, 0, 158, 772, 1, 0, 0, 0, 160, 774, 1, 0, 0, 0, 162, 776, 
		1, 0, 0, 0, 164, 778, 1, 0, 0, 0, 166, 782, 1, 0, 0, 0, 168, 784, 1, 0, 
		0, 0, 170, 786, 1, 0, 0, 0, 172, 794, 1, 0, 0, 0, 174, 798, 1, 0, 0, 0, 
		176, 178, 3, 2, 1, 0, 177, 179, 3, 14, 7, 0, 178, 177, 1, 0, 0, 0, 179, 
		180, 1, 0, 0, 0, 180, 178, 1, 0, 0, 0, 180, 181, 1, 0, 0, 0, 181, 182, 
		1, 0, 0, 0, 182, 183, 5, 0, 0, 1, 183, 1, 1, 0, 0, 0, 184, 185, 3, 4, 
		2, 0, 185, 189, 3, 6, 3, 0, 186, 188, 3, 8, 4, 0, 187, 186, 1, 0, 0, 0, 
		188, 191, 1, 0, 0, 0, 189, 187, 1, 0, 0, 0, 189, 190, 1, 0, 0, 0, 190, 
		3, 1, 0, 0, 0, 191, 189, 1, 0, 0, 0, 192, 193, 5, 2, 0, 0, 193, 194, 7, 
		0, 0, 0, 194, 195, 5, 117, 0, 0, 195, 196, 5, 8, 0, 0, 196, 5, 1, 0, 0, 
		0, 197, 198, 5, 2, 0, 0, 198, 199, 5, 5, 0, 0, 199, 200, 5, 117, 0, 0, 
		200, 201, 5, 7, 0, 0, 201, 7, 1, 0, 0, 0, 202, 203, 5, 2, 0, 0, 203, 204, 
		5, 6, 0, 0, 204, 205, 5, 117, 0, 0, 205, 206, 5, 7, 0, 0, 206, 9, 1, 0, 
		0, 0, 207, 208, 5, 9, 0, 0, 208, 11, 1, 0, 0, 0, 209, 210, 3, 174, 87, 
		0, 210, 211, 5, 112, 0, 0, 211, 212, 3, 16, 8, 0, 212, 217, 5, 113, 0, 
		0, 213, 214, 5, 114, 0, 0, 214, 215, 3, 170, 85, 0, 215, 216, 5, 115, 
		0, 0, 216, 218, 1, 0, 0, 0, 217, 213, 1, 0, 0, 0, 217, 218, 1, 0, 0, 0, 
		218, 219, 1, 0, 0, 0, 219, 220, 5, 105, 0, 0, 220, 221, 3, 18, 9, 0, 221, 
		13, 1, 0, 0, 0, 222, 224, 3, 10, 5, 0, 223, 222, 1, 0, 0, 0, 223, 224, 
		1, 0, 0, 0, 224, 226, 1, 0, 0, 0, 225, 227, 3, 12, 6, 0, 226, 225, 1, 
		0, 0, 0, 227, 228, 1, 0, 0, 0, 228, 226, 1, 0, 0, 0, 228, 229, 1, 0, 0, 
		0, 229, 239, 1, 0, 0, 0, 230, 232, 3, 10, 5, 0, 231, 230, 1, 0, 0, 0, 
		231, 232, 1, 0, 0, 0, 232, 234, 1, 0, 0, 0, 233, 235, 3, 22, 11, 0, 234, 
		233, 1, 0, 0, 0, 235, 236, 1, 0, 0, 0, 236, 234, 1, 0, 0, 0, 236, 237, 
		1, 0, 0, 0, 237, 239, 1, 0, 0, 0, 238, 223, 1, 0, 0, 0, 238, 231, 1, 0, 
		0, 0, 239, 15, 1, 0, 0, 0, 240, 245, 3, 20, 10, 0, 241, 242, 5, 116, 0, 
		0, 242, 244, 3, 20, 10, 0, 243, 241, 1, 0, 0, 0, 244, 247, 1, 0, 0, 0, 
		245, 243, 1, 0, 0, 0, 245, 246, 1, 0, 0, 0, 246, 17, 1, 0, 0, 0, 247, 
		245, 1, 0, 0, 0, 248, 251, 3, 20, 10, 0, 249, 251, 3, 162, 81, 0, 250, 
		248, 1, 0, 0, 0, 250, 249, 1, 0, 0, 0, 251, 19, 1, 0, 0, 0, 252, 277, 
		3, 48, 24, 0, 253, 277, 3, 92, 46, 0, 254, 277, 3, 50, 25, 0, 255, 277, 
		3, 52, 26, 0, 256, 277, 3, 56, 28, 0, 257, 277, 3, 58, 29, 0, 258, 277, 
		3, 54, 27, 0, 259, 277, 3, 60, 30, 0, 260, 277, 3, 62, 31, 0, 261, 277, 
		3, 64, 32, 0, 262, 277, 3, 66, 33, 0, 263, 277, 3, 68, 34, 0, 264, 277, 
		3, 70, 35, 0, 265, 277, 3, 72, 36, 0, 266, 277, 3, 74, 37, 0, 267, 277, 
		3, 76, 38, 0, 268, 277, 3, 78, 39, 0, 269, 277, 3, 80, 40, 0, 270, 277, 
		3, 82, 41, 0, 271, 277, 3, 84, 42, 0, 272, 277, 3, 86, 43, 0, 273, 277, 
		3, 88, 44, 0, 274, 277, 3, 90, 45, 0, 275, 277, 5, 126, 0, 0, 276, 252, 
		1, 0, 0, 0, 276, 253, 1, 0, 0, 0, 276, 254, 1, 0, 0, 0, 276, 255, 1, 0, 
		0, 0, 276, 256, 1, 0, 0, 0, 276, 257, 1, 0, 0, 0, 276, 258, 1, 0, 0, 0, 
		276, 259, 1, 0, 0, 0, 276, 260, 1, 0, 0, 0, 276, 261, 1, 0, 0, 0, 276, 
		262, 1, 0, 0, 0, 276, 263, 1, 0, 0, 0, 276, 264, 1, 0, 0, 0, 276, 265, 
		1, 0, 0, 0, 276, 266, 1, 0, 0, 0, 276, 267, 1, 0, 0, 0, 276, 268, 1, 0, 
		0, 0, 276, 269, 1, 0, 0, 0, 276, 270, 1, 0, 0, 0, 276, 271, 1, 0, 0, 0, 
		276, 272, 1, 0, 0, 0, 276, 273, 1, 0, 0, 0, 276, 274, 1, 0, 0, 0, 276, 
		275, 1, 0, 0, 0, 277, 21, 1, 0, 0, 0, 278, 283, 3, 24, 12, 0, 279, 280, 
		5, 114, 0, 0, 280, 281, 3, 170, 85, 0, 281, 282, 5, 115, 0, 0, 282, 284, 
		1, 0, 0, 0, 283, 279, 1, 0, 0, 0, 283, 284, 1, 0, 0, 0, 284, 285, 1, 0, 
		0, 0, 285, 286, 5, 105, 0, 0, 286, 287, 3, 18, 9, 0, 287, 23, 1, 0, 0, 
		0, 288, 289, 3, 26, 13, 0, 289, 290, 3, 174, 87, 0, 290, 292, 5, 112, 
		0, 0, 291, 293, 3, 36, 18, 0, 292, 291, 1, 0, 0, 0, 292, 293, 1, 0, 0, 
		0, 293, 294, 1, 0, 0, 0, 294, 295, 5, 113, 0, 0, 295, 310, 1, 0, 0, 0, 
		296, 297, 3, 28, 14, 0, 297, 298, 3, 174, 87, 0, 298, 300, 5, 112, 0, 
		0, 299, 301, 3, 38, 19, 0, 300, 299, 1, 0, 0, 0, 300, 301, 1, 0, 0, 0, 
		301, 302, 1, 0, 0, 0, 302, 303, 5, 113, 0, 0, 303, 310, 1, 0, 0, 0, 304, 
		305, 3, 174, 87, 0, 305, 306, 5, 112, 0, 0, 306, 307, 3, 30, 15, 0, 307, 
		308, 5, 113, 0, 0, 308, 310, 1, 0, 0, 0, 309, 288, 1, 0, 0, 0, 309, 296, 
		1, 0, 0, 0, 309, 304, 1, 0, 0, 0, 310, 25, 1, 0, 0, 0, 311, 312, 5, 10, 
		0, 0, 312, 313, 5, 126, 0, 0, 313, 314, 5, 112, 0, 0, 314, 319, 3, 110, 
		55, 0, 315, 316, 5, 116, 0, 0, 316, 318, 3, 110, 55, 0, 317, 315, 1, 0, 
		0, 0, 318, 321, 1, 0, 0, 0, 319, 317, 1, 0, 0, 0, 319, 320, 1, 0, 0, 0, 
		320, 322, 1, 0, 0, 0, 321, 319, 1, 0, 0, 0, 322, 323, 5, 113, 0, 0, 323, 
		324, 5, 105, 0, 0, 324, 325, 3, 28, 14, 0, 325, 27, 1, 0, 0, 0, 326, 335, 
		5, 112, 0, 0, 327, 332, 3, 32, 16, 0, 328, 329, 5, 116, 0, 0, 329, 331, 
		3, 32, 16, 0, 330, 328, 1, 0, 0, 0, 331, 334, 1, 0, 0, 0, 332, 330, 1, 
		0, 0, 0, 332, 333, 1, 0, 0, 0, 333, 336, 1, 0, 0, 0, 334, 332, 1, 0, 0, 
		0, 335, 327, 1, 0, 0, 0, 335, 336, 1, 0, 0, 0, 336, 337, 1, 0, 0, 0, 337, 
		338, 5, 113, 0, 0, 338, 29, 1, 0, 0, 0, 339, 340, 3, 32, 16, 0, 340, 341, 
		5, 99, 0, 0, 341, 342, 3, 110, 55, 0, 342, 31, 1, 0, 0, 0, 343, 352, 5, 
		112, 0, 0, 344, 349, 3, 34, 17, 0, 345, 346, 5, 116, 0, 0, 346, 348, 3, 
		34, 17, 0, 347, 345, 1, 0, 0, 0, 348, 351, 1, 0, 0, 0, 349, 347, 1, 0, 
		0, 0, 349, 350, 1, 0, 0, 0, 350, 353, 1, 0, 0, 0, 351, 349, 1, 0, 0, 0, 
		352, 344, 1, 0, 0, 0, 352, 353, 1, 0, 0, 0, 353, 354, 1, 0, 0, 0, 354, 
		355, 5, 113, 0, 0, 355, 33, 1, 0, 0, 0, 356, 368, 5, 46, 0, 0, 357, 368, 
		3, 44, 22, 0, 358, 368, 5, 28, 0, 0, 359, 368, 5, 47, 0, 0, 360, 368, 
		5, 32, 0, 0, 361, 368, 5, 31, 0, 0, 362, 368, 5, 30, 0, 0, 363, 368, 5, 
		29, 0, 0, 364, 368, 5, 43, 0, 0, 365, 368, 5, 44, 0, 0, 366, 368, 5, 45, 
		0, 0, 367, 356, 1, 0, 0, 0, 367, 357, 1, 0, 0, 0, 367, 358, 1, 0, 0, 0, 
		367, 359, 1, 0, 0, 0, 367, 360, 1, 0, 0, 0, 367, 361, 1, 0, 0, 0, 367, 
		362, 1, 0, 0, 0, 367, 363, 1, 0, 0, 0, 367, 364, 1, 0, 0, 0, 367, 365, 
		1, 0, 0, 0, 367, 366, 1, 0, 0, 0, 368, 35, 1, 0, 0, 0, 369, 374, 3, 40, 
		20, 0, 370, 371, 5, 116, 0, 0, 371, 373, 3, 40, 20, 0, 372, 370, 1, 0, 
		0, 0, 373, 376, 1, 0, 0, 0, 374, 372, 1, 0, 0, 0, 374, 375, 1, 0, 0, 0, 
		375, 37, 1, 0, 0, 0, 376, 374, 1, 0, 0, 0, 377, 382, 3, 42, 21, 0, 378, 
		379, 5, 116, 0, 0, 379, 381, 3, 42, 21, 0, 380, 378, 1, 0, 0, 0, 381, 
		384, 1, 0, 0, 0, 382, 380, 1, 0, 0, 0, 382, 383, 1, 0, 0, 0, 383, 39, 
		1, 0, 0, 0, 384, 382, 1, 0, 0, 0, 385, 386, 5, 126, 0, 0, 386, 387, 5, 
		120, 0, 0, 387, 390, 5, 51, 0, 0, 388, 390, 3, 20, 10, 0, 389, 385, 1, 
		0, 0, 0, 389, 388, 1, 0, 0, 0, 390, 41, 1, 0, 0, 0, 391, 392, 5, 51, 0, 
		0, 392, 393, 5, 99, 0, 0, 393, 396, 3, 110, 55, 0, 394, 396, 3, 20, 10, 
		0, 395, 391, 1, 0, 0, 0, 395, 394, 1, 0, 0, 0, 396, 43, 1, 0, 0, 0, 397, 
		401, 5, 26, 0, 0, 398, 401, 5, 25, 0, 0, 399, 401, 3, 46, 23, 0, 400, 
		397, 1, 0, 0, 0, 400, 398, 1, 0, 0, 0, 400, 399, 1, 0, 0, 0, 401, 45, 
		1, 0, 0, 0, 402, 403, 7, 1, 0, 0, 403, 47, 1, 0, 0, 0, 404, 405, 5, 46, 
		0, 0, 405, 406, 5, 99, 0, 0, 406, 407, 3, 110, 55, 0, 407, 49, 1, 0, 0, 
		0, 408, 409, 5, 25, 0, 0, 409, 410, 5, 99, 0, 0, 410, 411, 3, 122, 61, 
		0, 411, 51, 1, 0, 0, 0, 412, 413, 3, 44, 22, 0, 413, 414, 5, 99, 0, 0, 
		414, 415, 3, 124, 62, 0, 415, 53, 1, 0, 0, 0, 416, 417, 3, 44, 22, 0, 
		417, 418, 5, 99, 0, 0, 418, 419, 3, 140, 70, 0, 419, 55, 1, 0, 0, 0, 420, 
		421, 5, 28, 0, 0, 421, 422, 5, 99, 0, 0, 422, 423, 3, 116, 58, 0, 423, 
		57, 1, 0, 0, 0, 424, 425, 5, 47, 0, 0, 425, 426, 5, 99, 0, 0, 426, 427, 
		3, 118, 59, 0, 427, 59, 1, 0, 0, 0, 428, 429, 5, 32, 0, 0, 429, 430, 5, 
		99, 0, 0, 430, 431, 3, 126, 63, 0, 431, 61, 1, 0, 0, 0, 432, 433, 5, 43, 
		0, 0, 433, 434, 5, 99, 0, 0, 434, 435, 3, 128, 64, 0, 435, 63, 1, 0, 0, 
		0, 436, 437, 5, 44, 0, 0, 437, 438, 5, 99, 0, 0, 438, 439, 3, 130, 65, 
		0, 439, 65, 1, 0, 0, 0, 440, 441, 5, 45, 0, 0, 441, 442, 5, 99, 0, 0, 
		442, 443, 3, 132, 66, 0, 443, 67, 1, 0, 0, 0, 444, 445, 5, 47, 0, 0, 445, 
		446, 5, 99, 0, 0, 446, 447, 3, 134, 67, 0, 447, 69, 1, 0, 0, 0, 448, 449, 
		5, 47, 0, 0, 449, 450, 5, 99, 0, 0, 450, 451, 3, 136, 68, 0, 451, 71, 
		1, 0, 0, 0, 452, 453, 5, 47, 0, 0, 453, 454, 5, 99, 0, 0, 454, 455, 3, 
		138, 69, 0, 455, 73, 1, 0, 0, 0, 456, 457, 5, 31, 0, 0, 457, 458, 5, 99, 
		0, 0, 458, 459, 3, 142, 71, 0, 459, 75, 1, 0, 0, 0, 460, 461, 5, 30, 0, 
		0, 461, 462, 5, 99, 0, 0, 462, 463, 3, 144, 72, 0, 463, 77, 1, 0, 0, 0, 
		464, 465, 5, 29, 0, 0, 465, 466, 5, 99, 0, 0, 466, 467, 3, 146, 73, 0, 
		467, 79, 1, 0, 0, 0, 468, 469, 3, 94, 47, 0, 469, 470, 5, 99, 0, 0, 470, 
		471, 3, 148, 74, 0, 471, 81, 1, 0, 0, 0, 472, 473, 3, 96, 48, 0, 473, 
		474, 5, 99, 0, 0, 474, 475, 3, 150, 75, 0, 475, 83, 1, 0, 0, 0, 476, 477, 
		3, 98, 49, 0, 477, 478, 5, 99, 0, 0, 478, 479, 3, 152, 76, 0, 479, 85, 
		1, 0, 0, 0, 480, 481, 3, 96, 48, 0, 481, 482, 5, 99, 0, 0, 482, 483, 3, 
		114, 57, 0, 483, 87, 1, 0, 0, 0, 484, 485, 3, 104, 52, 0, 485, 486, 5, 
		99, 0, 0, 486, 487, 3, 154, 77, 0, 487, 89, 1, 0, 0, 0, 488, 489, 3, 174, 
		87, 0, 489, 490, 5, 112, 0, 0, 490, 491, 3, 16, 8, 0, 491, 492, 5, 113, 
		0, 0, 492, 91, 1, 0, 0, 0, 493, 494, 5, 126, 0, 0, 494, 495, 5, 99, 0, 
		0, 495, 496, 5, 48, 0, 0, 496, 93, 1, 0, 0, 0, 497, 506, 5, 114, 0, 0, 
		498, 503, 3, 102, 51, 0, 499, 500, 5, 116, 0, 0, 500, 502, 3, 102, 51, 
		0, 501, 499, 1, 0, 0, 0, 502, 505, 1, 0, 0, 0, 503, 501, 1, 0, 0, 0, 503, 
		504, 1, 0, 0, 0, 504, 507, 1, 0, 0, 0, 505, 503, 1, 0, 0, 0, 506, 498, 
		1, 0, 0, 0, 506, 507, 1, 0, 0, 0, 507, 508, 1, 0, 0, 0, 508, 509, 5, 115, 
		0, 0, 509, 95, 1, 0, 0, 0, 510, 519, 5, 112, 0, 0, 511, 516, 3, 102, 51, 
		0, 512, 513, 5, 116, 0, 0, 513, 515, 3, 102, 51, 0, 514, 512, 1, 0, 0, 
		0, 515, 518, 1, 0, 0, 0, 516, 514, 1, 0, 0, 0, 516, 517, 1, 0, 0, 0, 517, 
		520, 1, 0, 0, 0, 518, 516, 1, 0, 0, 0, 519, 511, 1, 0, 0, 0, 519, 520, 
		1, 0, 0, 0, 520, 521, 1, 0, 0, 0, 521, 522, 5, 113, 0, 0, 522, 97, 1, 
		0, 0, 0, 523, 532, 5, 49, 0, 0, 524, 529, 3, 100, 50, 0, 525, 526, 5, 
		116, 0, 0, 526, 528, 3, 100, 50, 0, 527, 525, 1, 0, 0, 0, 528, 531, 1, 
		0, 0, 0, 529, 527, 1, 0, 0, 0, 529, 530, 1, 0, 0, 0, 530, 533, 1, 0, 0, 
		0, 531, 529, 1, 0, 0, 0, 532, 524, 1, 0, 0, 0, 532, 533, 1, 0, 0, 0, 533, 
		534, 1, 0, 0, 0, 534, 535, 5, 50, 0, 0, 535, 99, 1, 0, 0, 0, 536, 537, 
		3, 102, 51, 0, 537, 538, 5, 117, 0, 0, 538, 539, 3, 102, 51, 0, 539, 101, 
		1, 0, 0, 0, 540, 545, 3, 34, 17, 0, 541, 545, 3, 94, 47, 0, 542, 545, 
		3, 96, 48, 0, 543, 545, 3, 98, 49, 0, 544, 540, 1, 0, 0, 0, 544, 541, 
		1, 0, 0, 0, 544, 542, 1, 0, 0, 0, 544, 543, 1, 0, 0, 0, 545, 103, 1, 0, 
		0, 0, 546, 547, 5, 112, 0, 0, 547, 548, 3, 106, 53, 0, 548, 549, 5, 124, 
		0, 0, 549, 550, 3, 108, 54, 0, 550, 551, 5, 113, 0, 0, 551, 105, 1, 0, 
		0, 0, 552, 563, 5, 126, 0, 0, 553, 554, 5, 112, 0, 0, 554, 557, 5, 126, 
		0, 0, 555, 556, 5, 116, 0, 0, 556, 558, 5, 126, 0, 0, 557, 555, 1, 0, 
		0, 0, 558, 559, 1, 0, 0, 0, 559, 557, 1, 0, 0, 0, 559, 560, 1, 0, 0, 0, 
		560, 561, 1, 0, 0, 0, 561, 563, 5, 113, 0, 0, 562, 552, 1, 0, 0, 0, 562, 
		553, 1, 0, 0, 0, 563, 107, 1, 0, 0, 0, 564, 565, 3, 174, 87, 0, 565, 566, 
		5, 112, 0, 0, 566, 567, 3, 16, 8, 0, 567, 568, 5, 113, 0, 0, 568, 109, 
		1, 0, 0, 0, 569, 572, 3, 112, 56, 0, 570, 572, 3, 158, 79, 0, 571, 569, 
		1, 0, 0, 0, 571, 570, 1, 0, 0, 0, 572, 111, 1, 0, 0, 0, 573, 586, 3, 116, 
		58, 0, 574, 586, 3, 122, 61, 0, 575, 586, 3, 124, 62, 0, 576, 586, 3, 
		118, 59, 0, 577, 586, 3, 120, 60, 0, 578, 586, 3, 126, 63, 0, 579, 586, 
		3, 128, 64, 0, 580, 582, 5, 71, 0, 0, 581, 583, 5, 118, 0, 0, 582, 581, 
		1, 0, 0, 0, 582, 583, 1, 0, 0, 0, 583, 586, 1, 0, 0, 0, 584, 586, 3, 114, 
		57, 0, 585, 573, 1, 0, 0, 0, 585, 574, 1, 0, 0, 0, 585, 575, 1, 0, 0, 
		0, 585, 576, 1, 0, 0, 0, 585, 577, 1, 0, 0, 0, 585, 578, 1, 0, 0, 0, 585, 
		579, 1, 0, 0, 0, 585, 580, 1, 0, 0, 0, 585, 584, 1, 0, 0, 0, 586, 113, 
		1, 0, 0, 0, 587, 588, 5, 83, 0, 0, 588, 590, 5, 126, 0, 0, 589, 591, 5, 
		118, 0, 0, 590, 589, 1, 0, 0, 0, 590, 591, 1, 0, 0, 0, 591, 115, 1, 0, 
		0, 0, 592, 594, 7, 2, 0, 0, 593, 595, 5, 118, 0, 0, 594, 593, 1, 0, 0, 
		0, 594, 595, 1, 0, 0, 0, 595, 117, 1, 0, 0, 0, 596, 598, 7, 3, 0, 0, 597, 
		599, 5, 118, 0, 0, 598, 597, 1, 0, 0, 0, 598, 599, 1, 0, 0, 0, 599, 119, 
		1, 0, 0, 0, 600, 602, 7, 4, 0, 0, 601, 603, 5, 118, 0, 0, 602, 601, 1, 
		0, 0, 0, 602, 603, 1, 0, 0, 0, 603, 121, 1, 0, 0, 0, 604, 606, 7, 5, 0, 
		0, 605, 607, 5, 118, 0, 0, 606, 605, 1, 0, 0, 0, 606, 607, 1, 0, 0, 0, 
		607, 123, 1, 0, 0, 0, 608, 610, 7, 6, 0, 0, 609, 611, 5, 118, 0, 0, 610, 
		609, 1, 0, 0, 0, 610, 611, 1, 0, 0, 0, 611, 125, 1, 0, 0, 0, 612, 614, 
		5, 67, 0, 0, 613, 615, 5, 118, 0, 0, 614, 613, 1, 0, 0, 0, 614, 615, 1, 
		0, 0, 0, 615, 127, 1, 0, 0, 0, 616, 618, 7, 7, 0, 0, 617, 619, 5, 118, 
		0, 0, 618, 617, 1, 0, 0, 0, 618, 619, 1, 0, 0, 0, 619, 129, 1, 0, 0, 0, 
		620, 622, 7, 8, 0, 0, 621, 623, 5, 118, 0, 0, 622, 621, 1, 0, 0, 0, 622, 
		623, 1, 0, 0, 0, 623, 628, 1, 0, 0, 0, 624, 625, 5, 41, 0, 0, 625, 626, 
		3, 160, 80, 0, 626, 627, 5, 42, 0, 0, 627, 629, 1, 0, 0, 0, 628, 624, 
		1, 0, 0, 0, 628, 629, 1, 0, 0, 0, 629, 131, 1, 0, 0, 0, 630, 632, 7, 9, 
		0, 0, 631, 633, 5, 118, 0, 0, 632, 631, 1, 0, 0, 0, 632, 633, 1, 0, 0, 
		0, 633, 638, 1, 0, 0, 0, 634, 635, 5, 41, 0, 0, 635, 636, 3, 160, 80, 
		0, 636, 637, 5, 42, 0, 0, 637, 639, 1, 0, 0, 0, 638, 634, 1, 0, 0, 0, 
		638, 639, 1, 0, 0, 0, 639, 133, 1, 0, 0, 0, 640, 642, 7, 10, 0, 0, 641, 
		643, 5, 118, 0, 0, 642, 641, 1, 0, 0, 0, 642, 643, 1, 0, 0, 0, 643, 644, 
		1, 0, 0, 0, 644, 645, 5, 41, 0, 0, 645, 646, 3, 160, 80, 0, 646, 647, 
		5, 42, 0, 0, 647, 135, 1, 0, 0, 0, 648, 650, 7, 11, 0, 0, 649, 651, 5, 
		118, 0, 0, 650, 649, 1, 0, 0, 0, 650, 651, 1, 0, 0, 0, 651, 652, 1, 0, 
		0, 0, 652, 653, 5, 41, 0, 0, 653, 654, 3, 160, 80, 0, 654, 655, 5, 42, 
		0, 0, 655, 137, 1, 0, 0, 0, 656, 658, 7, 12, 0, 0, 657, 659, 5, 118, 0, 
		0, 658, 657, 1, 0, 0, 0, 658, 659, 1, 0, 0, 0, 659, 660, 1, 0, 0, 0, 660, 
		661, 5, 41, 0, 0, 661, 662, 3, 160, 80, 0, 662, 663, 5, 42, 0, 0, 663, 
		139, 1, 0, 0, 0, 664, 666, 7, 13, 0, 0, 665, 667, 5, 118, 0, 0, 666, 665, 
		1, 0, 0, 0, 666, 667, 1, 0, 0, 0, 667, 674, 1, 0, 0, 0, 668, 669, 5, 41, 
		0, 0, 669, 670, 3, 160, 80, 0, 670, 671, 5, 116, 0, 0, 671, 672, 3, 160, 
		80, 0, 672, 673, 5, 42, 0, 0, 673, 675, 1, 0, 0, 0, 674, 668, 1, 0, 0, 
		0, 674, 675, 1, 0, 0, 0, 675, 141, 1, 0, 0, 0, 676, 678, 7, 14, 0, 0, 
		677, 679, 5, 118, 0, 0, 678, 677, 1, 0, 0, 0, 678, 679, 1, 0, 0, 0, 679, 
		680, 1, 0, 0, 0, 680, 681, 5, 41, 0, 0, 681, 682, 3, 160, 80, 0, 682, 
		683, 5, 42, 0, 0, 683, 143, 1, 0, 0, 0, 684, 686, 7, 15, 0, 0, 685, 687, 
		5, 118, 0, 0, 686, 685, 1, 0, 0, 0, 686, 687, 1, 0, 0, 0, 687, 688, 1, 
		0, 0, 0, 688, 689, 5, 41, 0, 0, 689, 690, 3, 160, 80, 0, 690, 691, 5, 
		42, 0, 0, 691, 145, 1, 0, 0, 0, 692, 694, 7, 16, 0, 0, 693, 695, 5, 118, 
		0, 0, 694, 693, 1, 0, 0, 0, 694, 695, 1, 0, 0, 0, 695, 696, 1, 0, 0, 0, 
		696, 697, 5, 41, 0, 0, 697, 698, 3, 160, 80, 0, 698, 699, 5, 42, 0, 0, 
		699, 147, 1, 0, 0, 0, 700, 702, 5, 81, 0, 0, 701, 703, 5, 118, 0, 0, 702, 
		701, 1, 0, 0, 0, 702, 703, 1, 0, 0, 0, 703, 704, 1, 0, 0, 0, 704, 705, 
		5, 41, 0, 0, 705, 706, 3, 110, 55, 0, 706, 707, 5, 42, 0, 0, 707, 149, 
		1, 0, 0, 0, 708, 710, 5, 79, 0, 0, 709, 711, 5, 118, 0, 0, 710, 709, 1, 
		0, 0, 0, 710, 711, 1, 0, 0, 0, 711, 712, 1, 0, 0, 0, 712, 721, 5, 41, 
		0, 0, 713, 718, 3, 110, 55, 0, 714, 715, 5, 116, 0, 0, 715, 717, 3, 110, 
		55, 0, 716, 714, 1, 0, 0, 0, 717, 720, 1, 0, 0, 0, 718, 716, 1, 0, 0, 
		0, 718, 719, 1, 0, 0, 0, 719, 722, 1, 0, 0, 0, 720, 718, 1, 0, 0, 0, 721, 
		713, 1, 0, 0, 0, 721, 722, 1, 0, 0, 0, 722, 723, 1, 0, 0, 0, 723, 724, 
		5, 42, 0, 0, 724, 151, 1, 0, 0, 0, 725, 727, 5, 82, 0, 0, 726, 728, 5, 
		118, 0, 0, 727, 726, 1, 0, 0, 0, 727, 728, 1, 0, 0, 0, 728, 729, 1, 0, 
		0, 0, 729, 730, 5, 41, 0, 0, 730, 731, 3, 110, 55, 0, 731, 732, 5, 116, 
		0, 0, 732, 733, 3, 110, 55, 0, 733, 734, 5, 42, 0, 0, 734, 153, 1, 0, 
		0, 0, 735, 737, 5, 57, 0, 0, 736, 738, 5, 118, 0, 0, 737, 736, 1, 0, 0, 
		0, 737, 738, 1, 0, 0, 0, 738, 739, 1, 0, 0, 0, 739, 740, 5, 41, 0, 0, 
		740, 741, 3, 156, 78, 0, 741, 742, 5, 124, 0, 0, 742, 743, 3, 110, 55, 
		0, 743, 744, 5, 42, 0, 0, 744, 155, 1, 0, 0, 0, 745, 758, 3, 110, 55, 
		0, 746, 747, 5, 112, 0, 0, 747, 752, 3, 110, 55, 0, 748, 749, 5, 116, 
		0, 0, 749, 751, 3, 110, 55, 0, 750, 748, 1, 0, 0, 0, 751, 754, 1, 0, 0, 
		0, 752, 750, 1, 0, 0, 0, 752, 753, 1, 0, 0, 0, 753, 755, 1, 0, 0, 0, 754, 
		752, 1, 0, 0, 0, 755, 756, 5, 113, 0, 0, 756, 758, 1, 0, 0, 0, 757, 745, 
		1, 0, 0, 0, 757, 746, 1, 0, 0, 0, 758, 157, 1, 0, 0, 0, 759, 773, 3, 134, 
		67, 0, 760, 773, 3, 136, 68, 0, 761, 773, 3, 138, 69, 0, 762, 773, 3, 
		140, 70, 0, 763, 773, 3, 130, 65, 0, 764, 773, 3, 132, 66, 0, 765, 773, 
		3, 142, 71, 0, 766, 773, 3, 144, 72, 0, 767, 773, 3, 146, 73, 0, 768, 
		773, 3, 148, 74, 0, 769, 773, 3, 150, 75, 0, 770, 773, 3, 152, 76, 0, 
		771, 773, 3, 154, 77, 0, 772, 759, 1, 0, 0, 0, 772, 760, 1, 0, 0, 0, 772, 
		761, 1, 0, 0, 0, 772, 762, 1, 0, 0, 0, 772, 763, 1, 0, 0, 0, 772, 764, 
		1, 0, 0, 0, 772, 765, 1, 0, 0, 0, 772, 766, 1, 0, 0, 0, 772, 767, 1, 0, 
		0, 0, 772, 768, 1, 0, 0, 0, 772, 769, 1, 0, 0, 0, 772, 770, 1, 0, 0, 0, 
		772, 771, 1, 0, 0, 0, 773, 159, 1, 0, 0, 0, 774, 775, 5, 25, 0, 0, 775, 
		161, 1, 0, 0, 0, 776, 777, 7, 17, 0, 0, 777, 163, 1, 0, 0, 0, 778, 779, 
		3, 166, 83, 0, 779, 780, 5, 117, 0, 0, 780, 781, 3, 168, 84, 0, 781, 165, 
		1, 0, 0, 0, 782, 783, 7, 18, 0, 0, 783, 167, 1, 0, 0, 0, 784, 785, 7, 
		19, 0, 0, 785, 169, 1, 0, 0, 0, 786, 791, 3, 164, 82, 0, 787, 788, 5, 
		116, 0, 0, 788, 790, 3, 164, 82, 0, 789, 787, 1, 0, 0, 0, 790, 793, 1, 
		0, 0, 0, 791, 789, 1, 0, 0, 0, 791, 792, 1, 0, 0, 0, 792, 171, 1, 0, 0, 
		0, 793, 791, 1, 0, 0, 0, 794, 795, 7, 20, 0, 0, 795, 173, 1, 0, 0, 0, 
		796, 799, 3, 172, 86, 0, 797, 799, 5, 126, 0, 0, 798, 796, 1, 0, 0, 0, 
		798, 797, 1, 0, 0, 0, 799, 175, 1, 0, 0, 0, 69, 180, 189, 217, 223, 228, 
		231, 236, 238, 245, 250, 276, 283, 292, 300, 309, 319, 332, 335, 349, 
		352, 367, 374, 382, 389, 395, 400, 503, 506, 516, 519, 529, 532, 544, 
		559, 562, 571, 582, 585, 590, 594, 598, 602, 606, 610, 614, 618, 622, 
		628, 632, 638, 642, 650, 658, 666, 674, 678, 686, 694, 702, 710, 718, 
		721, 727, 737, 752, 757, 772, 791, 798
	];
}
