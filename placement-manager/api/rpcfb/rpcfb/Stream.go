// Code generated by the FlatBuffers compiler. DO NOT EDIT.

package rpcfb

import (
	flatbuffers "github.com/google/flatbuffers/go"
)

type StreamT struct {
	StreamId int64 `json:"stream_id"`
	Replica int8 `json:"replica"`
	RetentionPeriodMs int64 `json:"retention_period_ms"`
}

func (t *StreamT) Pack(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	if t == nil { return 0 }
	StreamStart(builder)
	StreamAddStreamId(builder, t.StreamId)
	StreamAddReplica(builder, t.Replica)
	StreamAddRetentionPeriodMs(builder, t.RetentionPeriodMs)
	return StreamEnd(builder)
}

func (rcv *Stream) UnPackTo(t *StreamT) {
	t.StreamId = rcv.StreamId()
	t.Replica = rcv.Replica()
	t.RetentionPeriodMs = rcv.RetentionPeriodMs()
}

func (rcv *Stream) UnPack() *StreamT {
	if rcv == nil { return nil }
	t := &StreamT{}
	rcv.UnPackTo(t)
	return t
}

type Stream struct {
	_tab flatbuffers.Table
}

func GetRootAsStream(buf []byte, offset flatbuffers.UOffsetT) *Stream {
	n := flatbuffers.GetUOffsetT(buf[offset:])
	x := &Stream{}
	x.Init(buf, n+offset)
	return x
}

func GetSizePrefixedRootAsStream(buf []byte, offset flatbuffers.UOffsetT) *Stream {
	n := flatbuffers.GetUOffsetT(buf[offset+flatbuffers.SizeUint32:])
	x := &Stream{}
	x.Init(buf, n+offset+flatbuffers.SizeUint32)
	return x
}

func (rcv *Stream) Init(buf []byte, i flatbuffers.UOffsetT) {
	rcv._tab.Bytes = buf
	rcv._tab.Pos = i
}

func (rcv *Stream) Table() flatbuffers.Table {
	return rcv._tab
}

func (rcv *Stream) StreamId() int64 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(4))
	if o != 0 {
		return rcv._tab.GetInt64(o + rcv._tab.Pos)
	}
	return -1
}

func (rcv *Stream) MutateStreamId(n int64) bool {
	return rcv._tab.MutateInt64Slot(4, n)
}

func (rcv *Stream) Replica() int8 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(6))
	if o != 0 {
		return rcv._tab.GetInt8(o + rcv._tab.Pos)
	}
	return 0
}

func (rcv *Stream) MutateReplica(n int8) bool {
	return rcv._tab.MutateInt8Slot(6, n)
}

func (rcv *Stream) RetentionPeriodMs() int64 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(8))
	if o != 0 {
		return rcv._tab.GetInt64(o + rcv._tab.Pos)
	}
	return 0
}

func (rcv *Stream) MutateRetentionPeriodMs(n int64) bool {
	return rcv._tab.MutateInt64Slot(8, n)
}

func StreamStart(builder *flatbuffers.Builder) {
	builder.StartObject(3)
}
func StreamAddStreamId(builder *flatbuffers.Builder, streamId int64) {
	builder.PrependInt64Slot(0, streamId, -1)
}
func StreamAddReplica(builder *flatbuffers.Builder, replica int8) {
	builder.PrependInt8Slot(1, replica, 0)
}
func StreamAddRetentionPeriodMs(builder *flatbuffers.Builder, retentionPeriodMs int64) {
	builder.PrependInt64Slot(2, retentionPeriodMs, 0)
}
func StreamEnd(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	return builder.EndObject()
}
