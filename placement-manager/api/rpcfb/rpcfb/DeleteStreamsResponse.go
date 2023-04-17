// Code generated by the FlatBuffers compiler. DO NOT EDIT.

package rpcfb

import (
	flatbuffers "github.com/google/flatbuffers/go"
)

type DeleteStreamsResponseT struct {
	ThrottleTimeMs int32 `json:"throttle_time_ms"`
	DeleteResponses []*DeleteStreamResultT `json:"delete_responses"`
	Status *StatusT `json:"status"`
}

func (t *DeleteStreamsResponseT) Pack(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	if t == nil { return 0 }
	deleteResponsesOffset := flatbuffers.UOffsetT(0)
	if t.DeleteResponses != nil {
		deleteResponsesLength := len(t.DeleteResponses)
		deleteResponsesOffsets := make([]flatbuffers.UOffsetT, deleteResponsesLength)
		for j := 0; j < deleteResponsesLength; j++ {
			deleteResponsesOffsets[j] = t.DeleteResponses[j].Pack(builder)
		}
		DeleteStreamsResponseStartDeleteResponsesVector(builder, deleteResponsesLength)
		for j := deleteResponsesLength - 1; j >= 0; j-- {
			builder.PrependUOffsetT(deleteResponsesOffsets[j])
		}
		deleteResponsesOffset = builder.EndVector(deleteResponsesLength)
	}
	statusOffset := t.Status.Pack(builder)
	DeleteStreamsResponseStart(builder)
	DeleteStreamsResponseAddThrottleTimeMs(builder, t.ThrottleTimeMs)
	DeleteStreamsResponseAddDeleteResponses(builder, deleteResponsesOffset)
	DeleteStreamsResponseAddStatus(builder, statusOffset)
	return DeleteStreamsResponseEnd(builder)
}

func (rcv *DeleteStreamsResponse) UnPackTo(t *DeleteStreamsResponseT) {
	t.ThrottleTimeMs = rcv.ThrottleTimeMs()
	deleteResponsesLength := rcv.DeleteResponsesLength()
	t.DeleteResponses = make([]*DeleteStreamResultT, deleteResponsesLength)
	for j := 0; j < deleteResponsesLength; j++ {
		x := DeleteStreamResult{}
		rcv.DeleteResponses(&x, j)
		t.DeleteResponses[j] = x.UnPack()
	}
	t.Status = rcv.Status(nil).UnPack()
}

func (rcv *DeleteStreamsResponse) UnPack() *DeleteStreamsResponseT {
	if rcv == nil { return nil }
	t := &DeleteStreamsResponseT{}
	rcv.UnPackTo(t)
	return t
}

type DeleteStreamsResponse struct {
	_tab flatbuffers.Table
}

func GetRootAsDeleteStreamsResponse(buf []byte, offset flatbuffers.UOffsetT) *DeleteStreamsResponse {
	n := flatbuffers.GetUOffsetT(buf[offset:])
	x := &DeleteStreamsResponse{}
	x.Init(buf, n+offset)
	return x
}

func GetSizePrefixedRootAsDeleteStreamsResponse(buf []byte, offset flatbuffers.UOffsetT) *DeleteStreamsResponse {
	n := flatbuffers.GetUOffsetT(buf[offset+flatbuffers.SizeUint32:])
	x := &DeleteStreamsResponse{}
	x.Init(buf, n+offset+flatbuffers.SizeUint32)
	return x
}

func (rcv *DeleteStreamsResponse) Init(buf []byte, i flatbuffers.UOffsetT) {
	rcv._tab.Bytes = buf
	rcv._tab.Pos = i
}

func (rcv *DeleteStreamsResponse) Table() flatbuffers.Table {
	return rcv._tab
}

func (rcv *DeleteStreamsResponse) ThrottleTimeMs() int32 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(4))
	if o != 0 {
		return rcv._tab.GetInt32(o + rcv._tab.Pos)
	}
	return 0
}

func (rcv *DeleteStreamsResponse) MutateThrottleTimeMs(n int32) bool {
	return rcv._tab.MutateInt32Slot(4, n)
}

func (rcv *DeleteStreamsResponse) DeleteResponses(obj *DeleteStreamResult, j int) bool {
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

func (rcv *DeleteStreamsResponse) DeleteResponsesLength() int {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(6))
	if o != 0 {
		return rcv._tab.VectorLen(o)
	}
	return 0
}

func (rcv *DeleteStreamsResponse) Status(obj *Status) *Status {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(8))
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

func DeleteStreamsResponseStart(builder *flatbuffers.Builder) {
	builder.StartObject(3)
}
func DeleteStreamsResponseAddThrottleTimeMs(builder *flatbuffers.Builder, throttleTimeMs int32) {
	builder.PrependInt32Slot(0, throttleTimeMs, 0)
}
func DeleteStreamsResponseAddDeleteResponses(builder *flatbuffers.Builder, deleteResponses flatbuffers.UOffsetT) {
	builder.PrependUOffsetTSlot(1, flatbuffers.UOffsetT(deleteResponses), 0)
}
func DeleteStreamsResponseStartDeleteResponsesVector(builder *flatbuffers.Builder, numElems int) flatbuffers.UOffsetT {
	return builder.StartVector(4, numElems, 4)
}
func DeleteStreamsResponseAddStatus(builder *flatbuffers.Builder, status flatbuffers.UOffsetT) {
	builder.PrependUOffsetTSlot(2, flatbuffers.UOffsetT(status), 0)
}
func DeleteStreamsResponseEnd(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	return builder.EndObject()
}