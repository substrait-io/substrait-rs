// SPDX-License-Identifier: Apache-2.0
#![allow(clippy::all)]
#![allow(unused_parens)]
#![cfg_attr(rustfmt, rustfmt_skip)]
// Generated from FuncTestCaseParser.g4 by ANTLR 4.13.2

use super::functestcaseparser::*;
use antlr4rust::tree::ParseTreeListener;

// A complete Visitor for a parse tree produced by FuncTestCaseParser.

pub trait FuncTestCaseParserBaseListener<'input>:
    ParseTreeListener<'input, FuncTestCaseParserContextType> {

    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_doc(&mut self, _ctx: &DocContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_doc(&mut self, _ctx: &DocContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_header(&mut self, _ctx: &HeaderContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_header(&mut self, _ctx: &HeaderContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_version(&mut self, _ctx: &VersionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_version(&mut self, _ctx: &VersionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_include(&mut self, _ctx: &IncludeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_include(&mut self, _ctx: &IncludeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_dependency(&mut self, _ctx: &DependencyContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_dependency(&mut self, _ctx: &DependencyContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_testgroupdescription(&mut self, _ctx: &TestGroupDescriptionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_testgroupdescription(&mut self, _ctx: &TestGroupDescriptionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_testcase(&mut self, _ctx: &TestCaseContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_testcase(&mut self, _ctx: &TestCaseContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_scalarfunctestgroup(&mut self, _ctx: &ScalarFuncTestGroupContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_scalarfunctestgroup(&mut self, _ctx: &ScalarFuncTestGroupContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_aggregatefunctestgroup(&mut self, _ctx: &AggregateFuncTestGroupContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_aggregatefunctestgroup(&mut self, _ctx: &AggregateFuncTestGroupContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_arguments(&mut self, _ctx: &ArgumentsContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_arguments(&mut self, _ctx: &ArgumentsContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_result(&mut self, _ctx: &ResultContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_result(&mut self, _ctx: &ResultContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_argument(&mut self, _ctx: &ArgumentContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_argument(&mut self, _ctx: &ArgumentContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_aggfunctestcase(&mut self, _ctx: &AggFuncTestCaseContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_aggfunctestcase(&mut self, _ctx: &AggFuncTestCaseContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_multiargaggregatefunccall(&mut self, _ctx: &MultiArgAggregateFuncCallContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_multiargaggregatefunccall(&mut self, _ctx: &MultiArgAggregateFuncCallContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_compactaggregatefunccall(&mut self, _ctx: &CompactAggregateFuncCallContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_compactaggregatefunccall(&mut self, _ctx: &CompactAggregateFuncCallContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_singleargaggregatefunccall(&mut self, _ctx: &SingleArgAggregateFuncCallContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_singleargaggregatefunccall(&mut self, _ctx: &SingleArgAggregateFuncCallContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tabledata(&mut self, _ctx: &TableDataContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tabledata(&mut self, _ctx: &TableDataContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tablerows(&mut self, _ctx: &TableRowsContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tablerows(&mut self, _ctx: &TableRowsContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_datacolumn(&mut self, _ctx: &DataColumnContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_datacolumn(&mut self, _ctx: &DataColumnContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_columnvalues(&mut self, _ctx: &ColumnValuesContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_columnvalues(&mut self, _ctx: &ColumnValuesContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_literal(&mut self, _ctx: &LiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_literal(&mut self, _ctx: &LiteralContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_qualifiedaggregatefuncargs(&mut self, _ctx: &QualifiedAggregateFuncArgsContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_qualifiedaggregatefuncargs(&mut self, _ctx: &QualifiedAggregateFuncArgsContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_aggregatefuncargs(&mut self, _ctx: &AggregateFuncArgsContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_aggregatefuncargs(&mut self, _ctx: &AggregateFuncArgsContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_qualifiedaggregatefuncarg(&mut self, _ctx: &QualifiedAggregateFuncArgContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_qualifiedaggregatefuncarg(&mut self, _ctx: &QualifiedAggregateFuncArgContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_aggregatefuncarg(&mut self, _ctx: &AggregateFuncArgContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_aggregatefuncarg(&mut self, _ctx: &AggregateFuncArgContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_numericliteral(&mut self, _ctx: &NumericLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_numericliteral(&mut self, _ctx: &NumericLiteralContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_floatliteral(&mut self, _ctx: &FloatLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_floatliteral(&mut self, _ctx: &FloatLiteralContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_nullarg(&mut self, _ctx: &NullArgContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_nullarg(&mut self, _ctx: &NullArgContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_intarg(&mut self, _ctx: &IntArgContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_intarg(&mut self, _ctx: &IntArgContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_floatarg(&mut self, _ctx: &FloatArgContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_floatarg(&mut self, _ctx: &FloatArgContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_decimalarg(&mut self, _ctx: &DecimalArgContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_decimalarg(&mut self, _ctx: &DecimalArgContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_booleanarg(&mut self, _ctx: &BooleanArgContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_booleanarg(&mut self, _ctx: &BooleanArgContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_stringarg(&mut self, _ctx: &StringArgContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_stringarg(&mut self, _ctx: &StringArgContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_datearg(&mut self, _ctx: &DateArgContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_datearg(&mut self, _ctx: &DateArgContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_intervalyeararg(&mut self, _ctx: &IntervalYearArgContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_intervalyeararg(&mut self, _ctx: &IntervalYearArgContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_intervaldayarg(&mut self, _ctx: &IntervalDayArgContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_intervaldayarg(&mut self, _ctx: &IntervalDayArgContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_intervalcompoundarg(&mut self, _ctx: &IntervalCompoundArgContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_intervalcompoundarg(&mut self, _ctx: &IntervalCompoundArgContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fixedchararg(&mut self, _ctx: &FixedCharArgContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fixedchararg(&mut self, _ctx: &FixedCharArgContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_varchararg(&mut self, _ctx: &VarCharArgContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_varchararg(&mut self, _ctx: &VarCharArgContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fixedbinaryarg(&mut self, _ctx: &FixedBinaryArgContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fixedbinaryarg(&mut self, _ctx: &FixedBinaryArgContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_precisiontimearg(&mut self, _ctx: &PrecisionTimeArgContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_precisiontimearg(&mut self, _ctx: &PrecisionTimeArgContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_precisiontimestamparg(&mut self, _ctx: &PrecisionTimestampArgContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_precisiontimestamparg(&mut self, _ctx: &PrecisionTimestampArgContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_precisiontimestamptzarg(&mut self, _ctx: &PrecisionTimestampTZArgContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_precisiontimestamptzarg(&mut self, _ctx: &PrecisionTimestampTZArgContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_listarg(&mut self, _ctx: &ListArgContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_listarg(&mut self, _ctx: &ListArgContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_structarg(&mut self, _ctx: &StructArgContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_structarg(&mut self, _ctx: &StructArgContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_maparg(&mut self, _ctx: &MapArgContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_maparg(&mut self, _ctx: &MapArgContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_userdefinedarg(&mut self, _ctx: &UserDefinedArgContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_userdefinedarg(&mut self, _ctx: &UserDefinedArgContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_lambdaarg(&mut self, _ctx: &LambdaArgContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_lambdaarg(&mut self, _ctx: &LambdaArgContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_funccallarg(&mut self, _ctx: &FuncCallArgContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_funccallarg(&mut self, _ctx: &FuncCallArgContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_enumarg(&mut self, _ctx: &EnumArgContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_enumarg(&mut self, _ctx: &EnumArgContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_literallist(&mut self, _ctx: &LiteralListContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_literallist(&mut self, _ctx: &LiteralListContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_literalstruct(&mut self, _ctx: &LiteralStructContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_literalstruct(&mut self, _ctx: &LiteralStructContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_literalmap(&mut self, _ctx: &LiteralMapContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_literalmap(&mut self, _ctx: &LiteralMapContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_mapentry(&mut self, _ctx: &MapEntryContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_mapentry(&mut self, _ctx: &MapEntryContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_compoundliteral(&mut self, _ctx: &CompoundLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_compoundliteral(&mut self, _ctx: &CompoundLiteralContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_literallambda(&mut self, _ctx: &LiteralLambdaContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_literallambda(&mut self, _ctx: &LiteralLambdaContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_singleparam(&mut self, _ctx: &SingleParamContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_singleparam(&mut self, _ctx: &SingleParamContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_tupleparams(&mut self, _ctx: &TupleParamsContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_tupleparams(&mut self, _ctx: &TupleParamsContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_lambdabody(&mut self, _ctx: &LambdaBodyContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_lambdabody(&mut self, _ctx: &LambdaBodyContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_datatype(&mut self, _ctx: &DataTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_datatype(&mut self, _ctx: &DataTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_boolean(&mut self, _ctx: &BooleanContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_boolean(&mut self, _ctx: &BooleanContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_int(&mut self, _ctx: &IntContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_int(&mut self, _ctx: &IntContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_float(&mut self, _ctx: &FloatContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_float(&mut self, _ctx: &FloatContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_string(&mut self, _ctx: &StringContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_string(&mut self, _ctx: &StringContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_binary(&mut self, _ctx: &BinaryContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_binary(&mut self, _ctx: &BinaryContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_date(&mut self, _ctx: &DateContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_date(&mut self, _ctx: &DateContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_intervalyear(&mut self, _ctx: &IntervalYearContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_intervalyear(&mut self, _ctx: &IntervalYearContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_uuid(&mut self, _ctx: &UuidContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_uuid(&mut self, _ctx: &UuidContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_userdefined(&mut self, _ctx: &UserDefinedContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_userdefined(&mut self, _ctx: &UserDefinedContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_userdefinedtype(&mut self, _ctx: &UserDefinedTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_userdefinedtype(&mut self, _ctx: &UserDefinedTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_booleantype(&mut self, _ctx: &BooleanTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_booleantype(&mut self, _ctx: &BooleanTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_stringtype(&mut self, _ctx: &StringTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_stringtype(&mut self, _ctx: &StringTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_binarytype(&mut self, _ctx: &BinaryTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_binarytype(&mut self, _ctx: &BinaryTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_inttype(&mut self, _ctx: &IntTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_inttype(&mut self, _ctx: &IntTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_floattype(&mut self, _ctx: &FloatTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_floattype(&mut self, _ctx: &FloatTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_datetype(&mut self, _ctx: &DateTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_datetype(&mut self, _ctx: &DateTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_intervalyeartype(&mut self, _ctx: &IntervalYearTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_intervalyeartype(&mut self, _ctx: &IntervalYearTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_intervaldaytype(&mut self, _ctx: &IntervalDayTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_intervaldaytype(&mut self, _ctx: &IntervalDayTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_intervalcompoundtype(&mut self, _ctx: &IntervalCompoundTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_intervalcompoundtype(&mut self, _ctx: &IntervalCompoundTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fixedchartype(&mut self, _ctx: &FixedCharTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fixedchartype(&mut self, _ctx: &FixedCharTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_varchartype(&mut self, _ctx: &VarCharTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_varchartype(&mut self, _ctx: &VarCharTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_fixedbinarytype(&mut self, _ctx: &FixedBinaryTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fixedbinarytype(&mut self, _ctx: &FixedBinaryTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_decimaltype(&mut self, _ctx: &DecimalTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_decimaltype(&mut self, _ctx: &DecimalTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_precisiontimetype(&mut self, _ctx: &PrecisionTimeTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_precisiontimetype(&mut self, _ctx: &PrecisionTimeTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_precisiontimestamptype(&mut self, _ctx: &PrecisionTimestampTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_precisiontimestamptype(&mut self, _ctx: &PrecisionTimestampTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_precisiontimestamptztype(&mut self, _ctx: &PrecisionTimestampTZTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_precisiontimestamptztype(&mut self, _ctx: &PrecisionTimestampTZTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_list(&mut self, _ctx: &ListContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_list(&mut self, _ctx: &ListContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_structtype(&mut self, _ctx: &StructTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_structtype(&mut self, _ctx: &StructTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_maptype(&mut self, _ctx: &MapTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_maptype(&mut self, _ctx: &MapTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_functype(&mut self, _ctx: &FuncTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_functype(&mut self, _ctx: &FuncTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_singlefuncparam(&mut self, _ctx: &SingleFuncParamContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_singlefuncparam(&mut self, _ctx: &SingleFuncParamContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_funcparamswithparens(&mut self, _ctx: &FuncParamsWithParensContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_funcparamswithparens(&mut self, _ctx: &FuncParamsWithParensContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_parameterizedtype(&mut self, _ctx: &ParameterizedTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_parameterizedtype(&mut self, _ctx: &ParameterizedTypeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_integerliteral(&mut self, _ctx: &IntegerLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_integerliteral(&mut self, _ctx: &IntegerLiteralContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_substraiterror(&mut self, _ctx: &SubstraitErrorContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_substraiterror(&mut self, _ctx: &SubstraitErrorContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_funcoption(&mut self, _ctx: &FuncOptionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_funcoption(&mut self, _ctx: &FuncOptionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_optionname(&mut self, _ctx: &OptionNameContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_optionname(&mut self, _ctx: &OptionNameContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_optionvalue(&mut self, _ctx: &OptionValueContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_optionvalue(&mut self, _ctx: &OptionValueContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_funcoptions(&mut self, _ctx: &FuncOptionsContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_funcoptions(&mut self, _ctx: &FuncOptionsContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_nonreserved(&mut self, _ctx: &NonReservedContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_nonreserved(&mut self, _ctx: &NonReservedContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_identifier(&mut self, _ctx: &IdentifierContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  FuncTestCaseParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_identifier(&mut self, _ctx: &IdentifierContext<'input>) {}


}