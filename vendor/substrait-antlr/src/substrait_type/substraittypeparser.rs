// SPDX-License-Identifier: Apache-2.0
#![allow(clippy::all)]
#![allow(unused_parens)]
#![cfg_attr(rustfmt, rustfmt_skip)]
// Generated from SubstraitType.g4 by ANTLR 4.13.2
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
use super::substraittypelistener::*;
use super::substraittypevisitor::*;

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

		pub const SubstraitType_LineComment:i32=1; 
		pub const SubstraitType_BlockComment:i32=2; 
		pub const SubstraitType_Whitespace:i32=3; 
		pub const SubstraitType_If:i32=4; 
		pub const SubstraitType_Then:i32=5; 
		pub const SubstraitType_Else:i32=6; 
		pub const SubstraitType_Func:i32=7; 
		pub const SubstraitType_Boolean:i32=8; 
		pub const SubstraitType_I8:i32=9; 
		pub const SubstraitType_I16:i32=10; 
		pub const SubstraitType_I32:i32=11; 
		pub const SubstraitType_I64:i32=12; 
		pub const SubstraitType_FP32:i32=13; 
		pub const SubstraitType_FP64:i32=14; 
		pub const SubstraitType_String:i32=15; 
		pub const SubstraitType_Binary:i32=16; 
		pub const SubstraitType_Date:i32=17; 
		pub const SubstraitType_Interval_Year:i32=18; 
		pub const SubstraitType_Interval_Day:i32=19; 
		pub const SubstraitType_Interval_Compound:i32=20; 
		pub const SubstraitType_UUID:i32=21; 
		pub const SubstraitType_Decimal:i32=22; 
		pub const SubstraitType_Precision_Time:i32=23; 
		pub const SubstraitType_Precision_Timestamp:i32=24; 
		pub const SubstraitType_Precision_Timestamp_TZ:i32=25; 
		pub const SubstraitType_FixedChar:i32=26; 
		pub const SubstraitType_VarChar:i32=27; 
		pub const SubstraitType_FixedBinary:i32=28; 
		pub const SubstraitType_Struct:i32=29; 
		pub const SubstraitType_NStruct:i32=30; 
		pub const SubstraitType_List:i32=31; 
		pub const SubstraitType_Map:i32=32; 
		pub const SubstraitType_UserDefined:i32=33; 
		pub const SubstraitType_Bool:i32=34; 
		pub const SubstraitType_Str:i32=35; 
		pub const SubstraitType_VBin:i32=36; 
		pub const SubstraitType_IYear:i32=37; 
		pub const SubstraitType_IDay:i32=38; 
		pub const SubstraitType_ICompound:i32=39; 
		pub const SubstraitType_Dec:i32=40; 
		pub const SubstraitType_PT:i32=41; 
		pub const SubstraitType_PTs:i32=42; 
		pub const SubstraitType_PTsTZ:i32=43; 
		pub const SubstraitType_FChar:i32=44; 
		pub const SubstraitType_VChar:i32=45; 
		pub const SubstraitType_FBin:i32=46; 
		pub const SubstraitType_Any:i32=47; 
		pub const SubstraitType_AnyVar:i32=48; 
		pub const SubstraitType_DoubleColon:i32=49; 
		pub const SubstraitType_Plus:i32=50; 
		pub const SubstraitType_Minus:i32=51; 
		pub const SubstraitType_Asterisk:i32=52; 
		pub const SubstraitType_ForwardSlash:i32=53; 
		pub const SubstraitType_Percent:i32=54; 
		pub const SubstraitType_Eq:i32=55; 
		pub const SubstraitType_Ne:i32=56; 
		pub const SubstraitType_Gte:i32=57; 
		pub const SubstraitType_Lte:i32=58; 
		pub const SubstraitType_Gt:i32=59; 
		pub const SubstraitType_Lt:i32=60; 
		pub const SubstraitType_Bang:i32=61; 
		pub const SubstraitType_OAngleBracket:i32=62; 
		pub const SubstraitType_CAngleBracket:i32=63; 
		pub const SubstraitType_OParen:i32=64; 
		pub const SubstraitType_CParen:i32=65; 
		pub const SubstraitType_OBracket:i32=66; 
		pub const SubstraitType_CBracket:i32=67; 
		pub const SubstraitType_Comma:i32=68; 
		pub const SubstraitType_Colon:i32=69; 
		pub const SubstraitType_QMark:i32=70; 
		pub const SubstraitType_Hash:i32=71; 
		pub const SubstraitType_Dot:i32=72; 
		pub const SubstraitType_And:i32=73; 
		pub const SubstraitType_Or:i32=74; 
		pub const SubstraitType_Assign:i32=75; 
		pub const SubstraitType_Arrow:i32=76; 
		pub const SubstraitType_Number:i32=77; 
		pub const SubstraitType_Identifier:i32=78; 
		pub const SubstraitType_Newline:i32=79;
	pub const SubstraitType_EOF:i32=EOF;
	pub const RULE_startRule:usize = 0; 
	pub const RULE_typeStatement:usize = 1; 
	pub const RULE_scalarType:usize = 2; 
	pub const RULE_parameterizedType:usize = 3; 
	pub const RULE_funcParams:usize = 4; 
	pub const RULE_numericParameter:usize = 5; 
	pub const RULE_anyType:usize = 6; 
	pub const RULE_typeDef:usize = 7; 
	pub const RULE_expr:usize = 8;
	pub const ruleNames: [&'static str; 9] =  [
		"startRule", "typeStatement", "scalarType", "parameterizedType", "funcParams", 
		"numericParameter", "anyType", "typeDef", "expr"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;77] = [
		None, None, None, None, Some("'IF'"), Some("'THEN'"), Some("'ELSE'"), 
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
		Some("'>'"), Some("'<'"), Some("'!'"), None, None, Some("'('"), Some("')'"), 
		Some("'['"), Some("']'"), Some("','"), Some("':'"), Some("'?'"), Some("'#'"), 
		Some("'.'"), Some("'AND'"), Some("'OR'"), Some("':='"), Some("'->'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;80]  = [
		None, Some("LineComment"), Some("BlockComment"), Some("Whitespace"), Some("If"), 
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
		Some("Bang"), Some("OAngleBracket"), Some("CAngleBracket"), Some("OParen"), 
		Some("CParen"), Some("OBracket"), Some("CBracket"), Some("Comma"), Some("Colon"), 
		Some("QMark"), Some("Hash"), Some("Dot"), Some("And"), Some("Or"), Some("Assign"), 
		Some("Arrow"), Some("Number"), Some("Identifier"), Some("Newline")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,SubstraitTypeParserExt<'input>, I, SubstraitTypeParserContextType , dyn SubstraitTypeListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type SubstraitTypeTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, SubstraitTypeParserContextType , dyn SubstraitTypeListener<'input> + 'a>;

/// Parser for SubstraitType grammar
pub struct SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: Box<dyn ErrorStrategy<'input,BaseParserType<'input,I> > >,
}

impl<'input, I> SubstraitTypeParser<'input, I>
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
				SubstraitTypeParserExt{
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

impl<'input, I> SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for SubstraitTypeParser
pub trait SubstraitTypeParserContext<'input>:
	for<'x> Listenable<dyn SubstraitTypeListener<'input> + 'x > + 
	for<'x> Visitable<dyn SubstraitTypeVisitor<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=SubstraitTypeParserContextType>
{}

antlr4rust::coerce_from!{ 'input : SubstraitTypeParserContext<'input> }

impl<'input, 'x, T> VisitableDyn<T> for dyn SubstraitTypeParserContext<'input> + 'input
where
    T: SubstraitTypeVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn SubstraitTypeVisitor<'input> + 'x))
    }
}

impl<'input> SubstraitTypeParserContext<'input> for TerminalNode<'input,SubstraitTypeParserContextType> {}
impl<'input> SubstraitTypeParserContext<'input> for ErrorNode<'input,SubstraitTypeParserContextType> {}

antlr4rust::tid! { impl<'input> TidAble<'input> for dyn SubstraitTypeParserContext<'input> + 'input }

antlr4rust::tid! { impl<'input> TidAble<'input> for dyn SubstraitTypeListener<'input> + 'input }

pub struct SubstraitTypeParserContextType;
antlr4rust::tid!{SubstraitTypeParserContextType}

impl<'input> ParserNodeType<'input> for SubstraitTypeParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn SubstraitTypeParserContext<'input> + 'input;
}

impl<'input, I> Deref for SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I> DerefMut for SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct SubstraitTypeParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> SubstraitTypeParserExt<'input>{
}
antlr4rust::tid! { SubstraitTypeParserExt<'a> }

impl<'input> TokenAware<'input> for SubstraitTypeParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for SubstraitTypeParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for SubstraitTypeParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "SubstraitType.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
	fn sempred(_localctx: Option<&(dyn SubstraitTypeParserContext<'input> + 'input)>, rule_index: i32, pred_index: i32,
			   recog:&mut BaseParserType<'input,I>
	)->bool{
		match rule_index {
					8 => SubstraitTypeParser::<'input,I>::expr_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
			_ => true
		}
	}
}

impl<'input, I> SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	fn expr_sempred(_localctx: Option<&ExprContext<'input>>, pred_index:i32,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				0=>{
					recog.precpred(None, 9)
				}
				1=>{
					recog.precpred(None, 8)
				}
				2=>{
					recog.precpred(None, 7)
				}
				3=>{
					recog.precpred(None, 6)
				}
				4=>{
					recog.precpred(None, 5)
				}
				5=>{
					recog.precpred(None, 4)
				}
				6=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
}
//------------------- startRule ----------------
pub type StartRuleContextAll<'input> = StartRuleContext<'input>;


pub type StartRuleContext<'input> = BaseParserRuleContext<'input,StartRuleContextExt<'input>>;

#[derive(Clone)]
pub struct StartRuleContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitTypeParserContext<'input> for StartRuleContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for StartRuleContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_startRule(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_startRule(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for StartRuleContext<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_startRule(self);
	}
}

impl<'input> CustomRuleContext<'input> for StartRuleContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_startRule }
	//fn type_rule_index() -> usize where Self: Sized { RULE_startRule }
}
antlr4rust::tid!{StartRuleContextExt<'a>}

impl<'input> StartRuleContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<StartRuleContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StartRuleContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait StartRuleContextAttrs<'input>: SubstraitTypeParserContext<'input> + BorrowMut<StartRuleContextExt<'input>>{

fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
	self.get_token(SubstraitType_EOF, 0)
}

}

impl<'input> StartRuleContextAttrs<'input> for StartRuleContext<'input>{}

impl<'input, I> SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn startRule(&mut self,)
	-> Result<Rc<StartRuleContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StartRuleContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_startRule);
        let mut _localctx: Rc<StartRuleContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule expr*/
			recog.base.set_state(18);
			recog.expr_rec(0)?;

			recog.base.set_state(19);
			recog.base.match_token(SubstraitType_EOF,&mut recog.err_handler)?;

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
//------------------- typeStatement ----------------
pub type TypeStatementContextAll<'input> = TypeStatementContext<'input>;


pub type TypeStatementContext<'input> = BaseParserRuleContext<'input,TypeStatementContextExt<'input>>;

#[derive(Clone)]
pub struct TypeStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitTypeParserContext<'input> for TypeStatementContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for TypeStatementContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_typeStatement(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_typeStatement(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for TypeStatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_typeStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for TypeStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typeStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typeStatement }
}
antlr4rust::tid!{TypeStatementContextExt<'a>}

impl<'input> TypeStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<TypeStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypeStatementContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait TypeStatementContextAttrs<'input>: SubstraitTypeParserContext<'input> + BorrowMut<TypeStatementContextExt<'input>>{

fn typeDef(&self) -> Option<Rc<TypeDefContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
	self.get_token(SubstraitType_EOF, 0)
}

}

impl<'input> TypeStatementContextAttrs<'input> for TypeStatementContext<'input>{}

