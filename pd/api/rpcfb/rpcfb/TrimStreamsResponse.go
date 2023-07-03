// Code generated by the FlatBuffers compiler. DO NOT EDIT.

package rpcfb

import (
	flatbuffers "github.com/google/flatbuffers/go"
)

type TrimStreamsResponseT struct {
	Status *StatusT `json:"status"`
	ThrottleTimeMs int32 `json:"throttle_time_ms"`
	Stream *StreamT `json:"stream"`
	Range *RangeT `json:"range"`
}

func (t *TrimStreamsResponseT) Pack(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	if t == nil { return 0 }
	statusOffset := t.Status.Pack(builder)
	streamOffset := t.Stream.Pack(builder)
	range_Offset := t.Range.Pack(builder)
	TrimStreamsResponseStart(builder)
	TrimStreamsResponseAddStatus(builder, statusOffset)
	TrimStreamsResponseAddThrottleTimeMs(builder, t.ThrottleTimeMs)
	TrimStreamsResponseAddStream(builder, streamOffset)
	TrimStreamsResponseAddRange(builder, range_Offset)
	return TrimStreamsResponseEnd(builder)
}

func (rcv *TrimStreamsResponse) UnPackTo(t *TrimStreamsResponseT) {
	t.Status = rcv.Status(nil).UnPack()
	t.ThrottleTimeMs = rcv.ThrottleTimeMs()
	t.Stream = rcv.Stream(nil).UnPack()
	t.Range = rcv.Range(nil).UnPack()
}

func (rcv *TrimStreamsResponse) UnPack() *TrimStreamsResponseT {
	if rcv == nil { return nil }
	t := &TrimStreamsResponseT{}
	rcv.UnPackTo(t)
	return t
}

type TrimStreamsResponse struct {
	_tab flatbuffers.Table
}

func GetRootAsTrimStreamsResponse(buf []byte, offset flatbuffers.UOffsetT) *TrimStreamsResponse {
	n := flatbuffers.GetUOffsetT(buf[offset:])
	x := &TrimStreamsResponse{}
	x.Init(buf, n+offset)
	return x
}

func GetSizePrefixedRootAsTrimStreamsResponse(buf []byte, offset flatbuffers.UOffsetT) *TrimStreamsResponse {
	n := flatbuffers.GetUOffsetT(buf[offset+flatbuffers.SizeUint32:])
	x := &TrimStreamsResponse{}
	x.Init(buf, n+offset+flatbuffers.SizeUint32)
	return x
}

func (rcv *TrimStreamsResponse) Init(buf []byte, i flatbuffers.UOffsetT) {
	rcv._tab.Bytes = buf
	rcv._tab.Pos = i
}

func (rcv *TrimStreamsResponse) Table() flatbuffers.Table {
	return rcv._tab
}

func (rcv *TrimStreamsResponse) Status(obj *Status) *Status {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(4))
	if o != 0 {
		x := rcv._tab.Indirect(o + rcv._tab.Pos)
		if obj == nil {
			obj = new(Status)
		}
		obj.Init(rcv._tab.Bytes, x)
		return obj
	}
	return nil
}

func (rcv *TrimStreamsResponse) ThrottleTimeMs() int32 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(6))
	if o != 0 {
		return rcv._tab.GetInt32(o + rcv._tab.Pos)
	}
	return 0
}

func (rcv *TrimStreamsResponse) MutateThrottleTimeMs(n int32) bool {
	return rcv._tab.MutateInt32Slot(6, n)
}

func (rcv *TrimStreamsResponse) Stream(obj *Stream) *Stream {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(8))
	if o != 0 {
		x := rcv._tab.Indirect(o + rcv._tab.Pos)
		if obj == nil {
			obj = new(Stream)
		}
		obj.Init(rcv._tab.Bytes, x)
		return obj
	}
	return nil
}

func (rcv *TrimStreamsResponse) Range(obj *Range) *Range {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(10))
	if o != 0 {
		x := rcv._tab.Indirect(o + rcv._tab.Pos)
		if obj == nil {
			obj = new(Range)
		}
		obj.Init(rcv._tab.Bytes, x)
		return obj
	}
	return nil
}

func TrimStreamsResponseStart(builder *flatbuffers.Builder) {
	builder.StartObject(4)
}
func TrimStreamsResponseAddStatus(builder *flatbuffers.Builder, status flatbuffers.UOffsetT) {
	builder.PrependUOffsetTSlot(0, flatbuffers.UOffsetT(status), 0)
}
func TrimStreamsResponseAddThrottleTimeMs(builder *flatbuffers.Builder, throttleTimeMs int32) {
	builder.PrependInt32Slot(1, throttleTimeMs, 0)
}
func TrimStreamsResponseAddStream(builder *flatbuffers.Builder, stream flatbuffers.UOffsetT) {
	builder.PrependUOffsetTSlot(2, flatbuffers.UOffsetT(stream), 0)
}
func TrimStreamsResponseAddRange(builder *flatbuffers.Builder, range_ flatbuffers.UOffsetT) {
	builder.PrependUOffsetTSlot(3, flatbuffers.UOffsetT(range_), 0)
}
func TrimStreamsResponseEnd(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	return builder.EndObject()
}