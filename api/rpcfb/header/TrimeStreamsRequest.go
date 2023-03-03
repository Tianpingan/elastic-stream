// Code generated by the FlatBuffers compiler. DO NOT EDIT.

package header

import (
	flatbuffers "github.com/google/flatbuffers/go"
)

type TrimeStreamsRequest struct {
	_tab flatbuffers.Table
}

func GetRootAsTrimeStreamsRequest(buf []byte, offset flatbuffers.UOffsetT) *TrimeStreamsRequest {
	n := flatbuffers.GetUOffsetT(buf[offset:])
	x := &TrimeStreamsRequest{}
	x.Init(buf, n+offset)
	return x
}

func GetSizePrefixedRootAsTrimeStreamsRequest(buf []byte, offset flatbuffers.UOffsetT) *TrimeStreamsRequest {
	n := flatbuffers.GetUOffsetT(buf[offset+flatbuffers.SizeUint32:])
	x := &TrimeStreamsRequest{}
	x.Init(buf, n+offset+flatbuffers.SizeUint32)
	return x
}

func (rcv *TrimeStreamsRequest) Init(buf []byte, i flatbuffers.UOffsetT) {
	rcv._tab.Bytes = buf
	rcv._tab.Pos = i
}

func (rcv *TrimeStreamsRequest) Table() flatbuffers.Table {
	return rcv._tab
}

func (rcv *TrimeStreamsRequest) TimeoutMs() uint32 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(4))
	if o != 0 {
		return rcv._tab.GetUint32(o + rcv._tab.Pos)
	}
	return 0
}

func (rcv *TrimeStreamsRequest) MutateTimeoutMs(n uint32) bool {
	return rcv._tab.MutateUint32Slot(4, n)
}

func (rcv *TrimeStreamsRequest) TrimmedStreams(obj *TrimmedStream, j int) bool {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(6))
	if o != 0 {
		x := rcv._tab.Vector(o)
		x += flatbuffers.UOffsetT(j) * 4
		x = rcv._tab.Indirect(x)
		obj.Init(rcv._tab.Bytes, x)
		return true
	}
	return false
}

func (rcv *TrimeStreamsRequest) TrimmedStreamsLength() int {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(6))
	if o != 0 {
		return rcv._tab.VectorLen(o)
	}
	return 0
}

func TrimeStreamsRequestStart(builder *flatbuffers.Builder) {
	builder.StartObject(2)
}
func TrimeStreamsRequestAddTimeoutMs(builder *flatbuffers.Builder, timeoutMs uint32) {
	builder.PrependUint32Slot(0, timeoutMs, 0)
}
func TrimeStreamsRequestAddTrimmedStreams(builder *flatbuffers.Builder, trimmedStreams flatbuffers.UOffsetT) {
	builder.PrependUOffsetTSlot(1, flatbuffers.UOffsetT(trimmedStreams), 0)
}
func TrimeStreamsRequestStartTrimmedStreamsVector(builder *flatbuffers.Builder, numElems int) flatbuffers.UOffsetT {
	return builder.StartVector(4, numElems, 4)
}
func TrimeStreamsRequestEnd(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	return builder.EndObject()
}
