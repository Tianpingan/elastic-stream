// Code generated by the FlatBuffers compiler. DO NOT EDIT.

package rpcfb

import (
	flatbuffers "github.com/google/flatbuffers/go"
)

type SystemErrorResponseT struct {
	ErrorCode ErrorCode `json:"error_code"`
	ErrorMessage string `json:"error_message"`
}

func (t *SystemErrorResponseT) Pack(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	if t == nil { return 0 }
	errorMessageOffset := flatbuffers.UOffsetT(0)
	if t.ErrorMessage != "" {
		errorMessageOffset = builder.CreateString(t.ErrorMessage)
	}
	SystemErrorResponseStart(builder)
	SystemErrorResponseAddErrorCode(builder, t.ErrorCode)
	SystemErrorResponseAddErrorMessage(builder, errorMessageOffset)
	return SystemErrorResponseEnd(builder)
}

func (rcv *SystemErrorResponse) UnPackTo(t *SystemErrorResponseT) {
	t.ErrorCode = rcv.ErrorCode()
	t.ErrorMessage = string(rcv.ErrorMessage())
}

func (rcv *SystemErrorResponse) UnPack() *SystemErrorResponseT {
	if rcv == nil { return nil }
	t := &SystemErrorResponseT{}
	rcv.UnPackTo(t)
	return t
}

type SystemErrorResponse struct {
	_tab flatbuffers.Table
}

func GetRootAsSystemErrorResponse(buf []byte, offset flatbuffers.UOffsetT) *SystemErrorResponse {
	n := flatbuffers.GetUOffsetT(buf[offset:])
	x := &SystemErrorResponse{}
	x.Init(buf, n+offset)
	return x
}

func GetSizePrefixedRootAsSystemErrorResponse(buf []byte, offset flatbuffers.UOffsetT) *SystemErrorResponse {
	n := flatbuffers.GetUOffsetT(buf[offset+flatbuffers.SizeUint32:])
	x := &SystemErrorResponse{}
	x.Init(buf, n+offset+flatbuffers.SizeUint32)
	return x
}

func (rcv *SystemErrorResponse) Init(buf []byte, i flatbuffers.UOffsetT) {
	rcv._tab.Bytes = buf
	rcv._tab.Pos = i
}

func (rcv *SystemErrorResponse) Table() flatbuffers.Table {
	return rcv._tab
}

func (rcv *SystemErrorResponse) ErrorCode() ErrorCode {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(4))
	if o != 0 {
		return ErrorCode(rcv._tab.GetInt16(o + rcv._tab.Pos))
	}
	return 0
}

func (rcv *SystemErrorResponse) MutateErrorCode(n ErrorCode) bool {
	return rcv._tab.MutateInt16Slot(4, int16(n))
}

func (rcv *SystemErrorResponse) ErrorMessage() []byte {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(6))
	if o != 0 {
		return rcv._tab.ByteVector(o + rcv._tab.Pos)
	}
	return nil
}

func SystemErrorResponseStart(builder *flatbuffers.Builder) {
	builder.StartObject(2)
}
func SystemErrorResponseAddErrorCode(builder *flatbuffers.Builder, errorCode ErrorCode) {
	builder.PrependInt16Slot(0, int16(errorCode), 0)
}
func SystemErrorResponseAddErrorMessage(builder *flatbuffers.Builder, errorMessage flatbuffers.UOffsetT) {
	builder.PrependUOffsetTSlot(1, flatbuffers.UOffsetT(errorMessage), 0)
}
func SystemErrorResponseEnd(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	return builder.EndObject()
}
