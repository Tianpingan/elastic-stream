// Code generated by the FlatBuffers compiler. DO NOT EDIT.

package rpcfb

import "strconv"

type SealKind int8

const (
	SealKindUNSPECIFIED      SealKind = 0
	SealKindRANGE_SERVER     SealKind = 1
	SealKindPLACEMENT_DRIVER SealKind = 2
)

var EnumNamesSealKind = map[SealKind]string{
	SealKindUNSPECIFIED:      "UNSPECIFIED",
	SealKindRANGE_SERVER:     "RANGE_SERVER",
	SealKindPLACEMENT_DRIVER: "PLACEMENT_DRIVER",
}

var EnumValuesSealKind = map[string]SealKind{
	"UNSPECIFIED":      SealKindUNSPECIFIED,
	"RANGE_SERVER":     SealKindRANGE_SERVER,
	"PLACEMENT_DRIVER": SealKindPLACEMENT_DRIVER,
}

func (v SealKind) String() string {
	if s, ok := EnumNamesSealKind[v]; ok {
		return s
	}
	return "SealKind(" + strconv.FormatInt(int64(v), 10) + ")"
}
