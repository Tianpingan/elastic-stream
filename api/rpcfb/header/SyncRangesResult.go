// Code generated by the FlatBuffers compiler. DO NOT EDIT.

package header

import (
	flatbuffers "github.com/google/flatbuffers/go"
)

type SyncRangesResult struct {
	_tab flatbuffers.Table
}

func GetRootAsSyncRangesResult(buf []byte, offset flatbuffers.UOffsetT) *SyncRangesResult {
	n := flatbuffers.GetUOffsetT(buf[offset:])
	x := &SyncRangesResult{}
	x.Init(buf, n+offset)
	return x
}

func GetSizePrefixedRootAsSyncRangesResult(buf []byte, offset flatbuffers.UOffsetT) *SyncRangesResult {
	n := flatbuffers.GetUOffsetT(buf[offset+flatbuffers.SizeUint32:])
	x := &SyncRangesResult{}
	x.Init(buf, n+offset+flatbuffers.SizeUint32)
	return x
}

func (rcv *SyncRangesResult) Init(buf []byte, i flatbuffers.UOffsetT) {
	rcv._tab.Bytes = buf
	rcv._tab.Pos = i
}

func (rcv *SyncRangesResult) Table() flatbuffers.Table {
	return rcv._tab
}

func (rcv *SyncRangesResult) StreamId() uint64 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(4))
	if o != 0 {
		return rcv._tab.GetUint64(o + rcv._tab.Pos)
	}
	return 0
}

func (rcv *SyncRangesResult) MutateStreamId(n uint64) bool {
	return rcv._tab.MutateUint64Slot(4, n)
}

func (rcv *SyncRangesResult) ErrorCode() ErrorCode {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(6))
	if o != 0 {
		return ErrorCode(rcv._tab.GetUint16(o + rcv._tab.Pos))
	}
	return 0
}

func (rcv *SyncRangesResult) MutateErrorCode(n ErrorCode) bool {
	return rcv._tab.MutateUint16Slot(6, uint16(n))
}

func (rcv *SyncRangesResult) ErrorMessage() []byte {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(8))
	if o != 0 {
		return rcv._tab.ByteVector(o + rcv._tab.Pos)
	}
	return nil
}

func (rcv *SyncRangesResult) Ranges(obj *Range, j int) bool {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(10))
	if o != 0 {
		x := rcv._tab.Vector(o)
		x += flatbuffers.UOffsetT(j) * 4
		x = rcv._tab.Indirect(x)
		obj.Init(rcv._tab.Bytes, x)
		return true
	}
	return false
}

func (rcv *SyncRangesResult) RangesLength() int {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(10))
	if o != 0 {
		return rcv._tab.VectorLen(o)
	}
	return 0
}

func SyncRangesResultStart(builder *flatbuffers.Builder) {
	builder.StartObject(4)
}
func SyncRangesResultAddStreamId(builder *flatbuffers.Builder, streamId uint64) {
	builder.PrependUint64Slot(0, streamId, 0)
}
func SyncRangesResultAddErrorCode(builder *flatbuffers.Builder, errorCode ErrorCode) {
	builder.PrependUint16Slot(1, uint16(errorCode), 0)
}
func SyncRangesResultAddErrorMessage(builder *flatbuffers.Builder, errorMessage flatbuffers.UOffsetT) {
	builder.PrependUOffsetTSlot(2, flatbuffers.UOffsetT(errorMessage), 0)
}
func SyncRangesResultAddRanges(builder *flatbuffers.Builder, ranges flatbuffers.UOffsetT) {
	builder.PrependUOffsetTSlot(3, flatbuffers.UOffsetT(ranges), 0)
}
func SyncRangesResultStartRangesVector(builder *flatbuffers.Builder, numElems int) flatbuffers.UOffsetT {
	return builder.StartVector(4, numElems, 4)
}
func SyncRangesResultEnd(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	return builder.EndObject()
}
