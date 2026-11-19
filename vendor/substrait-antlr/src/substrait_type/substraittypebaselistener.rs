// SPDX-License-Identifier: Apache-2.0
#![allow(clippy::all)]
#![allow(unused_parens)]
#![cfg_attr(rustfmt, rustfmt_skip)]
// Generated from SubstraitType.g4 by ANTLR 4.13.2

use super::substraittypeparser::*;
use antlr4rust::tree::ParseTreeListener;

// A complete Visitor for a parse tree produced by SubstraitTypeParser.

pub trait SubstraitTypeBaseListener<'input>:
    ParseTreeListener<'input, SubstraitTypeParserContextType> {

    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_startrule(&mut self, _ctx: &StartRuleContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_startrule(&mut self, _ctx: &StartRuleContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_typestatement(&mut self, _ctx: &TypeStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_typestatement(&mut self, _ctx: &TypeStatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_boolean(&mut self, _ctx: &BooleanContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_boolean(&mut self, _ctx: &BooleanContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_i8(&mut self, _ctx: &I8Context<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_i8(&mut self, _ctx: &I8Context<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_i16(&mut self, _ctx: &I16Context<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_i16(&mut self, _ctx: &I16Context<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_i32(&mut self, _ctx: &I32Context<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_i32(&mut self, _ctx: &I32Context<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_i64(&mut self, _ctx: &I64Context<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_i64(&mut self, _ctx: &I64Context<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fp32(&mut self, _ctx: &Fp32Context<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fp32(&mut self, _ctx: &Fp32Context<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fp64(&mut self, _ctx: &Fp64Context<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fp64(&mut self, _ctx: &Fp64Context<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_string(&mut self, _ctx: &StringContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_string(&mut self, _ctx: &StringContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_binary(&mut self, _ctx: &BinaryContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_binary(&mut self, _ctx: &BinaryContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_date(&mut self, _ctx: &DateContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_date(&mut self, _ctx: &DateContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_intervalyear(&mut self, _ctx: &IntervalYearContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_intervalyear(&mut self, _ctx: &IntervalYearContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_uuid(&mut self, _ctx: &UuidContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_uuid(&mut self, _ctx: &UuidContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fixedchar(&mut self, _ctx: &FixedCharContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fixedchar(&mut self, _ctx: &FixedCharContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_varchar(&mut self, _ctx: &VarCharContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_varchar(&mut self, _ctx: &VarCharContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fixedbinary(&mut self, _ctx: &FixedBinaryContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fixedbinary(&mut self, _ctx: &FixedBinaryContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_decimal(&mut self, _ctx: &DecimalContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_decimal(&mut self, _ctx: &DecimalContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_precisionintervalday(&mut self, _ctx: &PrecisionIntervalDayContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_precisionintervalday(&mut self, _ctx: &PrecisionIntervalDayContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_precisionintervalcompound(&mut self, _ctx: &PrecisionIntervalCompoundContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_precisionintervalcompound(&mut self, _ctx: &PrecisionIntervalCompoundContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_precisiontime(&mut self, _ctx: &PrecisionTimeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_precisiontime(&mut self, _ctx: &PrecisionTimeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_precisiontimestamp(&mut self, _ctx: &PrecisionTimestampContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_precisiontimestamp(&mut self, _ctx: &PrecisionTimestampContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_precisiontimestamptz(&mut self, _ctx: &PrecisionTimestampTZContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_precisiontimestamptz(&mut self, _ctx: &PrecisionTimestampTZContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_struct(&mut self, _ctx: &StructContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_struct(&mut self, _ctx: &StructContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_nstruct(&mut self, _ctx: &NStructContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_nstruct(&mut self, _ctx: &NStructContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_list(&mut self, _ctx: &ListContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_list(&mut self, _ctx: &ListContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_map(&mut self, _ctx: &MapContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_map(&mut self, _ctx: &MapContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_func(&mut self, _ctx: &FuncContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_func(&mut self, _ctx: &FuncContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_userdefined(&mut self, _ctx: &UserDefinedContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_userdefined(&mut self, _ctx: &UserDefinedContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_singlefuncparam(&mut self, _ctx: &SingleFuncParamContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_singlefuncparam(&mut self, _ctx: &SingleFuncParamContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_funcparamswithparens(&mut self, _ctx: &FuncParamsWithParensContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_funcparamswithparens(&mut self, _ctx: &FuncParamsWithParensContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_numericliteral(&mut self, _ctx: &NumericLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_numericliteral(&mut self, _ctx: &NumericLiteralContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_numericparametername(&mut self, _ctx: &NumericParameterNameContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_numericparametername(&mut self, _ctx: &NumericParameterNameContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_numericexpression(&mut self, _ctx: &NumericExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_numericexpression(&mut self, _ctx: &NumericExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_anytype(&mut self, _ctx: &AnyTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_anytype(&mut self, _ctx: &AnyTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_typedef(&mut self, _ctx: &TypeDefContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_typedef(&mut self, _ctx: &TypeDefContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_ifexpr(&mut self, _ctx: &IfExprContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_ifexpr(&mut self, _ctx: &IfExprContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_or(&mut self, _ctx: &OrContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_or(&mut self, _ctx: &OrContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_multilinedefinition(&mut self, _ctx: &MultilineDefinitionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_multilinedefinition(&mut self, _ctx: &MultilineDefinitionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_muldiv(&mut self, _ctx: &MulDivContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_muldiv(&mut self, _ctx: &MulDivContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_addsub(&mut self, _ctx: &AddSubContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_addsub(&mut self, _ctx: &AddSubContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_ternary(&mut self, _ctx: &TernaryContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_ternary(&mut self, _ctx: &TernaryContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_parametername(&mut self, _ctx: &ParameterNameContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_parametername(&mut self, _ctx: &ParameterNameContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_typeliteral(&mut self, _ctx: &TypeLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_typeliteral(&mut self, _ctx: &TypeLiteralContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_comparison(&mut self, _ctx: &ComparisonContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_comparison(&mut self, _ctx: &ComparisonContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_and(&mut self, _ctx: &AndContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_and(&mut self, _ctx: &AndContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_parenexpression(&mut self, _ctx: &ParenExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_parenexpression(&mut self, _ctx: &ParenExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_functioncall(&mut self, _ctx: &FunctionCallContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_functioncall(&mut self, _ctx: &FunctionCallContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_notexpr(&mut self, _ctx: &NotExprContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_notexpr(&mut self, _ctx: &NotExprContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_equality(&mut self, _ctx: &EqualityContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_equality(&mut self, _ctx: &EqualityContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_literalnumber(&mut self, _ctx: &LiteralNumberContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  SubstraitTypeBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_literalnumber(&mut self, _ctx: &LiteralNumberContext<'input>) {}


}