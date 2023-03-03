// Code generated by the FlatBuffers compiler. DO NOT EDIT.

package header

import (
	flatbuffers "github.com/google/flatbuffers/go"
)

type UpdateStreamsResponse struct {
	_tab flatbuffers.Table
}

func GetRootAsUpdateStreamsResponse(buf []byte, offset flatbuffers.UOffsetT) *UpdateStreamsResponse {
	n := flatbuffers.GetUOffsetT(buf[offset:])
	x := &UpdateStreamsResponse{}
	x.Init(buf, n+offset)
	return x
}

func GetSizePrefixedRootAsUpdateStreamsResponse(buf []byte, offset flatbuffers.UOffsetT) *UpdateStreamsResponse {
	n := flatbuffers.GetUOffsetT(buf[offset+flatbuffers.SizeUint32:])
	x := &UpdateStreamsResponse{}
	x.Init(buf, n+offset+flatbuffers.SizeUint32)
	return x
}

func (rcv *UpdateStreamsResponse) Init(buf []byte, i flatbuffers.UOffsetT) {
	rcv._tab.Bytes = buf
	rcv._tab.Pos = i
}

func (rcv *UpdateStreamsResponse) Table() flatbuffers.Table {
	return rcv._tab
}

func (rcv *UpdateStreamsResponse) ThrottleTimeMs() uint32 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(4))
	if o != 0 {
		return rcv._tab.GetUint32(o + rcv._tab.Pos)
	}
	return 0
}

func (rcv *UpdateStreamsResponse) MutateThrottleTimeMs(n uint32) bool {
	return rcv._tab.MutateUint32Slot(4, n)
}

func (rcv *UpdateStreamsResponse) UpdateResponses(obj *UpdateStreamResult, j int) bool {
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

func (rcv *UpdateStreamsResponse) UpdateResponsesLength() int {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(6))
	if o != 0 {
		return rcv._tab.VectorLen(o)
	}
	return 0
}

func UpdateStreamsResponseStart(builder *flatbuffers.Builder) {
	builder.StartObject(2)
}
func UpdateStreamsResponseAddThrottleTimeMs(builder *flatbuffers.Builder, throttleTimeMs uint32) {
	builder.PrependUint32Slot(0, throttleTimeMs, 0)
}
func UpdateStreamsResponseAddUpdateResponses(builder *flatbuffers.Builder, updateResponses flatbuffers.UOffsetT) {
	builder.PrependUOffsetTSlot(1, flatbuffers.UOffsetT(updateResponses), 0)
}
func UpdateStreamsResponseStartUpdateResponsesVector(builder *flatbuffers.Builder, numElems int) flatbuffers.UOffsetT {
	return builder.StartVector(4, numElems, 4)
}
func UpdateStreamsResponseEnd(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	return builder.EndObject()
}
