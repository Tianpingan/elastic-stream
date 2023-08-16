package handler

import (
	"testing"

	"github.com/stretchr/testify/require"

	"github.com/AutoMQ/pd/api/rpcfb/rpcfb"
	"github.com/AutoMQ/pd/pkg/sbp/protocol"
)

func TestHandler_ListResource(t *testing.T) {
	type args struct {
		types        []rpcfb.ResourceType
		limit        int32
		continuation []byte
	}
	type want struct {
		resources    []rpcfb.ResourceT
		rv           int64
		continuation []byte

		wantErr bool
		errCode rpcfb.ErrorCode
		errMsg  string
	}
	tests := []struct {
		name string
		args args
		want want
	}{
		{
			name: "normal case",
			args: args{
				types: []rpcfb.ResourceType{rpcfb.ResourceTypeRESOURCE_RANGE_SERVER, rpcfb.ResourceTypeRESOURCE_STREAM, rpcfb.ResourceTypeRESOURCE_RANGE, rpcfb.ResourceTypeRESOURCE_OBJECT},
			},
			want: want{
				resources: []rpcfb.ResourceT{
					{Type: rpcfb.ResourceTypeRESOURCE_RANGE_SERVER, RangeServer: &rpcfb.RangeServerT{ServerId: 0, AdvertiseAddr: "addr-0", State: rpcfb.RangeServerStateRANGE_SERVER_STATE_READ_WRITE}},
					{Type: rpcfb.ResourceTypeRESOURCE_RANGE_SERVER, RangeServer: &rpcfb.RangeServerT{ServerId: 1, AdvertiseAddr: "addr-1", State: rpcfb.RangeServerStateRANGE_SERVER_STATE_READ_WRITE}},
					{Type: rpcfb.ResourceTypeRESOURCE_RANGE_SERVER, RangeServer: &rpcfb.RangeServerT{ServerId: 2, AdvertiseAddr: "addr-2", State: rpcfb.RangeServerStateRANGE_SERVER_STATE_READ_WRITE}},
					{Type: rpcfb.ResourceTypeRESOURCE_STREAM, Stream: &rpcfb.StreamT{Replica: 3, AckCount: 3, Epoch: 1}},
					{Type: rpcfb.ResourceTypeRESOURCE_RANGE, Range: &rpcfb.RangeT{Epoch: 1, End: -1}},
					{Type: rpcfb.ResourceTypeRESOURCE_OBJECT, Object: &rpcfb.ObjT{EndOffsetDelta: 1}},
					{Type: rpcfb.ResourceTypeRESOURCE_OBJECT, Object: &rpcfb.ObjT{StartOffset: 1, EndOffsetDelta: 1}},
				},
				rv: 11,
			},
		},
		{
			name: "list by order",
			args: args{
				types: []rpcfb.ResourceType{rpcfb.ResourceTypeRESOURCE_STREAM, rpcfb.ResourceTypeRESOURCE_OBJECT, rpcfb.ResourceTypeRESOURCE_RANGE, rpcfb.ResourceTypeRESOURCE_RANGE_SERVER},
			},
			want: want{
				resources: []rpcfb.ResourceT{
					{Type: rpcfb.ResourceTypeRESOURCE_STREAM, Stream: &rpcfb.StreamT{Replica: 3, AckCount: 3, Epoch: 1}},
					{Type: rpcfb.ResourceTypeRESOURCE_OBJECT, Object: &rpcfb.ObjT{EndOffsetDelta: 1}},
					{Type: rpcfb.ResourceTypeRESOURCE_OBJECT, Object: &rpcfb.ObjT{StartOffset: 1, EndOffsetDelta: 1}},
					{Type: rpcfb.ResourceTypeRESOURCE_RANGE, Range: &rpcfb.RangeT{Epoch: 1, End: -1}},
					{Type: rpcfb.ResourceTypeRESOURCE_RANGE_SERVER, RangeServer: &rpcfb.RangeServerT{ServerId: 0, AdvertiseAddr: "addr-0", State: rpcfb.RangeServerStateRANGE_SERVER_STATE_READ_WRITE}},
					{Type: rpcfb.ResourceTypeRESOURCE_RANGE_SERVER, RangeServer: &rpcfb.RangeServerT{ServerId: 1, AdvertiseAddr: "addr-1", State: rpcfb.RangeServerStateRANGE_SERVER_STATE_READ_WRITE}},
					{Type: rpcfb.ResourceTypeRESOURCE_RANGE_SERVER, RangeServer: &rpcfb.RangeServerT{ServerId: 2, AdvertiseAddr: "addr-2", State: rpcfb.RangeServerStateRANGE_SERVER_STATE_READ_WRITE}},
				},
				rv: 11,
			},
		},
		{
			name: "page 1",
			args: args{
				types: []rpcfb.ResourceType{rpcfb.ResourceTypeRESOURCE_STREAM, rpcfb.ResourceTypeRESOURCE_RANGE, rpcfb.ResourceTypeRESOURCE_OBJECT},
				limit: 3,
			},
			want: want{
				resources: []rpcfb.ResourceT{
					{Type: rpcfb.ResourceTypeRESOURCE_STREAM, Stream: &rpcfb.StreamT{Replica: 3, AckCount: 3, Epoch: 1}},
					{Type: rpcfb.ResourceTypeRESOURCE_RANGE, Range: &rpcfb.RangeT{Epoch: 1, End: -1}},
					{Type: rpcfb.ResourceTypeRESOURCE_OBJECT, Object: &rpcfb.ObjT{EndOffsetDelta: 1}},
				},
				rv:           11,
				continuation: []byte(`{"rv":11,"tokens":[{"rt":2},{"rt":3},{"rt":4,"start":"MDAwMDAwMDAwMDAwMDAwMDAwMDAvMDAwMDAwMDAwMDAvMDAwMDAwMDAwMDAwMDAwMDAwMDAA","more":true}]}`),
			},
		},
		{
			name: "page 2",
			args: args{
				types:        []rpcfb.ResourceType{rpcfb.ResourceTypeRESOURCE_STREAM, rpcfb.ResourceTypeRESOURCE_RANGE, rpcfb.ResourceTypeRESOURCE_OBJECT},
				limit:        3,
				continuation: []byte(`{"rv":11,"tokens":[{"rt":2},{"rt":3},{"rt":4,"start":"MDAwMDAwMDAwMDAwMDAwMDAwMDAvMDAwMDAwMDAwMDAvMDAwMDAwMDAwMDAwMDAwMDAwMDAA","more":true}]}`),
			},
			want: want{
				resources: []rpcfb.ResourceT{
					{Type: rpcfb.ResourceTypeRESOURCE_OBJECT, Object: &rpcfb.ObjT{StartOffset: 1, EndOffsetDelta: 1}},
				},
				rv: 11,
			},
		},
		{
			name: "empty types",
			args: args{
				types: []rpcfb.ResourceType{},
			},
			want: want{
				wantErr: true,
				errCode: rpcfb.ErrorCodeBAD_REQUEST,
				errMsg:  "resource type is empty",
			},
		},
		{
			name: "duplicate types",
			args: args{
				types: []rpcfb.ResourceType{rpcfb.ResourceTypeRESOURCE_RANGE_SERVER, rpcfb.ResourceTypeRESOURCE_RANGE_SERVER},
			},
			want: want{
				wantErr: true,
				errCode: rpcfb.ErrorCodeBAD_REQUEST,
				errMsg:  "duplicate resource type RESOURCE_RANGE_SERVER",
			},
		},
		{
			name: "unknown types",
			args: args{
				types: []rpcfb.ResourceType{rpcfb.ResourceTypeRESOURCE_UNKNOWN, rpcfb.ResourceTypeRESOURCE_RANGE_SERVER},
			},
			want: want{
				wantErr: true,
				errCode: rpcfb.ErrorCodeBAD_REQUEST,
				errMsg:  "invalid type RESOURCE_UNKNOWN",
			},
		},
		{
			name: "invalid continuation",
			args: args{
				types:        []rpcfb.ResourceType{rpcfb.ResourceTypeRESOURCE_RANGE_SERVER},
				continuation: []byte(`{"rv":10,"tokens":[{"rt":1,"more":"true"}]}`),
			},
			want: want{
				wantErr: true,
				errCode: rpcfb.ErrorCodeBAD_REQUEST,
				errMsg:  "unmarshal continuation string",
			},
		},
		{
			name: "mismatched continuation",
			args: args{
				types:        []rpcfb.ResourceType{rpcfb.ResourceTypeRESOURCE_RANGE_SERVER},
				continuation: []byte(`{"rv":10,"tokens":[{"rt":2,"more":true}]}`),
			},
			want: want{
				wantErr: true,
				errCode: rpcfb.ErrorCodeBAD_REQUEST,
				errMsg:  "check continuation string",
			},
		},
	}
	for _, tt := range tests {
		tt := tt
		t.Run(tt.name, func(t *testing.T) {
			t.Parallel()
			re := require.New(t)

			h, closeFunc := startSbpHandler(t, nil, true)
			defer closeFunc()

			// prepare
			preHeartbeats(t, h, 0, 1, 2)
			streamIDs := preCreateStreams(t, h, 3, 1)
			var streamID int64
			re.Equal([]int64{streamID}, streamIDs)
			preNewRange(t, h, streamID, false)
			preNewObject(t, h, preObject{endOffset: 1})
			preNewObject(t, h, preObject{startOffset: 1, endOffset: 2})

			// list resource
			req := &protocol.ListResourceRequest{ListResourceRequestT: rpcfb.ListResourceRequestT{
				ResourceType: tt.args.types,
				Limit:        tt.args.limit,
				Continuation: tt.args.continuation,
			}}
			resp := &protocol.ListResourceResponse{}
			h.ListResource(req, resp)

			// check response
			if tt.want.wantErr {
				re.Equal(tt.want.errCode, resp.Status.Code)
				re.Contains(resp.Status.Message, tt.want.errMsg)
			} else {
				re.Len(resp.Resources, len(tt.want.resources))
				for i, resource := range resp.Resources {
					if resource.Type == rpcfb.ResourceTypeRESOURCE_RANGE {
						fmtRangeServers(resource.Range)
						fillRangeInfo(tt.want.resources[i].Range)
					}
					re.Equal(tt.want.resources[i], *resource)
				}
				re.Equal(tt.want.rv, resp.ResourceVersion)
				re.Equal(tt.want.continuation, resp.Continuation)
			}
		})
	}
}

