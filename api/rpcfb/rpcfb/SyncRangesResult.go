// Code generated by the FlatBuffers compiler. DO NOT EDIT.

package rpcfb

import (
	flatbuffers "github.com/google/flatbuffers/go"
)

type SyncRangesResultT struct {
	StreamId int64 `json:"stream_id"`
	ErrorCode ErrorCode `json:"error_code"`
	ErrorMessage string `json:"error_message"`
	Ranges []*RangeT `json:"ranges"`
}

func (t *SyncRangesResultT) Pack(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	if t == nil { return 0 }
	errorMessageOffset := flatbuffers.UOffsetT(0)
	if t.ErrorMessage != "" {
		errorMessageOffset = builder.CreateString(t.ErrorMessage)
	}
	rangesOffset := flatbuffers.UOffsetT(0)
	if t.Ranges != nil {
		rangesLength := len(t.Ranges)
		rangesOffsets := make([]flatbuffers.UOffsetT, rangesLength)
		for j := 0; j < rangesLength; j++ {
			rangesOffsets[j] = t.Ranges[j].Pack(builder)
		}
		SyncRangesResultStartRangesVector(builder, rangesLength)
		for j := rangesLength - 1; j >= 0; j-- {
			builder.PrependUOffsetT(rangesOffsets[j])
		}
		rangesOffset = builder.EndVector(rangesLength)
	}
	SyncRangesResultStart(builder)
	SyncRangesResultAddStreamId(builder, t.StreamId)
	SyncRangesResultAddErrorCode(builder, t.ErrorCode)
	SyncRangesResultAddErrorMessage(builder, errorMessageOffset)
	SyncRangesResultAddRanges(builder, rangesOffset)
	return SyncRangesResultEnd(builder)
}

func (rcv *SyncRangesResult) UnPackTo(t *SyncRangesResultT) {
	t.StreamId = rcv.StreamId()
	t.ErrorCode = rcv.ErrorCode()
	t.ErrorMessage = string(rcv.ErrorMessage())
	rangesLength := rcv.RangesLength()
	t.Ranges = make([]*RangeT, rangesLength)
	for j := 0; j < rangesLength; j++ {
		x := Range{}
		rcv.Ranges(&x, j)
		t.Ranges[j] = x.UnPack()
	}
}

func (rcv *SyncRangesResult) UnPack() *SyncRangesResultT {
	if rcv == nil { return nil }
	t := &SyncRangesResultT{}
	rcv.UnPackTo(t)
	return t
}

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

func (rcv *SyncRangesResult) StreamId() int64 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(4))
	if o != 0 {
		return rcv._tab.GetInt64(o + rcv._tab.Pos)
	}
	return 0
}

func (rcv *SyncRangesResult) MutateStreamId(n int64) bool {
	return rcv._tab.MutateInt64Slot(4, n)
}

func (rcv *SyncRangesResult) ErrorCode() ErrorCode {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(6))
	if o != 0 {
		return ErrorCode(rcv._tab.GetInt16(o + rcv._tab.Pos))
	}
	return 0
}

func (rcv *SyncRangesResult) MutateErrorCode(n ErrorCode) bool {
	return rcv._tab.MutateInt16Slot(6, int16(n))
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
func SyncRangesResultAddStreamId(builder *flatbuffers.Builder, streamId int64) {
	builder.PrependInt64Slot(0, streamId, 0)
}
func SyncRangesResultAddErrorCode(builder *flatbuffers.Builder, errorCode ErrorCode) {
	builder.PrependInt16Slot(1, int16(errorCode), 0)
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
