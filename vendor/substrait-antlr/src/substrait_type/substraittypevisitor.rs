// SPDX-License-Identifier: Apache-2.0
#![allow(clippy::all)]
#![allow(unused_parens)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(nonstandard_style)]
// Generated from SubstraitType.g4 by ANTLR 4.13.2
use antlr4rust::tree::{ParseTreeVisitor,ParseTreeVisitorCompat};
use super::substraittypeparser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link SubstraitTypeParser}.
 */
pub trait SubstraitTypeVisitor<'input>: ParseTreeVisitor<'input,SubstraitTypeParserContextType>{
	/**
	 * Visit a parse tree produced by {@link SubstraitTypeParser#startRule}.
	 * @param ctx the parse tree
	 */
	fn visit_startRule(&mut self, ctx: &StartRuleContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SubstraitTypeParser#typeStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_typeStatement(&mut self, ctx: &TypeStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code boolean}
	 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
	 * @param ctx the parse tree
	 */
	fn visit_boolean(&mut self, ctx: &BooleanContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code i8}
	 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
	 * @param ctx the parse tree
	 */
	fn visit_i8(&mut self, ctx: &I8Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code i16}
	 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
	 * @param ctx the parse tree
	 */
	fn visit_i16(&mut self, ctx: &I16Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code i32}
	 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
	 * @param ctx the parse tree
	 */
	fn visit_i32(&mut self, ctx: &I32Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code i64}
	 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
	 * @param ctx the parse tree
	 */
	fn visit_i64(&mut self, ctx: &I64Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code fp32}
	 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
	 * @param ctx the parse tree
	 */
	fn visit_fp32(&mut self, ctx: &Fp32Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code fp64}
	 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
	 * @param ctx the parse tree
	 */
	fn visit_fp64(&mut self, ctx: &Fp64Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code string}
	 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
	 * @param ctx the parse tree
	 */
	fn visit_string(&mut self, ctx: &StringContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code binary}
	 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
	 * @param ctx the parse tree
	 */
	fn visit_binary(&mut self, ctx: &BinaryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code date}
	 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
	 * @param ctx the parse tree
	 */
	fn visit_date(&mut self, ctx: &DateContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code intervalYear}
	 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
	 * @param ctx the parse tree
	 */
	fn visit_intervalYear(&mut self, ctx: &IntervalYearContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code uuid}
	 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
	 * @param ctx the parse tree
	 */
	fn visit_uuid(&mut self, ctx: &UuidContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code fixedChar}
	 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
	 * @param ctx the parse tree
	 */
	fn visit_fixedChar(&mut self, ctx: &FixedCharContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code varChar}
	 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
	 * @param ctx the parse tree
	 */
	fn visit_varChar(&mut self, ctx: &VarCharContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code fixedBinary}
	 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
	 * @param ctx the parse tree
	 */
	fn visit_fixedBinary(&mut self, ctx: &FixedBinaryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code decimal}
	 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
	 * @param ctx the parse tree
	 */
	fn visit_decimal(&mut self, ctx: &DecimalContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code precisionIntervalDay}
	 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
	 * @param ctx the parse tree
	 */
	fn visit_precisionIntervalDay(&mut self, ctx: &PrecisionIntervalDayContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code precisionIntervalCompound}
	 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
	 * @param ctx the parse tree
	 */
	fn visit_precisionIntervalCompound(&mut self, ctx: &PrecisionIntervalCompoundContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code precisionTime}
	 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
	 * @param ctx the parse tree
	 */
	fn visit_precisionTime(&mut self, ctx: &PrecisionTimeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code precisionTimestamp}
	 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
	 * @param ctx the parse tree
	 */
	fn visit_precisionTimestamp(&mut self, ctx: &PrecisionTimestampContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code precisionTimestampTZ}
	 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
	 * @param ctx the parse tree
	 */
	fn visit_precisionTimestampTZ(&mut self, ctx: &PrecisionTimestampTZContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code struct}
	 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
	 * @param ctx the parse tree
	 */
	fn visit_struct(&mut self, ctx: &StructContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code nStruct}
	 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
	 * @param ctx the parse tree
	 */
	fn visit_nStruct(&mut self, ctx: &NStructContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code list}
	 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
	 * @param ctx the parse tree
	 */
	fn visit_list(&mut self, ctx: &ListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code map}
	 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
	 * @param ctx the parse tree
	 */
	fn visit_map(&mut self, ctx: &MapContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code func}
	 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
	 * @param ctx the parse tree
	 */
	fn visit_func(&mut self, ctx: &FuncContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code userDefined}
	 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
	 * @param ctx the parse tree
	 */
	fn visit_userDefined(&mut self, ctx: &UserDefinedContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code singleFuncParam}
	 * labeled alternative in {@link SubstraitTypeParser#funcParams}.
	 * @param ctx the parse tree
	 */
	fn visit_singleFuncParam(&mut self, ctx: &SingleFuncParamContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code funcParamsWithParens}
	 * labeled alternative in {@link SubstraitTypeParser#funcParams}.
	 * @param ctx the parse tree
	 */
	fn visit_funcParamsWithParens(&mut self, ctx: &FuncParamsWithParensContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code numericLiteral}
	 * labeled alternative in {@link SubstraitTypeParser#numericParameter}.
	 * @param ctx the parse tree
	 */
	fn visit_numericLiteral(&mut self, ctx: &NumericLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code numericParameterName}
	 * labeled alternative in {@link SubstraitTypeParser#numericParameter}.
	 * @param ctx the parse tree
	 */
	fn visit_numericParameterName(&mut self, ctx: &NumericParameterNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code numericExpression}
	 * labeled alternative in {@link SubstraitTypeParser#numericParameter}.
	 * @param ctx the parse tree
	 */
	fn visit_numericExpression(&mut self, ctx: &NumericExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SubstraitTypeParser#anyType}.
	 * @param ctx the parse tree
	 */
	fn visit_anyType(&mut self, ctx: &AnyTypeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SubstraitTypeParser#typeDef}.
	 * @param ctx the parse tree
	 */
	fn visit_typeDef(&mut self, ctx: &TypeDefContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code IfExpr}
	 * labeled alternative in {@link SubstraitTypeParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_IfExpr(&mut self, ctx: &IfExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code Or}
	 * labeled alternative in {@link SubstraitTypeParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_Or(&mut self, ctx: &OrContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code MultilineDefinition}
	 * labeled alternative in {@link SubstraitTypeParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_MultilineDefinition(&mut self, ctx: &MultilineDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code MulDiv}
	 * labeled alternative in {@link SubstraitTypeParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_MulDiv(&mut self, ctx: &MulDivContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code AddSub}
	 * labeled alternative in {@link SubstraitTypeParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_AddSub(&mut self, ctx: &AddSubContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code Ternary}
	 * labeled alternative in {@link SubstraitTypeParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_Ternary(&mut self, ctx: &TernaryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code ParameterName}
	 * labeled alternative in {@link SubstraitTypeParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_ParameterName(&mut self, ctx: &ParameterNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code TypeLiteral}
	 * labeled alternative in {@link SubstraitTypeParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_TypeLiteral(&mut self, ctx: &TypeLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code Comparison}
	 * labeled alternative in {@link SubstraitTypeParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_Comparison(&mut self, ctx: &ComparisonContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code And}
	 * labeled alternative in {@link SubstraitTypeParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_And(&mut self, ctx: &AndContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code ParenExpression}
	 * labeled alternative in {@link SubstraitTypeParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_ParenExpression(&mut self, ctx: &ParenExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code FunctionCall}
	 * labeled alternative in {@link SubstraitTypeParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_FunctionCall(&mut self, ctx: &FunctionCallContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code NotExpr}
	 * labeled alternative in {@link SubstraitTypeParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_NotExpr(&mut self, ctx: &NotExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code Equality}
	 * labeled alternative in {@link SubstraitTypeParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_Equality(&mut self, ctx: &EqualityContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code LiteralNumber}
	 * labeled alternative in {@link SubstraitTypeParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_LiteralNumber(&mut self, ctx: &LiteralNumberContext<'input>) { self.visit_children(ctx) }

}

pub trait SubstraitTypeVisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= SubstraitTypeParserContextType>{
	/**
	 * Visit a parse tree produced by {@link SubstraitTypeParser#startRule}.
	 * @param ctx the parse tree
	 */
		fn visit_startRule(&mut self, ctx: &StartRuleContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SubstraitTypeParser#typeStatement}.
	 * @param ctx the parse tree
	 */
		fn visit_typeStatement(&mut self, ctx: &TypeStatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code boolean}
	 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
	 * @param ctx the parse tree
	 */
		fn visit_boolean(&mut self, ctx: &BooleanContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code i8}
	 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
	 * @param ctx the parse tree
	 */
		fn visit_i8(&mut self, ctx: &I8Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code i16}
	 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
	 * @param ctx the parse tree
	 */
		fn visit_i16(&mut self, ctx: &I16Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code i32}
	 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
	 * @param ctx the parse tree
	 */
		fn visit_i32(&mut self, ctx: &I32Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code i64}
	 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
	 * @param ctx the parse tree
	 */
		fn visit_i64(&mut self, ctx: &I64Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code fp32}
	 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
	 * @param ctx the parse tree
	 */
		fn visit_fp32(&mut self, ctx: &Fp32Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code fp64}
	 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
	 * @param ctx the parse tree
	 */
		fn visit_fp64(&mut self, ctx: &Fp64Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code string}
	 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
	 * @param ctx the parse tree
	 */
		fn visit_string(&mut self, ctx: &StringContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code binary}
	 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
	 * @param ctx the parse tree
	 */
		fn visit_binary(&mut self, ctx: &BinaryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code date}
	 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
	 * @param ctx the parse tree
	 */
		fn visit_date(&mut self, ctx: &DateContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code intervalYear}
	 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
	 * @param ctx the parse tree
	 */
		fn visit_intervalYear(&mut self, ctx: &IntervalYearContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code uuid}
	 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
	 * @param ctx the parse tree
	 */
		fn visit_uuid(&mut self, ctx: &UuidContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code fixedChar}
	 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
	 * @param ctx the parse tree
	 */
		fn visit_fixedChar(&mut self, ctx: &FixedCharContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code varChar}
	 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
	 * @param ctx the parse tree
	 */
		fn visit_varChar(&mut self, ctx: &VarCharContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code fixedBinary}
	 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
	 * @param ctx the parse tree
	 */
		fn visit_fixedBinary(&mut self, ctx: &FixedBinaryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code decimal}
	 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
	 * @param ctx the parse tree
	 */
		fn visit_decimal(&mut self, ctx: &DecimalContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code precisionIntervalDay}
	 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
	 * @param ctx the parse tree
	 */
		fn visit_precisionIntervalDay(&mut self, ctx: &PrecisionIntervalDayContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code precisionIntervalCompound}
	 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
	 * @param ctx the parse tree
	 */
		fn visit_precisionIntervalCompound(&mut self, ctx: &PrecisionIntervalCompoundContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code precisionTime}
	 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
	 * @param ctx the parse tree
	 */
		fn visit_precisionTime(&mut self, ctx: &PrecisionTimeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code precisionTimestamp}
	 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
	 * @param ctx the parse tree
	 */
		fn visit_precisionTimestamp(&mut self, ctx: &PrecisionTimestampContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code precisionTimestampTZ}
	 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
	 * @param ctx the parse tree
	 */
		fn visit_precisionTimestampTZ(&mut self, ctx: &PrecisionTimestampTZContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code struct}
	 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
	 * @param ctx the parse tree
	 */
		fn visit_struct(&mut self, ctx: &StructContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code nStruct}
	 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
	 * @param ctx the parse tree
	 */
		fn visit_nStruct(&mut self, ctx: &NStructContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code list}
	 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
	 * @param ctx the parse tree
	 */
		fn visit_list(&mut self, ctx: &ListContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code map}
	 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
	 * @param ctx the parse tree
	 */
		fn visit_map(&mut self, ctx: &MapContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code func}
	 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
	 * @param ctx the parse tree
	 */
		fn visit_func(&mut self, ctx: &FuncContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code userDefined}
	 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
	 * @param ctx the parse tree
	 */
		fn visit_userDefined(&mut self, ctx: &UserDefinedContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code singleFuncParam}
	 * labeled alternative in {@link SubstraitTypeParser#funcParams}.
	 * @param ctx the parse tree
	 */
		fn visit_singleFuncParam(&mut self, ctx: &SingleFuncParamContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code funcParamsWithParens}
	 * labeled alternative in {@link SubstraitTypeParser#funcParams}.
	 * @param ctx the parse tree
	 */
		fn visit_funcParamsWithParens(&mut self, ctx: &FuncParamsWithParensContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code numericLiteral}
	 * labeled alternative in {@link SubstraitTypeParser#numericParameter}.
	 * @param ctx the parse tree
	 */
		fn visit_numericLiteral(&mut self, ctx: &NumericLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code numericParameterName}
	 * labeled alternative in {@link SubstraitTypeParser#numericParameter}.
	 * @param ctx the parse tree
	 */
		fn visit_numericParameterName(&mut self, ctx: &NumericParameterNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code numericExpression}
	 * labeled alternative in {@link SubstraitTypeParser#numericParameter}.
	 * @param ctx the parse tree
	 */
		fn visit_numericExpression(&mut self, ctx: &NumericExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SubstraitTypeParser#anyType}.
	 * @param ctx the parse tree
	 */
		fn visit_anyType(&mut self, ctx: &AnyTypeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link SubstraitTypeParser#typeDef}.
	 * @param ctx the parse tree
	 */
		fn visit_typeDef(&mut self, ctx: &TypeDefContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code IfExpr}
	 * labeled alternative in {@link SubstraitTypeParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_IfExpr(&mut self, ctx: &IfExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code Or}
	 * labeled alternative in {@link SubstraitTypeParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_Or(&mut self, ctx: &OrContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code MultilineDefinition}
	 * labeled alternative in {@link SubstraitTypeParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_MultilineDefinition(&mut self, ctx: &MultilineDefinitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code MulDiv}
	 * labeled alternative in {@link SubstraitTypeParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_MulDiv(&mut self, ctx: &MulDivContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code AddSub}
	 * labeled alternative in {@link SubstraitTypeParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_AddSub(&mut self, ctx: &AddSubContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code Ternary}
	 * labeled alternative in {@link SubstraitTypeParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_Ternary(&mut self, ctx: &TernaryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code ParameterName}
	 * labeled alternative in {@link SubstraitTypeParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_ParameterName(&mut self, ctx: &ParameterNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code TypeLiteral}
	 * labeled alternative in {@link SubstraitTypeParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_TypeLiteral(&mut self, ctx: &TypeLiteralContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code Comparison}
	 * labeled alternative in {@link SubstraitTypeParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_Comparison(&mut self, ctx: &ComparisonContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code And}
	 * labeled alternative in {@link SubstraitTypeParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_And(&mut self, ctx: &AndContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code ParenExpression}
	 * labeled alternative in {@link SubstraitTypeParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_ParenExpression(&mut self, ctx: &ParenExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code FunctionCall}
	 * labeled alternative in {@link SubstraitTypeParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_FunctionCall(&mut self, ctx: &FunctionCallContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code NotExpr}
	 * labeled alternative in {@link SubstraitTypeParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_NotExpr(&mut self, ctx: &NotExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code Equality}
	 * labeled alternative in {@link SubstraitTypeParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_Equality(&mut self, ctx: &EqualityContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code LiteralNumber}
	 * labeled alternative in {@link SubstraitTypeParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_LiteralNumber(&mut self, ctx: &LiteralNumberContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> SubstraitTypeVisitor<'input> for T
where
	T: SubstraitTypeVisitorCompat<'input>
{
	fn visit_startRule(&mut self, ctx: &StartRuleContext<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_startRule(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_typeStatement(&mut self, ctx: &TypeStatementContext<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_typeStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_boolean(&mut self, ctx: &BooleanContext<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_boolean(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_i8(&mut self, ctx: &I8Context<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_i8(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_i16(&mut self, ctx: &I16Context<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_i16(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_i32(&mut self, ctx: &I32Context<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_i32(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_i64(&mut self, ctx: &I64Context<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_i64(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fp32(&mut self, ctx: &Fp32Context<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_fp32(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fp64(&mut self, ctx: &Fp64Context<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_fp64(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_string(&mut self, ctx: &StringContext<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_string(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_binary(&mut self, ctx: &BinaryContext<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_binary(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_date(&mut self, ctx: &DateContext<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_date(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_intervalYear(&mut self, ctx: &IntervalYearContext<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_intervalYear(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_uuid(&mut self, ctx: &UuidContext<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_uuid(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fixedChar(&mut self, ctx: &FixedCharContext<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_fixedChar(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_varChar(&mut self, ctx: &VarCharContext<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_varChar(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fixedBinary(&mut self, ctx: &FixedBinaryContext<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_fixedBinary(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_decimal(&mut self, ctx: &DecimalContext<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_decimal(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_precisionIntervalDay(&mut self, ctx: &PrecisionIntervalDayContext<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_precisionIntervalDay(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_precisionIntervalCompound(&mut self, ctx: &PrecisionIntervalCompoundContext<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_precisionIntervalCompound(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_precisionTime(&mut self, ctx: &PrecisionTimeContext<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_precisionTime(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_precisionTimestamp(&mut self, ctx: &PrecisionTimestampContext<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_precisionTimestamp(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_precisionTimestampTZ(&mut self, ctx: &PrecisionTimestampTZContext<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_precisionTimestampTZ(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_struct(&mut self, ctx: &StructContext<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_struct(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_nStruct(&mut self, ctx: &NStructContext<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_nStruct(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_list(&mut self, ctx: &ListContext<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_list(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_map(&mut self, ctx: &MapContext<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_map(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_func(&mut self, ctx: &FuncContext<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_func(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_userDefined(&mut self, ctx: &UserDefinedContext<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_userDefined(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_singleFuncParam(&mut self, ctx: &SingleFuncParamContext<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_singleFuncParam(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_funcParamsWithParens(&mut self, ctx: &FuncParamsWithParensContext<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_funcParamsWithParens(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_numericLiteral(&mut self, ctx: &NumericLiteralContext<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_numericLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_numericParameterName(&mut self, ctx: &NumericParameterNameContext<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_numericParameterName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_numericExpression(&mut self, ctx: &NumericExpressionContext<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_numericExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_anyType(&mut self, ctx: &AnyTypeContext<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_anyType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_typeDef(&mut self, ctx: &TypeDefContext<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_typeDef(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_IfExpr(&mut self, ctx: &IfExprContext<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_IfExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_Or(&mut self, ctx: &OrContext<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_Or(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_MultilineDefinition(&mut self, ctx: &MultilineDefinitionContext<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_MultilineDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_MulDiv(&mut self, ctx: &MulDivContext<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_MulDiv(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_AddSub(&mut self, ctx: &AddSubContext<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_AddSub(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_Ternary(&mut self, ctx: &TernaryContext<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_Ternary(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ParameterName(&mut self, ctx: &ParameterNameContext<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_ParameterName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_TypeLiteral(&mut self, ctx: &TypeLiteralContext<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_TypeLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_Comparison(&mut self, ctx: &ComparisonContext<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_Comparison(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_And(&mut self, ctx: &AndContext<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_And(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ParenExpression(&mut self, ctx: &ParenExpressionContext<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_ParenExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_FunctionCall(&mut self, ctx: &FunctionCallContext<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_FunctionCall(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_NotExpr(&mut self, ctx: &NotExprContext<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_NotExpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_Equality(&mut self, ctx: &EqualityContext<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_Equality(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_LiteralNumber(&mut self, ctx: &LiteralNumberContext<'input>){
		let result = <Self as SubstraitTypeVisitorCompat>::visit_LiteralNumber(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}