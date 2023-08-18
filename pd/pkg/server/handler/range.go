package handler

import (
	"fmt"

	"github.com/pkg/errors"

	"github.com/AutoMQ/pd/api/rpcfb/rpcfb"
	"github.com/AutoMQ/pd/pkg/sbp/protocol"
	"github.com/AutoMQ/pd/pkg/server/model"
)

func (h *Handler) ListRange(req *protocol.ListRangeRequest, resp *protocol.ListRangeResponse) {
	ctx := req.Context()

	if req.Criteria == nil {
		resp.Error(&rpcfb.StatusT{Code: rpcfb.ErrorCodeBAD_REQUEST, Message: "criteria is nil"})
		return
	}

	ranges, err := h.c.ListRange(ctx, req.Criteria)
	if err != nil {
		switch {
		case errors.Is(err, model.ErrPDNotLeader):
			resp.Error(h.notLeaderError(ctx))
		default:
			resp.Error(&rpcfb.StatusT{Code: rpcfb.ErrorCodePD_INTERNAL_SERVER_ERROR, Message: err.Error()})
		}
		return
	}

	resp.Ranges = ranges
	resp.OK()
}

func (h *Handler) SealRange(req *protocol.SealRangeRequest, resp *protocol.SealRangeResponse) {
	ctx := req.Context()

	if req.Kind != rpcfb.SealKindPLACEMENT_DRIVER {
		resp.Error(&rpcfb.StatusT{Code: rpcfb.ErrorCodeBAD_REQUEST, Message: fmt.Sprintf("invalid seal kind: %s", req.Kind)})
		return
	}
	param, err := model.NewSealRangeParam(req.Range)
	if err != nil {
		resp.Error(&rpcfb.StatusT{Code: rpcfb.ErrorCodeBAD_REQUEST, Message: err.Error()})
		return
	}

	r, err := h.c.SealRange(ctx, param)
	if err != nil {
		switch {
		case errors.Is(err, model.ErrPDNotLeader):
			resp.Error(h.notLeaderError(ctx))
		case errors.Is(err, model.ErrStreamNotFound):
			resp.Error(&rpcfb.StatusT{Code: rpcfb.ErrorCodeNOT_FOUND, Message: err.Error()})
		case errors.Is(err, model.ErrRangeNotFound):
			resp.Error(&rpcfb.StatusT{Code: rpcfb.ErrorCodeRANGE_NOT_FOUND, Message: err.Error()})
		case errors.Is(err, model.ErrSealRangeTwice):
			resp.Range = r
			resp.OK()
		case errors.Is(err, model.ErrRangeAlreadySealed):
			resp.Error(&rpcfb.StatusT{Code: rpcfb.ErrorCodeBAD_REQUEST, Message: err.Error()})
		case errors.Is(err, model.ErrInvalidRangeEnd):
			resp.Error(&rpcfb.StatusT{Code: rpcfb.ErrorCodeBAD_REQUEST, Message: err.Error()})
		case errors.Is(err, model.ErrInvalidStreamEpoch):
			resp.Error(&rpcfb.StatusT{Code: rpcfb.ErrorCodeEXPIRED_STREAM_EPOCH, Message: err.Error()})
		default:
			resp.Error(&rpcfb.StatusT{Code: rpcfb.ErrorCodePD_INTERNAL_SERVER_ERROR, Message: err.Error()})
		}
		return
	}

	resp.Range = r
	resp.OK()
}

func (h *Handler) CreateRange(req *protocol.CreateRangeRequest, resp *protocol.CreateRangeResponse) {
	ctx := req.Context()

	param, err := model.NewCreateRangeParam(req.Range)
	if err != nil {
		resp.Error(&rpcfb.StatusT{Code: rpcfb.ErrorCodeBAD_REQUEST, Message: err.Error()})
		return
	}

	r, err := h.c.CreateRange(ctx, param)
	if err != nil {
		switch {
		case errors.Is(err, model.ErrPDNotLeader):
			resp.Error(h.notLeaderError(ctx))
		case errors.Is(err, model.ErrStreamNotFound):
			resp.Error(&rpcfb.StatusT{Code: rpcfb.ErrorCodeNOT_FOUND, Message: err.Error()})
		case errors.Is(err, model.ErrInvalidStreamEpoch):
			resp.Error(&rpcfb.StatusT{Code: rpcfb.ErrorCodeEXPIRED_STREAM_EPOCH, Message: err.Error()})
		case errors.Is(err, model.ErrCreateRangeTwice):
			resp.Range = r
			resp.OK()
		case errors.Is(err, model.ErrRangeAlreadyExist):
			resp.Error(&rpcfb.StatusT{Code: rpcfb.ErrorCodeBAD_REQUEST, Message: err.Error()})
		case errors.Is(err, model.ErrInvalidRangeIndex):
			resp.Error(&rpcfb.StatusT{Code: rpcfb.ErrorCodeBAD_REQUEST, Message: err.Error()})
		case errors.Is(err, model.ErrCreateRangeBeforeSeal):
			resp.Error(&rpcfb.StatusT{Code: rpcfb.ErrorCodeCREATE_RANGE_BEFORE_SEAL, Message: err.Error()})
		case errors.Is(err, model.ErrInvalidRangeStart):
			resp.Error(&rpcfb.StatusT{Code: rpcfb.ErrorCodeBAD_REQUEST, Message: err.Error()})
		case errors.Is(err, model.ErrNotEnoughRangeServers):
			resp.Error(&rpcfb.StatusT{Code: rpcfb.ErrorCodePD_NO_AVAILABLE_RS, Message: err.Error()})
		default:
			resp.Error(&rpcfb.StatusT{Code: rpcfb.ErrorCodePD_INTERNAL_SERVER_ERROR, Message: err.Error()})
		}
		return
	}

	resp.Range = r
	resp.OK()
}
