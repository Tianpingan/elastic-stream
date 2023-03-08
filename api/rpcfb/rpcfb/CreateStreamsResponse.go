// Code generated by the FlatBuffers compiler. DO NOT EDIT.

package rpcfb

import (
	flatbuffers "github.com/google/flatbuffers/go"
)

type CreateStreamsResponseT struct {
	ThrottleTimeMs int32 `json:"throttle_time_ms"`
	CreateResponses []*CreateStreamResultT `json:"create_responses"`
	ErrorCode ErrorCode `json:"error_code"`
	ErrorMessage string `json:"error_message"`
}

func (t *CreateStreamsResponseT) Pack(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	if t == nil { return 0 }
	createResponsesOffset := flatbuffers.UOffsetT(0)
	if t.CreateResponses != nil {
		createResponsesLength := len(t.CreateResponses)
		createResponsesOffsets := make([]flatbuffers.UOffsetT, createResponsesLength)
		for j := 0; j < createResponsesLength; j++ {
			createResponsesOffsets[j] = t.CreateResponses[j].Pack(builder)
		}
		CreateStreamsResponseStartCreateResponsesVector(builder, createResponsesLength)
		for j := createResponsesLength - 1; j >= 0; j-- {
			builder.PrependUOffsetT(createResponsesOffsets[j])
		}
		createResponsesOffset = builder.EndVector(createResponsesLength)
	}
	errorMessageOffset := flatbuffers.UOffsetT(0)
	if t.ErrorMessage != "" {
		errorMessageOffset = builder.CreateString(t.ErrorMessage)
	}
	CreateStreamsResponseStart(builder)
	CreateStreamsResponseAddThrottleTimeMs(builder, t.ThrottleTimeMs)
	CreateStreamsResponseAddCreateResponses(builder, createResponsesOffset)
	CreateStreamsResponseAddErrorCode(builder, t.ErrorCode)
	CreateStreamsResponseAddErrorMessage(builder, errorMessageOffset)
	return CreateStreamsResponseEnd(builder)
}

func (rcv *CreateStreamsResponse) UnPackTo(t *CreateStreamsResponseT) {
	t.ThrottleTimeMs = rcv.ThrottleTimeMs()
	createResponsesLength := rcv.CreateResponsesLength()
	t.CreateResponses = make([]*CreateStreamResultT, createResponsesLength)
	for j := 0; j < createResponsesLength; j++ {
		x := CreateStreamResult{}
		rcv.CreateResponses(&x, j)
		t.CreateResponses[j] = x.UnPack()
	}
	t.ErrorCode = rcv.ErrorCode()
	t.ErrorMessage = string(rcv.ErrorMessage())
}

func (rcv *CreateStreamsResponse) UnPack() *CreateStreamsResponseT {
	if rcv == nil { return nil }
	t := &CreateStreamsResponseT{}
	rcv.UnPackTo(t)
	return t
}

type CreateStreamsResponse struct {
	_tab flatbuffers.Table
}

func GetRootAsCreateStreamsResponse(buf []byte, offset flatbuffers.UOffsetT) *CreateStreamsResponse {
	n := flatbuffers.GetUOffsetT(buf[offset:])
	x := &CreateStreamsResponse{}
	x.Init(buf, n+offset)
	return x
}

func GetSizePrefixedRootAsCreateStreamsResponse(buf []byte, offset flatbuffers.UOffsetT) *CreateStreamsResponse {
	n := flatbuffers.GetUOffsetT(buf[offset+flatbuffers.SizeUint32:])
	x := &CreateStreamsResponse{}
	x.Init(buf, n+offset+flatbuffers.SizeUint32)
	return x
}

func (rcv *CreateStreamsResponse) Init(buf []byte, i flatbuffers.UOffsetT) {
	rcv._tab.Bytes = buf
	rcv._tab.Pos = i
}

func (rcv *CreateStreamsResponse) Table() flatbuffers.Table {
	return rcv._tab
}

func (rcv *CreateStreamsResponse) ThrottleTimeMs() int32 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(4))
	if o != 0 {
		return rcv._tab.GetInt32(o + rcv._tab.Pos)
	}
	return 0
}

func (rcv *CreateStreamsResponse) MutateThrottleTimeMs(n int32) bool {
	return rcv._tab.MutateInt32Slot(4, n)
}

func (rcv *CreateStreamsResponse) CreateResponses(obj *CreateStreamResult, j int) bool {
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

func (rcv *CreateStreamsResponse) CreateResponsesLength() int {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(6))
	if o != 0 {
		return rcv._tab.VectorLen(o)
	}
	return 0
}

func (rcv *CreateStreamsResponse) ErrorCode() ErrorCode {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(8))
	if o != 0 {
		return ErrorCode(rcv._tab.GetInt16(o + rcv._tab.Pos))
	}
	return 0
}

func (rcv *CreateStreamsResponse) MutateErrorCode(n ErrorCode) bool {
	return rcv._tab.MutateInt16Slot(8, int16(n))
}

func (rcv *CreateStreamsResponse) ErrorMessage() []byte {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(10))
	if o != 0 {
		return rcv._tab.ByteVector(o + rcv._tab.Pos)
	}
	return nil
}

func CreateStreamsResponseStart(builder *flatbuffers.Builder) {
	builder.StartObject(4)
}
func CreateStreamsResponseAddThrottleTimeMs(builder *flatbuffers.Builder, throttleTimeMs int32) {
	builder.PrependInt32Slot(0, throttleTimeMs, 0)
}
func CreateStreamsResponseAddCreateResponses(builder *flatbuffers.Builder, createResponses flatbuffers.UOffsetT) {
	builder.PrependUOffsetTSlot(1, flatbuffers.UOffsetT(createResponses), 0)
}
func CreateStreamsResponseStartCreateResponsesVector(builder *flatbuffers.Builder, numElems int) flatbuffers.UOffsetT {
	return builder.StartVector(4, numElems, 4)
}
func CreateStreamsResponseAddErrorCode(builder *flatbuffers.Builder, errorCode ErrorCode) {
	builder.PrependInt16Slot(2, int16(errorCode), 0)
}
func CreateStreamsResponseAddErrorMessage(builder *flatbuffers.Builder, errorMessage flatbuffers.UOffsetT) {
	builder.PrependUOffsetTSlot(3, flatbuffers.UOffsetT(errorMessage), 0)
}
func CreateStreamsResponseEnd(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	return builder.EndObject()
}
