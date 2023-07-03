package cluster

import (
	"context"

	"github.com/pkg/errors"
	"go.uber.org/zap"

	"github.com/AutoMQ/pd/api/rpcfb/rpcfb"
	"github.com/AutoMQ/pd/pkg/server/storage/endpoint"
	"github.com/AutoMQ/pd/pkg/server/storage/kv"
	"github.com/AutoMQ/pd/pkg/util/traceutil"
)

const (
	_writableRangeEnd int64 = -1
)

var (
	// ErrRangeNotFound is returned when the specified range is not found.
	ErrRangeNotFound = errors.New("range not found")
	// ErrExpiredRangeEpoch is returned when the range epoch is invalid.
	ErrExpiredRangeEpoch = errors.New("expired range epoch")

	// ErrRangeAlreadySealed is returned when the specified range is already sealed.
	ErrRangeAlreadySealed = errors.New("range already sealed")
	// ErrInvalidEndOffset is returned when the end offset is invalid.
	ErrInvalidEndOffset = errors.New("invalid end offset")

	// ErrInvalidRangeIndex is returned when the range index is invalid.
	ErrInvalidRangeIndex = errors.New("invalid range index")
	// ErrCreateBeforeSeal is returned when the last range is not sealed.
	ErrCreateBeforeSeal = errors.New("create range before sealing the previous one")
	// ErrInvalidStartOffset is returned when the end offset is invalid.
	ErrInvalidStartOffset = errors.New("invalid start offset")
)

type Range interface {
	ListRange(ctx context.Context, criteria *rpcfb.ListRangeCriteriaT) (ranges []*rpcfb.RangeT, err error)
	SealRange(ctx context.Context, r *rpcfb.RangeT) (*rpcfb.RangeT, error)
	CreateRange(ctx context.Context, r *rpcfb.RangeT) (*rpcfb.RangeT, error)
}

// ListRange lists ranges of
// 1. a stream
// 2. a range server
// 3. a range server and a stream
// It returns ErrNotLeader if the current PD node is not the leader.
func (c *RaftCluster) ListRange(ctx context.Context, criteria *rpcfb.ListRangeCriteriaT) (ranges []*rpcfb.RangeT, err error) {
	byStream := criteria.StreamId >= endpoint.MinStreamID
	byRangeServer := criteria.ServerId >= endpoint.MinRangeServerID
	switch {
	case byStream && byRangeServer:
		ranges, err = c.listRangeOnRangeServerInStream(ctx, criteria.StreamId, criteria.ServerId)
	case byStream && !byRangeServer:
		ranges, err = c.listRangeInStream(ctx, criteria.StreamId)
	case !byStream && byRangeServer:
		ranges, err = c.listRangeOnRangeServer(ctx, criteria.ServerId)
	default:
	}
	if errors.Is(err, kv.ErrTxnFailed) {
		err = ErrNotLeader
	}

	for _, r := range ranges {
		c.fillRangeServersInfo(r.Servers)
	}
	return
}

// listRangeOnRangeServerInStream lists ranges on a range server in a stream.
func (c *RaftCluster) listRangeOnRangeServerInStream(ctx context.Context, streamID int64, rangeServerID int32) ([]*rpcfb.RangeT, error) {
	logger := c.lg.With(zap.Int64("stream-id", streamID), zap.Int32("range-server-id", rangeServerID), traceutil.TraceLogField(ctx))

	logger.Debug("start to list ranges on range server in stream")
	rangeIDs, err := c.storage.GetRangeIDsByRangeServerAndStream(ctx, streamID, rangeServerID)
	if err != nil {
		return nil, err
	}

	ranges, err := c.storage.GetRanges(ctx, rangeIDs)
	if err != nil {
		return nil, err
	}

	logger.Debug("finish listing ranges on range server in stream", zap.Int("range-cnt", len(ranges)))
	return ranges, nil
}

// listRangeInStream lists ranges in a stream.
func (c *RaftCluster) listRangeInStream(ctx context.Context, streamID int64) ([]*rpcfb.RangeT, error) {
	logger := c.lg.With(zap.Int64("stream-id", streamID), traceutil.TraceLogField(ctx))

	logger.Debug("start to list ranges in stream")
	ranges, err := c.storage.GetRangesByStream(ctx, streamID)
	logger.Debug("finish listing ranges in stream", zap.Int("range-cnt", len(ranges)), zap.Error(err))

	return ranges, err
}

// listRangeOnRangeServer lists ranges on a range server.
func (c *RaftCluster) listRangeOnRangeServer(ctx context.Context, rangeServerID int32) ([]*rpcfb.RangeT, error) {
	logger := c.lg.With(zap.Int32("range-server-id", rangeServerID), traceutil.TraceLogField(ctx))

	logger.Debug("start to list ranges on range server")
	rangeIDs, err := c.storage.GetRangeIDsByRangeServer(ctx, rangeServerID)
	if err != nil {
		return nil, err
	}

	ranges, err := c.storage.GetRanges(ctx, rangeIDs)
	if err != nil {
		return nil, err
	}

	logger.Debug("finish listing ranges on range server", zap.Int("range-cnt", len(ranges)))
	return ranges, nil
}

