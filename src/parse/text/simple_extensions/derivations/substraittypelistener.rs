// SPDX-License-Identifier: Apache-2.0
#![allow(clippy::all)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(nonstandard_style)]
// Generated from SubstraitType.g4 by ANTLR 4.13.2
use antlr4rust::tree::ParseTreeListener;
use super::substraittypeparser::*;

pub trait SubstraitTypeListener<'input> : ParseTreeListener<'input,SubstraitTypeParserContextType>{
/**
 * Enter a parse tree produced by {@link SubstraitTypeParser#startRule}.
 * @param ctx the parse tree
 */
fn enter_startRule(&mut self, _ctx: &StartRuleContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitTypeParser#startRule}.
 * @param ctx the parse tree
 */
fn exit_startRule(&mut self, _ctx: &StartRuleContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitTypeParser#typeStatement}.
 * @param ctx the parse tree
 */
fn enter_typeStatement(&mut self, _ctx: &TypeStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitTypeParser#typeStatement}.
 * @param ctx the parse tree
 */
fn exit_typeStatement(&mut self, _ctx: &TypeStatementContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code boolean}
 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
 * @param ctx the parse tree
 */
fn enter_boolean(&mut self, _ctx: &BooleanContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code boolean}
 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
 * @param ctx the parse tree
 */
fn exit_boolean(&mut self, _ctx: &BooleanContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code i8}
 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
 * @param ctx the parse tree
 */
fn enter_i8(&mut self, _ctx: &I8Context<'input>) { }
/**
 * Exit a parse tree produced by the {@code i8}
 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
 * @param ctx the parse tree
 */
fn exit_i8(&mut self, _ctx: &I8Context<'input>) { }
/**
 * Enter a parse tree produced by the {@code i16}
 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
 * @param ctx the parse tree
 */
fn enter_i16(&mut self, _ctx: &I16Context<'input>) { }
/**
 * Exit a parse tree produced by the {@code i16}
 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
 * @param ctx the parse tree
 */
fn exit_i16(&mut self, _ctx: &I16Context<'input>) { }
/**
 * Enter a parse tree produced by the {@code i32}
 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
 * @param ctx the parse tree
 */
fn enter_i32(&mut self, _ctx: &I32Context<'input>) { }
/**
 * Exit a parse tree produced by the {@code i32}
 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
 * @param ctx the parse tree
 */
fn exit_i32(&mut self, _ctx: &I32Context<'input>) { }
/**
 * Enter a parse tree produced by the {@code i64}
 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
 * @param ctx the parse tree
 */
fn enter_i64(&mut self, _ctx: &I64Context<'input>) { }
/**
 * Exit a parse tree produced by the {@code i64}
 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
 * @param ctx the parse tree
 */
fn exit_i64(&mut self, _ctx: &I64Context<'input>) { }
/**
 * Enter a parse tree produced by the {@code fp32}
 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
 * @param ctx the parse tree
 */
fn enter_fp32(&mut self, _ctx: &Fp32Context<'input>) { }
/**
 * Exit a parse tree produced by the {@code fp32}
 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
 * @param ctx the parse tree
 */
fn exit_fp32(&mut self, _ctx: &Fp32Context<'input>) { }
/**
 * Enter a parse tree produced by the {@code fp64}
 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
 * @param ctx the parse tree
 */
fn enter_fp64(&mut self, _ctx: &Fp64Context<'input>) { }
/**
 * Exit a parse tree produced by the {@code fp64}
 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
 * @param ctx the parse tree
 */
fn exit_fp64(&mut self, _ctx: &Fp64Context<'input>) { }
/**
 * Enter a parse tree produced by the {@code string}
 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
 * @param ctx the parse tree
 */
fn enter_string(&mut self, _ctx: &StringContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code string}
 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
 * @param ctx the parse tree
 */
fn exit_string(&mut self, _ctx: &StringContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code binary}
 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
 * @param ctx the parse tree
 */
fn enter_binary(&mut self, _ctx: &BinaryContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code binary}
 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
 * @param ctx the parse tree
 */
fn exit_binary(&mut self, _ctx: &BinaryContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code timestamp}
 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
 * @param ctx the parse tree
 */
fn enter_timestamp(&mut self, _ctx: &TimestampContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code timestamp}
 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
 * @param ctx the parse tree
 */
fn exit_timestamp(&mut self, _ctx: &TimestampContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code timestampTz}
 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
 * @param ctx the parse tree
 */
fn enter_timestampTz(&mut self, _ctx: &TimestampTzContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code timestampTz}
 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
 * @param ctx the parse tree
 */
fn exit_timestampTz(&mut self, _ctx: &TimestampTzContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code date}
 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
 * @param ctx the parse tree
 */
fn enter_date(&mut self, _ctx: &DateContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code date}
 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
 * @param ctx the parse tree
 */
fn exit_date(&mut self, _ctx: &DateContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code time}
 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
 * @param ctx the parse tree
 */
fn enter_time(&mut self, _ctx: &TimeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code time}
 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
 * @param ctx the parse tree
 */
fn exit_time(&mut self, _ctx: &TimeContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code intervalYear}
 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
 * @param ctx the parse tree
 */
fn enter_intervalYear(&mut self, _ctx: &IntervalYearContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code intervalYear}
 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
 * @param ctx the parse tree
 */
fn exit_intervalYear(&mut self, _ctx: &IntervalYearContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code uuid}
 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
 * @param ctx the parse tree
 */
fn enter_uuid(&mut self, _ctx: &UuidContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code uuid}
 * labeled alternative in {@link SubstraitTypeParser#scalarType}.
 * @param ctx the parse tree
 */
fn exit_uuid(&mut self, _ctx: &UuidContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code fixedChar}
 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
 * @param ctx the parse tree
 */
fn enter_fixedChar(&mut self, _ctx: &FixedCharContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code fixedChar}
 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
 * @param ctx the parse tree
 */
fn exit_fixedChar(&mut self, _ctx: &FixedCharContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code varChar}
 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
 * @param ctx the parse tree
 */
fn enter_varChar(&mut self, _ctx: &VarCharContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code varChar}
 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
 * @param ctx the parse tree
 */
fn exit_varChar(&mut self, _ctx: &VarCharContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code fixedBinary}
 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
 * @param ctx the parse tree
 */
fn enter_fixedBinary(&mut self, _ctx: &FixedBinaryContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code fixedBinary}
 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
 * @param ctx the parse tree
 */
fn exit_fixedBinary(&mut self, _ctx: &FixedBinaryContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code decimal}
 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
 * @param ctx the parse tree
 */
fn enter_decimal(&mut self, _ctx: &DecimalContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code decimal}
 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
 * @param ctx the parse tree
 */
fn exit_decimal(&mut self, _ctx: &DecimalContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code precisionIntervalDay}
 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
 * @param ctx the parse tree
 */
fn enter_precisionIntervalDay(&mut self, _ctx: &PrecisionIntervalDayContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code precisionIntervalDay}
 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
 * @param ctx the parse tree
 */
fn exit_precisionIntervalDay(&mut self, _ctx: &PrecisionIntervalDayContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code precisionIntervalCompound}
 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
 * @param ctx the parse tree
 */
fn enter_precisionIntervalCompound(&mut self, _ctx: &PrecisionIntervalCompoundContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code precisionIntervalCompound}
 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
 * @param ctx the parse tree
 */
fn exit_precisionIntervalCompound(&mut self, _ctx: &PrecisionIntervalCompoundContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code precisionTime}
 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
 * @param ctx the parse tree
 */
fn enter_precisionTime(&mut self, _ctx: &PrecisionTimeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code precisionTime}
 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
 * @param ctx the parse tree
 */
fn exit_precisionTime(&mut self, _ctx: &PrecisionTimeContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code precisionTimestamp}
 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
 * @param ctx the parse tree
 */
fn enter_precisionTimestamp(&mut self, _ctx: &PrecisionTimestampContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code precisionTimestamp}
 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
 * @param ctx the parse tree
 */
fn exit_precisionTimestamp(&mut self, _ctx: &PrecisionTimestampContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code precisionTimestampTZ}
 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
 * @param ctx the parse tree
 */
fn enter_precisionTimestampTZ(&mut self, _ctx: &PrecisionTimestampTZContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code precisionTimestampTZ}
 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
 * @param ctx the parse tree
 */
fn exit_precisionTimestampTZ(&mut self, _ctx: &PrecisionTimestampTZContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code struct}
 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
 * @param ctx the parse tree
 */
fn enter_struct(&mut self, _ctx: &StructContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code struct}
 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
 * @param ctx the parse tree
 */
fn exit_struct(&mut self, _ctx: &StructContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code nStruct}
 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
 * @param ctx the parse tree
 */
fn enter_nStruct(&mut self, _ctx: &NStructContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code nStruct}
 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
 * @param ctx the parse tree
 */
fn exit_nStruct(&mut self, _ctx: &NStructContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code list}
 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
 * @param ctx the parse tree
 */
fn enter_list(&mut self, _ctx: &ListContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code list}
 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
 * @param ctx the parse tree
 */
fn exit_list(&mut self, _ctx: &ListContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code map}
 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
 * @param ctx the parse tree
 */
fn enter_map(&mut self, _ctx: &MapContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code map}
 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
 * @param ctx the parse tree
 */
fn exit_map(&mut self, _ctx: &MapContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code func}
 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
 * @param ctx the parse tree
 */
fn enter_func(&mut self, _ctx: &FuncContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code func}
 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
 * @param ctx the parse tree
 */
fn exit_func(&mut self, _ctx: &FuncContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code userDefined}
 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
 * @param ctx the parse tree
 */
fn enter_userDefined(&mut self, _ctx: &UserDefinedContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code userDefined}
 * labeled alternative in {@link SubstraitTypeParser#parameterizedType}.
 * @param ctx the parse tree
 */
fn exit_userDefined(&mut self, _ctx: &UserDefinedContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code singleFuncParam}
 * labeled alternative in {@link SubstraitTypeParser#funcParams}.
 * @param ctx the parse tree
 */
fn enter_singleFuncParam(&mut self, _ctx: &SingleFuncParamContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code singleFuncParam}
 * labeled alternative in {@link SubstraitTypeParser#funcParams}.
 * @param ctx the parse tree
 */
fn exit_singleFuncParam(&mut self, _ctx: &SingleFuncParamContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code funcParamsWithParens}
 * labeled alternative in {@link SubstraitTypeParser#funcParams}.
 * @param ctx the parse tree
 */
fn enter_funcParamsWithParens(&mut self, _ctx: &FuncParamsWithParensContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code funcParamsWithParens}
 * labeled alternative in {@link SubstraitTypeParser#funcParams}.
 * @param ctx the parse tree
 */
fn exit_funcParamsWithParens(&mut self, _ctx: &FuncParamsWithParensContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code numericLiteral}
 * labeled alternative in {@link SubstraitTypeParser#numericParameter}.
 * @param ctx the parse tree
 */
fn enter_numericLiteral(&mut self, _ctx: &NumericLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code numericLiteral}
 * labeled alternative in {@link SubstraitTypeParser#numericParameter}.
 * @param ctx the parse tree
 */
fn exit_numericLiteral(&mut self, _ctx: &NumericLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code numericParameterName}
 * labeled alternative in {@link SubstraitTypeParser#numericParameter}.
 * @param ctx the parse tree
 */
fn enter_numericParameterName(&mut self, _ctx: &NumericParameterNameContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code numericParameterName}
 * labeled alternative in {@link SubstraitTypeParser#numericParameter}.
 * @param ctx the parse tree
 */
fn exit_numericParameterName(&mut self, _ctx: &NumericParameterNameContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code numericExpression}
 * labeled alternative in {@link SubstraitTypeParser#numericParameter}.
 * @param ctx the parse tree
 */
fn enter_numericExpression(&mut self, _ctx: &NumericExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code numericExpression}
 * labeled alternative in {@link SubstraitTypeParser#numericParameter}.
 * @param ctx the parse tree
 */
fn exit_numericExpression(&mut self, _ctx: &NumericExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitTypeParser#anyType}.
 * @param ctx the parse tree
 */
fn enter_anyType(&mut self, _ctx: &AnyTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitTypeParser#anyType}.
 * @param ctx the parse tree
 */
fn exit_anyType(&mut self, _ctx: &AnyTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link SubstraitTypeParser#typeDef}.
 * @param ctx the parse tree
 */
fn enter_typeDef(&mut self, _ctx: &TypeDefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SubstraitTypeParser#typeDef}.
 * @param ctx the parse tree
 */
fn exit_typeDef(&mut self, _ctx: &TypeDefContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code IfExpr}
 * labeled alternative in {@link SubstraitTypeParser#expr}.
 * @param ctx the parse tree
 */
fn enter_IfExpr(&mut self, _ctx: &IfExprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code IfExpr}
 * labeled alternative in {@link SubstraitTypeParser#expr}.
 * @param ctx the parse tree
 */
fn exit_IfExpr(&mut self, _ctx: &IfExprContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code TypeLiteral}
 * labeled alternative in {@link SubstraitTypeParser#expr}.
 * @param ctx the parse tree
 */
fn enter_TypeLiteral(&mut self, _ctx: &TypeLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code TypeLiteral}
 * labeled alternative in {@link SubstraitTypeParser#expr}.
 * @param ctx the parse tree
 */
fn exit_TypeLiteral(&mut self, _ctx: &TypeLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code MultilineDefinition}
 * labeled alternative in {@link SubstraitTypeParser#expr}.
 * @param ctx the parse tree
 */
fn enter_MultilineDefinition(&mut self, _ctx: &MultilineDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code MultilineDefinition}
 * labeled alternative in {@link SubstraitTypeParser#expr}.
 * @param ctx the parse tree
 */
fn exit_MultilineDefinition(&mut self, _ctx: &MultilineDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Ternary}
 * labeled alternative in {@link SubstraitTypeParser#expr}.
 * @param ctx the parse tree
 */
fn enter_Ternary(&mut self, _ctx: &TernaryContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Ternary}
 * labeled alternative in {@link SubstraitTypeParser#expr}.
 * @param ctx the parse tree
 */
fn exit_Ternary(&mut self, _ctx: &TernaryContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code BinaryExpr}
 * labeled alternative in {@link SubstraitTypeParser#expr}.
 * @param ctx the parse tree
 */
fn enter_BinaryExpr(&mut self, _ctx: &BinaryExprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code BinaryExpr}
 * labeled alternative in {@link SubstraitTypeParser#expr}.
 * @param ctx the parse tree
 */
fn exit_BinaryExpr(&mut self, _ctx: &BinaryExprContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ParenExpression}
 * labeled alternative in {@link SubstraitTypeParser#expr}.
 * @param ctx the parse tree
 */
fn enter_ParenExpression(&mut self, _ctx: &ParenExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ParenExpression}
 * labeled alternative in {@link SubstraitTypeParser#expr}.
 * @param ctx the parse tree
 */
fn exit_ParenExpression(&mut self, _ctx: &ParenExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ParameterName}
 * labeled alternative in {@link SubstraitTypeParser#expr}.
 * @param ctx the parse tree
 */
fn enter_ParameterName(&mut self, _ctx: &ParameterNameContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ParameterName}
 * labeled alternative in {@link SubstraitTypeParser#expr}.
 * @param ctx the parse tree
 */
fn exit_ParameterName(&mut self, _ctx: &ParameterNameContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code FunctionCall}
 * labeled alternative in {@link SubstraitTypeParser#expr}.
 * @param ctx the parse tree
 */
fn enter_FunctionCall(&mut self, _ctx: &FunctionCallContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code FunctionCall}
 * labeled alternative in {@link SubstraitTypeParser#expr}.
 * @param ctx the parse tree
 */
fn exit_FunctionCall(&mut self, _ctx: &FunctionCallContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code NotExpr}
 * labeled alternative in {@link SubstraitTypeParser#expr}.
 * @param ctx the parse tree
 */
fn enter_NotExpr(&mut self, _ctx: &NotExprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code NotExpr}
 * labeled alternative in {@link SubstraitTypeParser#expr}.
 * @param ctx the parse tree
 */
fn exit_NotExpr(&mut self, _ctx: &NotExprContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code LiteralNumber}
 * labeled alternative in {@link SubstraitTypeParser#expr}.
 * @param ctx the parse tree
 */
fn enter_LiteralNumber(&mut self, _ctx: &LiteralNumberContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code LiteralNumber}
 * labeled alternative in {@link SubstraitTypeParser#expr}.
 * @param ctx the parse tree
 */
fn exit_LiteralNumber(&mut self, _ctx: &LiteralNumberContext<'input>) { }

}

antlr4rust::coerce_from!{ 'input : SubstraitTypeListener<'input> }


