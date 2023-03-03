// Code generated by the FlatBuffers compiler. DO NOT EDIT.

package header

import (
	flatbuffers "github.com/google/flatbuffers/go"
)

type SealRangesRequest struct {
	_tab flatbuffers.Table
}

func GetRootAsSealRangesRequest(buf []byte, offset flatbuffers.UOffsetT) *SealRangesRequest {
	n := flatbuffers.GetUOffsetT(buf[offset:])
	x := &SealRangesRequest{}
	x.Init(buf, n+offset)
	return x
}

func GetSizePrefixedRootAsSealRangesRequest(buf []byte, offset flatbuffers.UOffsetT) *SealRangesRequest {
	n := flatbuffers.GetUOffsetT(buf[offset+flatbuffers.SizeUint32:])
	x := &SealRangesRequest{}
	x.Init(buf, n+offset+flatbuffers.SizeUint32)
	return x
}

func (rcv *SealRangesRequest) Init(buf []byte, i flatbuffers.UOffsetT) {
	rcv._tab.Bytes = buf
	rcv._tab.Pos = i
}

func (rcv *SealRangesRequest) Table() flatbuffers.Table {
	return rcv._tab
}

func (rcv *SealRangesRequest) TimeoutMs() uint32 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(4))
	if o != 0 {
		return rcv._tab.GetUint32(o + rcv._tab.Pos)
	}
	return 0
}

func (rcv *SealRangesRequest) MutateTimeoutMs(n uint32) bool {
	return rcv._tab.MutateUint32Slot(4, n)
}

func (rcv *SealRangesRequest) Ranges(obj *RangeId, j int) bool {
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

func (rcv *SealRangesRequest) RangesLength() int {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(6))
	if o != 0 {
		return rcv._tab.VectorLen(o)
	}
	return 0
}

func SealRangesRequestStart(builder *flatbuffers.Builder) {
	builder.StartObject(2)
}
func SealRangesRequestAddTimeoutMs(builder *flatbuffers.Builder, timeoutMs uint32) {
	builder.PrependUint32Slot(0, timeoutMs, 0)
}
func SealRangesRequestAddRanges(builder *flatbuffers.Builder, ranges flatbuffers.UOffsetT) {
	builder.PrependUOffsetTSlot(1, flatbuffers.UOffsetT(ranges), 0)
}
func SealRangesRequestStartRangesVector(builder *flatbuffers.Builder, numElems int) flatbuffers.UOffsetT {
	return builder.StartVector(4, numElems, 4)
}
func SealRangesRequestEnd(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	return builder.EndObject()
}