// SealRange seals a range. It returns the sealed range if success.
// It returns ErrNotLeader if the current PD node is not the leader.
// It returns ErrStreamNotFound if the stream does not exist.
// It returns ErrRangeNotFound if the range does not exist.
// It returns ErrRangeAlreadySealed if the range is already sealed.
// It returns ErrInvalidEndOffset if the end offset is invalid.
// It returns ErrExpiredRangeEpoch if the range epoch is invalid.
func (c *RaftCluster) SealRange(ctx context.Context, r *rpcfb.RangeT) (*rpcfb.RangeT, error) {
	logger := c.lg.With(zap.Int64("stream-id", r.StreamId), zap.Int32("range-index", r.Index), traceutil.TraceLogField(ctx))
	lastRange, err := c.getLastRange(ctx, r.StreamId)
	if err != nil {
		logger.Error("failed to get last range", zap.Error(err))
		return nil, err
	}

	if lastRange == nil {
		// No range in the stream.
		logger.Error("no range in the stream")
		return nil, errors.Wrapf(ErrRangeNotFound, "no range in stream %d", r.StreamId)
	}
	if r.Index > lastRange.Index {
		// Range not found.
		logger.Error("range not found", zap.Int32("last-range-index", lastRange.Index))
		return nil, errors.Wrapf(ErrRangeNotFound, "range %d not found in stream %d", r.Index, r.StreamId)
	}
	if r.Index < lastRange.Index || !isWritable(lastRange) {
		// Range already sealed.
		logger.Error("range already sealed", zap.Int32("last-range-index", lastRange.Index))
		return nil, errors.Wrapf(ErrRangeAlreadySealed, "range %d already sealed in stream %d", r.Index, r.StreamId)
	}
	if r.End < lastRange.Start {
		logger.Error("invalid end offset", zap.Int64("end", r.End), zap.Int64("start", lastRange.Start))
		return nil, errors.Wrapf(ErrInvalidEndOffset, "invalid end offset %d (less than start offset %d) for range %d in stream %d",
			r.End, lastRange.Start, lastRange.Index, lastRange.StreamId)
	}
	if r.Epoch < lastRange.Epoch {
		logger.Error("invalid epoch", zap.Int64("epoch", r.Epoch), zap.Int64("last-epoch", lastRange.Epoch))
		return nil, errors.Wrapf(ErrExpiredRangeEpoch, "invalid epoch %d (less than %d) for range %d in stream %d",
			r.Epoch, lastRange.Epoch, lastRange.Index, lastRange.StreamId)
	}

	mu := c.sealMu(r.StreamId)
	select {
	case mu <- struct{}{}:
	case <-ctx.Done():
		logger.Warn("timeout to acquire stream lock")
		return nil, ctx.Err()
	}
	defer func() {
		<-mu
	}()

	sealedRange, err := c.sealRangeLocked(ctx, lastRange, r.End, r.Epoch)
	if err != nil {
		return nil, err
	}
	c.fillRangeServersInfo(sealedRange.Servers)
	return sealedRange, nil
}

// CreateRange creates a range. It returns the created range if success.
// It returns ErrNotLeader if the current PD node is not the leader.
// It returns ErrStreamNotFound if the stream does not exist.
// It returns ErrInvalidRangeIndex if the range index is invalid.
// It returns ErrCreateBeforeSeal if the last range is not sealed.
// It returns ErrInvalidStartOffset if the start offset is invalid.
// It returns ErrNotEnoughRangeServers if there are not enough range servers to allocate.
// It returns ErrExpiredRangeEpoch if the range epoch is invalid.
func (c *RaftCluster) CreateRange(ctx context.Context, r *rpcfb.RangeT) (*rpcfb.RangeT, error) {
	logger := c.lg.With(zap.Int64("stream-id", r.StreamId), zap.Int32("range-index", r.Index), traceutil.TraceLogField(ctx))
	lastRange, err := c.getLastRange(ctx, r.StreamId)
	if err != nil {
		logger.Error("failed to get last range", zap.Error(err))
		return nil, err
	}

	if lastRange == nil {
		// The stream exists, but no range in it.
		// Mock it for the following check.
		lastRange = &rpcfb.RangeT{StreamId: r.StreamId, Index: -1}
	}
	if isWritable(lastRange) {
		// The last range is writable.
		logger.Error("create range before sealing the last range", zap.Int32("last-range-index", lastRange.Index))
		return nil, errors.Wrapf(ErrCreateBeforeSeal, "create range %d before sealing the last range %d in stream %d", r.Index, lastRange.Index, r.StreamId)
	}
	if r.Index != lastRange.Index+1 {
		// The range index is not continuous.
		logger.Error("invalid range index", zap.Int32("last-range-index", lastRange.Index))
		return nil, errors.Wrapf(ErrInvalidRangeIndex, "invalid range index %d (should be %d) in stream %d", r.Index, lastRange.Index+1, r.StreamId)
	}
	if r.Start != lastRange.End {
		// The range start is not continuous.
		logger.Error("invalid range start", zap.Int64("start", r.Start), zap.Int64("end", lastRange.End))
		return nil, errors.Wrapf(ErrInvalidStartOffset, "invalid range start %d (should be %d) for range %d in stream %d", r.Start, lastRange.End, r.Index, r.StreamId)
	}
	if r.Epoch < lastRange.Epoch {
		// The range epoch is invalid.
		logger.Error("invalid range epoch", zap.Int64("epoch", r.Epoch), zap.Int64("last-epoch", lastRange.Epoch))
		return nil, errors.Wrapf(ErrExpiredRangeEpoch, "invalid range epoch %d (less than %d) for range %d in stream %d", r.Epoch, lastRange.Epoch, r.Index, r.StreamId)
	}

	mu := c.sealMu(r.StreamId)
	select {
	case mu <- struct{}{}:
	case <-ctx.Done():
		logger.Warn("timeout to acquire stream lock")
		return nil, ctx.Err()
	}
	defer func() {
		<-mu
	}()

	newRange, err := c.newRangeLocked(ctx, r)
	if err != nil {
		return nil, err
	}
	c.fillRangeServersInfo(newRange.Servers)
	return newRange, nil
}

