// Code generated by the FlatBuffers compiler. DO NOT EDIT.

package rpcfb

import (
	flatbuffers "github.com/google/flatbuffers/go"
)

type CompareTargetValueT struct {
	Value []byte `json:"value"`
}

func (t *CompareTargetValueT) Pack(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	if t == nil { return 0 }
	valueOffset := flatbuffers.UOffsetT(0)
	if t.Value != nil {
		valueOffset = builder.CreateByteString(t.Value)
	}
	CompareTargetValueStart(builder)
	CompareTargetValueAddValue(builder, valueOffset)
	return CompareTargetValueEnd(builder)
}

func (rcv *CompareTargetValue) UnPackTo(t *CompareTargetValueT) {
	t.Value = rcv.ValueBytes()
}

func (rcv *CompareTargetValue) UnPack() *CompareTargetValueT {
	if rcv == nil { return nil }
	t := &CompareTargetValueT{}
	rcv.UnPackTo(t)
	return t
}

type CompareTargetValue struct {
	_tab flatbuffers.Table
}

func GetRootAsCompareTargetValue(buf []byte, offset flatbuffers.UOffsetT) *CompareTargetValue {
	n := flatbuffers.GetUOffsetT(buf[offset:])
	x := &CompareTargetValue{}
	x.Init(buf, n+offset)
	return x
}

func GetSizePrefixedRootAsCompareTargetValue(buf []byte, offset flatbuffers.UOffsetT) *CompareTargetValue {
	n := flatbuffers.GetUOffsetT(buf[offset+flatbuffers.SizeUint32:])
	x := &CompareTargetValue{}
	x.Init(buf, n+offset+flatbuffers.SizeUint32)
	return x
}

func (rcv *CompareTargetValue) Init(buf []byte, i flatbuffers.UOffsetT) {
	rcv._tab.Bytes = buf
	rcv._tab.Pos = i
}

func (rcv *CompareTargetValue) Table() flatbuffers.Table {
	return rcv._tab
}

func (rcv *CompareTargetValue) Value(j int) byte {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(4))
	if o != 0 {
		a := rcv._tab.Vector(o)
		return rcv._tab.GetByte(a + flatbuffers.UOffsetT(j*1))
	}
	return 0
}

func (rcv *CompareTargetValue) ValueLength() int {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(4))
	if o != 0 {
		return rcv._tab.VectorLen(o)
	}
	return 0
}

func (rcv *CompareTargetValue) ValueBytes() []byte {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(4))
	if o != 0 {
		return rcv._tab.ByteVector(o + rcv._tab.Pos)
	}
	return nil
}

func (rcv *CompareTargetValue) MutateValue(j int, n byte) bool {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(4))
	if o != 0 {
		a := rcv._tab.Vector(o)
		return rcv._tab.MutateByte(a+flatbuffers.UOffsetT(j*1), n)
	}
	return false
}

func CompareTargetValueStart(builder *flatbuffers.Builder) {
	builder.StartObject(1)
}
func CompareTargetValueAddValue(builder *flatbuffers.Builder, value flatbuffers.UOffsetT) {
	builder.PrependUOffsetTSlot(0, flatbuffers.UOffsetT(value), 0)
}
func CompareTargetValueStartValueVector(builder *flatbuffers.Builder, numElems int) flatbuffers.UOffsetT {
	return builder.StartVector(1, numElems, 1)
}
func CompareTargetValueEnd(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	return builder.EndObject()
}