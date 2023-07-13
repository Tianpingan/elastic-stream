// Code generated by the FlatBuffers compiler. DO NOT EDIT.

package rpcfb

import "strconv"

type EventType int8

const (
	EventTypeUNKNOWN  EventType = 0
	EventTypeADDED    EventType = 1
	EventTypeMODIFIED EventType = 2
	EventTypeDELETED  EventType = 3
)

var EnumNamesEventType = map[EventType]string{
	EventTypeUNKNOWN:  "UNKNOWN",
	EventTypeADDED:    "ADDED",
	EventTypeMODIFIED: "MODIFIED",
	EventTypeDELETED:  "DELETED",
}

var EnumValuesEventType = map[string]EventType{
	"UNKNOWN":  EventTypeUNKNOWN,
	"ADDED":    EventTypeADDED,
	"MODIFIED": EventTypeMODIFIED,
	"DELETED":  EventTypeDELETED,
}

func (v EventType) String() string {
	if s, ok := EnumNamesEventType[v]; ok {
		return s
	}
	return "EventType(" + strconv.FormatInt(int64(v), 10) + ")"
}