func (c *RaftCluster) getLastRange(ctx context.Context, streamID int64) (*rpcfb.RangeT, error) {
	r, err := c.storage.GetLastRange(ctx, streamID)
	if err != nil {
		if errors.Is(err, kv.ErrTxnFailed) {
			err = ErrNotLeader
		}
		return nil, err
	}
	if r == nil {
		stream, err := c.storage.GetStream(ctx, streamID)
		if err != nil {
			if errors.Is(err, kv.ErrTxnFailed) {
				err = ErrNotLeader
			}
			return nil, err
		}
		if stream == nil {
			return nil, errors.Wrapf(ErrStreamNotFound, "stream %d not found", streamID)
		}
		return nil, nil
	}
	return r, nil
}

func (c *RaftCluster) sealMu(streamID int64) chan struct{} {
	sealMu := c.sealMus.Upsert(streamID, nil, func(exist bool, valueInMap, _ chan struct{}) chan struct{} {
		if exist {
			return valueInMap
		}
		return make(chan struct{}, 1)
	})
	return sealMu
}

// sealRangeLocked seals a range and saves it to the storage.
// It returns the sealed range if the range is sealed successfully.
// It should be called with the stream lock held.
func (c *RaftCluster) sealRangeLocked(ctx context.Context, lastRange *rpcfb.RangeT, end int64, epoch int64) (*rpcfb.RangeT, error) {
	logger := c.lg.With(zap.Int64("stream-id", lastRange.StreamId), zap.Int32("range-index", lastRange.Index), zap.Int64("end", end), traceutil.TraceLogField(ctx))

	sealedRange := &rpcfb.RangeT{
		StreamId:     lastRange.StreamId,
		Epoch:        epoch,
		Index:        lastRange.Index,
		Start:        lastRange.Start,
		End:          end,
		Servers:      eraseRangeServersInfo(lastRange.Servers),
		ReplicaCount: lastRange.ReplicaCount,
		AckCount:     lastRange.AckCount,
	}

	logger.Info("start to seal range")
	_, err := c.storage.UpdateRange(ctx, sealedRange)
	logger.Info("finish sealing range", zap.Error(err))
	if err != nil {
		return nil, err
	}

	return sealedRange, nil
}

// newRangeLocked creates a new range and saves it to the storage.
// It returns the new range if the range is created successfully.
// It should be called with the stream lock held.
func (c *RaftCluster) newRangeLocked(ctx context.Context, newRange *rpcfb.RangeT) (*rpcfb.RangeT, error) {
	logger := c.lg.With(zap.Int64("stream-id", newRange.StreamId), zap.Int32("range-index", newRange.Index), zap.Int64("start", newRange.Start), traceutil.TraceLogField(ctx))

	stream, err := c.storage.GetStream(ctx, newRange.StreamId)
	if err != nil {
		return nil, err
	}
	if stream == nil {
		return nil, errors.Wrapf(ErrStreamNotFound, "stream %d not found", newRange.StreamId)
	}

	servers, err := c.chooseRangeServers(int(stream.Replica))
	if err != nil {
		logger.Error("failed to choose range servers", zap.Error(err))
		return nil, err
	}
	ids := make([]int32, 0, len(servers))
	for _, rs := range servers {
		ids = append(ids, rs.ServerId)
	}
	logger = logger.With(zap.Int32s("server-ids", ids))

	nr := &rpcfb.RangeT{
		StreamId:     newRange.StreamId,
		Epoch:        newRange.Epoch,
		Index:        newRange.Index,
		Start:        newRange.Start,
		End:          _writableRangeEnd,
		Servers:      servers,
		ReplicaCount: stream.Replica,
		AckCount:     stream.AckCount,
	}

	logger.Info("start to create range")
	err = c.storage.CreateRange(ctx, nr)
	logger.Info("finish creating range", zap.Error(err))
	if err != nil {
		return nil, err
	}

	return nr, nil
}

func isWritable(r *rpcfb.RangeT) bool {
	return r.End == _writableRangeEnd
}
