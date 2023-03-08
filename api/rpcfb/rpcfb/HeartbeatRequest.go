// Code generated by the FlatBuffers compiler. DO NOT EDIT.

package rpcfb

import (
	flatbuffers "github.com/google/flatbuffers/go"
)

type HeartbeatRequestT struct {
	ClientId string `json:"client_id"`
	ClientRole ClientRole `json:"client_role"`
	DataNode *DataNodeT `json:"data_node"`
}

func (t *HeartbeatRequestT) Pack(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	if t == nil { return 0 }
	clientIdOffset := flatbuffers.UOffsetT(0)
	if t.ClientId != "" {
		clientIdOffset = builder.CreateString(t.ClientId)
	}
	dataNodeOffset := t.DataNode.Pack(builder)
	HeartbeatRequestStart(builder)
	HeartbeatRequestAddClientId(builder, clientIdOffset)
	HeartbeatRequestAddClientRole(builder, t.ClientRole)
	HeartbeatRequestAddDataNode(builder, dataNodeOffset)
	return HeartbeatRequestEnd(builder)
}

func (rcv *HeartbeatRequest) UnPackTo(t *HeartbeatRequestT) {
	t.ClientId = string(rcv.ClientId())
	t.ClientRole = rcv.ClientRole()
	t.DataNode = rcv.DataNode(nil).UnPack()
}

func (rcv *HeartbeatRequest) UnPack() *HeartbeatRequestT {
	if rcv == nil { return nil }
	t := &HeartbeatRequestT{}
	rcv.UnPackTo(t)
	return t
}

type HeartbeatRequest struct {
	_tab flatbuffers.Table
}

func GetRootAsHeartbeatRequest(buf []byte, offset flatbuffers.UOffsetT) *HeartbeatRequest {
	n := flatbuffers.GetUOffsetT(buf[offset:])
	x := &HeartbeatRequest{}
	x.Init(buf, n+offset)
	return x
}

func GetSizePrefixedRootAsHeartbeatRequest(buf []byte, offset flatbuffers.UOffsetT) *HeartbeatRequest {
	n := flatbuffers.GetUOffsetT(buf[offset+flatbuffers.SizeUint32:])
	x := &HeartbeatRequest{}
	x.Init(buf, n+offset+flatbuffers.SizeUint32)
	return x
}

func (rcv *HeartbeatRequest) Init(buf []byte, i flatbuffers.UOffsetT) {
	rcv._tab.Bytes = buf
	rcv._tab.Pos = i
}

func (rcv *HeartbeatRequest) Table() flatbuffers.Table {
	return rcv._tab
}

func (rcv *HeartbeatRequest) ClientId() []byte {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(4))
	if o != 0 {
		return rcv._tab.ByteVector(o + rcv._tab.Pos)
	}
	return nil
}

func (rcv *HeartbeatRequest) ClientRole() ClientRole {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(6))
	if o != 0 {
		return ClientRole(rcv._tab.GetInt8(o + rcv._tab.Pos))
	}
	return 0
}

func (rcv *HeartbeatRequest) MutateClientRole(n ClientRole) bool {
	return rcv._tab.MutateInt8Slot(6, int8(n))
}

func (rcv *HeartbeatRequest) DataNode(obj *DataNode) *DataNode {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(8))
	if o != 0 {
		x := rcv._tab.Indirect(o + rcv._tab.Pos)
		if obj == nil {
			obj = new(DataNode)
		}
		obj.Init(rcv._tab.Bytes, x)
		return obj
	}
	return nil
}

func HeartbeatRequestStart(builder *flatbuffers.Builder) {
	builder.StartObject(3)
}
func HeartbeatRequestAddClientId(builder *flatbuffers.Builder, clientId flatbuffers.UOffsetT) {
	builder.PrependUOffsetTSlot(0, flatbuffers.UOffsetT(clientId), 0)
}
func HeartbeatRequestAddClientRole(builder *flatbuffers.Builder, clientRole ClientRole) {
	builder.PrependInt8Slot(1, int8(clientRole), 0)
}
func HeartbeatRequestAddDataNode(builder *flatbuffers.Builder, dataNode flatbuffers.UOffsetT) {
	builder.PrependUOffsetTSlot(2, flatbuffers.UOffsetT(dataNode), 0)
}
func HeartbeatRequestEnd(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	return builder.EndObject()
}
