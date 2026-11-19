// SPDX-License-Identifier: Apache-2.0
#![allow(clippy::all)]
#![allow(unused_parens)]
#![cfg_attr(rustfmt, rustfmt_skip)]

// Generated from SubstraitType.g4 by ANTLR 4.13.2

use antlr4rust::tree::ParseTreeVisitor;
use super::substraittypeparser::*;

// A complete Visitor for a parse tree produced by SubstraitTypeParser.

pub trait SubstraitTypeBaseVisitor<'input>:
    ParseTreeVisitor<'input, SubstraitTypeParserContextType> {
	// Visit a parse tree produced by SubstraitTypeParser#startRule.
	fn visit_startrule(&mut self, ctx: &StartRuleContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#typeStatement.
	fn visit_typestatement(&mut self, ctx: &TypeStatementContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#boolean.
	fn visit_boolean(&mut self, ctx: &BooleanContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#i8.
	fn visit_i8(&mut self, ctx: &I8Context<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#i16.
	fn visit_i16(&mut self, ctx: &I16Context<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#i32.
	fn visit_i32(&mut self, ctx: &I32Context<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#i64.
	fn visit_i64(&mut self, ctx: &I64Context<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#fp32.
	fn visit_fp32(&mut self, ctx: &Fp32Context<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#fp64.
	fn visit_fp64(&mut self, ctx: &Fp64Context<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#string.
	fn visit_string(&mut self, ctx: &StringContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#binary.
	fn visit_binary(&mut self, ctx: &BinaryContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#date.
	fn visit_date(&mut self, ctx: &DateContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#intervalYear.
	fn visit_intervalyear(&mut self, ctx: &IntervalYearContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#uuid.
	fn visit_uuid(&mut self, ctx: &UuidContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#fixedChar.
	fn visit_fixedchar(&mut self, ctx: &FixedCharContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#varChar.
	fn visit_varchar(&mut self, ctx: &VarCharContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#fixedBinary.
	fn visit_fixedbinary(&mut self, ctx: &FixedBinaryContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#decimal.
	fn visit_decimal(&mut self, ctx: &DecimalContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#precisionIntervalDay.
	fn visit_precisionintervalday(&mut self, ctx: &PrecisionIntervalDayContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#precisionIntervalCompound.
	fn visit_precisionintervalcompound(&mut self, ctx: &PrecisionIntervalCompoundContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#precisionTime.
	fn visit_precisiontime(&mut self, ctx: &PrecisionTimeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#precisionTimestamp.
	fn visit_precisiontimestamp(&mut self, ctx: &PrecisionTimestampContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#precisionTimestampTZ.
	fn visit_precisiontimestamptz(&mut self, ctx: &PrecisionTimestampTZContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#struct.
	fn visit_struct(&mut self, ctx: &StructContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#nStruct.
	fn visit_nstruct(&mut self, ctx: &NStructContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#list.
	fn visit_list(&mut self, ctx: &ListContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#map.
	fn visit_map(&mut self, ctx: &MapContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#func.
	fn visit_func(&mut self, ctx: &FuncContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#userDefined.
	fn visit_userdefined(&mut self, ctx: &UserDefinedContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#singleFuncParam.
	fn visit_singlefuncparam(&mut self, ctx: &SingleFuncParamContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#funcParamsWithParens.
	fn visit_funcparamswithparens(&mut self, ctx: &FuncParamsWithParensContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#numericLiteral.
	fn visit_numericliteral(&mut self, ctx: &NumericLiteralContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#numericParameterName.
	fn visit_numericparametername(&mut self, ctx: &NumericParameterNameContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#numericExpression.
	fn visit_numericexpression(&mut self, ctx: &NumericExpressionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#anyType.
	fn visit_anytype(&mut self, ctx: &AnyTypeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#typeDef.
	fn visit_typedef(&mut self, ctx: &TypeDefContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#IfExpr.
	fn visit_ifexpr(&mut self, ctx: &IfExprContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#Or.
	fn visit_or(&mut self, ctx: &OrContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#MultilineDefinition.
	fn visit_multilinedefinition(&mut self, ctx: &MultilineDefinitionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#MulDiv.
	fn visit_muldiv(&mut self, ctx: &MulDivContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#AddSub.
	fn visit_addsub(&mut self, ctx: &AddSubContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#Ternary.
	fn visit_ternary(&mut self, ctx: &TernaryContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#ParameterName.
	fn visit_parametername(&mut self, ctx: &ParameterNameContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#TypeLiteral.
	fn visit_typeliteral(&mut self, ctx: &TypeLiteralContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#Comparison.
	fn visit_comparison(&mut self, ctx: &ComparisonContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#And.
	fn visit_and(&mut self, ctx: &AndContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#ParenExpression.
	fn visit_parenexpression(&mut self, ctx: &ParenExpressionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#FunctionCall.
	fn visit_functioncall(&mut self, ctx: &FunctionCallContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#NotExpr.
	fn visit_notexpr(&mut self, ctx: &NotExprContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#Equality.
	fn visit_equality(&mut self, ctx: &EqualityContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by SubstraitTypeParser#LiteralNumber.
	fn visit_literalnumber(&mut self, ctx: &LiteralNumberContext<'input>) {
            self.visit_children(ctx)
        }

}