func TestHandler_WatchResource(t *testing.T) {
	type args struct {
		types []rpcfb.ResourceType
		rv    int64
	}
	type want struct {
		events []rpcfb.ResourceEventT

		wantErr bool
		errCode rpcfb.ErrorCode
		errMsg  string
	}
	tests := []struct {
		name string
		args args
		want want
	}{
		{
			name: "normal case",
			args: args{
				types: []rpcfb.ResourceType{rpcfb.ResourceTypeRESOURCE_RANGE_SERVER, rpcfb.ResourceTypeRESOURCE_STREAM, rpcfb.ResourceTypeRESOURCE_RANGE, rpcfb.ResourceTypeRESOURCE_OBJECT},
			},
			want: want{
				events: []rpcfb.ResourceEventT{
					{Type: rpcfb.EventTypeEVENT_ADDED, Resource: &rpcfb.ResourceT{Type: rpcfb.ResourceTypeRESOURCE_RANGE_SERVER, RangeServer: &rpcfb.RangeServerT{ServerId: 0, AdvertiseAddr: "addr-0", State: rpcfb.RangeServerStateRANGE_SERVER_STATE_READ_WRITE}}},
					{Type: rpcfb.EventTypeEVENT_ADDED, Resource: &rpcfb.ResourceT{Type: rpcfb.ResourceTypeRESOURCE_RANGE_SERVER, RangeServer: &rpcfb.RangeServerT{ServerId: 1, AdvertiseAddr: "addr-1", State: rpcfb.RangeServerStateRANGE_SERVER_STATE_READ_WRITE}}},
					{Type: rpcfb.EventTypeEVENT_ADDED, Resource: &rpcfb.ResourceT{Type: rpcfb.ResourceTypeRESOURCE_RANGE_SERVER, RangeServer: &rpcfb.RangeServerT{ServerId: 2, AdvertiseAddr: "addr-2", State: rpcfb.RangeServerStateRANGE_SERVER_STATE_READ_WRITE}}},
					{Type: rpcfb.EventTypeEVENT_ADDED, Resource: &rpcfb.ResourceT{Type: rpcfb.ResourceTypeRESOURCE_STREAM, Stream: &rpcfb.StreamT{Replica: 3, AckCount: 3}}},
					{Type: rpcfb.EventTypeEVENT_MODIFIED, Resource: &rpcfb.ResourceT{Type: rpcfb.ResourceTypeRESOURCE_STREAM, Stream: &rpcfb.StreamT{Replica: 3, AckCount: 3, Epoch: 1}}},
					{Type: rpcfb.EventTypeEVENT_ADDED, Resource: &rpcfb.ResourceT{Type: rpcfb.ResourceTypeRESOURCE_RANGE, Range: &rpcfb.RangeT{Epoch: 1, End: -1}}},
					{Type: rpcfb.EventTypeEVENT_ADDED, Resource: &rpcfb.ResourceT{Type: rpcfb.ResourceTypeRESOURCE_OBJECT, Object: &rpcfb.ObjT{EndOffsetDelta: 1}}},
					{Type: rpcfb.EventTypeEVENT_ADDED, Resource: &rpcfb.ResourceT{Type: rpcfb.ResourceTypeRESOURCE_OBJECT, Object: &rpcfb.ObjT{StartOffset: 1, EndOffsetDelta: 1}}},
				},
			},
		},
		{
			name: "specify resource version",
			args: args{
				types: []rpcfb.ResourceType{rpcfb.ResourceTypeRESOURCE_RANGE_SERVER, rpcfb.ResourceTypeRESOURCE_STREAM, rpcfb.ResourceTypeRESOURCE_RANGE, rpcfb.ResourceTypeRESOURCE_OBJECT},
				rv:    4,
			},
			want: want{
				events: []rpcfb.ResourceEventT{
					{Type: rpcfb.EventTypeEVENT_ADDED, Resource: &rpcfb.ResourceT{Type: rpcfb.ResourceTypeRESOURCE_STREAM, Stream: &rpcfb.StreamT{Replica: 3, AckCount: 3}}},
					{Type: rpcfb.EventTypeEVENT_MODIFIED, Resource: &rpcfb.ResourceT{Type: rpcfb.ResourceTypeRESOURCE_STREAM, Stream: &rpcfb.StreamT{Replica: 3, AckCount: 3, Epoch: 1}}},
					{Type: rpcfb.EventTypeEVENT_ADDED, Resource: &rpcfb.ResourceT{Type: rpcfb.ResourceTypeRESOURCE_RANGE, Range: &rpcfb.RangeT{Epoch: 1, End: -1}}},
					{Type: rpcfb.EventTypeEVENT_ADDED, Resource: &rpcfb.ResourceT{Type: rpcfb.ResourceTypeRESOURCE_OBJECT, Object: &rpcfb.ObjT{EndOffsetDelta: 1}}},
					{Type: rpcfb.EventTypeEVENT_ADDED, Resource: &rpcfb.ResourceT{Type: rpcfb.ResourceTypeRESOURCE_OBJECT, Object: &rpcfb.ObjT{StartOffset: 1, EndOffsetDelta: 1}}},
				},
			},
		},
		{
			name: "empty resource type",
			args: args{
				types: []rpcfb.ResourceType{},
			},
			want: want{
				wantErr: true,
				errCode: rpcfb.ErrorCodeBAD_REQUEST,
				errMsg:  "resource type is empty",
			},
		},
		{
			name: "unknown resource type",
			args: args{
				types: []rpcfb.ResourceType{rpcfb.ResourceTypeRESOURCE_UNKNOWN, rpcfb.ResourceTypeRESOURCE_RANGE_SERVER},
			},
			want: want{
				wantErr: true,
				errCode: rpcfb.ErrorCodeBAD_REQUEST,
				errMsg:  "invalid type RESOURCE_UNKNOWN",
			},
		},
	}
	for _, tt := range tests {
		tt := tt
		t.Run(tt.name, func(t *testing.T) {
			t.Parallel()
			re := require.New(t)

			h, closeFunc := startSbpHandler(t, nil, true)
			defer closeFunc()

			// prepare
			preHeartbeats(t, h, 0, 1, 2)
			streamIDs := preCreateStreams(t, h, 3, 1)
			var streamID int64
			re.Equal([]int64{streamID}, streamIDs)
			preNewRange(t, h, streamID, false)
			preNewObject(t, h, preObject{endOffset: 1})
			preNewObject(t, h, preObject{startOffset: 1, endOffset: 2})

			// watch resource
			req := &protocol.WatchResourceRequest{WatchResourceRequestT: rpcfb.WatchResourceRequestT{
				ResourceType:    tt.args.types,
				ResourceVersion: tt.args.rv,
			}}
			resp := &protocol.WatchResourceResponse{}
			h.WatchResource(req, resp)

			// check response
			if tt.want.wantErr {
				re.Equal(tt.want.errCode, resp.Status.Code)
				re.Contains(resp.Status.Message, tt.want.errMsg)
			} else {
				re.Len(resp.Events, len(tt.want.events))
				for i, event := range resp.Events {
					re.Equal(tt.want.events[i].Type, event.Type)
					if event.Resource.Type == rpcfb.ResourceTypeRESOURCE_RANGE {
						fmtRangeServers(event.Resource.Range)
						fillRangeInfo(tt.want.events[i].Resource.Range)
					}
					re.Equal(tt.want.events[i].Resource, event.Resource)
				}
			}
		})
	}
}
