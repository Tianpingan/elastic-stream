// Code generated by the FlatBuffers compiler. DO NOT EDIT.

package rpcfb

import "strconv"

type CompareResult byte

const (
	CompareResultEQUAL     CompareResult = 0
	CompareResultGREATER   CompareResult = 1
	CompareResultLESS      CompareResult = 2
	CompareResultNOT_EQUAL CompareResult = 3
)

var EnumNamesCompareResult = map[CompareResult]string{
	CompareResultEQUAL:     "EQUAL",
	CompareResultGREATER:   "GREATER",
	CompareResultLESS:      "LESS",
	CompareResultNOT_EQUAL: "NOT_EQUAL",
}

var EnumValuesCompareResult = map[string]CompareResult{
	"EQUAL":     CompareResultEQUAL,
	"GREATER":   CompareResultGREATER,
	"LESS":      CompareResultLESS,
	"NOT_EQUAL": CompareResultNOT_EQUAL,
}

func (v CompareResult) String() string {
	if s, ok := EnumNamesCompareResult[v]; ok {
		return s
	}
	return "CompareResult(" + strconv.FormatInt(int64(v), 10) + ")"
}
