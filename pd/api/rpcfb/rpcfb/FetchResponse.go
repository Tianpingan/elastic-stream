// Code generated by the FlatBuffers compiler. DO NOT EDIT.

package rpcfb

import (
	flatbuffers "github.com/google/flatbuffers/go"
)

type FetchResponseT struct {
	Status *StatusT `json:"status"`
	ThrottleTimeMs int32 `json:"throttle_time_ms"`
	ObjectMetadataList []*ObjectMetadataT `json:"object_metadata_list"`
}

func (t *FetchResponseT) Pack(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	if t == nil { return 0 }
	statusOffset := t.Status.Pack(builder)
	objectMetadataListOffset := flatbuffers.UOffsetT(0)
	if t.ObjectMetadataList != nil {
		objectMetadataListLength := len(t.ObjectMetadataList)
		objectMetadataListOffsets := make([]flatbuffers.UOffsetT, objectMetadataListLength)
		for j := 0; j < objectMetadataListLength; j++ {
			objectMetadataListOffsets[j] = t.ObjectMetadataList[j].Pack(builder)
		}
		FetchResponseStartObjectMetadataListVector(builder, objectMetadataListLength)
		for j := objectMetadataListLength - 1; j >= 0; j-- {
			builder.PrependUOffsetT(objectMetadataListOffsets[j])
		}
		objectMetadataListOffset = builder.EndVector(objectMetadataListLength)
	}
	FetchResponseStart(builder)
	FetchResponseAddStatus(builder, statusOffset)
	FetchResponseAddThrottleTimeMs(builder, t.ThrottleTimeMs)
	FetchResponseAddObjectMetadataList(builder, objectMetadataListOffset)
	return FetchResponseEnd(builder)
}

func (rcv *FetchResponse) UnPackTo(t *FetchResponseT) {
	t.Status = rcv.Status(nil).UnPack()
	t.ThrottleTimeMs = rcv.ThrottleTimeMs()
	objectMetadataListLength := rcv.ObjectMetadataListLength()
	t.ObjectMetadataList = make([]*ObjectMetadataT, objectMetadataListLength)
	for j := 0; j < objectMetadataListLength; j++ {
		x := ObjectMetadata{}
		rcv.ObjectMetadataList(&x, j)
		t.ObjectMetadataList[j] = x.UnPack()
	}
}

func (rcv *FetchResponse) UnPack() *FetchResponseT {
	if rcv == nil { return nil }
	t := &FetchResponseT{}
	rcv.UnPackTo(t)
	return t
}

type FetchResponse struct {
	_tab flatbuffers.Table
}

func GetRootAsFetchResponse(buf []byte, offset flatbuffers.UOffsetT) *FetchResponse {
	n := flatbuffers.GetUOffsetT(buf[offset:])
	x := &FetchResponse{}
	x.Init(buf, n+offset)
	return x
}

func GetSizePrefixedRootAsFetchResponse(buf []byte, offset flatbuffers.UOffsetT) *FetchResponse {
	n := flatbuffers.GetUOffsetT(buf[offset+flatbuffers.SizeUint32:])
	x := &FetchResponse{}
	x.Init(buf, n+offset+flatbuffers.SizeUint32)
	return x
}

func (rcv *FetchResponse) Init(buf []byte, i flatbuffers.UOffsetT) {
	rcv._tab.Bytes = buf
	rcv._tab.Pos = i
}

func (rcv *FetchResponse) Table() flatbuffers.Table {
	return rcv._tab
}

func (rcv *FetchResponse) Status(obj *Status) *Status {
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

func (rcv *FetchResponse) ThrottleTimeMs() int32 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(6))
	if o != 0 {
		return rcv._tab.GetInt32(o + rcv._tab.Pos)
	}
	return 0
}

func (rcv *FetchResponse) MutateThrottleTimeMs(n int32) bool {
	return rcv._tab.MutateInt32Slot(6, n)
}

func (rcv *FetchResponse) ObjectMetadataList(obj *ObjectMetadata, j int) bool {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(8))
	if o != 0 {
		x := rcv._tab.Vector(o)
		x += flatbuffers.UOffsetT(j) * 4
		x = rcv._tab.Indirect(x)
		obj.Init(rcv._tab.Bytes, x)
		return true
	}
	return false
}

func (rcv *FetchResponse) ObjectMetadataListLength() int {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(8))
	if o != 0 {
		return rcv._tab.VectorLen(o)
	}
	return 0
}

func FetchResponseStart(builder *flatbuffers.Builder) {
	builder.StartObject(3)
}
func FetchResponseAddStatus(builder *flatbuffers.Builder, status flatbuffers.UOffsetT) {
	builder.PrependUOffsetTSlot(0, flatbuffers.UOffsetT(status), 0)
}
func FetchResponseAddThrottleTimeMs(builder *flatbuffers.Builder, throttleTimeMs int32) {
	builder.PrependInt32Slot(1, throttleTimeMs, 0)
}
func FetchResponseAddObjectMetadataList(builder *flatbuffers.Builder, objectMetadataList flatbuffers.UOffsetT) {
	builder.PrependUOffsetTSlot(2, flatbuffers.UOffsetT(objectMetadataList), 0)
}
func FetchResponseStartObjectMetadataListVector(builder *flatbuffers.Builder, numElems int) flatbuffers.UOffsetT {
	return builder.StartVector(4, numElems, 4)
}
func FetchResponseEnd(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	return builder.EndObject()
}