impl<'input, I> SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn typeStatement(&mut self,)
	-> Result<Rc<TypeStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TypeStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_typeStatement);
        let mut _localctx: Rc<TypeStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule typeDef*/
			recog.base.set_state(21);
			recog.typeDef()?;

			recog.base.set_state(22);
			recog.base.match_token(SubstraitType_EOF,&mut recog.err_handler)?;

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
	I64Context(I64Context<'input>),
	BinaryContext(BinaryContext<'input>),
	Fp64Context(Fp64Context<'input>),
	I32Context(I32Context<'input>),
	Fp32Context(Fp32Context<'input>),
	IntervalYearContext(IntervalYearContext<'input>),
	UuidContext(UuidContext<'input>),
	I8Context(I8Context<'input>),
	I16Context(I16Context<'input>),
Error(ScalarTypeContext<'input>)
}
antlr4rust::tid!{ScalarTypeContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for ScalarTypeContextAll<'input>{}

impl<'input> SubstraitTypeParserContext<'input> for ScalarTypeContextAll<'input>{}

impl<'input> Deref for ScalarTypeContextAll<'input>{
	type Target = dyn ScalarTypeContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use ScalarTypeContextAll::*;
		match self{
			DateContext(inner) => inner,
			BooleanContext(inner) => inner,
			StringContext(inner) => inner,
			I64Context(inner) => inner,
			BinaryContext(inner) => inner,
			Fp64Context(inner) => inner,
			I32Context(inner) => inner,
			Fp32Context(inner) => inner,
			IntervalYearContext(inner) => inner,
			UuidContext(inner) => inner,
			I8Context(inner) => inner,
			I16Context(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for ScalarTypeContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for ScalarTypeContextAll<'input>{
    fn enter(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
}



pub type ScalarTypeContext<'input> = BaseParserRuleContext<'input,ScalarTypeContextExt<'input>>;

#[derive(Clone)]
pub struct ScalarTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitTypeParserContext<'input> for ScalarTypeContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for ScalarTypeContext<'input>{
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for ScalarTypeContext<'input>{
}

impl<'input> CustomRuleContext<'input> for ScalarTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_scalarType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_scalarType }
}
antlr4rust::tid!{ScalarTypeContextExt<'a>}

impl<'input> ScalarTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ScalarTypeContextAll<'input>> {
		Rc::new(
		ScalarTypeContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ScalarTypeContextExt{

				ph:PhantomData
			}),
		)
		)
	}
}

pub trait ScalarTypeContextAttrs<'input>: SubstraitTypeParserContext<'input> + BorrowMut<ScalarTypeContextExt<'input>>{


}

impl<'input> ScalarTypeContextAttrs<'input> for ScalarTypeContext<'input>{}

pub type DateContext<'input> = BaseParserRuleContext<'input,DateContextExt<'input>>;

pub trait DateContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token Date
	/// Returns `None` if there is no child corresponding to token Date
	fn Date(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Date, 0)
	}
}

impl<'input> DateContextAttrs<'input> for DateContext<'input>{}

pub struct DateContextExt<'input>{
	base:ScalarTypeContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{DateContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for DateContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for DateContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_date(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_date(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for DateContext<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_date(self);
	}
}

impl<'input> CustomRuleContext<'input> for DateContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
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

pub trait BooleanContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token Boolean
	/// Returns `None` if there is no child corresponding to token Boolean
	fn Boolean(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Boolean, 0)
	}
}

impl<'input> BooleanContextAttrs<'input> for BooleanContext<'input>{}

pub struct BooleanContextExt<'input>{
	base:ScalarTypeContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{BooleanContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for BooleanContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for BooleanContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_boolean(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_boolean(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for BooleanContext<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_boolean(self);
	}
}

impl<'input> CustomRuleContext<'input> for BooleanContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
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

pub trait StringContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token String
	/// Returns `None` if there is no child corresponding to token String
	fn String(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_String, 0)
	}
}

impl<'input> StringContextAttrs<'input> for StringContext<'input>{}

pub struct StringContextExt<'input>{
	base:ScalarTypeContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{StringContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for StringContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for StringContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_string(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_string(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for StringContext<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_string(self);
	}
}

impl<'input> CustomRuleContext<'input> for StringContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
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

pub type I64Context<'input> = BaseParserRuleContext<'input,I64ContextExt<'input>>;

pub trait I64ContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token I64
	/// Returns `None` if there is no child corresponding to token I64
	fn I64(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_I64, 0)
	}
}

impl<'input> I64ContextAttrs<'input> for I64Context<'input>{}

pub struct I64ContextExt<'input>{
	base:ScalarTypeContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{I64ContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for I64Context<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for I64Context<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_i64(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_i64(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for I64Context<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_i64(self);
	}
}

impl<'input> CustomRuleContext<'input> for I64ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_scalarType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_scalarType }
}

impl<'input> Borrow<ScalarTypeContextExt<'input>> for I64Context<'input>{
	fn borrow(&self) -> &ScalarTypeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ScalarTypeContextExt<'input>> for I64Context<'input>{
	fn borrow_mut(&mut self) -> &mut ScalarTypeContextExt<'input> { &mut self.base }
}

impl<'input> ScalarTypeContextAttrs<'input> for I64Context<'input> {}

impl<'input> I64ContextExt<'input>{
	fn new(ctx: &dyn ScalarTypeContextAttrs<'input>) -> Rc<ScalarTypeContextAll<'input>>  {
		Rc::new(
			ScalarTypeContextAll::I64Context(
				BaseParserRuleContext::copy_from(ctx,I64ContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type BinaryContext<'input> = BaseParserRuleContext<'input,BinaryContextExt<'input>>;

pub trait BinaryContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token Binary
	/// Returns `None` if there is no child corresponding to token Binary
	fn Binary(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Binary, 0)
	}
}

impl<'input> BinaryContextAttrs<'input> for BinaryContext<'input>{}

pub struct BinaryContextExt<'input>{
	base:ScalarTypeContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{BinaryContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for BinaryContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for BinaryContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_binary(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_binary(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for BinaryContext<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_binary(self);
	}
}

impl<'input> CustomRuleContext<'input> for BinaryContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
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

pub type Fp64Context<'input> = BaseParserRuleContext<'input,Fp64ContextExt<'input>>;

pub trait Fp64ContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token FP64
	/// Returns `None` if there is no child corresponding to token FP64
	fn FP64(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_FP64, 0)
	}
}

impl<'input> Fp64ContextAttrs<'input> for Fp64Context<'input>{}

pub struct Fp64ContextExt<'input>{
	base:ScalarTypeContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{Fp64ContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for Fp64Context<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for Fp64Context<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_fp64(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_fp64(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for Fp64Context<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_fp64(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fp64ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_scalarType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_scalarType }
}

impl<'input> Borrow<ScalarTypeContextExt<'input>> for Fp64Context<'input>{
	fn borrow(&self) -> &ScalarTypeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ScalarTypeContextExt<'input>> for Fp64Context<'input>{
	fn borrow_mut(&mut self) -> &mut ScalarTypeContextExt<'input> { &mut self.base }
}

impl<'input> ScalarTypeContextAttrs<'input> for Fp64Context<'input> {}

impl<'input> Fp64ContextExt<'input>{
	fn new(ctx: &dyn ScalarTypeContextAttrs<'input>) -> Rc<ScalarTypeContextAll<'input>>  {
		Rc::new(
			ScalarTypeContextAll::Fp64Context(
				BaseParserRuleContext::copy_from(ctx,Fp64ContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type I32Context<'input> = BaseParserRuleContext<'input,I32ContextExt<'input>>;

pub trait I32ContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token I32
	/// Returns `None` if there is no child corresponding to token I32
	fn I32(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_I32, 0)
	}
}

impl<'input> I32ContextAttrs<'input> for I32Context<'input>{}

pub struct I32ContextExt<'input>{
	base:ScalarTypeContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{I32ContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for I32Context<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for I32Context<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_i32(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_i32(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for I32Context<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_i32(self);
	}
}

impl<'input> CustomRuleContext<'input> for I32ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_scalarType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_scalarType }
}

impl<'input> Borrow<ScalarTypeContextExt<'input>> for I32Context<'input>{
	fn borrow(&self) -> &ScalarTypeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ScalarTypeContextExt<'input>> for I32Context<'input>{
	fn borrow_mut(&mut self) -> &mut ScalarTypeContextExt<'input> { &mut self.base }
}

impl<'input> ScalarTypeContextAttrs<'input> for I32Context<'input> {}

impl<'input> I32ContextExt<'input>{
	fn new(ctx: &dyn ScalarTypeContextAttrs<'input>) -> Rc<ScalarTypeContextAll<'input>>  {
		Rc::new(
			ScalarTypeContextAll::I32Context(
				BaseParserRuleContext::copy_from(ctx,I32ContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type Fp32Context<'input> = BaseParserRuleContext<'input,Fp32ContextExt<'input>>;

pub trait Fp32ContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token FP32
	/// Returns `None` if there is no child corresponding to token FP32
	fn FP32(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_FP32, 0)
	}
}

impl<'input> Fp32ContextAttrs<'input> for Fp32Context<'input>{}

pub struct Fp32ContextExt<'input>{
	base:ScalarTypeContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{Fp32ContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for Fp32Context<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for Fp32Context<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_fp32(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_fp32(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for Fp32Context<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_fp32(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fp32ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_scalarType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_scalarType }
}

impl<'input> Borrow<ScalarTypeContextExt<'input>> for Fp32Context<'input>{
	fn borrow(&self) -> &ScalarTypeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ScalarTypeContextExt<'input>> for Fp32Context<'input>{
	fn borrow_mut(&mut self) -> &mut ScalarTypeContextExt<'input> { &mut self.base }
}

impl<'input> ScalarTypeContextAttrs<'input> for Fp32Context<'input> {}

impl<'input> Fp32ContextExt<'input>{
	fn new(ctx: &dyn ScalarTypeContextAttrs<'input>) -> Rc<ScalarTypeContextAll<'input>>  {
		Rc::new(
			ScalarTypeContextAll::Fp32Context(
				BaseParserRuleContext::copy_from(ctx,Fp32ContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type IntervalYearContext<'input> = BaseParserRuleContext<'input,IntervalYearContextExt<'input>>;

pub trait IntervalYearContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token Interval_Year
	/// Returns `None` if there is no child corresponding to token Interval_Year
	fn Interval_Year(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Interval_Year, 0)
	}
}

impl<'input> IntervalYearContextAttrs<'input> for IntervalYearContext<'input>{}

pub struct IntervalYearContextExt<'input>{
	base:ScalarTypeContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{IntervalYearContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for IntervalYearContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for IntervalYearContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_intervalYear(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_intervalYear(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for IntervalYearContext<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_intervalYear(self);
	}
}

impl<'input> CustomRuleContext<'input> for IntervalYearContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
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

pub trait UuidContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token UUID
	/// Returns `None` if there is no child corresponding to token UUID
	fn UUID(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_UUID, 0)
	}
}

impl<'input> UuidContextAttrs<'input> for UuidContext<'input>{}

pub struct UuidContextExt<'input>{
	base:ScalarTypeContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{UuidContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for UuidContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for UuidContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_uuid(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_uuid(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for UuidContext<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_uuid(self);
	}
}

impl<'input> CustomRuleContext<'input> for UuidContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
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
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type I8Context<'input> = BaseParserRuleContext<'input,I8ContextExt<'input>>;

pub trait I8ContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token I8
	/// Returns `None` if there is no child corresponding to token I8
	fn I8(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_I8, 0)
	}
}

impl<'input> I8ContextAttrs<'input> for I8Context<'input>{}

pub struct I8ContextExt<'input>{
	base:ScalarTypeContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{I8ContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for I8Context<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for I8Context<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_i8(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_i8(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for I8Context<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_i8(self);
	}
}

impl<'input> CustomRuleContext<'input> for I8ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_scalarType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_scalarType }
}

impl<'input> Borrow<ScalarTypeContextExt<'input>> for I8Context<'input>{
	fn borrow(&self) -> &ScalarTypeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ScalarTypeContextExt<'input>> for I8Context<'input>{
	fn borrow_mut(&mut self) -> &mut ScalarTypeContextExt<'input> { &mut self.base }
}

impl<'input> ScalarTypeContextAttrs<'input> for I8Context<'input> {}

impl<'input> I8ContextExt<'input>{
	fn new(ctx: &dyn ScalarTypeContextAttrs<'input>) -> Rc<ScalarTypeContextAll<'input>>  {
		Rc::new(
			ScalarTypeContextAll::I8Context(
				BaseParserRuleContext::copy_from(ctx,I8ContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type I16Context<'input> = BaseParserRuleContext<'input,I16ContextExt<'input>>;

pub trait I16ContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token I16
	/// Returns `None` if there is no child corresponding to token I16
	fn I16(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_I16, 0)
	}
}

impl<'input> I16ContextAttrs<'input> for I16Context<'input>{}

pub struct I16ContextExt<'input>{
	base:ScalarTypeContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{I16ContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for I16Context<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for I16Context<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_i16(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_i16(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for I16Context<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_i16(self);
	}
}

impl<'input> CustomRuleContext<'input> for I16ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_scalarType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_scalarType }
}

impl<'input> Borrow<ScalarTypeContextExt<'input>> for I16Context<'input>{
	fn borrow(&self) -> &ScalarTypeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ScalarTypeContextExt<'input>> for I16Context<'input>{
	fn borrow_mut(&mut self) -> &mut ScalarTypeContextExt<'input> { &mut self.base }
}

impl<'input> ScalarTypeContextAttrs<'input> for I16Context<'input> {}

impl<'input> I16ContextExt<'input>{
	fn new(ctx: &dyn ScalarTypeContextAttrs<'input>) -> Rc<ScalarTypeContextAll<'input>>  {
		Rc::new(
			ScalarTypeContextAll::I16Context(
				BaseParserRuleContext::copy_from(ctx,I16ContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I> SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn scalarType(&mut self,)
	-> Result<Rc<ScalarTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ScalarTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_scalarType);
        let mut _localctx: Rc<ScalarTypeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(36);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			SubstraitType_Boolean 
				=> {
					let tmp = BooleanContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
					_localctx = tmp;
					{
					recog.base.set_state(24);
					recog.base.match_token(SubstraitType_Boolean,&mut recog.err_handler)?;

					}
				}

			SubstraitType_I8 
				=> {
					let tmp = I8ContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2)?;
					_localctx = tmp;
					{
					recog.base.set_state(25);
					recog.base.match_token(SubstraitType_I8,&mut recog.err_handler)?;

					}
				}

			SubstraitType_I16 
				=> {
					let tmp = I16ContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3)?;
					_localctx = tmp;
					{
					recog.base.set_state(26);
					recog.base.match_token(SubstraitType_I16,&mut recog.err_handler)?;

					}
				}

			SubstraitType_I32 
				=> {
					let tmp = I32ContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 4)?;
					_localctx = tmp;
					{
					recog.base.set_state(27);
					recog.base.match_token(SubstraitType_I32,&mut recog.err_handler)?;

					}
				}

			SubstraitType_I64 
				=> {
					let tmp = I64ContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 5)?;
					_localctx = tmp;
					{
					recog.base.set_state(28);
					recog.base.match_token(SubstraitType_I64,&mut recog.err_handler)?;

					}
				}

			SubstraitType_FP32 
				=> {
					let tmp = Fp32ContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 6)?;
					_localctx = tmp;
					{
					recog.base.set_state(29);
					recog.base.match_token(SubstraitType_FP32,&mut recog.err_handler)?;

					}
				}

			SubstraitType_FP64 
				=> {
					let tmp = Fp64ContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 7)?;
					_localctx = tmp;
					{
					recog.base.set_state(30);
					recog.base.match_token(SubstraitType_FP64,&mut recog.err_handler)?;

					}
				}

			SubstraitType_String 
				=> {
					let tmp = StringContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 8)?;
					_localctx = tmp;
					{
					recog.base.set_state(31);
					recog.base.match_token(SubstraitType_String,&mut recog.err_handler)?;

					}
				}

			SubstraitType_Binary 
				=> {
					let tmp = BinaryContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 9)?;
					_localctx = tmp;
					{
					recog.base.set_state(32);
					recog.base.match_token(SubstraitType_Binary,&mut recog.err_handler)?;

					}
				}

			SubstraitType_Date 
				=> {
					let tmp = DateContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 10)?;
					_localctx = tmp;
					{
					recog.base.set_state(33);
					recog.base.match_token(SubstraitType_Date,&mut recog.err_handler)?;

					}
				}

			SubstraitType_Interval_Year 
				=> {
					let tmp = IntervalYearContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 11)?;
					_localctx = tmp;
					{
					recog.base.set_state(34);
					recog.base.match_token(SubstraitType_Interval_Year,&mut recog.err_handler)?;

					}
				}

			SubstraitType_UUID 
				=> {
					let tmp = UuidContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 12)?;
					_localctx = tmp;
					{
					recog.base.set_state(35);
					recog.base.match_token(SubstraitType_UUID,&mut recog.err_handler)?;

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
#[derive(Debug)]
pub enum ParameterizedTypeContextAll<'input>{
	StructContext(StructContext<'input>),
	PrecisionTimestampTZContext(PrecisionTimestampTZContext<'input>),
	NStructContext(NStructContext<'input>),
	FixedBinaryContext(FixedBinaryContext<'input>),
	UserDefinedContext(UserDefinedContext<'input>),
	FixedCharContext(FixedCharContext<'input>),
	ListContext(ListContext<'input>),
	PrecisionIntervalDayContext(PrecisionIntervalDayContext<'input>),
	FuncContext(FuncContext<'input>),
	VarCharContext(VarCharContext<'input>),
	PrecisionIntervalCompoundContext(PrecisionIntervalCompoundContext<'input>),
	PrecisionTimestampContext(PrecisionTimestampContext<'input>),
	DecimalContext(DecimalContext<'input>),
	PrecisionTimeContext(PrecisionTimeContext<'input>),
	MapContext(MapContext<'input>),
Error(ParameterizedTypeContext<'input>)
}
antlr4rust::tid!{ParameterizedTypeContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for ParameterizedTypeContextAll<'input>{}

impl<'input> SubstraitTypeParserContext<'input> for ParameterizedTypeContextAll<'input>{}

impl<'input> Deref for ParameterizedTypeContextAll<'input>{
	type Target = dyn ParameterizedTypeContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use ParameterizedTypeContextAll::*;
		match self{
			StructContext(inner) => inner,
			PrecisionTimestampTZContext(inner) => inner,
			NStructContext(inner) => inner,
			FixedBinaryContext(inner) => inner,
			UserDefinedContext(inner) => inner,
			FixedCharContext(inner) => inner,
			ListContext(inner) => inner,
			PrecisionIntervalDayContext(inner) => inner,
			FuncContext(inner) => inner,
			VarCharContext(inner) => inner,
			PrecisionIntervalCompoundContext(inner) => inner,
			PrecisionTimestampContext(inner) => inner,
			DecimalContext(inner) => inner,
			PrecisionTimeContext(inner) => inner,
			MapContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for ParameterizedTypeContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for ParameterizedTypeContextAll<'input>{
    fn enter(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
}



pub type ParameterizedTypeContext<'input> = BaseParserRuleContext<'input,ParameterizedTypeContextExt<'input>>;

#[derive(Clone)]
pub struct ParameterizedTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitTypeParserContext<'input> for ParameterizedTypeContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for ParameterizedTypeContext<'input>{
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for ParameterizedTypeContext<'input>{
}

impl<'input> CustomRuleContext<'input> for ParameterizedTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parameterizedType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parameterizedType }
}
antlr4rust::tid!{ParameterizedTypeContextExt<'a>}

impl<'input> ParameterizedTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ParameterizedTypeContextAll<'input>> {
		Rc::new(
		ParameterizedTypeContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ParameterizedTypeContextExt{

				ph:PhantomData
			}),
		)
		)
	}
}

pub trait ParameterizedTypeContextAttrs<'input>: SubstraitTypeParserContext<'input> + BorrowMut<ParameterizedTypeContextExt<'input>>{


}

impl<'input> ParameterizedTypeContextAttrs<'input> for ParameterizedTypeContext<'input>{}

pub type StructContext<'input> = BaseParserRuleContext<'input,StructContextExt<'input>>;

pub trait StructContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token Struct
	/// Returns `None` if there is no child corresponding to token Struct
	fn Struct(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Struct, 0)
	}
	/// Retrieves first TerminalNode corresponding to token Lt
	/// Returns `None` if there is no child corresponding to token Lt
	fn Lt(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Lt, 0)
	}
	fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token Gt
	/// Returns `None` if there is no child corresponding to token Gt
	fn Gt(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Gt, 0)
	}
	/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
	fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
	/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
	fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Comma, i)
	}
	/// Retrieves first TerminalNode corresponding to token QMark
	/// Returns `None` if there is no child corresponding to token QMark
	fn QMark(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_QMark, 0)
	}
}

impl<'input> StructContextAttrs<'input> for StructContext<'input>{}

pub struct StructContextExt<'input>{
	base:ParameterizedTypeContextExt<'input>,
	pub isnull: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{StructContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for StructContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for StructContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_struct(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_struct(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for StructContext<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_struct(self);
	}
}

impl<'input> CustomRuleContext<'input> for StructContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parameterizedType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parameterizedType }
}

impl<'input> Borrow<ParameterizedTypeContextExt<'input>> for StructContext<'input>{
	fn borrow(&self) -> &ParameterizedTypeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ParameterizedTypeContextExt<'input>> for StructContext<'input>{
	fn borrow_mut(&mut self) -> &mut ParameterizedTypeContextExt<'input> { &mut self.base }
}

impl<'input> ParameterizedTypeContextAttrs<'input> for StructContext<'input> {}

impl<'input> StructContextExt<'input>{
	fn new(ctx: &dyn ParameterizedTypeContextAttrs<'input>) -> Rc<ParameterizedTypeContextAll<'input>>  {
		Rc::new(
			ParameterizedTypeContextAll::StructContext(
				BaseParserRuleContext::copy_from(ctx,StructContextExt{
					isnull:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type PrecisionTimestampTZContext<'input> = BaseParserRuleContext<'input,PrecisionTimestampTZContextExt<'input>>;

pub trait PrecisionTimestampTZContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token Precision_Timestamp_TZ
	/// Returns `None` if there is no child corresponding to token Precision_Timestamp_TZ
	fn Precision_Timestamp_TZ(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Precision_Timestamp_TZ, 0)
	}
	/// Retrieves first TerminalNode corresponding to token Lt
	/// Returns `None` if there is no child corresponding to token Lt
	fn Lt(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Lt, 0)
	}
	/// Retrieves first TerminalNode corresponding to token Gt
	/// Returns `None` if there is no child corresponding to token Gt
	fn Gt(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Gt, 0)
	}
	fn numericParameter(&self) -> Option<Rc<NumericParameterContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token QMark
	/// Returns `None` if there is no child corresponding to token QMark
	fn QMark(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_QMark, 0)
	}
}

impl<'input> PrecisionTimestampTZContextAttrs<'input> for PrecisionTimestampTZContext<'input>{}

pub struct PrecisionTimestampTZContextExt<'input>{
	base:ParameterizedTypeContextExt<'input>,
	pub isnull: Option<TokenType<'input>>,
	pub precision: Option<Rc<NumericParameterContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{PrecisionTimestampTZContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for PrecisionTimestampTZContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for PrecisionTimestampTZContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_precisionTimestampTZ(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_precisionTimestampTZ(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for PrecisionTimestampTZContext<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_precisionTimestampTZ(self);
	}
}

impl<'input> CustomRuleContext<'input> for PrecisionTimestampTZContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parameterizedType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parameterizedType }
}

impl<'input> Borrow<ParameterizedTypeContextExt<'input>> for PrecisionTimestampTZContext<'input>{
	fn borrow(&self) -> &ParameterizedTypeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ParameterizedTypeContextExt<'input>> for PrecisionTimestampTZContext<'input>{
	fn borrow_mut(&mut self) -> &mut ParameterizedTypeContextExt<'input> { &mut self.base }
}

impl<'input> ParameterizedTypeContextAttrs<'input> for PrecisionTimestampTZContext<'input> {}

impl<'input> PrecisionTimestampTZContextExt<'input>{
	fn new(ctx: &dyn ParameterizedTypeContextAttrs<'input>) -> Rc<ParameterizedTypeContextAll<'input>>  {
		Rc::new(
			ParameterizedTypeContextAll::PrecisionTimestampTZContext(
				BaseParserRuleContext::copy_from(ctx,PrecisionTimestampTZContextExt{
					isnull:None, 
        			precision:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type NStructContext<'input> = BaseParserRuleContext<'input,NStructContextExt<'input>>;

pub trait NStructContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token NStruct
	/// Returns `None` if there is no child corresponding to token NStruct
	fn NStruct(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_NStruct, 0)
	}
	/// Retrieves first TerminalNode corresponding to token Lt
	/// Returns `None` if there is no child corresponding to token Lt
	fn Lt(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Lt, 0)
	}
	/// Retrieves all `TerminalNode`s corresponding to token Identifier in current rule
	fn Identifier_all(&self) -> Vec<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token Identifier, starting from 0.
	/// Returns `None` if number of children corresponding to token Identifier is less or equal than `i`.
	fn Identifier(&self, i: usize) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Identifier, i)
	}
	/// Retrieves all `TerminalNode`s corresponding to token Colon in current rule
	fn Colon_all(&self) -> Vec<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token Colon, starting from 0.
	/// Returns `None` if number of children corresponding to token Colon is less or equal than `i`.
	fn Colon(&self, i: usize) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Colon, i)
	}
	fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token Gt
	/// Returns `None` if there is no child corresponding to token Gt
	fn Gt(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Gt, 0)
	}
	/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
	fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
	/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
	fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Comma, i)
	}
	/// Retrieves first TerminalNode corresponding to token QMark
	/// Returns `None` if there is no child corresponding to token QMark
	fn QMark(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_QMark, 0)
	}
}

impl<'input> NStructContextAttrs<'input> for NStructContext<'input>{}

pub struct NStructContextExt<'input>{
	base:ParameterizedTypeContextExt<'input>,
	pub isnull: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{NStructContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for NStructContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for NStructContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_nStruct(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_nStruct(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for NStructContext<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_nStruct(self);
	}
}

impl<'input> CustomRuleContext<'input> for NStructContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parameterizedType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parameterizedType }
}

impl<'input> Borrow<ParameterizedTypeContextExt<'input>> for NStructContext<'input>{
	fn borrow(&self) -> &ParameterizedTypeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ParameterizedTypeContextExt<'input>> for NStructContext<'input>{
	fn borrow_mut(&mut self) -> &mut ParameterizedTypeContextExt<'input> { &mut self.base }
}

impl<'input> ParameterizedTypeContextAttrs<'input> for NStructContext<'input> {}

impl<'input> NStructContextExt<'input>{
	fn new(ctx: &dyn ParameterizedTypeContextAttrs<'input>) -> Rc<ParameterizedTypeContextAll<'input>>  {
		Rc::new(
			ParameterizedTypeContextAll::NStructContext(
				BaseParserRuleContext::copy_from(ctx,NStructContextExt{
					isnull:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type FixedBinaryContext<'input> = BaseParserRuleContext<'input,FixedBinaryContextExt<'input>>;

pub trait FixedBinaryContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token FixedBinary
	/// Returns `None` if there is no child corresponding to token FixedBinary
	fn FixedBinary(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_FixedBinary, 0)
	}
	/// Retrieves first TerminalNode corresponding to token Lt
	/// Returns `None` if there is no child corresponding to token Lt
	fn Lt(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Lt, 0)
	}
	/// Retrieves first TerminalNode corresponding to token Gt
	/// Returns `None` if there is no child corresponding to token Gt
	fn Gt(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Gt, 0)
	}
	fn numericParameter(&self) -> Option<Rc<NumericParameterContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token QMark
	/// Returns `None` if there is no child corresponding to token QMark
	fn QMark(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_QMark, 0)
	}
}

impl<'input> FixedBinaryContextAttrs<'input> for FixedBinaryContext<'input>{}

pub struct FixedBinaryContextExt<'input>{
	base:ParameterizedTypeContextExt<'input>,
	pub isnull: Option<TokenType<'input>>,
	pub length: Option<Rc<NumericParameterContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{FixedBinaryContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for FixedBinaryContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for FixedBinaryContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_fixedBinary(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_fixedBinary(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for FixedBinaryContext<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_fixedBinary(self);
	}
}

impl<'input> CustomRuleContext<'input> for FixedBinaryContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parameterizedType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parameterizedType }
}

impl<'input> Borrow<ParameterizedTypeContextExt<'input>> for FixedBinaryContext<'input>{
	fn borrow(&self) -> &ParameterizedTypeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ParameterizedTypeContextExt<'input>> for FixedBinaryContext<'input>{
	fn borrow_mut(&mut self) -> &mut ParameterizedTypeContextExt<'input> { &mut self.base }
}

impl<'input> ParameterizedTypeContextAttrs<'input> for FixedBinaryContext<'input> {}

impl<'input> FixedBinaryContextExt<'input>{
	fn new(ctx: &dyn ParameterizedTypeContextAttrs<'input>) -> Rc<ParameterizedTypeContextAll<'input>>  {
		Rc::new(
			ParameterizedTypeContextAll::FixedBinaryContext(
				BaseParserRuleContext::copy_from(ctx,FixedBinaryContextExt{
					isnull:None, 
        			length:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type UserDefinedContext<'input> = BaseParserRuleContext<'input,UserDefinedContextExt<'input>>;

pub trait UserDefinedContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token UserDefined
	/// Returns `None` if there is no child corresponding to token UserDefined
	fn UserDefined(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_UserDefined, 0)
	}
	/// Retrieves all `TerminalNode`s corresponding to token Identifier in current rule
	fn Identifier_all(&self) -> Vec<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token Identifier, starting from 0.
	/// Returns `None` if number of children corresponding to token Identifier is less or equal than `i`.
	fn Identifier(&self, i: usize) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Identifier, i)
	}
	/// Retrieves first TerminalNode corresponding to token Dot
	/// Returns `None` if there is no child corresponding to token Dot
	fn Dot(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Dot, 0)
	}
	/// Retrieves first TerminalNode corresponding to token Lt
	/// Returns `None` if there is no child corresponding to token Lt
	fn Lt(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Lt, 0)
	}
	fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token Gt
	/// Returns `None` if there is no child corresponding to token Gt
	fn Gt(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Gt, 0)
	}
	/// Retrieves first TerminalNode corresponding to token QMark
	/// Returns `None` if there is no child corresponding to token QMark
	fn QMark(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_QMark, 0)
	}
	/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
	fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
	/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
	fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Comma, i)
	}
}

impl<'input> UserDefinedContextAttrs<'input> for UserDefinedContext<'input>{}

pub struct UserDefinedContextExt<'input>{
	base:ParameterizedTypeContextExt<'input>,
	pub dependencyAlias: Option<TokenType<'input>>,
	pub isnull: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{UserDefinedContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for UserDefinedContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for UserDefinedContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_userDefined(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_userDefined(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for UserDefinedContext<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_userDefined(self);
	}
}

impl<'input> CustomRuleContext<'input> for UserDefinedContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parameterizedType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parameterizedType }
}

impl<'input> Borrow<ParameterizedTypeContextExt<'input>> for UserDefinedContext<'input>{
	fn borrow(&self) -> &ParameterizedTypeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ParameterizedTypeContextExt<'input>> for UserDefinedContext<'input>{
	fn borrow_mut(&mut self) -> &mut ParameterizedTypeContextExt<'input> { &mut self.base }
}

impl<'input> ParameterizedTypeContextAttrs<'input> for UserDefinedContext<'input> {}

impl<'input> UserDefinedContextExt<'input>{
	fn new(ctx: &dyn ParameterizedTypeContextAttrs<'input>) -> Rc<ParameterizedTypeContextAll<'input>>  {
		Rc::new(
			ParameterizedTypeContextAll::UserDefinedContext(
				BaseParserRuleContext::copy_from(ctx,UserDefinedContextExt{
					dependencyAlias:None, isnull:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type FixedCharContext<'input> = BaseParserRuleContext<'input,FixedCharContextExt<'input>>;

pub trait FixedCharContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token FixedChar
	/// Returns `None` if there is no child corresponding to token FixedChar
	fn FixedChar(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_FixedChar, 0)
	}
	/// Retrieves first TerminalNode corresponding to token Lt
	/// Returns `None` if there is no child corresponding to token Lt
	fn Lt(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Lt, 0)
	}
	/// Retrieves first TerminalNode corresponding to token Gt
	/// Returns `None` if there is no child corresponding to token Gt
	fn Gt(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Gt, 0)
	}
	fn numericParameter(&self) -> Option<Rc<NumericParameterContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token QMark
	/// Returns `None` if there is no child corresponding to token QMark
	fn QMark(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_QMark, 0)
	}
}

impl<'input> FixedCharContextAttrs<'input> for FixedCharContext<'input>{}

pub struct FixedCharContextExt<'input>{
	base:ParameterizedTypeContextExt<'input>,
	pub isnull: Option<TokenType<'input>>,
	pub length: Option<Rc<NumericParameterContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{FixedCharContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for FixedCharContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for FixedCharContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_fixedChar(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_fixedChar(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for FixedCharContext<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_fixedChar(self);
	}
}

impl<'input> CustomRuleContext<'input> for FixedCharContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parameterizedType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parameterizedType }
}

impl<'input> Borrow<ParameterizedTypeContextExt<'input>> for FixedCharContext<'input>{
	fn borrow(&self) -> &ParameterizedTypeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ParameterizedTypeContextExt<'input>> for FixedCharContext<'input>{
	fn borrow_mut(&mut self) -> &mut ParameterizedTypeContextExt<'input> { &mut self.base }
}

impl<'input> ParameterizedTypeContextAttrs<'input> for FixedCharContext<'input> {}

impl<'input> FixedCharContextExt<'input>{
	fn new(ctx: &dyn ParameterizedTypeContextAttrs<'input>) -> Rc<ParameterizedTypeContextAll<'input>>  {
		Rc::new(
			ParameterizedTypeContextAll::FixedCharContext(
				BaseParserRuleContext::copy_from(ctx,FixedCharContextExt{
					isnull:None, 
        			length:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ListContext<'input> = BaseParserRuleContext<'input,ListContextExt<'input>>;

pub trait ListContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token List
	/// Returns `None` if there is no child corresponding to token List
	fn List(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_List, 0)
	}
	/// Retrieves first TerminalNode corresponding to token Lt
	/// Returns `None` if there is no child corresponding to token Lt
	fn Lt(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Lt, 0)
	}
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token Gt
	/// Returns `None` if there is no child corresponding to token Gt
	fn Gt(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Gt, 0)
	}
	/// Retrieves first TerminalNode corresponding to token QMark
	/// Returns `None` if there is no child corresponding to token QMark
	fn QMark(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_QMark, 0)
	}
}

impl<'input> ListContextAttrs<'input> for ListContext<'input>{}

pub struct ListContextExt<'input>{
	base:ParameterizedTypeContextExt<'input>,
	pub isnull: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{ListContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for ListContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for ListContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_list(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_list(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for ListContext<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_list(self);
	}
}

impl<'input> CustomRuleContext<'input> for ListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parameterizedType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parameterizedType }
}

impl<'input> Borrow<ParameterizedTypeContextExt<'input>> for ListContext<'input>{
	fn borrow(&self) -> &ParameterizedTypeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ParameterizedTypeContextExt<'input>> for ListContext<'input>{
	fn borrow_mut(&mut self) -> &mut ParameterizedTypeContextExt<'input> { &mut self.base }
}

impl<'input> ParameterizedTypeContextAttrs<'input> for ListContext<'input> {}

impl<'input> ListContextExt<'input>{
	fn new(ctx: &dyn ParameterizedTypeContextAttrs<'input>) -> Rc<ParameterizedTypeContextAll<'input>>  {
		Rc::new(
			ParameterizedTypeContextAll::ListContext(
				BaseParserRuleContext::copy_from(ctx,ListContextExt{
					isnull:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type PrecisionIntervalDayContext<'input> = BaseParserRuleContext<'input,PrecisionIntervalDayContextExt<'input>>;

pub trait PrecisionIntervalDayContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token Interval_Day
	/// Returns `None` if there is no child corresponding to token Interval_Day
	fn Interval_Day(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Interval_Day, 0)
	}
	/// Retrieves first TerminalNode corresponding to token Lt
	/// Returns `None` if there is no child corresponding to token Lt
	fn Lt(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Lt, 0)
	}
	/// Retrieves first TerminalNode corresponding to token Gt
	/// Returns `None` if there is no child corresponding to token Gt
	fn Gt(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Gt, 0)
	}
	fn numericParameter(&self) -> Option<Rc<NumericParameterContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token QMark
	/// Returns `None` if there is no child corresponding to token QMark
	fn QMark(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_QMark, 0)
	}
}

impl<'input> PrecisionIntervalDayContextAttrs<'input> for PrecisionIntervalDayContext<'input>{}

pub struct PrecisionIntervalDayContextExt<'input>{
	base:ParameterizedTypeContextExt<'input>,
	pub isnull: Option<TokenType<'input>>,
	pub precision: Option<Rc<NumericParameterContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{PrecisionIntervalDayContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for PrecisionIntervalDayContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for PrecisionIntervalDayContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_precisionIntervalDay(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_precisionIntervalDay(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for PrecisionIntervalDayContext<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_precisionIntervalDay(self);
	}
}

impl<'input> CustomRuleContext<'input> for PrecisionIntervalDayContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parameterizedType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parameterizedType }
}

impl<'input> Borrow<ParameterizedTypeContextExt<'input>> for PrecisionIntervalDayContext<'input>{
	fn borrow(&self) -> &ParameterizedTypeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ParameterizedTypeContextExt<'input>> for PrecisionIntervalDayContext<'input>{
	fn borrow_mut(&mut self) -> &mut ParameterizedTypeContextExt<'input> { &mut self.base }
}

impl<'input> ParameterizedTypeContextAttrs<'input> for PrecisionIntervalDayContext<'input> {}

impl<'input> PrecisionIntervalDayContextExt<'input>{
	fn new(ctx: &dyn ParameterizedTypeContextAttrs<'input>) -> Rc<ParameterizedTypeContextAll<'input>>  {
		Rc::new(
			ParameterizedTypeContextAll::PrecisionIntervalDayContext(
				BaseParserRuleContext::copy_from(ctx,PrecisionIntervalDayContextExt{
					isnull:None, 
        			precision:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type FuncContext<'input> = BaseParserRuleContext<'input,FuncContextExt<'input>>;

pub trait FuncContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token Func
	/// Returns `None` if there is no child corresponding to token Func
	fn Func(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Func, 0)
	}
	/// Retrieves first TerminalNode corresponding to token Lt
	/// Returns `None` if there is no child corresponding to token Lt
	fn Lt(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Lt, 0)
	}
	/// Retrieves first TerminalNode corresponding to token Arrow
	/// Returns `None` if there is no child corresponding to token Arrow
	fn Arrow(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Arrow, 0)
	}
	/// Retrieves first TerminalNode corresponding to token Gt
	/// Returns `None` if there is no child corresponding to token Gt
	fn Gt(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Gt, 0)
	}
	fn funcParams(&self) -> Option<Rc<FuncParamsContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token QMark
	/// Returns `None` if there is no child corresponding to token QMark
	fn QMark(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_QMark, 0)
	}
}

impl<'input> FuncContextAttrs<'input> for FuncContext<'input>{}

pub struct FuncContextExt<'input>{
	base:ParameterizedTypeContextExt<'input>,
	pub isnull: Option<TokenType<'input>>,
	pub params: Option<Rc<FuncParamsContextAll<'input>>>,
	pub returnType: Option<Rc<ExprContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{FuncContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for FuncContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for FuncContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_func(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_func(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for FuncContext<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_func(self);
	}
}

impl<'input> CustomRuleContext<'input> for FuncContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parameterizedType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parameterizedType }
}

impl<'input> Borrow<ParameterizedTypeContextExt<'input>> for FuncContext<'input>{
	fn borrow(&self) -> &ParameterizedTypeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ParameterizedTypeContextExt<'input>> for FuncContext<'input>{
	fn borrow_mut(&mut self) -> &mut ParameterizedTypeContextExt<'input> { &mut self.base }
}

impl<'input> ParameterizedTypeContextAttrs<'input> for FuncContext<'input> {}

impl<'input> FuncContextExt<'input>{
	fn new(ctx: &dyn ParameterizedTypeContextAttrs<'input>) -> Rc<ParameterizedTypeContextAll<'input>>  {
		Rc::new(
			ParameterizedTypeContextAll::FuncContext(
				BaseParserRuleContext::copy_from(ctx,FuncContextExt{
					isnull:None, 
        			params:None, returnType:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type VarCharContext<'input> = BaseParserRuleContext<'input,VarCharContextExt<'input>>;

pub trait VarCharContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token VarChar
	/// Returns `None` if there is no child corresponding to token VarChar
	fn VarChar(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_VarChar, 0)
	}
	/// Retrieves first TerminalNode corresponding to token Lt
	/// Returns `None` if there is no child corresponding to token Lt
	fn Lt(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Lt, 0)
	}
	/// Retrieves first TerminalNode corresponding to token Gt
	/// Returns `None` if there is no child corresponding to token Gt
	fn Gt(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Gt, 0)
	}
	fn numericParameter(&self) -> Option<Rc<NumericParameterContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token QMark
	/// Returns `None` if there is no child corresponding to token QMark
	fn QMark(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_QMark, 0)
	}
}

impl<'input> VarCharContextAttrs<'input> for VarCharContext<'input>{}

pub struct VarCharContextExt<'input>{
	base:ParameterizedTypeContextExt<'input>,
	pub isnull: Option<TokenType<'input>>,
	pub length: Option<Rc<NumericParameterContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{VarCharContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for VarCharContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for VarCharContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_varChar(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_varChar(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for VarCharContext<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_varChar(self);
	}
}

impl<'input> CustomRuleContext<'input> for VarCharContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parameterizedType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parameterizedType }
}

impl<'input> Borrow<ParameterizedTypeContextExt<'input>> for VarCharContext<'input>{
	fn borrow(&self) -> &ParameterizedTypeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ParameterizedTypeContextExt<'input>> for VarCharContext<'input>{
	fn borrow_mut(&mut self) -> &mut ParameterizedTypeContextExt<'input> { &mut self.base }
}

impl<'input> ParameterizedTypeContextAttrs<'input> for VarCharContext<'input> {}

impl<'input> VarCharContextExt<'input>{
	fn new(ctx: &dyn ParameterizedTypeContextAttrs<'input>) -> Rc<ParameterizedTypeContextAll<'input>>  {
		Rc::new(
			ParameterizedTypeContextAll::VarCharContext(
				BaseParserRuleContext::copy_from(ctx,VarCharContextExt{
					isnull:None, 
        			length:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type PrecisionIntervalCompoundContext<'input> = BaseParserRuleContext<'input,PrecisionIntervalCompoundContextExt<'input>>;

pub trait PrecisionIntervalCompoundContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token Interval_Compound
	/// Returns `None` if there is no child corresponding to token Interval_Compound
	fn Interval_Compound(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Interval_Compound, 0)
	}
	/// Retrieves first TerminalNode corresponding to token Lt
	/// Returns `None` if there is no child corresponding to token Lt
	fn Lt(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Lt, 0)
	}
	/// Retrieves first TerminalNode corresponding to token Gt
	/// Returns `None` if there is no child corresponding to token Gt
	fn Gt(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Gt, 0)
	}
	fn numericParameter(&self) -> Option<Rc<NumericParameterContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token QMark
	/// Returns `None` if there is no child corresponding to token QMark
	fn QMark(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_QMark, 0)
	}
}

impl<'input> PrecisionIntervalCompoundContextAttrs<'input> for PrecisionIntervalCompoundContext<'input>{}

pub struct PrecisionIntervalCompoundContextExt<'input>{
	base:ParameterizedTypeContextExt<'input>,
	pub isnull: Option<TokenType<'input>>,
	pub precision: Option<Rc<NumericParameterContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{PrecisionIntervalCompoundContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for PrecisionIntervalCompoundContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for PrecisionIntervalCompoundContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_precisionIntervalCompound(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_precisionIntervalCompound(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for PrecisionIntervalCompoundContext<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_precisionIntervalCompound(self);
	}
}

impl<'input> CustomRuleContext<'input> for PrecisionIntervalCompoundContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parameterizedType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parameterizedType }
}

impl<'input> Borrow<ParameterizedTypeContextExt<'input>> for PrecisionIntervalCompoundContext<'input>{
	fn borrow(&self) -> &ParameterizedTypeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ParameterizedTypeContextExt<'input>> for PrecisionIntervalCompoundContext<'input>{
	fn borrow_mut(&mut self) -> &mut ParameterizedTypeContextExt<'input> { &mut self.base }
}

impl<'input> ParameterizedTypeContextAttrs<'input> for PrecisionIntervalCompoundContext<'input> {}

impl<'input> PrecisionIntervalCompoundContextExt<'input>{
	fn new(ctx: &dyn ParameterizedTypeContextAttrs<'input>) -> Rc<ParameterizedTypeContextAll<'input>>  {
		Rc::new(
			ParameterizedTypeContextAll::PrecisionIntervalCompoundContext(
				BaseParserRuleContext::copy_from(ctx,PrecisionIntervalCompoundContextExt{
					isnull:None, 
        			precision:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type PrecisionTimestampContext<'input> = BaseParserRuleContext<'input,PrecisionTimestampContextExt<'input>>;

pub trait PrecisionTimestampContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token Precision_Timestamp
	/// Returns `None` if there is no child corresponding to token Precision_Timestamp
	fn Precision_Timestamp(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Precision_Timestamp, 0)
	}
	/// Retrieves first TerminalNode corresponding to token Lt
	/// Returns `None` if there is no child corresponding to token Lt
	fn Lt(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Lt, 0)
	}
	/// Retrieves first TerminalNode corresponding to token Gt
	/// Returns `None` if there is no child corresponding to token Gt
	fn Gt(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Gt, 0)
	}
	fn numericParameter(&self) -> Option<Rc<NumericParameterContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token QMark
	/// Returns `None` if there is no child corresponding to token QMark
	fn QMark(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_QMark, 0)
	}
}

impl<'input> PrecisionTimestampContextAttrs<'input> for PrecisionTimestampContext<'input>{}

pub struct PrecisionTimestampContextExt<'input>{
	base:ParameterizedTypeContextExt<'input>,
	pub isnull: Option<TokenType<'input>>,
	pub precision: Option<Rc<NumericParameterContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{PrecisionTimestampContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for PrecisionTimestampContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for PrecisionTimestampContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_precisionTimestamp(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_precisionTimestamp(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for PrecisionTimestampContext<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_precisionTimestamp(self);
	}
}

impl<'input> CustomRuleContext<'input> for PrecisionTimestampContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parameterizedType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parameterizedType }
}

impl<'input> Borrow<ParameterizedTypeContextExt<'input>> for PrecisionTimestampContext<'input>{
	fn borrow(&self) -> &ParameterizedTypeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ParameterizedTypeContextExt<'input>> for PrecisionTimestampContext<'input>{
	fn borrow_mut(&mut self) -> &mut ParameterizedTypeContextExt<'input> { &mut self.base }
}

impl<'input> ParameterizedTypeContextAttrs<'input> for PrecisionTimestampContext<'input> {}

impl<'input> PrecisionTimestampContextExt<'input>{
	fn new(ctx: &dyn ParameterizedTypeContextAttrs<'input>) -> Rc<ParameterizedTypeContextAll<'input>>  {
		Rc::new(
			ParameterizedTypeContextAll::PrecisionTimestampContext(
				BaseParserRuleContext::copy_from(ctx,PrecisionTimestampContextExt{
					isnull:None, 
        			precision:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type DecimalContext<'input> = BaseParserRuleContext<'input,DecimalContextExt<'input>>;

pub trait DecimalContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token Decimal
	/// Returns `None` if there is no child corresponding to token Decimal
	fn Decimal(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Decimal, 0)
	}
	/// Retrieves first TerminalNode corresponding to token Lt
	/// Returns `None` if there is no child corresponding to token Lt
	fn Lt(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Lt, 0)
	}
	/// Retrieves first TerminalNode corresponding to token Comma
	/// Returns `None` if there is no child corresponding to token Comma
	fn Comma(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Comma, 0)
	}
	/// Retrieves first TerminalNode corresponding to token Gt
	/// Returns `None` if there is no child corresponding to token Gt
	fn Gt(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Gt, 0)
	}
	fn numericParameter_all(&self) ->  Vec<Rc<NumericParameterContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn numericParameter(&self, i: usize) -> Option<Rc<NumericParameterContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token QMark
	/// Returns `None` if there is no child corresponding to token QMark
	fn QMark(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_QMark, 0)
	}
}

impl<'input> DecimalContextAttrs<'input> for DecimalContext<'input>{}

pub struct DecimalContextExt<'input>{
	base:ParameterizedTypeContextExt<'input>,
	pub isnull: Option<TokenType<'input>>,
	pub precision: Option<Rc<NumericParameterContextAll<'input>>>,
	pub scale: Option<Rc<NumericParameterContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{DecimalContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for DecimalContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for DecimalContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_decimal(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_decimal(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for DecimalContext<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_decimal(self);
	}
}

impl<'input> CustomRuleContext<'input> for DecimalContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parameterizedType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parameterizedType }
}

impl<'input> Borrow<ParameterizedTypeContextExt<'input>> for DecimalContext<'input>{
	fn borrow(&self) -> &ParameterizedTypeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ParameterizedTypeContextExt<'input>> for DecimalContext<'input>{
	fn borrow_mut(&mut self) -> &mut ParameterizedTypeContextExt<'input> { &mut self.base }
}

impl<'input> ParameterizedTypeContextAttrs<'input> for DecimalContext<'input> {}

impl<'input> DecimalContextExt<'input>{
	fn new(ctx: &dyn ParameterizedTypeContextAttrs<'input>) -> Rc<ParameterizedTypeContextAll<'input>>  {
		Rc::new(
			ParameterizedTypeContextAll::DecimalContext(
				BaseParserRuleContext::copy_from(ctx,DecimalContextExt{
					isnull:None, 
        			precision:None, scale:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type PrecisionTimeContext<'input> = BaseParserRuleContext<'input,PrecisionTimeContextExt<'input>>;

pub trait PrecisionTimeContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token Precision_Time
	/// Returns `None` if there is no child corresponding to token Precision_Time
	fn Precision_Time(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Precision_Time, 0)
	}
	/// Retrieves first TerminalNode corresponding to token Lt
	/// Returns `None` if there is no child corresponding to token Lt
	fn Lt(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Lt, 0)
	}
	/// Retrieves first TerminalNode corresponding to token Gt
	/// Returns `None` if there is no child corresponding to token Gt
	fn Gt(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Gt, 0)
	}
	fn numericParameter(&self) -> Option<Rc<NumericParameterContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token QMark
	/// Returns `None` if there is no child corresponding to token QMark
	fn QMark(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_QMark, 0)
	}
}

impl<'input> PrecisionTimeContextAttrs<'input> for PrecisionTimeContext<'input>{}

pub struct PrecisionTimeContextExt<'input>{
	base:ParameterizedTypeContextExt<'input>,
	pub isnull: Option<TokenType<'input>>,
	pub precision: Option<Rc<NumericParameterContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{PrecisionTimeContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for PrecisionTimeContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for PrecisionTimeContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_precisionTime(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_precisionTime(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for PrecisionTimeContext<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_precisionTime(self);
	}
}

impl<'input> CustomRuleContext<'input> for PrecisionTimeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parameterizedType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parameterizedType }
}

impl<'input> Borrow<ParameterizedTypeContextExt<'input>> for PrecisionTimeContext<'input>{
	fn borrow(&self) -> &ParameterizedTypeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ParameterizedTypeContextExt<'input>> for PrecisionTimeContext<'input>{
	fn borrow_mut(&mut self) -> &mut ParameterizedTypeContextExt<'input> { &mut self.base }
}

impl<'input> ParameterizedTypeContextAttrs<'input> for PrecisionTimeContext<'input> {}

impl<'input> PrecisionTimeContextExt<'input>{
	fn new(ctx: &dyn ParameterizedTypeContextAttrs<'input>) -> Rc<ParameterizedTypeContextAll<'input>>  {
		Rc::new(
			ParameterizedTypeContextAll::PrecisionTimeContext(
				BaseParserRuleContext::copy_from(ctx,PrecisionTimeContextExt{
					isnull:None, 
        			precision:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type MapContext<'input> = BaseParserRuleContext<'input,MapContextExt<'input>>;

pub trait MapContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token Map
	/// Returns `None` if there is no child corresponding to token Map
	fn Map(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Map, 0)
	}
	/// Retrieves first TerminalNode corresponding to token Lt
	/// Returns `None` if there is no child corresponding to token Lt
	fn Lt(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Lt, 0)
	}
	/// Retrieves first TerminalNode corresponding to token Comma
	/// Returns `None` if there is no child corresponding to token Comma
	fn Comma(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Comma, 0)
	}
	/// Retrieves first TerminalNode corresponding to token Gt
	/// Returns `None` if there is no child corresponding to token Gt
	fn Gt(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Gt, 0)
	}
	fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token QMark
	/// Returns `None` if there is no child corresponding to token QMark
	fn QMark(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_QMark, 0)
	}
}

impl<'input> MapContextAttrs<'input> for MapContext<'input>{}

pub struct MapContextExt<'input>{
	base:ParameterizedTypeContextExt<'input>,
	pub isnull: Option<TokenType<'input>>,
	pub key: Option<Rc<ExprContextAll<'input>>>,
	pub value: Option<Rc<ExprContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{MapContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for MapContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for MapContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_map(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_map(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for MapContext<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_map(self);
	}
}

impl<'input> CustomRuleContext<'input> for MapContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parameterizedType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parameterizedType }
}

impl<'input> Borrow<ParameterizedTypeContextExt<'input>> for MapContext<'input>{
	fn borrow(&self) -> &ParameterizedTypeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ParameterizedTypeContextExt<'input>> for MapContext<'input>{
	fn borrow_mut(&mut self) -> &mut ParameterizedTypeContextExt<'input> { &mut self.base }
}

impl<'input> ParameterizedTypeContextAttrs<'input> for MapContext<'input> {}

impl<'input> MapContextExt<'input>{
	fn new(ctx: &dyn ParameterizedTypeContextAttrs<'input>) -> Rc<ParameterizedTypeContextAll<'input>>  {
		Rc::new(
			ParameterizedTypeContextAll::MapContext(
				BaseParserRuleContext::copy_from(ctx,MapContextExt{
					isnull:None, 
        			key:None, value:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I> SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn parameterizedType(&mut self,)
	-> Result<Rc<ParameterizedTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ParameterizedTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_parameterizedType);
        let mut _localctx: Rc<ParameterizedTypeContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(196);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			SubstraitType_FixedChar 
				=> {
					let tmp = FixedCharContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
					_localctx = tmp;
					{
					recog.base.set_state(38);
					recog.base.match_token(SubstraitType_FixedChar,&mut recog.err_handler)?;

					recog.base.set_state(40);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==SubstraitType_QMark {
						{
						recog.base.set_state(39);
						let tmp = recog.base.match_token(SubstraitType_QMark,&mut recog.err_handler)?;
						if let ParameterizedTypeContextAll::FixedCharContext(ctx) = cast_mut::<_,ParameterizedTypeContextAll >(&mut _localctx){
						ctx.isnull = Some(tmp.clone()); } else {unreachable!("cant cast");}  

						}
					}

					recog.base.set_state(42);
					recog.base.match_token(SubstraitType_Lt,&mut recog.err_handler)?;

					/*InvokeRule numericParameter*/
					recog.base.set_state(43);
					let tmp = recog.numericParameter()?;
					if let ParameterizedTypeContextAll::FixedCharContext(ctx) = cast_mut::<_,ParameterizedTypeContextAll >(&mut _localctx){
					ctx.length = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(44);
					recog.base.match_token(SubstraitType_Gt,&mut recog.err_handler)?;

					}
				}

			SubstraitType_VarChar 
				=> {
					let tmp = VarCharContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2)?;
					_localctx = tmp;
					{
					recog.base.set_state(46);
					recog.base.match_token(SubstraitType_VarChar,&mut recog.err_handler)?;

					recog.base.set_state(48);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==SubstraitType_QMark {
						{
						recog.base.set_state(47);
						let tmp = recog.base.match_token(SubstraitType_QMark,&mut recog.err_handler)?;
						if let ParameterizedTypeContextAll::VarCharContext(ctx) = cast_mut::<_,ParameterizedTypeContextAll >(&mut _localctx){
						ctx.isnull = Some(tmp.clone()); } else {unreachable!("cant cast");}  

						}
					}

					recog.base.set_state(50);
					recog.base.match_token(SubstraitType_Lt,&mut recog.err_handler)?;

					/*InvokeRule numericParameter*/
					recog.base.set_state(51);
					let tmp = recog.numericParameter()?;
					if let ParameterizedTypeContextAll::VarCharContext(ctx) = cast_mut::<_,ParameterizedTypeContextAll >(&mut _localctx){
					ctx.length = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(52);
					recog.base.match_token(SubstraitType_Gt,&mut recog.err_handler)?;

					}
				}

			SubstraitType_FixedBinary 
				=> {
					let tmp = FixedBinaryContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3)?;
					_localctx = tmp;
					{
					recog.base.set_state(54);
					recog.base.match_token(SubstraitType_FixedBinary,&mut recog.err_handler)?;

					recog.base.set_state(56);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==SubstraitType_QMark {
						{
						recog.base.set_state(55);
						let tmp = recog.base.match_token(SubstraitType_QMark,&mut recog.err_handler)?;
						if let ParameterizedTypeContextAll::FixedBinaryContext(ctx) = cast_mut::<_,ParameterizedTypeContextAll >(&mut _localctx){
						ctx.isnull = Some(tmp.clone()); } else {unreachable!("cant cast");}  

						}
					}

					recog.base.set_state(58);
					recog.base.match_token(SubstraitType_Lt,&mut recog.err_handler)?;

					/*InvokeRule numericParameter*/
					recog.base.set_state(59);
					let tmp = recog.numericParameter()?;
					if let ParameterizedTypeContextAll::FixedBinaryContext(ctx) = cast_mut::<_,ParameterizedTypeContextAll >(&mut _localctx){
					ctx.length = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(60);
					recog.base.match_token(SubstraitType_Gt,&mut recog.err_handler)?;

					}
				}

			SubstraitType_Decimal 
				=> {
					let tmp = DecimalContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 4)?;
					_localctx = tmp;
					{
					recog.base.set_state(62);
					recog.base.match_token(SubstraitType_Decimal,&mut recog.err_handler)?;

					recog.base.set_state(64);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==SubstraitType_QMark {
						{
						recog.base.set_state(63);
						let tmp = recog.base.match_token(SubstraitType_QMark,&mut recog.err_handler)?;
						if let ParameterizedTypeContextAll::DecimalContext(ctx) = cast_mut::<_,ParameterizedTypeContextAll >(&mut _localctx){
						ctx.isnull = Some(tmp.clone()); } else {unreachable!("cant cast");}  

						}
					}

					recog.base.set_state(66);
					recog.base.match_token(SubstraitType_Lt,&mut recog.err_handler)?;

					/*InvokeRule numericParameter*/
					recog.base.set_state(67);
					let tmp = recog.numericParameter()?;
					if let ParameterizedTypeContextAll::DecimalContext(ctx) = cast_mut::<_,ParameterizedTypeContextAll >(&mut _localctx){
					ctx.precision = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(68);
					recog.base.match_token(SubstraitType_Comma,&mut recog.err_handler)?;

					/*InvokeRule numericParameter*/
					recog.base.set_state(69);
					let tmp = recog.numericParameter()?;
					if let ParameterizedTypeContextAll::DecimalContext(ctx) = cast_mut::<_,ParameterizedTypeContextAll >(&mut _localctx){
					ctx.scale = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(70);
					recog.base.match_token(SubstraitType_Gt,&mut recog.err_handler)?;

					}
				}

			SubstraitType_Interval_Day 
				=> {
					let tmp = PrecisionIntervalDayContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 5)?;
					_localctx = tmp;
					{
					recog.base.set_state(72);
					recog.base.match_token(SubstraitType_Interval_Day,&mut recog.err_handler)?;

					recog.base.set_state(74);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==SubstraitType_QMark {
						{
						recog.base.set_state(73);
						let tmp = recog.base.match_token(SubstraitType_QMark,&mut recog.err_handler)?;
						if let ParameterizedTypeContextAll::PrecisionIntervalDayContext(ctx) = cast_mut::<_,ParameterizedTypeContextAll >(&mut _localctx){
						ctx.isnull = Some(tmp.clone()); } else {unreachable!("cant cast");}  

						}
					}

					recog.base.set_state(76);
					recog.base.match_token(SubstraitType_Lt,&mut recog.err_handler)?;

					/*InvokeRule numericParameter*/
					recog.base.set_state(77);
					let tmp = recog.numericParameter()?;
					if let ParameterizedTypeContextAll::PrecisionIntervalDayContext(ctx) = cast_mut::<_,ParameterizedTypeContextAll >(&mut _localctx){
					ctx.precision = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(78);
					recog.base.match_token(SubstraitType_Gt,&mut recog.err_handler)?;

					}
				}

			SubstraitType_Interval_Compound 
				=> {
					let tmp = PrecisionIntervalCompoundContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 6)?;
					_localctx = tmp;
					{
					recog.base.set_state(80);
					recog.base.match_token(SubstraitType_Interval_Compound,&mut recog.err_handler)?;

					recog.base.set_state(82);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==SubstraitType_QMark {
						{
						recog.base.set_state(81);
						let tmp = recog.base.match_token(SubstraitType_QMark,&mut recog.err_handler)?;
						if let ParameterizedTypeContextAll::PrecisionIntervalCompoundContext(ctx) = cast_mut::<_,ParameterizedTypeContextAll >(&mut _localctx){
						ctx.isnull = Some(tmp.clone()); } else {unreachable!("cant cast");}  

						}
					}

					recog.base.set_state(84);
					recog.base.match_token(SubstraitType_Lt,&mut recog.err_handler)?;

					/*InvokeRule numericParameter*/
					recog.base.set_state(85);
					let tmp = recog.numericParameter()?;
					if let ParameterizedTypeContextAll::PrecisionIntervalCompoundContext(ctx) = cast_mut::<_,ParameterizedTypeContextAll >(&mut _localctx){
					ctx.precision = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(86);
					recog.base.match_token(SubstraitType_Gt,&mut recog.err_handler)?;

					}
				}

			SubstraitType_Precision_Time 
				=> {
					let tmp = PrecisionTimeContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 7)?;
					_localctx = tmp;
					{
					recog.base.set_state(88);
					recog.base.match_token(SubstraitType_Precision_Time,&mut recog.err_handler)?;

					recog.base.set_state(90);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==SubstraitType_QMark {
						{
						recog.base.set_state(89);
						let tmp = recog.base.match_token(SubstraitType_QMark,&mut recog.err_handler)?;
						if let ParameterizedTypeContextAll::PrecisionTimeContext(ctx) = cast_mut::<_,ParameterizedTypeContextAll >(&mut _localctx){
						ctx.isnull = Some(tmp.clone()); } else {unreachable!("cant cast");}  

						}
					}

					recog.base.set_state(92);
					recog.base.match_token(SubstraitType_Lt,&mut recog.err_handler)?;

					/*InvokeRule numericParameter*/
					recog.base.set_state(93);
					let tmp = recog.numericParameter()?;
					if let ParameterizedTypeContextAll::PrecisionTimeContext(ctx) = cast_mut::<_,ParameterizedTypeContextAll >(&mut _localctx){
					ctx.precision = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(94);
					recog.base.match_token(SubstraitType_Gt,&mut recog.err_handler)?;

					}
				}

			SubstraitType_Precision_Timestamp 
				=> {
					let tmp = PrecisionTimestampContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 8)?;
					_localctx = tmp;
					{
					recog.base.set_state(96);
					recog.base.match_token(SubstraitType_Precision_Timestamp,&mut recog.err_handler)?;

					recog.base.set_state(98);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==SubstraitType_QMark {
						{
						recog.base.set_state(97);
						let tmp = recog.base.match_token(SubstraitType_QMark,&mut recog.err_handler)?;
						if let ParameterizedTypeContextAll::PrecisionTimestampContext(ctx) = cast_mut::<_,ParameterizedTypeContextAll >(&mut _localctx){
						ctx.isnull = Some(tmp.clone()); } else {unreachable!("cant cast");}  

						}
					}

					recog.base.set_state(100);
					recog.base.match_token(SubstraitType_Lt,&mut recog.err_handler)?;

					/*InvokeRule numericParameter*/
					recog.base.set_state(101);
					let tmp = recog.numericParameter()?;
					if let ParameterizedTypeContextAll::PrecisionTimestampContext(ctx) = cast_mut::<_,ParameterizedTypeContextAll >(&mut _localctx){
					ctx.precision = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(102);
					recog.base.match_token(SubstraitType_Gt,&mut recog.err_handler)?;

					}
				}

			SubstraitType_Precision_Timestamp_TZ 
				=> {
					let tmp = PrecisionTimestampTZContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 9)?;
					_localctx = tmp;
					{
					recog.base.set_state(104);
					recog.base.match_token(SubstraitType_Precision_Timestamp_TZ,&mut recog.err_handler)?;

					recog.base.set_state(106);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==SubstraitType_QMark {
						{
						recog.base.set_state(105);
						let tmp = recog.base.match_token(SubstraitType_QMark,&mut recog.err_handler)?;
						if let ParameterizedTypeContextAll::PrecisionTimestampTZContext(ctx) = cast_mut::<_,ParameterizedTypeContextAll >(&mut _localctx){
						ctx.isnull = Some(tmp.clone()); } else {unreachable!("cant cast");}  

						}
					}

					recog.base.set_state(108);
					recog.base.match_token(SubstraitType_Lt,&mut recog.err_handler)?;

					/*InvokeRule numericParameter*/
					recog.base.set_state(109);
					let tmp = recog.numericParameter()?;
					if let ParameterizedTypeContextAll::PrecisionTimestampTZContext(ctx) = cast_mut::<_,ParameterizedTypeContextAll >(&mut _localctx){
					ctx.precision = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(110);
					recog.base.match_token(SubstraitType_Gt,&mut recog.err_handler)?;

					}
				}

			SubstraitType_Struct 
				=> {
					let tmp = StructContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 10)?;
					_localctx = tmp;
					{
					recog.base.set_state(112);
					recog.base.match_token(SubstraitType_Struct,&mut recog.err_handler)?;

					recog.base.set_state(114);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==SubstraitType_QMark {
						{
						recog.base.set_state(113);
						let tmp = recog.base.match_token(SubstraitType_QMark,&mut recog.err_handler)?;
						if let ParameterizedTypeContextAll::StructContext(ctx) = cast_mut::<_,ParameterizedTypeContextAll >(&mut _localctx){
						ctx.isnull = Some(tmp.clone()); } else {unreachable!("cant cast");}  

						}
					}

					recog.base.set_state(116);
					recog.base.match_token(SubstraitType_Lt,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(117);
					recog.expr_rec(0)?;

					recog.base.set_state(122);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==SubstraitType_Comma {
						{
						{
						recog.base.set_state(118);
						recog.base.match_token(SubstraitType_Comma,&mut recog.err_handler)?;

						/*InvokeRule expr*/
						recog.base.set_state(119);
						recog.expr_rec(0)?;

						}
						}
						recog.base.set_state(124);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(125);
					recog.base.match_token(SubstraitType_Gt,&mut recog.err_handler)?;

					}
				}

			SubstraitType_NStruct 
				=> {
					let tmp = NStructContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 11)?;
					_localctx = tmp;
					{
					recog.base.set_state(127);
					recog.base.match_token(SubstraitType_NStruct,&mut recog.err_handler)?;

					recog.base.set_state(129);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==SubstraitType_QMark {
						{
						recog.base.set_state(128);
						let tmp = recog.base.match_token(SubstraitType_QMark,&mut recog.err_handler)?;
						if let ParameterizedTypeContextAll::NStructContext(ctx) = cast_mut::<_,ParameterizedTypeContextAll >(&mut _localctx){
						ctx.isnull = Some(tmp.clone()); } else {unreachable!("cant cast");}  

						}
					}

					recog.base.set_state(131);
					recog.base.match_token(SubstraitType_Lt,&mut recog.err_handler)?;

					recog.base.set_state(132);
					recog.base.match_token(SubstraitType_Identifier,&mut recog.err_handler)?;

					recog.base.set_state(133);
					recog.base.match_token(SubstraitType_Colon,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(134);
					recog.expr_rec(0)?;

					recog.base.set_state(141);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==SubstraitType_Comma {
						{
						{
						recog.base.set_state(135);
						recog.base.match_token(SubstraitType_Comma,&mut recog.err_handler)?;

						recog.base.set_state(136);
						recog.base.match_token(SubstraitType_Identifier,&mut recog.err_handler)?;

						recog.base.set_state(137);
						recog.base.match_token(SubstraitType_Colon,&mut recog.err_handler)?;

						/*InvokeRule expr*/
						recog.base.set_state(138);
						recog.expr_rec(0)?;

						}
						}
						recog.base.set_state(143);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(144);
					recog.base.match_token(SubstraitType_Gt,&mut recog.err_handler)?;

					}
				}

			SubstraitType_List 
				=> {
					let tmp = ListContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 12)?;
					_localctx = tmp;
					{
					recog.base.set_state(146);
					recog.base.match_token(SubstraitType_List,&mut recog.err_handler)?;

					recog.base.set_state(148);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==SubstraitType_QMark {
						{
						recog.base.set_state(147);
						let tmp = recog.base.match_token(SubstraitType_QMark,&mut recog.err_handler)?;
						if let ParameterizedTypeContextAll::ListContext(ctx) = cast_mut::<_,ParameterizedTypeContextAll >(&mut _localctx){
						ctx.isnull = Some(tmp.clone()); } else {unreachable!("cant cast");}  

						}
					}

					recog.base.set_state(150);
					recog.base.match_token(SubstraitType_Lt,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(151);
					recog.expr_rec(0)?;

					recog.base.set_state(152);
					recog.base.match_token(SubstraitType_Gt,&mut recog.err_handler)?;

					}
				}

			SubstraitType_Map 
				=> {
					let tmp = MapContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 13)?;
					_localctx = tmp;
					{
					recog.base.set_state(154);
					recog.base.match_token(SubstraitType_Map,&mut recog.err_handler)?;

					recog.base.set_state(156);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==SubstraitType_QMark {
						{
						recog.base.set_state(155);
						let tmp = recog.base.match_token(SubstraitType_QMark,&mut recog.err_handler)?;
						if let ParameterizedTypeContextAll::MapContext(ctx) = cast_mut::<_,ParameterizedTypeContextAll >(&mut _localctx){
						ctx.isnull = Some(tmp.clone()); } else {unreachable!("cant cast");}  

						}
					}

					recog.base.set_state(158);
					recog.base.match_token(SubstraitType_Lt,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(159);
					let tmp = recog.expr_rec(0)?;
					if let ParameterizedTypeContextAll::MapContext(ctx) = cast_mut::<_,ParameterizedTypeContextAll >(&mut _localctx){
					ctx.key = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(160);
					recog.base.match_token(SubstraitType_Comma,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(161);
					let tmp = recog.expr_rec(0)?;
					if let ParameterizedTypeContextAll::MapContext(ctx) = cast_mut::<_,ParameterizedTypeContextAll >(&mut _localctx){
					ctx.value = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(162);
					recog.base.match_token(SubstraitType_Gt,&mut recog.err_handler)?;

					}
				}

			SubstraitType_Func 
				=> {
					let tmp = FuncContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 14)?;
					_localctx = tmp;
					{
					recog.base.set_state(164);
					recog.base.match_token(SubstraitType_Func,&mut recog.err_handler)?;

					recog.base.set_state(166);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==SubstraitType_QMark {
						{
						recog.base.set_state(165);
						let tmp = recog.base.match_token(SubstraitType_QMark,&mut recog.err_handler)?;
						if let ParameterizedTypeContextAll::FuncContext(ctx) = cast_mut::<_,ParameterizedTypeContextAll >(&mut _localctx){
						ctx.isnull = Some(tmp.clone()); } else {unreachable!("cant cast");}  

						}
					}

					recog.base.set_state(168);
					recog.base.match_token(SubstraitType_Lt,&mut recog.err_handler)?;

					/*InvokeRule funcParams*/
					recog.base.set_state(169);
					let tmp = recog.funcParams()?;
					if let ParameterizedTypeContextAll::FuncContext(ctx) = cast_mut::<_,ParameterizedTypeContextAll >(&mut _localctx){
					ctx.params = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(170);
					recog.base.match_token(SubstraitType_Arrow,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(171);
					let tmp = recog.expr_rec(0)?;
					if let ParameterizedTypeContextAll::FuncContext(ctx) = cast_mut::<_,ParameterizedTypeContextAll >(&mut _localctx){
					ctx.returnType = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(172);
					recog.base.match_token(SubstraitType_Gt,&mut recog.err_handler)?;

					}
				}

			SubstraitType_UserDefined |SubstraitType_Identifier 
				=> {
					let tmp = UserDefinedContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 15)?;
					_localctx = tmp;
					{
					recog.base.set_state(176);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==SubstraitType_Identifier {
						{
						recog.base.set_state(174);
						let tmp = recog.base.match_token(SubstraitType_Identifier,&mut recog.err_handler)?;
						if let ParameterizedTypeContextAll::UserDefinedContext(ctx) = cast_mut::<_,ParameterizedTypeContextAll >(&mut _localctx){
						ctx.dependencyAlias = Some(tmp.clone()); } else {unreachable!("cant cast");}  

						recog.base.set_state(175);
						recog.base.match_token(SubstraitType_Dot,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(178);
					recog.base.match_token(SubstraitType_UserDefined,&mut recog.err_handler)?;

					recog.base.set_state(179);
					recog.base.match_token(SubstraitType_Identifier,&mut recog.err_handler)?;

					recog.base.set_state(181);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(18,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(180);
							let tmp = recog.base.match_token(SubstraitType_QMark,&mut recog.err_handler)?;
							if let ParameterizedTypeContextAll::UserDefinedContext(ctx) = cast_mut::<_,ParameterizedTypeContextAll >(&mut _localctx){
							ctx.isnull = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
						}

						_ => {}
					}
					recog.base.set_state(194);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(20,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(183);
							recog.base.match_token(SubstraitType_Lt,&mut recog.err_handler)?;

							/*InvokeRule expr*/
							recog.base.set_state(184);
							recog.expr_rec(0)?;

							recog.base.set_state(189);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							while _la==SubstraitType_Comma {
								{
								{
								recog.base.set_state(185);
								recog.base.match_token(SubstraitType_Comma,&mut recog.err_handler)?;

								/*InvokeRule expr*/
								recog.base.set_state(186);
								recog.expr_rec(0)?;

								}
								}
								recog.base.set_state(191);
								recog.err_handler.sync(&mut recog.base)?;
								_la = recog.base.input.la(1);
							}
							recog.base.set_state(192);
							recog.base.match_token(SubstraitType_Gt,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
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
//------------------- funcParams ----------------
#[derive(Debug)]
pub enum FuncParamsContextAll<'input>{
	SingleFuncParamContext(SingleFuncParamContext<'input>),
	FuncParamsWithParensContext(FuncParamsWithParensContext<'input>),
Error(FuncParamsContext<'input>)
}
antlr4rust::tid!{FuncParamsContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for FuncParamsContextAll<'input>{}

impl<'input> SubstraitTypeParserContext<'input> for FuncParamsContextAll<'input>{}

impl<'input> Deref for FuncParamsContextAll<'input>{
	type Target = dyn FuncParamsContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use FuncParamsContextAll::*;
		match self{
			SingleFuncParamContext(inner) => inner,
			FuncParamsWithParensContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for FuncParamsContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for FuncParamsContextAll<'input>{
    fn enter(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
}



pub type FuncParamsContext<'input> = BaseParserRuleContext<'input,FuncParamsContextExt<'input>>;

#[derive(Clone)]
pub struct FuncParamsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitTypeParserContext<'input> for FuncParamsContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for FuncParamsContext<'input>{
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for FuncParamsContext<'input>{
}

impl<'input> CustomRuleContext<'input> for FuncParamsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_funcParams }
	//fn type_rule_index() -> usize where Self: Sized { RULE_funcParams }
}
antlr4rust::tid!{FuncParamsContextExt<'a>}

impl<'input> FuncParamsContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<FuncParamsContextAll<'input>> {
		Rc::new(
		FuncParamsContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FuncParamsContextExt{

				ph:PhantomData
			}),
		)
		)
	}
}

pub trait FuncParamsContextAttrs<'input>: SubstraitTypeParserContext<'input> + BorrowMut<FuncParamsContextExt<'input>>{


}

impl<'input> FuncParamsContextAttrs<'input> for FuncParamsContext<'input>{}

pub type SingleFuncParamContext<'input> = BaseParserRuleContext<'input,SingleFuncParamContextExt<'input>>;

pub trait SingleFuncParamContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> SingleFuncParamContextAttrs<'input> for SingleFuncParamContext<'input>{}

pub struct SingleFuncParamContextExt<'input>{
	base:FuncParamsContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{SingleFuncParamContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for SingleFuncParamContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for SingleFuncParamContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_singleFuncParam(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_singleFuncParam(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for SingleFuncParamContext<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_singleFuncParam(self);
	}
}

impl<'input> CustomRuleContext<'input> for SingleFuncParamContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_funcParams }
	//fn type_rule_index() -> usize where Self: Sized { RULE_funcParams }
}

impl<'input> Borrow<FuncParamsContextExt<'input>> for SingleFuncParamContext<'input>{
	fn borrow(&self) -> &FuncParamsContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<FuncParamsContextExt<'input>> for SingleFuncParamContext<'input>{
	fn borrow_mut(&mut self) -> &mut FuncParamsContextExt<'input> { &mut self.base }
}

impl<'input> FuncParamsContextAttrs<'input> for SingleFuncParamContext<'input> {}

impl<'input> SingleFuncParamContextExt<'input>{
	fn new(ctx: &dyn FuncParamsContextAttrs<'input>) -> Rc<FuncParamsContextAll<'input>>  {
		Rc::new(
			FuncParamsContextAll::SingleFuncParamContext(
				BaseParserRuleContext::copy_from(ctx,SingleFuncParamContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type FuncParamsWithParensContext<'input> = BaseParserRuleContext<'input,FuncParamsWithParensContextExt<'input>>;

pub trait FuncParamsWithParensContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token OParen
	/// Returns `None` if there is no child corresponding to token OParen
	fn OParen(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_OParen, 0)
	}
	fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token CParen
	/// Returns `None` if there is no child corresponding to token CParen
	fn CParen(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_CParen, 0)
	}
	/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
	fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
	/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
	fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Comma, i)
	}
}

impl<'input> FuncParamsWithParensContextAttrs<'input> for FuncParamsWithParensContext<'input>{}

pub struct FuncParamsWithParensContextExt<'input>{
	base:FuncParamsContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{FuncParamsWithParensContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for FuncParamsWithParensContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for FuncParamsWithParensContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_funcParamsWithParens(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_funcParamsWithParens(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for FuncParamsWithParensContext<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_funcParamsWithParens(self);
	}
}

impl<'input> CustomRuleContext<'input> for FuncParamsWithParensContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_funcParams }
	//fn type_rule_index() -> usize where Self: Sized { RULE_funcParams }
}

impl<'input> Borrow<FuncParamsContextExt<'input>> for FuncParamsWithParensContext<'input>{
	fn borrow(&self) -> &FuncParamsContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<FuncParamsContextExt<'input>> for FuncParamsWithParensContext<'input>{
	fn borrow_mut(&mut self) -> &mut FuncParamsContextExt<'input> { &mut self.base }
}

impl<'input> FuncParamsContextAttrs<'input> for FuncParamsWithParensContext<'input> {}

impl<'input> FuncParamsWithParensContextExt<'input>{
	fn new(ctx: &dyn FuncParamsContextAttrs<'input>) -> Rc<FuncParamsContextAll<'input>>  {
		Rc::new(
			FuncParamsContextAll::FuncParamsWithParensContext(
				BaseParserRuleContext::copy_from(ctx,FuncParamsWithParensContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I> SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn funcParams(&mut self,)
	-> Result<Rc<FuncParamsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FuncParamsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_funcParams);
        let mut _localctx: Rc<FuncParamsContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(210);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(23,&mut recog.base)? {
				1 =>{
					let tmp = SingleFuncParamContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
					_localctx = tmp;
					{
					/*InvokeRule expr*/
					recog.base.set_state(198);
					recog.expr_rec(0)?;

					}
				}
			,
				2 =>{
					let tmp = FuncParamsWithParensContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2)?;
					_localctx = tmp;
					{
					recog.base.set_state(199);
					recog.base.match_token(SubstraitType_OParen,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(200);
					recog.expr_rec(0)?;

					recog.base.set_state(205);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==SubstraitType_Comma {
						{
						{
						recog.base.set_state(201);
						recog.base.match_token(SubstraitType_Comma,&mut recog.err_handler)?;

						/*InvokeRule expr*/
						recog.base.set_state(202);
						recog.expr_rec(0)?;

						}
						}
						recog.base.set_state(207);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(208);
					recog.base.match_token(SubstraitType_CParen,&mut recog.err_handler)?;

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
//------------------- numericParameter ----------------
#[derive(Debug)]
pub enum NumericParameterContextAll<'input>{
	NumericParameterNameContext(NumericParameterNameContext<'input>),
	NumericLiteralContext(NumericLiteralContext<'input>),
	NumericExpressionContext(NumericExpressionContext<'input>),
Error(NumericParameterContext<'input>)
}
antlr4rust::tid!{NumericParameterContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for NumericParameterContextAll<'input>{}

impl<'input> SubstraitTypeParserContext<'input> for NumericParameterContextAll<'input>{}

impl<'input> Deref for NumericParameterContextAll<'input>{
	type Target = dyn NumericParameterContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use NumericParameterContextAll::*;
		match self{
			NumericParameterNameContext(inner) => inner,
			NumericLiteralContext(inner) => inner,
			NumericExpressionContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for NumericParameterContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for NumericParameterContextAll<'input>{
    fn enter(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
}



pub type NumericParameterContext<'input> = BaseParserRuleContext<'input,NumericParameterContextExt<'input>>;

#[derive(Clone)]
pub struct NumericParameterContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitTypeParserContext<'input> for NumericParameterContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for NumericParameterContext<'input>{
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for NumericParameterContext<'input>{
}

impl<'input> CustomRuleContext<'input> for NumericParameterContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_numericParameter }
	//fn type_rule_index() -> usize where Self: Sized { RULE_numericParameter }
}
antlr4rust::tid!{NumericParameterContextExt<'a>}

impl<'input> NumericParameterContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<NumericParameterContextAll<'input>> {
		Rc::new(
		NumericParameterContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NumericParameterContextExt{

				ph:PhantomData
			}),
		)
		)
	}
}

pub trait NumericParameterContextAttrs<'input>: SubstraitTypeParserContext<'input> + BorrowMut<NumericParameterContextExt<'input>>{


}

impl<'input> NumericParameterContextAttrs<'input> for NumericParameterContext<'input>{}

pub type NumericParameterNameContext<'input> = BaseParserRuleContext<'input,NumericParameterNameContextExt<'input>>;

pub trait NumericParameterNameContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token Identifier
	/// Returns `None` if there is no child corresponding to token Identifier
	fn Identifier(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Identifier, 0)
	}
}

impl<'input> NumericParameterNameContextAttrs<'input> for NumericParameterNameContext<'input>{}

pub struct NumericParameterNameContextExt<'input>{
	base:NumericParameterContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{NumericParameterNameContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for NumericParameterNameContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for NumericParameterNameContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_numericParameterName(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_numericParameterName(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for NumericParameterNameContext<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_numericParameterName(self);
	}
}

impl<'input> CustomRuleContext<'input> for NumericParameterNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_numericParameter }
	//fn type_rule_index() -> usize where Self: Sized { RULE_numericParameter }
}

impl<'input> Borrow<NumericParameterContextExt<'input>> for NumericParameterNameContext<'input>{
	fn borrow(&self) -> &NumericParameterContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<NumericParameterContextExt<'input>> for NumericParameterNameContext<'input>{
	fn borrow_mut(&mut self) -> &mut NumericParameterContextExt<'input> { &mut self.base }
}

impl<'input> NumericParameterContextAttrs<'input> for NumericParameterNameContext<'input> {}

impl<'input> NumericParameterNameContextExt<'input>{
	fn new(ctx: &dyn NumericParameterContextAttrs<'input>) -> Rc<NumericParameterContextAll<'input>>  {
		Rc::new(
			NumericParameterContextAll::NumericParameterNameContext(
				BaseParserRuleContext::copy_from(ctx,NumericParameterNameContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type NumericLiteralContext<'input> = BaseParserRuleContext<'input,NumericLiteralContextExt<'input>>;

pub trait NumericLiteralContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token Number
	/// Returns `None` if there is no child corresponding to token Number
	fn Number(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Number, 0)
	}
}

impl<'input> NumericLiteralContextAttrs<'input> for NumericLiteralContext<'input>{}

pub struct NumericLiteralContextExt<'input>{
	base:NumericParameterContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{NumericLiteralContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for NumericLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for NumericLiteralContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_numericLiteral(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_numericLiteral(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for NumericLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_numericLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for NumericLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_numericParameter }
	//fn type_rule_index() -> usize where Self: Sized { RULE_numericParameter }
}

impl<'input> Borrow<NumericParameterContextExt<'input>> for NumericLiteralContext<'input>{
	fn borrow(&self) -> &NumericParameterContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<NumericParameterContextExt<'input>> for NumericLiteralContext<'input>{
	fn borrow_mut(&mut self) -> &mut NumericParameterContextExt<'input> { &mut self.base }
}

impl<'input> NumericParameterContextAttrs<'input> for NumericLiteralContext<'input> {}

impl<'input> NumericLiteralContextExt<'input>{
	fn new(ctx: &dyn NumericParameterContextAttrs<'input>) -> Rc<NumericParameterContextAll<'input>>  {
		Rc::new(
			NumericParameterContextAll::NumericLiteralContext(
				BaseParserRuleContext::copy_from(ctx,NumericLiteralContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type NumericExpressionContext<'input> = BaseParserRuleContext<'input,NumericExpressionContextExt<'input>>;

pub trait NumericExpressionContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> NumericExpressionContextAttrs<'input> for NumericExpressionContext<'input>{}

pub struct NumericExpressionContextExt<'input>{
	base:NumericParameterContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{NumericExpressionContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for NumericExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for NumericExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_numericExpression(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_numericExpression(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for NumericExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_numericExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for NumericExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_numericParameter }
	//fn type_rule_index() -> usize where Self: Sized { RULE_numericParameter }
}

impl<'input> Borrow<NumericParameterContextExt<'input>> for NumericExpressionContext<'input>{
	fn borrow(&self) -> &NumericParameterContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<NumericParameterContextExt<'input>> for NumericExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut NumericParameterContextExt<'input> { &mut self.base }
}

impl<'input> NumericParameterContextAttrs<'input> for NumericExpressionContext<'input> {}

impl<'input> NumericExpressionContextExt<'input>{
	fn new(ctx: &dyn NumericParameterContextAttrs<'input>) -> Rc<NumericParameterContextAll<'input>>  {
		Rc::new(
			NumericParameterContextAll::NumericExpressionContext(
				BaseParserRuleContext::copy_from(ctx,NumericExpressionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I> SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn numericParameter(&mut self,)
	-> Result<Rc<NumericParameterContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NumericParameterContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_numericParameter);
        let mut _localctx: Rc<NumericParameterContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(215);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(24,&mut recog.base)? {
				1 =>{
					let tmp = NumericLiteralContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
					_localctx = tmp;
					{
					recog.base.set_state(212);
					recog.base.match_token(SubstraitType_Number,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					let tmp = NumericParameterNameContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2)?;
					_localctx = tmp;
					{
					recog.base.set_state(213);
					recog.base.match_token(SubstraitType_Identifier,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					let tmp = NumericExpressionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3)?;
					_localctx = tmp;
					{
					/*InvokeRule expr*/
					recog.base.set_state(214);
					recog.expr_rec(0)?;

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
//------------------- anyType ----------------
pub type AnyTypeContextAll<'input> = AnyTypeContext<'input>;


pub type AnyTypeContext<'input> = BaseParserRuleContext<'input,AnyTypeContextExt<'input>>;

#[derive(Clone)]
pub struct AnyTypeContextExt<'input>{
	pub isnull: Option<TokenType<'input>>,
ph:PhantomData<&'input str>
}

impl<'input> SubstraitTypeParserContext<'input> for AnyTypeContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for AnyTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_anyType(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_anyType(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for AnyTypeContext<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_anyType(self);
	}
}

impl<'input> CustomRuleContext<'input> for AnyTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_anyType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_anyType }
}
antlr4rust::tid!{AnyTypeContextExt<'a>}

impl<'input> AnyTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<AnyTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AnyTypeContextExt{
				isnull: None, 

				ph:PhantomData
			}),
		)
	}
}

pub trait AnyTypeContextAttrs<'input>: SubstraitTypeParserContext<'input> + BorrowMut<AnyTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Any
/// Returns `None` if there is no child corresponding to token Any
fn Any(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
	self.get_token(SubstraitType_Any, 0)
}
/// Retrieves first TerminalNode corresponding to token QMark
/// Returns `None` if there is no child corresponding to token QMark
fn QMark(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
	self.get_token(SubstraitType_QMark, 0)
}
/// Retrieves first TerminalNode corresponding to token AnyVar
/// Returns `None` if there is no child corresponding to token AnyVar
fn AnyVar(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
	self.get_token(SubstraitType_AnyVar, 0)
}

}

impl<'input> AnyTypeContextAttrs<'input> for AnyTypeContext<'input>{}

impl<'input, I> SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn anyType(&mut self,)
	-> Result<Rc<AnyTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AnyTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_anyType);
        let mut _localctx: Rc<AnyTypeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(225);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			SubstraitType_Any 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					recog.base.set_state(217);
					recog.base.match_token(SubstraitType_Any,&mut recog.err_handler)?;

					recog.base.set_state(219);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(25,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(218);
							let tmp = recog.base.match_token(SubstraitType_QMark,&mut recog.err_handler)?;
							 cast_mut::<_,AnyTypeContext >(&mut _localctx).isnull = Some(tmp.clone());
							  

							}
						}

						_ => {}
					}
					}
				}

			SubstraitType_AnyVar 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					recog.base.set_state(221);
					recog.base.match_token(SubstraitType_AnyVar,&mut recog.err_handler)?;

					recog.base.set_state(223);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(26,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(222);
							let tmp = recog.base.match_token(SubstraitType_QMark,&mut recog.err_handler)?;
							 cast_mut::<_,AnyTypeContext >(&mut _localctx).isnull = Some(tmp.clone());
							  

							}
						}

						_ => {}
					}
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
//------------------- typeDef ----------------
pub type TypeDefContextAll<'input> = TypeDefContext<'input>;


pub type TypeDefContext<'input> = BaseParserRuleContext<'input,TypeDefContextExt<'input>>;

#[derive(Clone)]
pub struct TypeDefContextExt<'input>{
	pub isnull: Option<TokenType<'input>>,
ph:PhantomData<&'input str>
}

impl<'input> SubstraitTypeParserContext<'input> for TypeDefContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for TypeDefContext<'input>{
		fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_typeDef(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_typeDef(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for TypeDefContext<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_typeDef(self);
	}
}

impl<'input> CustomRuleContext<'input> for TypeDefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typeDef }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typeDef }
}
antlr4rust::tid!{TypeDefContextExt<'a>}

impl<'input> TypeDefContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<TypeDefContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypeDefContextExt{
				isnull: None, 

				ph:PhantomData
			}),
		)
	}
}

pub trait TypeDefContextAttrs<'input>: SubstraitTypeParserContext<'input> + BorrowMut<TypeDefContextExt<'input>>{

fn scalarType(&self) -> Option<Rc<ScalarTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token QMark
/// Returns `None` if there is no child corresponding to token QMark
fn QMark(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
	self.get_token(SubstraitType_QMark, 0)
}
fn parameterizedType(&self) -> Option<Rc<ParameterizedTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn anyType(&self) -> Option<Rc<AnyTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> TypeDefContextAttrs<'input> for TypeDefContext<'input>{}

impl<'input, I> SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn typeDef(&mut self,)
	-> Result<Rc<TypeDefContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TypeDefContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_typeDef);
        let mut _localctx: Rc<TypeDefContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(233);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			SubstraitType_Boolean |SubstraitType_I8 |SubstraitType_I16 |SubstraitType_I32 |
			SubstraitType_I64 |SubstraitType_FP32 |SubstraitType_FP64 |SubstraitType_String |
			SubstraitType_Binary |SubstraitType_Date |SubstraitType_Interval_Year |
			SubstraitType_UUID 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule scalarType*/
					recog.base.set_state(227);
					recog.scalarType()?;

					recog.base.set_state(229);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(28,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(228);
							let tmp = recog.base.match_token(SubstraitType_QMark,&mut recog.err_handler)?;
							 cast_mut::<_,TypeDefContext >(&mut _localctx).isnull = Some(tmp.clone());
							  

							}
						}

						_ => {}
					}
					}
				}

			SubstraitType_Func |SubstraitType_Interval_Day |SubstraitType_Interval_Compound |
			SubstraitType_Decimal |SubstraitType_Precision_Time |SubstraitType_Precision_Timestamp |
			SubstraitType_Precision_Timestamp_TZ |SubstraitType_FixedChar |SubstraitType_VarChar |
			SubstraitType_FixedBinary |SubstraitType_Struct |SubstraitType_NStruct |
			SubstraitType_List |SubstraitType_Map |SubstraitType_UserDefined |SubstraitType_Identifier 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule parameterizedType*/
					recog.base.set_state(231);
					recog.parameterizedType()?;

					}
				}

			SubstraitType_Any |SubstraitType_AnyVar 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule anyType*/
					recog.base.set_state(232);
					recog.anyType()?;

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
//------------------- expr ----------------
#[derive(Debug)]
pub enum ExprContextAll<'input>{
	IfExprContext(IfExprContext<'input>),
	OrContext(OrContext<'input>),
	MultilineDefinitionContext(MultilineDefinitionContext<'input>),
	MulDivContext(MulDivContext<'input>),
	AddSubContext(AddSubContext<'input>),
	TernaryContext(TernaryContext<'input>),
	ParameterNameContext(ParameterNameContext<'input>),
	TypeLiteralContext(TypeLiteralContext<'input>),
	ComparisonContext(ComparisonContext<'input>),
	AndContext(AndContext<'input>),
	ParenExpressionContext(ParenExpressionContext<'input>),
	FunctionCallContext(FunctionCallContext<'input>),
	NotExprContext(NotExprContext<'input>),
	EqualityContext(EqualityContext<'input>),
	LiteralNumberContext(LiteralNumberContext<'input>),
Error(ExprContext<'input>)
}
antlr4rust::tid!{ExprContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for ExprContextAll<'input>{}

impl<'input> SubstraitTypeParserContext<'input> for ExprContextAll<'input>{}

impl<'input> Deref for ExprContextAll<'input>{
	type Target = dyn ExprContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use ExprContextAll::*;
		match self{
			IfExprContext(inner) => inner,
			OrContext(inner) => inner,
			MultilineDefinitionContext(inner) => inner,
			MulDivContext(inner) => inner,
			AddSubContext(inner) => inner,
			TernaryContext(inner) => inner,
			ParameterNameContext(inner) => inner,
			TypeLiteralContext(inner) => inner,
			ComparisonContext(inner) => inner,
			AndContext(inner) => inner,
			ParenExpressionContext(inner) => inner,
			FunctionCallContext(inner) => inner,
			NotExprContext(inner) => inner,
			EqualityContext(inner) => inner,
			LiteralNumberContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for ExprContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for ExprContextAll<'input>{
    fn enter(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
}



pub type ExprContext<'input> = BaseParserRuleContext<'input,ExprContextExt<'input>>;

#[derive(Clone)]
pub struct ExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SubstraitTypeParserContext<'input> for ExprContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for ExprContext<'input>{
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for ExprContext<'input>{
}

impl<'input> CustomRuleContext<'input> for ExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}
antlr4rust::tid!{ExprContextExt<'a>}

impl<'input> ExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn SubstraitTypeParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ExprContextAll<'input>> {
		Rc::new(
		ExprContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExprContextExt{

				ph:PhantomData
			}),
		)
		)
	}
}

pub trait ExprContextAttrs<'input>: SubstraitTypeParserContext<'input> + BorrowMut<ExprContextExt<'input>>{


}

impl<'input> ExprContextAttrs<'input> for ExprContext<'input>{}

pub type IfExprContext<'input> = BaseParserRuleContext<'input,IfExprContextExt<'input>>;

pub trait IfExprContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token If
	/// Returns `None` if there is no child corresponding to token If
	fn If(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_If, 0)
	}
	/// Retrieves first TerminalNode corresponding to token Then
	/// Returns `None` if there is no child corresponding to token Then
	fn Then(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Then, 0)
	}
	/// Retrieves first TerminalNode corresponding to token Else
	/// Returns `None` if there is no child corresponding to token Else
	fn Else(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Else, 0)
	}
	fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> IfExprContextAttrs<'input> for IfExprContext<'input>{}

pub struct IfExprContextExt<'input>{
	base:ExprContextExt<'input>,
	pub ifExpr: Option<Rc<ExprContextAll<'input>>>,
	pub thenExpr: Option<Rc<ExprContextAll<'input>>>,
	pub elseExpr: Option<Rc<ExprContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{IfExprContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for IfExprContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for IfExprContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_IfExpr(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_IfExpr(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for IfExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_IfExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for IfExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for IfExprContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for IfExprContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for IfExprContext<'input> {}

impl<'input> IfExprContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::IfExprContext(
				BaseParserRuleContext::copy_from(ctx,IfExprContextExt{
        			ifExpr:None, thenExpr:None, elseExpr:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type OrContext<'input> = BaseParserRuleContext<'input,OrContextExt<'input>>;

pub trait OrContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token Or
	/// Returns `None` if there is no child corresponding to token Or
	fn Or(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Or, 0)
	}
}

impl<'input> OrContextAttrs<'input> for OrContext<'input>{}

pub struct OrContextExt<'input>{
	base:ExprContextExt<'input>,
	pub left: Option<Rc<ExprContextAll<'input>>>,
	pub op: Option<TokenType<'input>>,
	pub right: Option<Rc<ExprContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{OrContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for OrContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for OrContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_Or(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_Or(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for OrContext<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_Or(self);
	}
}

impl<'input> CustomRuleContext<'input> for OrContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for OrContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for OrContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for OrContext<'input> {}

impl<'input> OrContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::OrContext(
				BaseParserRuleContext::copy_from(ctx,OrContextExt{
					op:None, 
        			left:None, right:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type MultilineDefinitionContext<'input> = BaseParserRuleContext<'input,MultilineDefinitionContextExt<'input>>;

pub trait MultilineDefinitionContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves all `TerminalNode`s corresponding to token Identifier in current rule
	fn Identifier_all(&self) -> Vec<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token Identifier, starting from 0.
	/// Returns `None` if number of children corresponding to token Identifier is less or equal than `i`.
	fn Identifier(&self, i: usize) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Identifier, i)
	}
	/// Retrieves all `TerminalNode`s corresponding to token Eq in current rule
	fn Eq_all(&self) -> Vec<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token Eq, starting from 0.
	/// Returns `None` if number of children corresponding to token Eq is less or equal than `i`.
	fn Eq(&self, i: usize) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Eq, i)
	}
	fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	fn typeDef(&self) -> Option<Rc<TypeDefContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves all `TerminalNode`s corresponding to token Newline in current rule
	fn Newline_all(&self) -> Vec<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token Newline, starting from 0.
	/// Returns `None` if number of children corresponding to token Newline is less or equal than `i`.
	fn Newline(&self, i: usize) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Newline, i)
	}
}

impl<'input> MultilineDefinitionContextAttrs<'input> for MultilineDefinitionContext<'input>{}

pub struct MultilineDefinitionContextExt<'input>{
	base:ExprContextExt<'input>,
	pub finalType: Option<Rc<TypeDefContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{MultilineDefinitionContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for MultilineDefinitionContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for MultilineDefinitionContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_MultilineDefinition(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_MultilineDefinition(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for MultilineDefinitionContext<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_MultilineDefinition(self);
	}
}

impl<'input> CustomRuleContext<'input> for MultilineDefinitionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for MultilineDefinitionContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for MultilineDefinitionContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for MultilineDefinitionContext<'input> {}

impl<'input> MultilineDefinitionContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::MultilineDefinitionContext(
				BaseParserRuleContext::copy_from(ctx,MultilineDefinitionContextExt{
        			finalType:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type MulDivContext<'input> = BaseParserRuleContext<'input,MulDivContextExt<'input>>;

pub trait MulDivContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token Asterisk
	/// Returns `None` if there is no child corresponding to token Asterisk
	fn Asterisk(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Asterisk, 0)
	}
	/// Retrieves first TerminalNode corresponding to token ForwardSlash
	/// Returns `None` if there is no child corresponding to token ForwardSlash
	fn ForwardSlash(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_ForwardSlash, 0)
	}
}

impl<'input> MulDivContextAttrs<'input> for MulDivContext<'input>{}

pub struct MulDivContextExt<'input>{
	base:ExprContextExt<'input>,
	pub left: Option<Rc<ExprContextAll<'input>>>,
	pub op: Option<TokenType<'input>>,
	pub right: Option<Rc<ExprContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{MulDivContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for MulDivContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for MulDivContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_MulDiv(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_MulDiv(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for MulDivContext<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_MulDiv(self);
	}
}

impl<'input> CustomRuleContext<'input> for MulDivContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for MulDivContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for MulDivContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for MulDivContext<'input> {}

impl<'input> MulDivContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::MulDivContext(
				BaseParserRuleContext::copy_from(ctx,MulDivContextExt{
					op:None, 
        			left:None, right:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type AddSubContext<'input> = BaseParserRuleContext<'input,AddSubContextExt<'input>>;

pub trait AddSubContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token Plus
	/// Returns `None` if there is no child corresponding to token Plus
	fn Plus(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Plus, 0)
	}
	/// Retrieves first TerminalNode corresponding to token Minus
	/// Returns `None` if there is no child corresponding to token Minus
	fn Minus(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Minus, 0)
	}
}

impl<'input> AddSubContextAttrs<'input> for AddSubContext<'input>{}

pub struct AddSubContextExt<'input>{
	base:ExprContextExt<'input>,
	pub left: Option<Rc<ExprContextAll<'input>>>,
	pub op: Option<TokenType<'input>>,
	pub right: Option<Rc<ExprContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{AddSubContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for AddSubContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for AddSubContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_AddSub(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_AddSub(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for AddSubContext<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_AddSub(self);
	}
}

impl<'input> CustomRuleContext<'input> for AddSubContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for AddSubContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for AddSubContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for AddSubContext<'input> {}

impl<'input> AddSubContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::AddSubContext(
				BaseParserRuleContext::copy_from(ctx,AddSubContextExt{
					op:None, 
        			left:None, right:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type TernaryContext<'input> = BaseParserRuleContext<'input,TernaryContextExt<'input>>;

pub trait TernaryContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token QMark
	/// Returns `None` if there is no child corresponding to token QMark
	fn QMark(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_QMark, 0)
	}
	/// Retrieves first TerminalNode corresponding to token Colon
	/// Returns `None` if there is no child corresponding to token Colon
	fn Colon(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Colon, 0)
	}
	fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> TernaryContextAttrs<'input> for TernaryContext<'input>{}

pub struct TernaryContextExt<'input>{
	base:ExprContextExt<'input>,
	pub ifExpr: Option<Rc<ExprContextAll<'input>>>,
	pub thenExpr: Option<Rc<ExprContextAll<'input>>>,
	pub elseExpr: Option<Rc<ExprContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{TernaryContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for TernaryContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for TernaryContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_Ternary(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_Ternary(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for TernaryContext<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_Ternary(self);
	}
}

impl<'input> CustomRuleContext<'input> for TernaryContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for TernaryContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for TernaryContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for TernaryContext<'input> {}

impl<'input> TernaryContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::TernaryContext(
				BaseParserRuleContext::copy_from(ctx,TernaryContextExt{
        			ifExpr:None, thenExpr:None, elseExpr:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ParameterNameContext<'input> = BaseParserRuleContext<'input,ParameterNameContextExt<'input>>;

pub trait ParameterNameContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token Identifier
	/// Returns `None` if there is no child corresponding to token Identifier
	fn Identifier(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Identifier, 0)
	}
	/// Retrieves first TerminalNode corresponding to token QMark
	/// Returns `None` if there is no child corresponding to token QMark
	fn QMark(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_QMark, 0)
	}
}

impl<'input> ParameterNameContextAttrs<'input> for ParameterNameContext<'input>{}

pub struct ParameterNameContextExt<'input>{
	base:ExprContextExt<'input>,
	pub isnull: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{ParameterNameContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for ParameterNameContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for ParameterNameContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_ParameterName(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_ParameterName(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for ParameterNameContext<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_ParameterName(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParameterNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for ParameterNameContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for ParameterNameContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for ParameterNameContext<'input> {}

impl<'input> ParameterNameContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::ParameterNameContext(
				BaseParserRuleContext::copy_from(ctx,ParameterNameContextExt{
					isnull:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type TypeLiteralContext<'input> = BaseParserRuleContext<'input,TypeLiteralContextExt<'input>>;

pub trait TypeLiteralContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	fn typeDef(&self) -> Option<Rc<TypeDefContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> TypeLiteralContextAttrs<'input> for TypeLiteralContext<'input>{}

pub struct TypeLiteralContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{TypeLiteralContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for TypeLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for TypeLiteralContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_TypeLiteral(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_TypeLiteral(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for TypeLiteralContext<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_TypeLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for TypeLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for TypeLiteralContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for TypeLiteralContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for TypeLiteralContext<'input> {}

impl<'input> TypeLiteralContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::TypeLiteralContext(
				BaseParserRuleContext::copy_from(ctx,TypeLiteralContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ComparisonContext<'input> = BaseParserRuleContext<'input,ComparisonContextExt<'input>>;

pub trait ComparisonContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token Lt
	/// Returns `None` if there is no child corresponding to token Lt
	fn Lt(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Lt, 0)
	}
	/// Retrieves first TerminalNode corresponding to token Gt
	/// Returns `None` if there is no child corresponding to token Gt
	fn Gt(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Gt, 0)
	}
	/// Retrieves first TerminalNode corresponding to token Lte
	/// Returns `None` if there is no child corresponding to token Lte
	fn Lte(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Lte, 0)
	}
	/// Retrieves first TerminalNode corresponding to token Gte
	/// Returns `None` if there is no child corresponding to token Gte
	fn Gte(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Gte, 0)
	}
}

impl<'input> ComparisonContextAttrs<'input> for ComparisonContext<'input>{}

pub struct ComparisonContextExt<'input>{
	base:ExprContextExt<'input>,
	pub left: Option<Rc<ExprContextAll<'input>>>,
	pub op: Option<TokenType<'input>>,
	pub right: Option<Rc<ExprContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{ComparisonContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for ComparisonContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for ComparisonContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_Comparison(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_Comparison(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for ComparisonContext<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_Comparison(self);
	}
}

impl<'input> CustomRuleContext<'input> for ComparisonContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for ComparisonContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for ComparisonContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for ComparisonContext<'input> {}

impl<'input> ComparisonContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::ComparisonContext(
				BaseParserRuleContext::copy_from(ctx,ComparisonContextExt{
					op:None, 
        			left:None, right:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type AndContext<'input> = BaseParserRuleContext<'input,AndContextExt<'input>>;

pub trait AndContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token And
	/// Returns `None` if there is no child corresponding to token And
	fn And(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_And, 0)
	}
}

impl<'input> AndContextAttrs<'input> for AndContext<'input>{}

pub struct AndContextExt<'input>{
	base:ExprContextExt<'input>,
	pub left: Option<Rc<ExprContextAll<'input>>>,
	pub op: Option<TokenType<'input>>,
	pub right: Option<Rc<ExprContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{AndContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for AndContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for AndContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_And(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_And(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for AndContext<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_And(self);
	}
}

impl<'input> CustomRuleContext<'input> for AndContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for AndContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for AndContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for AndContext<'input> {}

impl<'input> AndContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::AndContext(
				BaseParserRuleContext::copy_from(ctx,AndContextExt{
					op:None, 
        			left:None, right:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ParenExpressionContext<'input> = BaseParserRuleContext<'input,ParenExpressionContextExt<'input>>;

pub trait ParenExpressionContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token OParen
	/// Returns `None` if there is no child corresponding to token OParen
	fn OParen(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_OParen, 0)
	}
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token CParen
	/// Returns `None` if there is no child corresponding to token CParen
	fn CParen(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_CParen, 0)
	}
}

impl<'input> ParenExpressionContextAttrs<'input> for ParenExpressionContext<'input>{}

pub struct ParenExpressionContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{ParenExpressionContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for ParenExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for ParenExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_ParenExpression(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_ParenExpression(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for ParenExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_ParenExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParenExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for ParenExpressionContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for ParenExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for ParenExpressionContext<'input> {}

impl<'input> ParenExpressionContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::ParenExpressionContext(
				BaseParserRuleContext::copy_from(ctx,ParenExpressionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type FunctionCallContext<'input> = BaseParserRuleContext<'input,FunctionCallContextExt<'input>>;

pub trait FunctionCallContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token Identifier
	/// Returns `None` if there is no child corresponding to token Identifier
	fn Identifier(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Identifier, 0)
	}
	/// Retrieves first TerminalNode corresponding to token OParen
	/// Returns `None` if there is no child corresponding to token OParen
	fn OParen(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_OParen, 0)
	}
	/// Retrieves first TerminalNode corresponding to token CParen
	/// Returns `None` if there is no child corresponding to token CParen
	fn CParen(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_CParen, 0)
	}
	fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves all `TerminalNode`s corresponding to token Comma in current rule
	fn Comma_all(&self) -> Vec<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token Comma, starting from 0.
	/// Returns `None` if number of children corresponding to token Comma is less or equal than `i`.
	fn Comma(&self, i: usize) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Comma, i)
	}
}

impl<'input> FunctionCallContextAttrs<'input> for FunctionCallContext<'input>{}

pub struct FunctionCallContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{FunctionCallContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for FunctionCallContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for FunctionCallContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_FunctionCall(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_FunctionCall(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for FunctionCallContext<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_FunctionCall(self);
	}
}

impl<'input> CustomRuleContext<'input> for FunctionCallContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for FunctionCallContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for FunctionCallContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for FunctionCallContext<'input> {}

impl<'input> FunctionCallContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::FunctionCallContext(
				BaseParserRuleContext::copy_from(ctx,FunctionCallContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type NotExprContext<'input> = BaseParserRuleContext<'input,NotExprContextExt<'input>>;

pub trait NotExprContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token Bang
	/// Returns `None` if there is no child corresponding to token Bang
	fn Bang(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Bang, 0)
	}
}

impl<'input> NotExprContextAttrs<'input> for NotExprContext<'input>{}

pub struct NotExprContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{NotExprContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for NotExprContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for NotExprContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_NotExpr(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_NotExpr(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for NotExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_NotExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for NotExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for NotExprContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for NotExprContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for NotExprContext<'input> {}

impl<'input> NotExprContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::NotExprContext(
				BaseParserRuleContext::copy_from(ctx,NotExprContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type EqualityContext<'input> = BaseParserRuleContext<'input,EqualityContextExt<'input>>;

pub trait EqualityContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token Eq
	/// Returns `None` if there is no child corresponding to token Eq
	fn Eq(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Eq, 0)
	}
	/// Retrieves first TerminalNode corresponding to token Ne
	/// Returns `None` if there is no child corresponding to token Ne
	fn Ne(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Ne, 0)
	}
}

impl<'input> EqualityContextAttrs<'input> for EqualityContext<'input>{}

pub struct EqualityContextExt<'input>{
	base:ExprContextExt<'input>,
	pub left: Option<Rc<ExprContextAll<'input>>>,
	pub op: Option<TokenType<'input>>,
	pub right: Option<Rc<ExprContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{EqualityContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for EqualityContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for EqualityContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_Equality(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_Equality(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for EqualityContext<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_Equality(self);
	}
}

impl<'input> CustomRuleContext<'input> for EqualityContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for EqualityContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for EqualityContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for EqualityContext<'input> {}

impl<'input> EqualityContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::EqualityContext(
				BaseParserRuleContext::copy_from(ctx,EqualityContextExt{
					op:None, 
        			left:None, right:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type LiteralNumberContext<'input> = BaseParserRuleContext<'input,LiteralNumberContextExt<'input>>;

pub trait LiteralNumberContextAttrs<'input>: SubstraitTypeParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token Number
	/// Returns `None` if there is no child corresponding to token Number
	fn Number(&self) -> Option<Rc<TerminalNode<'input,SubstraitTypeParserContextType>>> where Self:Sized{
		self.get_token(SubstraitType_Number, 0)
	}
}

impl<'input> LiteralNumberContextAttrs<'input> for LiteralNumberContext<'input>{}

pub struct LiteralNumberContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{LiteralNumberContextExt<'a>}

impl<'input> SubstraitTypeParserContext<'input> for LiteralNumberContext<'input>{}

impl<'input,'a> Listenable<dyn SubstraitTypeListener<'input> + 'a> for LiteralNumberContext<'input>{
	fn enter(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_LiteralNumber(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn SubstraitTypeListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_LiteralNumber(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn SubstraitTypeVisitor<'input> + 'a> for LiteralNumberContext<'input>{
	fn accept(&self,visitor: &mut (dyn SubstraitTypeVisitor<'input> + 'a)) {
		visitor.visit_LiteralNumber(self);
	}
}

impl<'input> CustomRuleContext<'input> for LiteralNumberContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SubstraitTypeParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for LiteralNumberContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for LiteralNumberContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for LiteralNumberContext<'input> {}

impl<'input> LiteralNumberContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::LiteralNumberContext(
				BaseParserRuleContext::copy_from(ctx,LiteralNumberContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I> SubstraitTypeParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn  expr(&mut self,)
	-> Result<Rc<ExprContextAll<'input>>,ANTLRError> {
		self.expr_rec(0)
	}

	fn expr_rec(&mut self, _p: i32)
	-> Result<Rc<ExprContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = ExprContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 16, RULE_expr, _p);
	    let mut _localctx: Rc<ExprContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 16;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(296);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(37,&mut recog.base)? {
				1 =>{
					{
					let mut tmp = ParenExpressionContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();

					recog.base.set_state(236);
					recog.base.match_token(SubstraitType_OParen,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(237);
					recog.expr_rec(0)?;

					recog.base.set_state(238);
					recog.base.match_token(SubstraitType_CParen,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					{
					let mut tmp = MultilineDefinitionContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(240);
					recog.base.match_token(SubstraitType_Identifier,&mut recog.err_handler)?;

					recog.base.set_state(241);
					recog.base.match_token(SubstraitType_Eq,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(242);
					recog.expr_rec(0)?;

					recog.base.set_state(244); 
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					loop {
						{
						{
						recog.base.set_state(243);
						recog.base.match_token(SubstraitType_Newline,&mut recog.err_handler)?;

						}
						}
						recog.base.set_state(246); 
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						if !(_la==SubstraitType_Newline) {break}
					}
					recog.base.set_state(258);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(32,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							recog.base.set_state(248);
							recog.base.match_token(SubstraitType_Identifier,&mut recog.err_handler)?;

							recog.base.set_state(249);
							recog.base.match_token(SubstraitType_Eq,&mut recog.err_handler)?;

							/*InvokeRule expr*/
							recog.base.set_state(250);
							recog.expr_rec(0)?;

							recog.base.set_state(252); 
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							loop {
								{
								{
								recog.base.set_state(251);
								recog.base.match_token(SubstraitType_Newline,&mut recog.err_handler)?;

								}
								}
								recog.base.set_state(254); 
								recog.err_handler.sync(&mut recog.base)?;
								_la = recog.base.input.la(1);
								if !(_la==SubstraitType_Newline) {break}
							}
							}
							} 
						}
						recog.base.set_state(260);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(32,&mut recog.base)?;
					}
					/*InvokeRule typeDef*/
					recog.base.set_state(261);
					let tmp = recog.typeDef()?;
					if let ExprContextAll::MultilineDefinitionContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
					ctx.finalType = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(265);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(33,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							recog.base.set_state(262);
							recog.base.match_token(SubstraitType_Newline,&mut recog.err_handler)?;

							}
							} 
						}
						recog.base.set_state(267);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(33,&mut recog.base)?;
					}
					}
				}
			,
				3 =>{
					{
					let mut tmp = TypeLiteralContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					/*InvokeRule typeDef*/
					recog.base.set_state(268);
					recog.typeDef()?;

					}
				}
			,
				4 =>{
					{
					let mut tmp = LiteralNumberContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(269);
					recog.base.match_token(SubstraitType_Number,&mut recog.err_handler)?;

					}
				}
			,
				5 =>{
					{
					let mut tmp = ParameterNameContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(270);
					recog.base.match_token(SubstraitType_Identifier,&mut recog.err_handler)?;

					recog.base.set_state(272);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(34,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(271);
							let tmp = recog.base.match_token(SubstraitType_QMark,&mut recog.err_handler)?;
							if let ExprContextAll::ParameterNameContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
							ctx.isnull = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
						}

						_ => {}
					}
					}
				}
			,
				6 =>{
					{
					let mut tmp = FunctionCallContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(274);
					recog.base.match_token(SubstraitType_Identifier,&mut recog.err_handler)?;

					recog.base.set_state(275);
					recog.base.match_token(SubstraitType_OParen,&mut recog.err_handler)?;

					recog.base.set_state(284);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if ((((_la - 4)) & !0x3f) == 0 && ((1usize << (_la - 4)) & 1073741817) != 0) || ((((_la - 47)) & !0x3f) == 0 && ((1usize << (_la - 47)) & 3221372931) != 0) {
						{
						/*InvokeRule expr*/
						recog.base.set_state(276);
						recog.expr_rec(0)?;

						recog.base.set_state(281);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						while _la==SubstraitType_Comma {
							{
							{
							recog.base.set_state(277);
							recog.base.match_token(SubstraitType_Comma,&mut recog.err_handler)?;

							/*InvokeRule expr*/
							recog.base.set_state(278);
							recog.expr_rec(0)?;

							}
							}
							recog.base.set_state(283);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
						}
						}
					}

					recog.base.set_state(286);
					recog.base.match_token(SubstraitType_CParen,&mut recog.err_handler)?;

					}
				}
			,
				7 =>{
					{
					let mut tmp = IfExprContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(287);
					recog.base.match_token(SubstraitType_If,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(288);
					let tmp = recog.expr_rec(0)?;
					if let ExprContextAll::IfExprContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
					ctx.ifExpr = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(289);
					recog.base.match_token(SubstraitType_Then,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(290);
					let tmp = recog.expr_rec(0)?;
					if let ExprContextAll::IfExprContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
					ctx.thenExpr = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(291);
					recog.base.match_token(SubstraitType_Else,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(292);
					let tmp = recog.expr_rec(3)?;
					if let ExprContextAll::IfExprContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
					ctx.elseExpr = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					}
				}
			,
				8 =>{
					{
					let mut tmp = NotExprContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					{
					recog.base.set_state(294);
					recog.base.match_token(SubstraitType_Bang,&mut recog.err_handler)?;

					}
					/*InvokeRule expr*/
					recog.base.set_state(295);
					recog.expr_rec(2)?;

					}
				}

				_ => {}
			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(324);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(39,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event()?;
					_prevctx = _localctx.clone();
					{
					recog.base.set_state(322);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(38,&mut recog.base)? {
						1 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = MulDivContextExt::new(&**ExprContextExt::new(_parentctx.clone(), _parentState));
							if let ExprContextAll::MulDivContext(ctx) = cast_mut::<_,ExprContextAll >(&mut tmp){
								ctx.left = Some(_prevctx.clone());
							} else {unreachable!("cant cast");}
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr)?;
							_localctx = tmp;
							recog.base.set_state(298);
							if !({let _localctx = Some(_localctx.clone());
							recog.precpred(None, 9)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 9)".to_owned()), None))?;
							}
							recog.base.set_state(299);
							if let ExprContextAll::MulDivContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
							ctx.op = recog.base.input.lt(1).cloned(); } else {unreachable!("cant cast");} 
							_la = recog.base.input.la(1);
							if { !(_la==SubstraitType_Asterisk || _la==SubstraitType_ForwardSlash) } {
								let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
								if let ExprContextAll::MulDivContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
								ctx.op = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule expr*/
							recog.base.set_state(300);
							let tmp = recog.expr_rec(10)?;
							if let ExprContextAll::MulDivContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
							ctx.right = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
						}
					,
						2 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = AddSubContextExt::new(&**ExprContextExt::new(_parentctx.clone(), _parentState));
							if let ExprContextAll::AddSubContext(ctx) = cast_mut::<_,ExprContextAll >(&mut tmp){
								ctx.left = Some(_prevctx.clone());
							} else {unreachable!("cant cast");}
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr)?;
							_localctx = tmp;
							recog.base.set_state(301);
							if !({let _localctx = Some(_localctx.clone());
							recog.precpred(None, 8)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 8)".to_owned()), None))?;
							}
							recog.base.set_state(302);
							if let ExprContextAll::AddSubContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
							ctx.op = recog.base.input.lt(1).cloned(); } else {unreachable!("cant cast");} 
							_la = recog.base.input.la(1);
							if { !(_la==SubstraitType_Plus || _la==SubstraitType_Minus) } {
								let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
								if let ExprContextAll::AddSubContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
								ctx.op = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule expr*/
							recog.base.set_state(303);
							let tmp = recog.expr_rec(9)?;
							if let ExprContextAll::AddSubContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
							ctx.right = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
						}
					,
						3 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = ComparisonContextExt::new(&**ExprContextExt::new(_parentctx.clone(), _parentState));
							if let ExprContextAll::ComparisonContext(ctx) = cast_mut::<_,ExprContextAll >(&mut tmp){
								ctx.left = Some(_prevctx.clone());
							} else {unreachable!("cant cast");}
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr)?;
							_localctx = tmp;
							recog.base.set_state(304);
							if !({let _localctx = Some(_localctx.clone());
							recog.precpred(None, 7)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 7)".to_owned()), None))?;
							}
							recog.base.set_state(305);
							if let ExprContextAll::ComparisonContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
							ctx.op = recog.base.input.lt(1).cloned(); } else {unreachable!("cant cast");} 
							_la = recog.base.input.la(1);
							if { !(((((_la - 57)) & !0x3f) == 0 && ((1usize << (_la - 57)) & 15) != 0)) } {
								let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
								if let ExprContextAll::ComparisonContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
								ctx.op = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule expr*/
							recog.base.set_state(306);
							let tmp = recog.expr_rec(8)?;
							if let ExprContextAll::ComparisonContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
							ctx.right = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
						}
					,
						4 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = EqualityContextExt::new(&**ExprContextExt::new(_parentctx.clone(), _parentState));
							if let ExprContextAll::EqualityContext(ctx) = cast_mut::<_,ExprContextAll >(&mut tmp){
								ctx.left = Some(_prevctx.clone());
							} else {unreachable!("cant cast");}
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr)?;
							_localctx = tmp;
							recog.base.set_state(307);
							if !({let _localctx = Some(_localctx.clone());
							recog.precpred(None, 6)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 6)".to_owned()), None))?;
							}
							recog.base.set_state(308);
							if let ExprContextAll::EqualityContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
							ctx.op = recog.base.input.lt(1).cloned(); } else {unreachable!("cant cast");} 
							_la = recog.base.input.la(1);
							if { !(_la==SubstraitType_Eq || _la==SubstraitType_Ne) } {
								let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
								if let ExprContextAll::EqualityContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
								ctx.op = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule expr*/
							recog.base.set_state(309);
							let tmp = recog.expr_rec(7)?;
							if let ExprContextAll::EqualityContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
							ctx.right = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
						}
					,
						5 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = AndContextExt::new(&**ExprContextExt::new(_parentctx.clone(), _parentState));
							if let ExprContextAll::AndContext(ctx) = cast_mut::<_,ExprContextAll >(&mut tmp){
								ctx.left = Some(_prevctx.clone());
							} else {unreachable!("cant cast");}
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr)?;
							_localctx = tmp;
							recog.base.set_state(310);
							if !({let _localctx = Some(_localctx.clone());
							recog.precpred(None, 5)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 5)".to_owned()), None))?;
							}
							recog.base.set_state(311);
							let tmp = recog.base.match_token(SubstraitType_And,&mut recog.err_handler)?;
							if let ExprContextAll::AndContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
							ctx.op = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							/*InvokeRule expr*/
							recog.base.set_state(312);
							let tmp = recog.expr_rec(6)?;
							if let ExprContextAll::AndContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
							ctx.right = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
						}
					,
						6 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = OrContextExt::new(&**ExprContextExt::new(_parentctx.clone(), _parentState));
							if let ExprContextAll::OrContext(ctx) = cast_mut::<_,ExprContextAll >(&mut tmp){
								ctx.left = Some(_prevctx.clone());
							} else {unreachable!("cant cast");}
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr)?;
							_localctx = tmp;
							recog.base.set_state(313);
							if !({let _localctx = Some(_localctx.clone());
							recog.precpred(None, 4)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 4)".to_owned()), None))?;
							}
							recog.base.set_state(314);
							let tmp = recog.base.match_token(SubstraitType_Or,&mut recog.err_handler)?;
							if let ExprContextAll::OrContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
							ctx.op = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							/*InvokeRule expr*/
							recog.base.set_state(315);
							let tmp = recog.expr_rec(5)?;
							if let ExprContextAll::OrContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
							ctx.right = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
						}
					,
						7 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = TernaryContextExt::new(&**ExprContextExt::new(_parentctx.clone(), _parentState));
							if let ExprContextAll::TernaryContext(ctx) = cast_mut::<_,ExprContextAll >(&mut tmp){
								ctx.ifExpr = Some(_prevctx.clone());
							} else {unreachable!("cant cast");}
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr)?;
							_localctx = tmp;
							recog.base.set_state(316);
							if !({let _localctx = Some(_localctx.clone());
							recog.precpred(None, 1)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
							}
							recog.base.set_state(317);
							recog.base.match_token(SubstraitType_QMark,&mut recog.err_handler)?;

							/*InvokeRule expr*/
							recog.base.set_state(318);
							let tmp = recog.expr_rec(0)?;
							if let ExprContextAll::TernaryContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
							ctx.thenExpr = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							recog.base.set_state(319);
							recog.base.match_token(SubstraitType_Colon,&mut recog.err_handler)?;

							/*InvokeRule expr*/
							recog.base.set_state(320);
							let tmp = recog.expr_rec(2)?;
							if let ExprContextAll::TernaryContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
							ctx.elseExpr = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(326);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(39,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx)?;

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
		4, 1, 79, 328, 2, 0, 7, 0, 2, 1, 7, 1, 2, 2, 7, 2, 2, 3, 7, 3, 2, 4, 7, 
		4, 2, 5, 7, 5, 2, 6, 7, 6, 2, 7, 7, 7, 2, 8, 7, 8, 1, 0, 1, 0, 1, 0, 1, 
		1, 1, 1, 1, 1, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 
		2, 1, 2, 1, 2, 3, 2, 37, 8, 2, 1, 3, 1, 3, 3, 3, 41, 8, 3, 1, 3, 1, 3, 
		1, 3, 1, 3, 1, 3, 1, 3, 3, 3, 49, 8, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 
		1, 3, 3, 3, 57, 8, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 3, 3, 65, 8, 
		3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 3, 3, 75, 8, 3, 1, 
		3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 3, 3, 83, 8, 3, 1, 3, 1, 3, 1, 3, 1, 
		3, 1, 3, 1, 3, 3, 3, 91, 8, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 3, 
		3, 99, 8, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 3, 3, 107, 8, 3, 1, 3, 
		1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 3, 3, 115, 8, 3, 1, 3, 1, 3, 1, 3, 1, 3, 
		5, 3, 121, 8, 3, 10, 3, 12, 3, 124, 9, 3, 1, 3, 1, 3, 1, 3, 1, 3, 3, 3, 
		130, 8, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 5, 3, 140, 
		8, 3, 10, 3, 12, 3, 143, 9, 3, 1, 3, 1, 3, 1, 3, 1, 3, 3, 3, 149, 8, 3, 
		1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 3, 3, 157, 8, 3, 1, 3, 1, 3, 1, 3, 
		1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 3, 3, 167, 8, 3, 1, 3, 1, 3, 1, 3, 1, 3, 
		1, 3, 1, 3, 1, 3, 1, 3, 3, 3, 177, 8, 3, 1, 3, 1, 3, 1, 3, 3, 3, 182, 
		8, 3, 1, 3, 1, 3, 1, 3, 1, 3, 5, 3, 188, 8, 3, 10, 3, 12, 3, 191, 9, 3, 
		1, 3, 1, 3, 3, 3, 195, 8, 3, 3, 3, 197, 8, 3, 1, 4, 1, 4, 1, 4, 1, 4, 
		1, 4, 5, 4, 204, 8, 4, 10, 4, 12, 4, 207, 9, 4, 1, 4, 1, 4, 3, 4, 211, 
		8, 4, 1, 5, 1, 5, 1, 5, 3, 5, 216, 8, 5, 1, 6, 1, 6, 3, 6, 220, 8, 6, 
		1, 6, 1, 6, 3, 6, 224, 8, 6, 3, 6, 226, 8, 6, 1, 7, 1, 7, 3, 7, 230, 8, 
		7, 1, 7, 1, 7, 3, 7, 234, 8, 7, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 
		8, 1, 8, 1, 8, 4, 8, 245, 8, 8, 11, 8, 12, 8, 246, 1, 8, 1, 8, 1, 8, 1, 
		8, 4, 8, 253, 8, 8, 11, 8, 12, 8, 254, 5, 8, 257, 8, 8, 10, 8, 12, 8, 
		260, 9, 8, 1, 8, 1, 8, 5, 8, 264, 8, 8, 10, 8, 12, 8, 267, 9, 8, 1, 8, 
		1, 8, 1, 8, 1, 8, 3, 8, 273, 8, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 5, 8, 
		280, 8, 8, 10, 8, 12, 8, 283, 9, 8, 3, 8, 285, 8, 8, 1, 8, 1, 8, 1, 8, 
		1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 3, 8, 297, 8, 8, 1, 8, 1, 8, 
		1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 
		1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 5, 8, 323, 
		8, 8, 10, 8, 12, 8, 326, 9, 8, 1, 8, 0, 1, 16, 9, 0, 2, 4, 6, 8, 10, 12, 
		14, 16, 0, 4, 1, 0, 52, 53, 1, 0, 50, 51, 1, 0, 57, 60, 1, 0, 55, 56, 
		394, 0, 18, 1, 0, 0, 0, 2, 21, 1, 0, 0, 0, 4, 36, 1, 0, 0, 0, 6, 196, 
		1, 0, 0, 0, 8, 210, 1, 0, 0, 0, 10, 215, 1, 0, 0, 0, 12, 225, 1, 0, 0, 
		0, 14, 233, 1, 0, 0, 0, 16, 296, 1, 0, 0, 0, 18, 19, 3, 16, 8, 0, 19, 
		20, 5, 0, 0, 1, 20, 1, 1, 0, 0, 0, 21, 22, 3, 14, 7, 0, 22, 23, 5, 0, 
		0, 1, 23, 3, 1, 0, 0, 0, 24, 37, 5, 8, 0, 0, 25, 37, 5, 9, 0, 0, 26, 37, 
		5, 10, 0, 0, 27, 37, 5, 11, 0, 0, 28, 37, 5, 12, 0, 0, 29, 37, 5, 13, 
		0, 0, 30, 37, 5, 14, 0, 0, 31, 37, 5, 15, 0, 0, 32, 37, 5, 16, 0, 0, 33, 
		37, 5, 17, 0, 0, 34, 37, 5, 18, 0, 0, 35, 37, 5, 21, 0, 0, 36, 24, 1, 
		0, 0, 0, 36, 25, 1, 0, 0, 0, 36, 26, 1, 0, 0, 0, 36, 27, 1, 0, 0, 0, 36, 
		28, 1, 0, 0, 0, 36, 29, 1, 0, 0, 0, 36, 30, 1, 0, 0, 0, 36, 31, 1, 0, 
		0, 0, 36, 32, 1, 0, 0, 0, 36, 33, 1, 0, 0, 0, 36, 34, 1, 0, 0, 0, 36, 
		35, 1, 0, 0, 0, 37, 5, 1, 0, 0, 0, 38, 40, 5, 26, 0, 0, 39, 41, 5, 70, 
		0, 0, 40, 39, 1, 0, 0, 0, 40, 41, 1, 0, 0, 0, 41, 42, 1, 0, 0, 0, 42, 
		43, 5, 60, 0, 0, 43, 44, 3, 10, 5, 0, 44, 45, 5, 59, 0, 0, 45, 197, 1, 
		0, 0, 0, 46, 48, 5, 27, 0, 0, 47, 49, 5, 70, 0, 0, 48, 47, 1, 0, 0, 0, 
		48, 49, 1, 0, 0, 0, 49, 50, 1, 0, 0, 0, 50, 51, 5, 60, 0, 0, 51, 52, 3, 
		10, 5, 0, 52, 53, 5, 59, 0, 0, 53, 197, 1, 0, 0, 0, 54, 56, 5, 28, 0, 
		0, 55, 57, 5, 70, 0, 0, 56, 55, 1, 0, 0, 0, 56, 57, 1, 0, 0, 0, 57, 58, 
		1, 0, 0, 0, 58, 59, 5, 60, 0, 0, 59, 60, 3, 10, 5, 0, 60, 61, 5, 59, 0, 
		0, 61, 197, 1, 0, 0, 0, 62, 64, 5, 22, 0, 0, 63, 65, 5, 70, 0, 0, 64, 
		63, 1, 0, 0, 0, 64, 65, 1, 0, 0, 0, 65, 66, 1, 0, 0, 0, 66, 67, 5, 60, 
		0, 0, 67, 68, 3, 10, 5, 0, 68, 69, 5, 68, 0, 0, 69, 70, 3, 10, 5, 0, 70, 
		71, 5, 59, 0, 0, 71, 197, 1, 0, 0, 0, 72, 74, 5, 19, 0, 0, 73, 75, 5, 
		70, 0, 0, 74, 73, 1, 0, 0, 0, 74, 75, 1, 0, 0, 0, 75, 76, 1, 0, 0, 0, 
		76, 77, 5, 60, 0, 0, 77, 78, 3, 10, 5, 0, 78, 79, 5, 59, 0, 0, 79, 197, 
		1, 0, 0, 0, 80, 82, 5, 20, 0, 0, 81, 83, 5, 70, 0, 0, 82, 81, 1, 0, 0, 
		0, 82, 83, 1, 0, 0, 0, 83, 84, 1, 0, 0, 0, 84, 85, 5, 60, 0, 0, 85, 86, 
		3, 10, 5, 0, 86, 87, 5, 59, 0, 0, 87, 197, 1, 0, 0, 0, 88, 90, 5, 23, 
		0, 0, 89, 91, 5, 70, 0, 0, 90, 89, 1, 0, 0, 0, 90, 91, 1, 0, 0, 0, 91, 
		92, 1, 0, 0, 0, 92, 93, 5, 60, 0, 0, 93, 94, 3, 10, 5, 0, 94, 95, 5, 59, 
		0, 0, 95, 197, 1, 0, 0, 0, 96, 98, 5, 24, 0, 0, 97, 99, 5, 70, 0, 0, 98, 
		97, 1, 0, 0, 0, 98, 99, 1, 0, 0, 0, 99, 100, 1, 0, 0, 0, 100, 101, 5, 
		60, 0, 0, 101, 102, 3, 10, 5, 0, 102, 103, 5, 59, 0, 0, 103, 197, 1, 0, 
		0, 0, 104, 106, 5, 25, 0, 0, 105, 107, 5, 70, 0, 0, 106, 105, 1, 0, 0, 
		0, 106, 107, 1, 0, 0, 0, 107, 108, 1, 0, 0, 0, 108, 109, 5, 60, 0, 0, 
		109, 110, 3, 10, 5, 0, 110, 111, 5, 59, 0, 0, 111, 197, 1, 0, 0, 0, 112, 
		114, 5, 29, 0, 0, 113, 115, 5, 70, 0, 0, 114, 113, 1, 0, 0, 0, 114, 115, 
		1, 0, 0, 0, 115, 116, 1, 0, 0, 0, 116, 117, 5, 60, 0, 0, 117, 122, 3, 
		16, 8, 0, 118, 119, 5, 68, 0, 0, 119, 121, 3, 16, 8, 0, 120, 118, 1, 0, 
		0, 0, 121, 124, 1, 0, 0, 0, 122, 120, 1, 0, 0, 0, 122, 123, 1, 0, 0, 0, 
		123, 125, 1, 0, 0, 0, 124, 122, 1, 0, 0, 0, 125, 126, 5, 59, 0, 0, 126, 
		197, 1, 0, 0, 0, 127, 129, 5, 30, 0, 0, 128, 130, 5, 70, 0, 0, 129, 128, 
		1, 0, 0, 0, 129, 130, 1, 0, 0, 0, 130, 131, 1, 0, 0, 0, 131, 132, 5, 60, 
		0, 0, 132, 133, 5, 78, 0, 0, 133, 134, 5, 69, 0, 0, 134, 141, 3, 16, 8, 
		0, 135, 136, 5, 68, 0, 0, 136, 137, 5, 78, 0, 0, 137, 138, 5, 69, 0, 0, 
		138, 140, 3, 16, 8, 0, 139, 135, 1, 0, 0, 0, 140, 143, 1, 0, 0, 0, 141, 
		139, 1, 0, 0, 0, 141, 142, 1, 0, 0, 0, 142, 144, 1, 0, 0, 0, 143, 141, 
		1, 0, 0, 0, 144, 145, 5, 59, 0, 0, 145, 197, 1, 0, 0, 0, 146, 148, 5, 
		31, 0, 0, 147, 149, 5, 70, 0, 0, 148, 147, 1, 0, 0, 0, 148, 149, 1, 0, 
		0, 0, 149, 150, 1, 0, 0, 0, 150, 151, 5, 60, 0, 0, 151, 152, 3, 16, 8, 
		0, 152, 153, 5, 59, 0, 0, 153, 197, 1, 0, 0, 0, 154, 156, 5, 32, 0, 0, 
		155, 157, 5, 70, 0, 0, 156, 155, 1, 0, 0, 0, 156, 157, 1, 0, 0, 0, 157, 
		158, 1, 0, 0, 0, 158, 159, 5, 60, 0, 0, 159, 160, 3, 16, 8, 0, 160, 161, 
		5, 68, 0, 0, 161, 162, 3, 16, 8, 0, 162, 163, 5, 59, 0, 0, 163, 197, 1, 
		0, 0, 0, 164, 166, 5, 7, 0, 0, 165, 167, 5, 70, 0, 0, 166, 165, 1, 0, 
		0, 0, 166, 167, 1, 0, 0, 0, 167, 168, 1, 0, 0, 0, 168, 169, 5, 60, 0, 
		0, 169, 170, 3, 8, 4, 0, 170, 171, 5, 76, 0, 0, 171, 172, 3, 16, 8, 0, 
		172, 173, 5, 59, 0, 0, 173, 197, 1, 0, 0, 0, 174, 175, 5, 78, 0, 0, 175, 
		177, 5, 72, 0, 0, 176, 174, 1, 0, 0, 0, 176, 177, 1, 0, 0, 0, 177, 178, 
		1, 0, 0, 0, 178, 179, 5, 33, 0, 0, 179, 181, 5, 78, 0, 0, 180, 182, 5, 
		70, 0, 0, 181, 180, 1, 0, 0, 0, 181, 182, 1, 0, 0, 0, 182, 194, 1, 0, 
		0, 0, 183, 184, 5, 60, 0, 0, 184, 189, 3, 16, 8, 0, 185, 186, 5, 68, 0, 
		0, 186, 188, 3, 16, 8, 0, 187, 185, 1, 0, 0, 0, 188, 191, 1, 0, 0, 0, 
		189, 187, 1, 0, 0, 0, 189, 190, 1, 0, 0, 0, 190, 192, 1, 0, 0, 0, 191, 
		189, 1, 0, 0, 0, 192, 193, 5, 59, 0, 0, 193, 195, 1, 0, 0, 0, 194, 183, 
		1, 0, 0, 0, 194, 195, 1, 0, 0, 0, 195, 197, 1, 0, 0, 0, 196, 38, 1, 0, 
		0, 0, 196, 46, 1, 0, 0, 0, 196, 54, 1, 0, 0, 0, 196, 62, 1, 0, 0, 0, 196, 
		72, 1, 0, 0, 0, 196, 80, 1, 0, 0, 0, 196, 88, 1, 0, 0, 0, 196, 96, 1, 
		0, 0, 0, 196, 104, 1, 0, 0, 0, 196, 112, 1, 0, 0, 0, 196, 127, 1, 0, 0, 
		0, 196, 146, 1, 0, 0, 0, 196, 154, 1, 0, 0, 0, 196, 164, 1, 0, 0, 0, 196, 
		176, 1, 0, 0, 0, 197, 7, 1, 0, 0, 0, 198, 211, 3, 16, 8, 0, 199, 200, 
		5, 64, 0, 0, 200, 205, 3, 16, 8, 0, 201, 202, 5, 68, 0, 0, 202, 204, 3, 
		16, 8, 0, 203, 201, 1, 0, 0, 0, 204, 207, 1, 0, 0, 0, 205, 203, 1, 0, 
		0, 0, 205, 206, 1, 0, 0, 0, 206, 208, 1, 0, 0, 0, 207, 205, 1, 0, 0, 0, 
		208, 209, 5, 65, 0, 0, 209, 211, 1, 0, 0, 0, 210, 198, 1, 0, 0, 0, 210, 
		199, 1, 0, 0, 0, 211, 9, 1, 0, 0, 0, 212, 216, 5, 77, 0, 0, 213, 216, 
		5, 78, 0, 0, 214, 216, 3, 16, 8, 0, 215, 212, 1, 0, 0, 0, 215, 213, 1, 
		0, 0, 0, 215, 214, 1, 0, 0, 0, 216, 11, 1, 0, 0, 0, 217, 219, 5, 47, 0, 
		0, 218, 220, 5, 70, 0, 0, 219, 218, 1, 0, 0, 0, 219, 220, 1, 0, 0, 0, 
		220, 226, 1, 0, 0, 0, 221, 223, 5, 48, 0, 0, 222, 224, 5, 70, 0, 0, 223, 
		222, 1, 0, 0, 0, 223, 224, 1, 0, 0, 0, 224, 226, 1, 0, 0, 0, 225, 217, 
		1, 0, 0, 0, 225, 221, 1, 0, 0, 0, 226, 13, 1, 0, 0, 0, 227, 229, 3, 4, 
		2, 0, 228, 230, 5, 70, 0, 0, 229, 228, 1, 0, 0, 0, 229, 230, 1, 0, 0, 
		0, 230, 234, 1, 0, 0, 0, 231, 234, 3, 6, 3, 0, 232, 234, 3, 12, 6, 0, 
		233, 227, 1, 0, 0, 0, 233, 231, 1, 0, 0, 0, 233, 232, 1, 0, 0, 0, 234, 
		15, 1, 0, 0, 0, 235, 236, 6, 8, -1, 0, 236, 237, 5, 64, 0, 0, 237, 238, 
		3, 16, 8, 0, 238, 239, 5, 65, 0, 0, 239, 297, 1, 0, 0, 0, 240, 241, 5, 
		78, 0, 0, 241, 242, 5, 55, 0, 0, 242, 244, 3, 16, 8, 0, 243, 245, 5, 79, 
		0, 0, 244, 243, 1, 0, 0, 0, 245, 246, 1, 0, 0, 0, 246, 244, 1, 0, 0, 0, 
		246, 247, 1, 0, 0, 0, 247, 258, 1, 0, 0, 0, 248, 249, 5, 78, 0, 0, 249, 
		250, 5, 55, 0, 0, 250, 252, 3, 16, 8, 0, 251, 253, 5, 79, 0, 0, 252, 251, 
		1, 0, 0, 0, 253, 254, 1, 0, 0, 0, 254, 252, 1, 0, 0, 0, 254, 255, 1, 0, 
		0, 0, 255, 257, 1, 0, 0, 0, 256, 248, 1, 0, 0, 0, 257, 260, 1, 0, 0, 0, 
		258, 256, 1, 0, 0, 0, 258, 259, 1, 0, 0, 0, 259, 261, 1, 0, 0, 0, 260, 
		258, 1, 0, 0, 0, 261, 265, 3, 14, 7, 0, 262, 264, 5, 79, 0, 0, 263, 262, 
		1, 0, 0, 0, 264, 267, 1, 0, 0, 0, 265, 263, 1, 0, 0, 0, 265, 266, 1, 0, 
		0, 0, 266, 297, 1, 0, 0, 0, 267, 265, 1, 0, 0, 0, 268, 297, 3, 14, 7, 
		0, 269, 297, 5, 77, 0, 0, 270, 272, 5, 78, 0, 0, 271, 273, 5, 70, 0, 0, 
		272, 271, 1, 0, 0, 0, 272, 273, 1, 0, 0, 0, 273, 297, 1, 0, 0, 0, 274, 
		275, 5, 78, 0, 0, 275, 284, 5, 64, 0, 0, 276, 281, 3, 16, 8, 0, 277, 278, 
		5, 68, 0, 0, 278, 280, 3, 16, 8, 0, 279, 277, 1, 0, 0, 0, 280, 283, 1, 
		0, 0, 0, 281, 279, 1, 0, 0, 0, 281, 282, 1, 0, 0, 0, 282, 285, 1, 0, 0, 
		0, 283, 281, 1, 0, 0, 0, 284, 276, 1, 0, 0, 0, 284, 285, 1, 0, 0, 0, 285, 
		286, 1, 0, 0, 0, 286, 297, 5, 65, 0, 0, 287, 288, 5, 4, 0, 0, 288, 289, 
		3, 16, 8, 0, 289, 290, 5, 5, 0, 0, 290, 291, 3, 16, 8, 0, 291, 292, 5, 
		6, 0, 0, 292, 293, 3, 16, 8, 3, 293, 297, 1, 0, 0, 0, 294, 295, 5, 61, 
		0, 0, 295, 297, 3, 16, 8, 2, 296, 235, 1, 0, 0, 0, 296, 240, 1, 0, 0, 
		0, 296, 268, 1, 0, 0, 0, 296, 269, 1, 0, 0, 0, 296, 270, 1, 0, 0, 0, 296, 
		274, 1, 0, 0, 0, 296, 287, 1, 0, 0, 0, 296, 294, 1, 0, 0, 0, 297, 324, 
		1, 0, 0, 0, 298, 299, 10, 9, 0, 0, 299, 300, 7, 0, 0, 0, 300, 323, 3, 
		16, 8, 10, 301, 302, 10, 8, 0, 0, 302, 303, 7, 1, 0, 0, 303, 323, 3, 16, 
		8, 9, 304, 305, 10, 7, 0, 0, 305, 306, 7, 2, 0, 0, 306, 323, 3, 16, 8, 
		8, 307, 308, 10, 6, 0, 0, 308, 309, 7, 3, 0, 0, 309, 323, 3, 16, 8, 7, 
		310, 311, 10, 5, 0, 0, 311, 312, 5, 73, 0, 0, 312, 323, 3, 16, 8, 6, 313, 
		314, 10, 4, 0, 0, 314, 315, 5, 74, 0, 0, 315, 323, 3, 16, 8, 5, 316, 317, 
		10, 1, 0, 0, 317, 318, 5, 70, 0, 0, 318, 319, 3, 16, 8, 0, 319, 320, 5, 
		69, 0, 0, 320, 321, 3, 16, 8, 2, 321, 323, 1, 0, 0, 0, 322, 298, 1, 0, 
		0, 0, 322, 301, 1, 0, 0, 0, 322, 304, 1, 0, 0, 0, 322, 307, 1, 0, 0, 0, 
		322, 310, 1, 0, 0, 0, 322, 313, 1, 0, 0, 0, 322, 316, 1, 0, 0, 0, 323, 
		326, 1, 0, 0, 0, 324, 322, 1, 0, 0, 0, 324, 325, 1, 0, 0, 0, 325, 17, 
		1, 0, 0, 0, 326, 324, 1, 0, 0, 0, 40, 36, 40, 48, 56, 64, 74, 82, 90, 
		98, 106, 114, 122, 129, 141, 148, 156, 166, 176, 181, 189, 194, 196, 205, 
		210, 215, 219, 223, 225, 229, 233, 246, 254, 258, 265, 272, 281, 284, 
		296, 322, 324
	];
}
