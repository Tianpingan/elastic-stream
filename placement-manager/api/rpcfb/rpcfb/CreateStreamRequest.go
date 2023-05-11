// Code generated by the FlatBuffers compiler. DO NOT EDIT.

package rpcfb

import (
	flatbuffers "github.com/google/flatbuffers/go"
)

type CreateStreamRequestT struct {
	TimeoutMs int32 `json:"timeout_ms"`
	Stream *StreamT `json:"stream"`
}

func (t *CreateStreamRequestT) Pack(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	if t == nil { return 0 }
	streamOffset := t.Stream.Pack(builder)
	CreateStreamRequestStart(builder)
	CreateStreamRequestAddTimeoutMs(builder, t.TimeoutMs)
	CreateStreamRequestAddStream(builder, streamOffset)
	return CreateStreamRequestEnd(builder)
}

func (rcv *CreateStreamRequest) UnPackTo(t *CreateStreamRequestT) {
	t.TimeoutMs = rcv.TimeoutMs()
	t.Stream = rcv.Stream(nil).UnPack()
}

func (rcv *CreateStreamRequest) UnPack() *CreateStreamRequestT {
	if rcv == nil { return nil }
	t := &CreateStreamRequestT{}
	rcv.UnPackTo(t)
	return t
}

type CreateStreamRequest struct {
	_tab flatbuffers.Table
}

func GetRootAsCreateStreamRequest(buf []byte, offset flatbuffers.UOffsetT) *CreateStreamRequest {
	n := flatbuffers.GetUOffsetT(buf[offset:])
	x := &CreateStreamRequest{}
	x.Init(buf, n+offset)
	return x
}

func GetSizePrefixedRootAsCreateStreamRequest(buf []byte, offset flatbuffers.UOffsetT) *CreateStreamRequest {
	n := flatbuffers.GetUOffsetT(buf[offset+flatbuffers.SizeUint32:])
	x := &CreateStreamRequest{}
	x.Init(buf, n+offset+flatbuffers.SizeUint32)
	return x
}

func (rcv *CreateStreamRequest) Init(buf []byte, i flatbuffers.UOffsetT) {
	rcv._tab.Bytes = buf
	rcv._tab.Pos = i
}

func (rcv *CreateStreamRequest) Table() flatbuffers.Table {
	return rcv._tab
}

func (rcv *CreateStreamRequest) TimeoutMs() int32 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(4))
	if o != 0 {
		return rcv._tab.GetInt32(o + rcv._tab.Pos)
	}
	return 0
}

func (rcv *CreateStreamRequest) MutateTimeoutMs(n int32) bool {
	return rcv._tab.MutateInt32Slot(4, n)
}

func (rcv *CreateStreamRequest) Stream(obj *Stream) *Stream {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(6))
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

func CreateStreamRequestStart(builder *flatbuffers.Builder) {
	builder.StartObject(2)
}
func CreateStreamRequestAddTimeoutMs(builder *flatbuffers.Builder, timeoutMs int32) {
	builder.PrependInt32Slot(0, timeoutMs, 0)
}
func CreateStreamRequestAddStream(builder *flatbuffers.Builder, stream flatbuffers.UOffsetT) {
	builder.PrependUOffsetTSlot(1, flatbuffers.UOffsetT(stream), 0)
}
func CreateStreamRequestEnd(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	return builder.EndObject()
}
