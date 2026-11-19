// SPDX-License-Identifier: Apache-2.0
#![allow(clippy::all)]
#![allow(unused_parens)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(nonstandard_style)]
// Generated from FuncTestCaseParser.g4 by ANTLR 4.13.2
use antlr4rust::tree::ParseTreeListener;
use super::functestcaseparser::*;

pub trait FuncTestCaseParserListener<'input> : ParseTreeListener<'input,FuncTestCaseParserContextType>{
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#doc}.
 * @param ctx the parse tree
 */
fn enter_doc(&mut self, _ctx: &DocContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#doc}.
 * @param ctx the parse tree
 */
fn exit_doc(&mut self, _ctx: &DocContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#header}.
 * @param ctx the parse tree
 */
fn enter_header(&mut self, _ctx: &HeaderContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#header}.
 * @param ctx the parse tree
 */
fn exit_header(&mut self, _ctx: &HeaderContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#version}.
 * @param ctx the parse tree
 */
fn enter_version(&mut self, _ctx: &VersionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#version}.
 * @param ctx the parse tree
 */
fn exit_version(&mut self, _ctx: &VersionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#include}.
 * @param ctx the parse tree
 */
fn enter_include(&mut self, _ctx: &IncludeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#include}.
 * @param ctx the parse tree
 */
fn exit_include(&mut self, _ctx: &IncludeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#dependency}.
 * @param ctx the parse tree
 */
fn enter_dependency(&mut self, _ctx: &DependencyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#dependency}.
 * @param ctx the parse tree
 */
fn exit_dependency(&mut self, _ctx: &DependencyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#testGroupDescription}.
 * @param ctx the parse tree
 */
fn enter_testGroupDescription(&mut self, _ctx: &TestGroupDescriptionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#testGroupDescription}.
 * @param ctx the parse tree
 */
fn exit_testGroupDescription(&mut self, _ctx: &TestGroupDescriptionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#testCase}.
 * @param ctx the parse tree
 */
fn enter_testCase(&mut self, _ctx: &TestCaseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#testCase}.
 * @param ctx the parse tree
 */
fn exit_testCase(&mut self, _ctx: &TestCaseContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code scalarFuncTestGroup}
 * labeled alternative in {@link FuncTestCaseParser#testGroup}.
 * @param ctx the parse tree
 */
fn enter_scalarFuncTestGroup(&mut self, _ctx: &ScalarFuncTestGroupContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code scalarFuncTestGroup}
 * labeled alternative in {@link FuncTestCaseParser#testGroup}.
 * @param ctx the parse tree
 */
fn exit_scalarFuncTestGroup(&mut self, _ctx: &ScalarFuncTestGroupContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code aggregateFuncTestGroup}
 * labeled alternative in {@link FuncTestCaseParser#testGroup}.
 * @param ctx the parse tree
 */
fn enter_aggregateFuncTestGroup(&mut self, _ctx: &AggregateFuncTestGroupContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code aggregateFuncTestGroup}
 * labeled alternative in {@link FuncTestCaseParser#testGroup}.
 * @param ctx the parse tree
 */
fn exit_aggregateFuncTestGroup(&mut self, _ctx: &AggregateFuncTestGroupContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#arguments}.
 * @param ctx the parse tree
 */
fn enter_arguments(&mut self, _ctx: &ArgumentsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#arguments}.
 * @param ctx the parse tree
 */
fn exit_arguments(&mut self, _ctx: &ArgumentsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#result}.
 * @param ctx the parse tree
 */
fn enter_result(&mut self, _ctx: &ResultContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#result}.
 * @param ctx the parse tree
 */
fn exit_result(&mut self, _ctx: &ResultContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#argument}.
 * @param ctx the parse tree
 */
fn enter_argument(&mut self, _ctx: &ArgumentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#argument}.
 * @param ctx the parse tree
 */
fn exit_argument(&mut self, _ctx: &ArgumentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#aggFuncTestCase}.
 * @param ctx the parse tree
 */
fn enter_aggFuncTestCase(&mut self, _ctx: &AggFuncTestCaseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#aggFuncTestCase}.
 * @param ctx the parse tree
 */
fn exit_aggFuncTestCase(&mut self, _ctx: &AggFuncTestCaseContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code multiArgAggregateFuncCall}
 * labeled alternative in {@link FuncTestCaseParser#aggFuncCall}.
 * @param ctx the parse tree
 */
fn enter_multiArgAggregateFuncCall(&mut self, _ctx: &MultiArgAggregateFuncCallContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code multiArgAggregateFuncCall}
 * labeled alternative in {@link FuncTestCaseParser#aggFuncCall}.
 * @param ctx the parse tree
 */
fn exit_multiArgAggregateFuncCall(&mut self, _ctx: &MultiArgAggregateFuncCallContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code compactAggregateFuncCall}
 * labeled alternative in {@link FuncTestCaseParser#aggFuncCall}.
 * @param ctx the parse tree
 */
fn enter_compactAggregateFuncCall(&mut self, _ctx: &CompactAggregateFuncCallContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code compactAggregateFuncCall}
 * labeled alternative in {@link FuncTestCaseParser#aggFuncCall}.
 * @param ctx the parse tree
 */
fn exit_compactAggregateFuncCall(&mut self, _ctx: &CompactAggregateFuncCallContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code singleArgAggregateFuncCall}
 * labeled alternative in {@link FuncTestCaseParser#aggFuncCall}.
 * @param ctx the parse tree
 */
fn enter_singleArgAggregateFuncCall(&mut self, _ctx: &SingleArgAggregateFuncCallContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code singleArgAggregateFuncCall}
 * labeled alternative in {@link FuncTestCaseParser#aggFuncCall}.
 * @param ctx the parse tree
 */
fn exit_singleArgAggregateFuncCall(&mut self, _ctx: &SingleArgAggregateFuncCallContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#tableData}.
 * @param ctx the parse tree
 */
fn enter_tableData(&mut self, _ctx: &TableDataContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#tableData}.
 * @param ctx the parse tree
 */
fn exit_tableData(&mut self, _ctx: &TableDataContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#tableRows}.
 * @param ctx the parse tree
 */
fn enter_tableRows(&mut self, _ctx: &TableRowsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#tableRows}.
 * @param ctx the parse tree
 */
fn exit_tableRows(&mut self, _ctx: &TableRowsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#dataColumn}.
 * @param ctx the parse tree
 */
fn enter_dataColumn(&mut self, _ctx: &DataColumnContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#dataColumn}.
 * @param ctx the parse tree
 */
fn exit_dataColumn(&mut self, _ctx: &DataColumnContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#columnValues}.
 * @param ctx the parse tree
 */
fn enter_columnValues(&mut self, _ctx: &ColumnValuesContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#columnValues}.
 * @param ctx the parse tree
 */
fn exit_columnValues(&mut self, _ctx: &ColumnValuesContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#literal}.
 * @param ctx the parse tree
 */
fn enter_literal(&mut self, _ctx: &LiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#literal}.
 * @param ctx the parse tree
 */
fn exit_literal(&mut self, _ctx: &LiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#qualifiedAggregateFuncArgs}.
 * @param ctx the parse tree
 */
fn enter_qualifiedAggregateFuncArgs(&mut self, _ctx: &QualifiedAggregateFuncArgsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#qualifiedAggregateFuncArgs}.
 * @param ctx the parse tree
 */
fn exit_qualifiedAggregateFuncArgs(&mut self, _ctx: &QualifiedAggregateFuncArgsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#aggregateFuncArgs}.
 * @param ctx the parse tree
 */
fn enter_aggregateFuncArgs(&mut self, _ctx: &AggregateFuncArgsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#aggregateFuncArgs}.
 * @param ctx the parse tree
 */
fn exit_aggregateFuncArgs(&mut self, _ctx: &AggregateFuncArgsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#qualifiedAggregateFuncArg}.
 * @param ctx the parse tree
 */
fn enter_qualifiedAggregateFuncArg(&mut self, _ctx: &QualifiedAggregateFuncArgContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#qualifiedAggregateFuncArg}.
 * @param ctx the parse tree
 */
fn exit_qualifiedAggregateFuncArg(&mut self, _ctx: &QualifiedAggregateFuncArgContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#aggregateFuncArg}.
 * @param ctx the parse tree
 */
fn enter_aggregateFuncArg(&mut self, _ctx: &AggregateFuncArgContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#aggregateFuncArg}.
 * @param ctx the parse tree
 */
fn exit_aggregateFuncArg(&mut self, _ctx: &AggregateFuncArgContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#numericLiteral}.
 * @param ctx the parse tree
 */
fn enter_numericLiteral(&mut self, _ctx: &NumericLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#numericLiteral}.
 * @param ctx the parse tree
 */
fn exit_numericLiteral(&mut self, _ctx: &NumericLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#floatLiteral}.
 * @param ctx the parse tree
 */
fn enter_floatLiteral(&mut self, _ctx: &FloatLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#floatLiteral}.
 * @param ctx the parse tree
 */
fn exit_floatLiteral(&mut self, _ctx: &FloatLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#nullArg}.
 * @param ctx the parse tree
 */
fn enter_nullArg(&mut self, _ctx: &NullArgContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#nullArg}.
 * @param ctx the parse tree
 */
fn exit_nullArg(&mut self, _ctx: &NullArgContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#intArg}.
 * @param ctx the parse tree
 */
fn enter_intArg(&mut self, _ctx: &IntArgContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#intArg}.
 * @param ctx the parse tree
 */
fn exit_intArg(&mut self, _ctx: &IntArgContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#floatArg}.
 * @param ctx the parse tree
 */
fn enter_floatArg(&mut self, _ctx: &FloatArgContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#floatArg}.
 * @param ctx the parse tree
 */
fn exit_floatArg(&mut self, _ctx: &FloatArgContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#decimalArg}.
 * @param ctx the parse tree
 */
fn enter_decimalArg(&mut self, _ctx: &DecimalArgContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#decimalArg}.
 * @param ctx the parse tree
 */
fn exit_decimalArg(&mut self, _ctx: &DecimalArgContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#booleanArg}.
 * @param ctx the parse tree
 */
fn enter_booleanArg(&mut self, _ctx: &BooleanArgContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#booleanArg}.
 * @param ctx the parse tree
 */
fn exit_booleanArg(&mut self, _ctx: &BooleanArgContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#stringArg}.
 * @param ctx the parse tree
 */
fn enter_stringArg(&mut self, _ctx: &StringArgContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#stringArg}.
 * @param ctx the parse tree
 */
fn exit_stringArg(&mut self, _ctx: &StringArgContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#dateArg}.
 * @param ctx the parse tree
 */
fn enter_dateArg(&mut self, _ctx: &DateArgContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#dateArg}.
 * @param ctx the parse tree
 */
fn exit_dateArg(&mut self, _ctx: &DateArgContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#intervalYearArg}.
 * @param ctx the parse tree
 */
fn enter_intervalYearArg(&mut self, _ctx: &IntervalYearArgContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#intervalYearArg}.
 * @param ctx the parse tree
 */
fn exit_intervalYearArg(&mut self, _ctx: &IntervalYearArgContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#intervalDayArg}.
 * @param ctx the parse tree
 */
fn enter_intervalDayArg(&mut self, _ctx: &IntervalDayArgContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#intervalDayArg}.
 * @param ctx the parse tree
 */
fn exit_intervalDayArg(&mut self, _ctx: &IntervalDayArgContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#intervalCompoundArg}.
 * @param ctx the parse tree
 */
fn enter_intervalCompoundArg(&mut self, _ctx: &IntervalCompoundArgContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#intervalCompoundArg}.
 * @param ctx the parse tree
 */
fn exit_intervalCompoundArg(&mut self, _ctx: &IntervalCompoundArgContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#fixedCharArg}.
 * @param ctx the parse tree
 */
fn enter_fixedCharArg(&mut self, _ctx: &FixedCharArgContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#fixedCharArg}.
 * @param ctx the parse tree
 */
fn exit_fixedCharArg(&mut self, _ctx: &FixedCharArgContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#varCharArg}.
 * @param ctx the parse tree
 */
fn enter_varCharArg(&mut self, _ctx: &VarCharArgContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#varCharArg}.
 * @param ctx the parse tree
 */
fn exit_varCharArg(&mut self, _ctx: &VarCharArgContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#fixedBinaryArg}.
 * @param ctx the parse tree
 */
fn enter_fixedBinaryArg(&mut self, _ctx: &FixedBinaryArgContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#fixedBinaryArg}.
 * @param ctx the parse tree
 */
fn exit_fixedBinaryArg(&mut self, _ctx: &FixedBinaryArgContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#precisionTimeArg}.
 * @param ctx the parse tree
 */
fn enter_precisionTimeArg(&mut self, _ctx: &PrecisionTimeArgContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#precisionTimeArg}.
 * @param ctx the parse tree
 */
fn exit_precisionTimeArg(&mut self, _ctx: &PrecisionTimeArgContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#precisionTimestampArg}.
 * @param ctx the parse tree
 */
fn enter_precisionTimestampArg(&mut self, _ctx: &PrecisionTimestampArgContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#precisionTimestampArg}.
 * @param ctx the parse tree
 */
fn exit_precisionTimestampArg(&mut self, _ctx: &PrecisionTimestampArgContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#precisionTimestampTZArg}.
 * @param ctx the parse tree
 */
fn enter_precisionTimestampTZArg(&mut self, _ctx: &PrecisionTimestampTZArgContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#precisionTimestampTZArg}.
 * @param ctx the parse tree
 */
fn exit_precisionTimestampTZArg(&mut self, _ctx: &PrecisionTimestampTZArgContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#listArg}.
 * @param ctx the parse tree
 */
fn enter_listArg(&mut self, _ctx: &ListArgContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#listArg}.
 * @param ctx the parse tree
 */
fn exit_listArg(&mut self, _ctx: &ListArgContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#structArg}.
 * @param ctx the parse tree
 */
fn enter_structArg(&mut self, _ctx: &StructArgContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#structArg}.
 * @param ctx the parse tree
 */
fn exit_structArg(&mut self, _ctx: &StructArgContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#mapArg}.
 * @param ctx the parse tree
 */
fn enter_mapArg(&mut self, _ctx: &MapArgContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#mapArg}.
 * @param ctx the parse tree
 */
fn exit_mapArg(&mut self, _ctx: &MapArgContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#userDefinedArg}.
 * @param ctx the parse tree
 */
fn enter_userDefinedArg(&mut self, _ctx: &UserDefinedArgContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#userDefinedArg}.
 * @param ctx the parse tree
 */
fn exit_userDefinedArg(&mut self, _ctx: &UserDefinedArgContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#lambdaArg}.
 * @param ctx the parse tree
 */
fn enter_lambdaArg(&mut self, _ctx: &LambdaArgContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#lambdaArg}.
 * @param ctx the parse tree
 */
fn exit_lambdaArg(&mut self, _ctx: &LambdaArgContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#funcCallArg}.
 * @param ctx the parse tree
 */
fn enter_funcCallArg(&mut self, _ctx: &FuncCallArgContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#funcCallArg}.
 * @param ctx the parse tree
 */
fn exit_funcCallArg(&mut self, _ctx: &FuncCallArgContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#enumArg}.
 * @param ctx the parse tree
 */
fn enter_enumArg(&mut self, _ctx: &EnumArgContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#enumArg}.
 * @param ctx the parse tree
 */
fn exit_enumArg(&mut self, _ctx: &EnumArgContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#literalList}.
 * @param ctx the parse tree
 */
fn enter_literalList(&mut self, _ctx: &LiteralListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#literalList}.
 * @param ctx the parse tree
 */
fn exit_literalList(&mut self, _ctx: &LiteralListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#literalStruct}.
 * @param ctx the parse tree
 */
fn enter_literalStruct(&mut self, _ctx: &LiteralStructContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#literalStruct}.
 * @param ctx the parse tree
 */
fn exit_literalStruct(&mut self, _ctx: &LiteralStructContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#literalMap}.
 * @param ctx the parse tree
 */
fn enter_literalMap(&mut self, _ctx: &LiteralMapContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#literalMap}.
 * @param ctx the parse tree
 */
fn exit_literalMap(&mut self, _ctx: &LiteralMapContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#mapEntry}.
 * @param ctx the parse tree
 */
fn enter_mapEntry(&mut self, _ctx: &MapEntryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#mapEntry}.
 * @param ctx the parse tree
 */
fn exit_mapEntry(&mut self, _ctx: &MapEntryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#compoundLiteral}.
 * @param ctx the parse tree
 */
fn enter_compoundLiteral(&mut self, _ctx: &CompoundLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#compoundLiteral}.
 * @param ctx the parse tree
 */
fn exit_compoundLiteral(&mut self, _ctx: &CompoundLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#literalLambda}.
 * @param ctx the parse tree
 */
fn enter_literalLambda(&mut self, _ctx: &LiteralLambdaContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#literalLambda}.
 * @param ctx the parse tree
 */
fn exit_literalLambda(&mut self, _ctx: &LiteralLambdaContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code singleParam}
 * labeled alternative in {@link FuncTestCaseParser#lambdaParameters}.
 * @param ctx the parse tree
 */
fn enter_singleParam(&mut self, _ctx: &SingleParamContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code singleParam}
 * labeled alternative in {@link FuncTestCaseParser#lambdaParameters}.
 * @param ctx the parse tree
 */
fn exit_singleParam(&mut self, _ctx: &SingleParamContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code tupleParams}
 * labeled alternative in {@link FuncTestCaseParser#lambdaParameters}.
 * @param ctx the parse tree
 */
fn enter_tupleParams(&mut self, _ctx: &TupleParamsContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code tupleParams}
 * labeled alternative in {@link FuncTestCaseParser#lambdaParameters}.
 * @param ctx the parse tree
 */
fn exit_tupleParams(&mut self, _ctx: &TupleParamsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#lambdaBody}.
 * @param ctx the parse tree
 */
fn enter_lambdaBody(&mut self, _ctx: &LambdaBodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#lambdaBody}.
 * @param ctx the parse tree
 */
fn exit_lambdaBody(&mut self, _ctx: &LambdaBodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#dataType}.
 * @param ctx the parse tree
 */
fn enter_dataType(&mut self, _ctx: &DataTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#dataType}.
 * @param ctx the parse tree
 */
fn exit_dataType(&mut self, _ctx: &DataTypeContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code boolean}
 * labeled alternative in {@link FuncTestCaseParser#scalarType}.
 * @param ctx the parse tree
 */
fn enter_boolean(&mut self, _ctx: &BooleanContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code boolean}
 * labeled alternative in {@link FuncTestCaseParser#scalarType}.
 * @param ctx the parse tree
 */
fn exit_boolean(&mut self, _ctx: &BooleanContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code int}
 * labeled alternative in {@link FuncTestCaseParser#scalarType}.
 * @param ctx the parse tree
 */
fn enter_int(&mut self, _ctx: &IntContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code int}
 * labeled alternative in {@link FuncTestCaseParser#scalarType}.
 * @param ctx the parse tree
 */
fn exit_int(&mut self, _ctx: &IntContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code float}
 * labeled alternative in {@link FuncTestCaseParser#scalarType}.
 * @param ctx the parse tree
 */
fn enter_float(&mut self, _ctx: &FloatContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code float}
 * labeled alternative in {@link FuncTestCaseParser#scalarType}.
 * @param ctx the parse tree
 */
fn exit_float(&mut self, _ctx: &FloatContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code string}
 * labeled alternative in {@link FuncTestCaseParser#scalarType}.
 * @param ctx the parse tree
 */
fn enter_string(&mut self, _ctx: &StringContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code string}
 * labeled alternative in {@link FuncTestCaseParser#scalarType}.
 * @param ctx the parse tree
 */
fn exit_string(&mut self, _ctx: &StringContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code binary}
 * labeled alternative in {@link FuncTestCaseParser#scalarType}.
 * @param ctx the parse tree
 */
fn enter_binary(&mut self, _ctx: &BinaryContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code binary}
 * labeled alternative in {@link FuncTestCaseParser#scalarType}.
 * @param ctx the parse tree
 */
fn exit_binary(&mut self, _ctx: &BinaryContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code date}
 * labeled alternative in {@link FuncTestCaseParser#scalarType}.
 * @param ctx the parse tree
 */
fn enter_date(&mut self, _ctx: &DateContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code date}
 * labeled alternative in {@link FuncTestCaseParser#scalarType}.
 * @param ctx the parse tree
 */
fn exit_date(&mut self, _ctx: &DateContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code intervalYear}
 * labeled alternative in {@link FuncTestCaseParser#scalarType}.
 * @param ctx the parse tree
 */
fn enter_intervalYear(&mut self, _ctx: &IntervalYearContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code intervalYear}
 * labeled alternative in {@link FuncTestCaseParser#scalarType}.
 * @param ctx the parse tree
 */
fn exit_intervalYear(&mut self, _ctx: &IntervalYearContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code uuid}
 * labeled alternative in {@link FuncTestCaseParser#scalarType}.
 * @param ctx the parse tree
 */
fn enter_uuid(&mut self, _ctx: &UuidContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code uuid}
 * labeled alternative in {@link FuncTestCaseParser#scalarType}.
 * @param ctx the parse tree
 */
fn exit_uuid(&mut self, _ctx: &UuidContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code userDefined}
 * labeled alternative in {@link FuncTestCaseParser#scalarType}.
 * @param ctx the parse tree
 */
fn enter_userDefined(&mut self, _ctx: &UserDefinedContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code userDefined}
 * labeled alternative in {@link FuncTestCaseParser#scalarType}.
 * @param ctx the parse tree
 */
fn exit_userDefined(&mut self, _ctx: &UserDefinedContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#userDefinedType}.
 * @param ctx the parse tree
 */
fn enter_userDefinedType(&mut self, _ctx: &UserDefinedTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#userDefinedType}.
 * @param ctx the parse tree
 */
fn exit_userDefinedType(&mut self, _ctx: &UserDefinedTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#booleanType}.
 * @param ctx the parse tree
 */
fn enter_booleanType(&mut self, _ctx: &BooleanTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#booleanType}.
 * @param ctx the parse tree
 */
fn exit_booleanType(&mut self, _ctx: &BooleanTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#stringType}.
 * @param ctx the parse tree
 */
fn enter_stringType(&mut self, _ctx: &StringTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#stringType}.
 * @param ctx the parse tree
 */
fn exit_stringType(&mut self, _ctx: &StringTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#binaryType}.
 * @param ctx the parse tree
 */
fn enter_binaryType(&mut self, _ctx: &BinaryTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#binaryType}.
 * @param ctx the parse tree
 */
fn exit_binaryType(&mut self, _ctx: &BinaryTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#intType}.
 * @param ctx the parse tree
 */
fn enter_intType(&mut self, _ctx: &IntTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#intType}.
 * @param ctx the parse tree
 */
fn exit_intType(&mut self, _ctx: &IntTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#floatType}.
 * @param ctx the parse tree
 */
fn enter_floatType(&mut self, _ctx: &FloatTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#floatType}.
 * @param ctx the parse tree
 */
fn exit_floatType(&mut self, _ctx: &FloatTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#dateType}.
 * @param ctx the parse tree
 */
fn enter_dateType(&mut self, _ctx: &DateTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#dateType}.
 * @param ctx the parse tree
 */
fn exit_dateType(&mut self, _ctx: &DateTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#intervalYearType}.
 * @param ctx the parse tree
 */
fn enter_intervalYearType(&mut self, _ctx: &IntervalYearTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#intervalYearType}.
 * @param ctx the parse tree
 */
fn exit_intervalYearType(&mut self, _ctx: &IntervalYearTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#intervalDayType}.
 * @param ctx the parse tree
 */
fn enter_intervalDayType(&mut self, _ctx: &IntervalDayTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#intervalDayType}.
 * @param ctx the parse tree
 */
fn exit_intervalDayType(&mut self, _ctx: &IntervalDayTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#intervalCompoundType}.
 * @param ctx the parse tree
 */
fn enter_intervalCompoundType(&mut self, _ctx: &IntervalCompoundTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#intervalCompoundType}.
 * @param ctx the parse tree
 */
fn exit_intervalCompoundType(&mut self, _ctx: &IntervalCompoundTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#fixedCharType}.
 * @param ctx the parse tree
 */
fn enter_fixedCharType(&mut self, _ctx: &FixedCharTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#fixedCharType}.
 * @param ctx the parse tree
 */
fn exit_fixedCharType(&mut self, _ctx: &FixedCharTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#varCharType}.
 * @param ctx the parse tree
 */
fn enter_varCharType(&mut self, _ctx: &VarCharTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#varCharType}.
 * @param ctx the parse tree
 */
fn exit_varCharType(&mut self, _ctx: &VarCharTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#fixedBinaryType}.
 * @param ctx the parse tree
 */
fn enter_fixedBinaryType(&mut self, _ctx: &FixedBinaryTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#fixedBinaryType}.
 * @param ctx the parse tree
 */
fn exit_fixedBinaryType(&mut self, _ctx: &FixedBinaryTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#decimalType}.
 * @param ctx the parse tree
 */
fn enter_decimalType(&mut self, _ctx: &DecimalTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#decimalType}.
 * @param ctx the parse tree
 */
fn exit_decimalType(&mut self, _ctx: &DecimalTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#precisionTimeType}.
 * @param ctx the parse tree
 */
fn enter_precisionTimeType(&mut self, _ctx: &PrecisionTimeTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#precisionTimeType}.
 * @param ctx the parse tree
 */
fn exit_precisionTimeType(&mut self, _ctx: &PrecisionTimeTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#precisionTimestampType}.
 * @param ctx the parse tree
 */
fn enter_precisionTimestampType(&mut self, _ctx: &PrecisionTimestampTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#precisionTimestampType}.
 * @param ctx the parse tree
 */
fn exit_precisionTimestampType(&mut self, _ctx: &PrecisionTimestampTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#precisionTimestampTZType}.
 * @param ctx the parse tree
 */
fn enter_precisionTimestampTZType(&mut self, _ctx: &PrecisionTimestampTZTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#precisionTimestampTZType}.
 * @param ctx the parse tree
 */
fn exit_precisionTimestampTZType(&mut self, _ctx: &PrecisionTimestampTZTypeContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code list}
 * labeled alternative in {@link FuncTestCaseParser#listType}.
 * @param ctx the parse tree
 */
fn enter_list(&mut self, _ctx: &ListContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code list}
 * labeled alternative in {@link FuncTestCaseParser#listType}.
 * @param ctx the parse tree
 */
fn exit_list(&mut self, _ctx: &ListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#structType}.
 * @param ctx the parse tree
 */
fn enter_structType(&mut self, _ctx: &StructTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#structType}.
 * @param ctx the parse tree
 */
fn exit_structType(&mut self, _ctx: &StructTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#mapType}.
 * @param ctx the parse tree
 */
fn enter_mapType(&mut self, _ctx: &MapTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#mapType}.
 * @param ctx the parse tree
 */
fn exit_mapType(&mut self, _ctx: &MapTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#funcType}.
 * @param ctx the parse tree
 */
fn enter_funcType(&mut self, _ctx: &FuncTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#funcType}.
 * @param ctx the parse tree
 */
fn exit_funcType(&mut self, _ctx: &FuncTypeContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code singleFuncParam}
 * labeled alternative in {@link FuncTestCaseParser#funcParameters}.
 * @param ctx the parse tree
 */
fn enter_singleFuncParam(&mut self, _ctx: &SingleFuncParamContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code singleFuncParam}
 * labeled alternative in {@link FuncTestCaseParser#funcParameters}.
 * @param ctx the parse tree
 */
fn exit_singleFuncParam(&mut self, _ctx: &SingleFuncParamContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code funcParamsWithParens}
 * labeled alternative in {@link FuncTestCaseParser#funcParameters}.
 * @param ctx the parse tree
 */
fn enter_funcParamsWithParens(&mut self, _ctx: &FuncParamsWithParensContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code funcParamsWithParens}
 * labeled alternative in {@link FuncTestCaseParser#funcParameters}.
 * @param ctx the parse tree
 */
fn exit_funcParamsWithParens(&mut self, _ctx: &FuncParamsWithParensContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#parameterizedType}.
 * @param ctx the parse tree
 */
fn enter_parameterizedType(&mut self, _ctx: &ParameterizedTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#parameterizedType}.
 * @param ctx the parse tree
 */
fn exit_parameterizedType(&mut self, _ctx: &ParameterizedTypeContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code integerLiteral}
 * labeled alternative in {@link FuncTestCaseParser#numericParameter}.
 * @param ctx the parse tree
 */
fn enter_integerLiteral(&mut self, _ctx: &IntegerLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code integerLiteral}
 * labeled alternative in {@link FuncTestCaseParser#numericParameter}.
 * @param ctx the parse tree
 */
fn exit_integerLiteral(&mut self, _ctx: &IntegerLiteralContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#substraitError}.
 * @param ctx the parse tree
 */
fn enter_substraitError(&mut self, _ctx: &SubstraitErrorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#substraitError}.
 * @param ctx the parse tree
 */
fn exit_substraitError(&mut self, _ctx: &SubstraitErrorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#funcOption}.
 * @param ctx the parse tree
 */
fn enter_funcOption(&mut self, _ctx: &FuncOptionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#funcOption}.
 * @param ctx the parse tree
 */
fn exit_funcOption(&mut self, _ctx: &FuncOptionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#optionName}.
 * @param ctx the parse tree
 */
fn enter_optionName(&mut self, _ctx: &OptionNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#optionName}.
 * @param ctx the parse tree
 */
fn exit_optionName(&mut self, _ctx: &OptionNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#optionValue}.
 * @param ctx the parse tree
 */
fn enter_optionValue(&mut self, _ctx: &OptionValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#optionValue}.
 * @param ctx the parse tree
 */
fn exit_optionValue(&mut self, _ctx: &OptionValueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#funcOptions}.
 * @param ctx the parse tree
 */
fn enter_funcOptions(&mut self, _ctx: &FuncOptionsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#funcOptions}.
 * @param ctx the parse tree
 */
fn exit_funcOptions(&mut self, _ctx: &FuncOptionsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#nonReserved}.
 * @param ctx the parse tree
 */
fn enter_nonReserved(&mut self, _ctx: &NonReservedContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#nonReserved}.
 * @param ctx the parse tree
 */
fn exit_nonReserved(&mut self, _ctx: &NonReservedContext<'input>) { }
/**
 * Enter a parse tree produced by {@link FuncTestCaseParser#identifier}.
 * @param ctx the parse tree
 */
fn enter_identifier(&mut self, _ctx: &IdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link FuncTestCaseParser#identifier}.
 * @param ctx the parse tree
 */
fn exit_identifier(&mut self, _ctx: &IdentifierContext<'input>) { }

}

antlr4rust::coerce_from!{ 'input : FuncTestCaseParserListener<'input> }


