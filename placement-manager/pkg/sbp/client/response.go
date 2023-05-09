package client

import (
	"github.com/AutoMQ/placement-manager/pkg/sbp/codec/operation"
	"github.com/AutoMQ/placement-manager/pkg/sbp/protocol"
)

// newInResponse returns a new empty response for the given operation.
// It returns nil if the operation is not supported.
//
//nolint:gocritic
func newInResponse(op operation.Operation) protocol.InResponse {
	switch op.Code {
	default:
		return nil
	}
}
